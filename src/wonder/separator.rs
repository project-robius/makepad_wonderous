use makepad_widgets::widget::WidgetCache;
use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import makepad_draw::shader::std::*;

    import crate::shared::styles::*;
    import crate::shared::curved_label::*;
    import crate::shared::widgets::*;

    IMG_COMPASS = dep("crate://self/resources/images/compass-icon.png")

    Separator = {{Separator}} {
        width: Fill,
        height: Fit,

        align: { x: 0.5, y: 0.5 }
        spacing: 80.0
        margin: {left: 80.0, right: 80.0}

        <Line> {
            draw_bg: {
                color: #ccc
            }
        }
        compass = <RotatedImage> {
            width: 30,
            height: 30,
            source: (IMG_COMPASS),

            draw_bg: {
                instance rotation: 0.0

                // Overriden version of this function to tint the white image
                fn get_color(self) -> vec4 {
                    let rot_padding = rotation_vertex_expansion(self.rotation, self.rect_size.x, self.rect_size.y) / 2.0;
                    
                    // Current position is a traslated one, so let's get the original position
                    let current_pos = self.pos.xy - rot_padding;
                    let original_pos = rotate_2d_from_center(current_pos, self.rotation, self.rect_size);
                    
                    // Scale the current position by the scale factor
                    let scaled_pos = original_pos / self.scale;
                    
                    // Take pixel color from the original image
                    let color = sample2d(self.image, scaled_pos).xyzw

                    // Tint the white image
                    if color.w > 0.01 {
                        color = mix(
                            #666,
                            sample2d(self.image, scaled_pos).xyzw,
                            0.5
                        );
                    }
                    
                    let faded_color = color * vec4(1.0, 1.0, 1.0, self.opacity);
                    return faded_color;
                }
            }
        }
        <Line> {
            draw_bg: {
                color: #ccc
            }
        }

        animate_at: 1000.0

        animator: {
            separator = {
                default: hide,
                hide = {
                    redraw: true,
                    ease: OutBack,
                    from: {all: Forward {duration: 2.0}}
                    apply: {
                        spacing: 80.0,
                        margin: {left: 80.0, right: 80.0},
                        compass = { draw_bg: { rotation: 0.0 } }
                    }
                }
                show = {
                    redraw: true,
                    ease: OutBack,
                    from: {all: Forward {duration: 2.0}}
                    apply: {
                        spacing: 15.0,
                        margin: {left: 15.0, right: 15.0},
                        compass = { draw_bg: { rotation: -3.0 } }
                    }
                }
            }
        }
    }
}

#[derive(Live)]
pub struct Separator {
    #[deref]
    view: View,

    #[live]
    animate_at: f64,

    #[animator]
    animator: Animator,
}

impl LiveHook for Separator {
    fn before_live_design(cx: &mut Cx) {
        register_widget!(cx, Separator);
    }
}

impl Widget for Separator {
    fn handle_widget_event_with(
        &mut self,
        cx: &mut Cx,
        event: &Event,
        dispatch_action: &mut dyn FnMut(&mut Cx, WidgetActionItem),
    ) {
        self.view.handle_widget_event_with(cx, event, dispatch_action);

        let animatation_state = self.animator_handle_event(cx, event);
        
        if animatation_state.must_redraw() {
            self.redraw(cx);
        }
    }

    fn walk(&mut self, cx: &mut Cx) -> Walk {
        self.view.walk(cx)
    }

    fn redraw(&mut self, cx: &mut Cx) {
        self.view.redraw(cx);
    }

    fn find_widgets(&mut self, path: &[LiveId], cached: WidgetCache, results: &mut WidgetSet) {
        self.view.find_widgets(path, cached, results);
    }

    fn draw_walk_widget(&mut self, cx: &mut Cx2d, walk: Walk) -> WidgetDraw {
        let _ = self.view.draw_walk_widget(cx, walk);
        WidgetDraw::done()
    }
}

impl Separator {
    fn update_animation(&mut self, cx: &mut Cx, scroll: f64) {
        if self.animator.is_track_animating(cx, id!(separator)) {
            return;
        }
        if scroll > self.animate_at {
            self.animator_play(cx, id!(separator.show));
        } else {
            self.animator_play(cx, id!(separator.hide));
        }
    }
}

#[derive(Clone, PartialEq, WidgetRef)]
pub struct SeparatorRef(WidgetRef);

impl SeparatorRef {
    pub fn update_animation(&mut self, cx: &mut Cx, scroll: f64) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.update_animation(cx, scroll);
        }
    }
}
