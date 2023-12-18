use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import makepad_draw::shader::std::*;

    import crate::shared::styles::*;
    import crate::shared::widgets::*;

    GalleryOverlay = {{GalleryOverlay}} {
        abs_pos: vec2(0, 0)
        flow: Overlay
        width: Fill
        height: Fill

        show_bg: true
        draw_bg: {
            instance radius: 3.;
            instance crop_width: 250.;
            instance crop_height: 400.;
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                sdf.box(
                    -2.0,
                    -2.0,
                    self.rect_size.x + 2,
                    self.rect_size.y + 2,
                    1.0
                );

                sdf.box(
                    (self.rect_size.x - self.crop_width) / 2.0 + 2,
                    (self.rect_size.y - self.crop_height) / 2.0 + 2,
                    self.crop_width - 4,
                    self.crop_height - 4,
                    max(1.0, self.radius)
                );

                sdf.subtract();
                sdf.fill_keep(vec4(0.0, 0.0, 0.0, 0.6));

                return sdf.result;
            }
        }

        animator: {
            shrink_horizontally = {
                default: off
                on = {
                    from: {all: Forward {duration: 0.1}}
                    apply: {
                        draw_bg: {crop_width: 170}
                    }
                }
                off = {
                    from: {all: Forward {duration: 0.1}}
                    apply: {
                        draw_bg: {crop_width: 250}
                    }
                }
            }
            shrink_vertically = {
                default: off
                on = {
                    from: {all: Forward {duration: 0.1}}
                    apply: {
                        draw_bg: {crop_height: 320}
                    }
                }
                off = {
                    from: {all: Forward {duration: 0.1}}
                    apply: {
                        draw_bg: {crop_height: 400}
                    }
                }
            }
        }
    }
}

#[derive(Live)]
pub struct GalleryOverlay {
    #[deref]
    view: View,
    #[walk]
    walk: Walk,
    #[layout]
    layout: Layout,
    #[animator]
    animator: Animator,
    #[rust]
    ready_to_swipe: bool,
}

impl LiveHook for GalleryOverlay {
    fn before_live_design(cx: &mut Cx) {
        register_widget!(cx, GalleryOverlay);
    }

    fn after_new_from_doc(&mut self, cx: &mut Cx) {
        self.ready_to_swipe = true;
    }
}

impl Widget for GalleryOverlay {
    fn handle_widget_event_with(
        &mut self,
        cx: &mut Cx,
        event: &Event,
        dispatch_action: &mut dyn FnMut(&mut Cx, WidgetActionItem),
    ) {
        let actions = self.view.handle_widget_event(cx, event);

        for action in actions.into_iter() {
            dispatch_action(cx, action);
        }

        match event.hits(cx, self.view.area()) {
            Hit::FingerMove(fe) => {
                if !self.ready_to_swipe {
                    return;
                }

                let mut swipe_vector = fe.abs - fe.abs_start;
                // Negate y values because makepad's y axis grows to the south
                swipe_vector.y = -swipe_vector.y;

                // only trigger swipe if it is larger than some pixels
                let swipe_trigger_value = 60.;
                let diagonal_trigger_value = swipe_trigger_value / 2.;
                if (swipe_vector.x.abs() > swipe_trigger_value)
                    || (swipe_vector.y.abs() > swipe_trigger_value)
                {
                    // compensate diagonal swipe case (both trigger the diagonal value)
                    if swipe_vector.x.abs() > diagonal_trigger_value {
                        // play animations (shrink overlay)
                        self.animator_play(cx, id!(shrink_horizontally.on));
                    }
                    if swipe_vector.y.abs() > diagonal_trigger_value {
                        self.animator_play(cx, id!(shrink_vertically.on));
                        // play animations (shrink overlay)
                    }

                    self.ready_to_swipe = false;
                }
            }
            Hit::FingerUp(_fe) => self.ready_to_swipe = true,
            _ => {}
        }
        self.animator_handle_event(cx, event);
        self.update_animation(cx);
    }

    fn walk(&mut self, cx: &mut Cx) -> Walk {
        self.view.walk(cx)
    }

    fn redraw(&mut self, cx: &mut Cx) {
        self.view.redraw(cx);
    }

    fn draw_walk_widget(&mut self, cx: &mut Cx2d, walk: Walk) -> WidgetDraw {
        let _ = self.view.draw_walk_widget(cx, walk);
        self.draw_walk(cx, walk);
        WidgetDraw::done()
    }
}

impl GalleryOverlay {
    fn update_animation(&mut self, cx: &mut Cx) {
        if self.animator.is_track_animating(cx, id!(shrink_vertically))
            || self
                .animator
                .is_track_animating(cx, id!(shrink_horizontally))
        {
            return;
        }

        if self
            .animator
            .animator_in_state(cx, id!(shrink_horizontally.on))
        {
            self.animator_play(cx, id!(shrink_horizontally.off));
        }

        if self
            .animator
            .animator_in_state(cx, id!(shrink_vertically.on))
        {
            self.animator_play(cx, id!(shrink_vertically.off));
        }
    }
}
