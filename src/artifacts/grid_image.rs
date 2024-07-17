use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import makepad_draw::shader::std::*;

    import crate::shared::styles::*;

    const PI = 3.14159
    const THICCNESS = 0.01
    const RADIUS = 0.1
    const SPEED = 3.8
    const SCALE = 1.0  // bigger scale = smaller spinner

    GridImage = {{GridImage}}<RoundedView> {
        image = <Image> {
            width: Fill,
            height: Fill
            min_width: 100,
            min_height: 350,
            fit: Horizontal,
            draw_bg: {
                instance hover: 0.0
                instance down: 0.0

                instance texture_is_ready: 0.0
                instance spinner_angle: 0.0
                instance opacity: 0.0

                fn pixel(self) -> vec4 {
                    if self.texture_is_ready > 0.5 {
                        let sdf = Sdf2d::viewport(self.pos * self.rect_size)
                        sdf.box(1, 1, self.rect_size.x - 2, self.rect_size.y - 2, 4.0)
                       
                        let max_scale = vec2(0.92);
                        let scale = mix(vec2(1.0), max_scale, self.hover);
                        let pan = mix(vec2(0.0), (vec2(1.0) - max_scale) * 0.5, self.hover);
                        
                        let color = self.get_color_scale_pan(scale, pan) + mix(vec4(0.0), vec4(0.1), self.down);
                        let final_color = Pal::premul(vec4(color.xyz, color.w * self.opacity));
                        sdf.fill_keep(final_color);

                        return sdf.result
                    }

                    // If the texture isn't ready, draw the spinner
                    // Spinner shader adapted from https://www.shadertoy.com/view/Xd3cR8

                    // Center and scale the UV coordinates
                    let uv = (self.pos - 0.5) * (SCALE);
                    let aspect = self.rect_size.x / self.rect_size.y;
                    uv.x *= aspect;

                    let geo = ring(uv, vec2(0.0), (RADIUS) - (THICCNESS), (RADIUS));

                    let rot = self.time * (SPEED);
                    let rot_mat = mat2(cos(rot), sin(rot), -sin(rot), cos(rot));
                    let uv_rot = uv * rot_mat;

                    let a = 1.0 - (atan(uv_rot.x, uv_rot.y) / (2.0 * PI) + 0.5);
                    // let a = atan(uv_rot.x, uv_rot.y) * PI * 0.05 + 0.5; // counter-clockwise, requires rot = -self.time * SPEED

                    let circle_val = 1.0 - smoothstep((THICCNESS) / 2.0, (THICCNESS) / 2.0 + 0.005,
                        length(uv_rot - vec2(0.0, -(RADIUS) + (THICCNESS) / 2.0)));
                    a = max(a, circle_val);

                    let color = vec4(a * geo);

                    return mix(#0f0e0c, #333535, color.x)
                }

                fn ring(uv: vec2, pos: vec2, inner_rad: float, outer_rad: float) -> float {
                    let dist = length(uv - pos);
                    return (1.0 - smoothstep(outer_rad, outer_rad + 0.005, dist)) *
                           smoothstep(inner_rad - 0.005, inner_rad, dist)
                }
            }
        }

        animator: {
            spinner = {
                default: spin
                spin = {
                    redraw: true,
                    from: {all: Loop {duration: 1.0, end: 1.0}}
                    apply: {
                        image = {
                            draw_bg: {time: 1.0}
                        }
                    }
                }
            }

            fade_in = {
                default: off
                off = {
                    from: {
                        ease: OutExp,
                        all: Forward {duration: 0.4}
                    }
                    apply: {
                        image = {draw_bg: {opacity: 0.0}}
                    }
                }
                on = {
                    from: {
                        ease: OutExp,
                        all: Forward {duration: 0.9}
                    }
                    apply: {
                        image = { draw_bg: {opacity: 1.0} }
                    }
                }
            }
        }
    }
}

#[derive(Live, Widget)]
pub struct GridImage {
    #[deref]
    view: View,

    #[animator]
    animator: Animator,
}

impl LiveHook for GridImage {
    fn after_new_from_doc(&mut self, cx: &mut Cx) {
        let image_ref = self.view.image(id!(image));
        let new_texture = Texture::new_with_format(
            cx,
            TextureFormat::VecBGRAu8_32 {
                width: 4,
                height: 4,
                data: vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            },
        );
        image_ref.set_texture(cx, Some(new_texture));
    }
}

impl Widget for GridImage {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.animator_handle_event(cx, event);

        if self.animator.need_init() {
            self.animator_play(cx, id!(spinner.spin));
        }
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl GridImageRef {
    pub fn set_animator_play(&mut self, cx: &mut Cx, id: &[LiveId; 2]) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.animator_play(cx, id);
        }
    }
}
