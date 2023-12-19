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

#[derive(Live, LiveHook, Widget)]
pub struct Separator {
    #[deref]
    view: View,

    #[live]
    animate_at: f64,

    #[animator]
    animator: Animator,
}

impl Widget for Separator {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        if self.animator_handle_event(cx, event).must_redraw() {
            self.redraw(cx);
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
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

impl SeparatorRef {
    pub fn update_animation(&mut self, cx: &mut Cx, scroll: f64) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.update_animation(cx, scroll);
        }
    }
}
