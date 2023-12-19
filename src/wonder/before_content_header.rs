use makepad_widgets::widget::WidgetCache;
use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import makepad_draw::shader::std::*;

    import crate::shared::styles::*;
    import crate::shared::curved_label::*;
    import crate::shared::widgets::*;

    IMG_GREAT_WALL_CONTENT_1 = dep("crate://self/resources/images/great-wall-content-1.jpg")

    BeforeContentHeader = {{BeforeContentHeader}} {
        width: Fill,
        height: Fit,

        image = <CenteredOnTop> {
            source: (IMG_GREAT_WALL_CONTENT_1),
            width: Fill,
            height: 430,

            draw_bg: { opacity: 0.7 }
        }

        animator: {
            header = {
                default: hide,
                hide = {
                    redraw: true,
                    from: {all: Forward {duration: 0.5}}
                    apply: {
                        image = { draw_bg: { opacity: 0.0 } }
                    }
                }
                show = {
                    redraw: true,
                    from: {all: Forward {duration: 1.0}}
                    apply: {
                        image = { draw_bg: { opacity: 0.7 } }
                    }
                }
            },
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct BeforeContentHeader {
    #[deref]
    view: View,

    #[animator]
    animator: Animator,
}

impl Widget for BeforeContentHeader {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if self.animator_handle_event(cx, event).must_redraw() {
            self.redraw(cx);
        }
        self.view.handle_event(cx, event, scope)
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl BeforeContentHeader {
    fn show_and_update_values(&mut self, cx: &mut Cx, scale: f64, vertical_pan: f64, opacity: f64) {
        if self.animator.is_track_animating(cx, id!(header)) ||
                self.animator_in_state(cx, id!(header.show)) {
            self.image(id!(image)).apply_over(
                cx,
                live! {
                    draw_bg: {
                        image_scale: (dvec2(scale, scale)),
                        image_pan: (dvec2(0.0, vertical_pan)),
                        opacity: (opacity)
                    }
                },
            );
        } else {
            self.animator_play(cx, id!(header.show));

            self.image(id!(image)).apply_over(
                cx,
                live! {
                    draw_bg: {
                        image_scale: (dvec2(scale, scale)),
                        image_pan: (dvec2(0.0, vertical_pan)),
                    }
                },
            );
        } 
    }

    fn hide(&mut self, cx: &mut Cx) {
        self.animator_play(cx, id!(header.hide));
    }
}

impl BeforeContentHeaderRef {
    pub fn hide(&mut self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.hide(cx);
        }
    }

    pub fn show(&mut self, cx: &mut Cx, scale: f64, vertical_pan: f64, opacity: f64) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.show_and_update_values(cx, scale, vertical_pan, opacity);
        }
    }
}
