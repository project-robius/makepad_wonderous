use makepad_widgets::{image_cache::ImageCacheImpl, *};

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import makepad_draw::shader::std::*;

    GalleryImage = {{GalleryImage}} {
        align: {x: 0.5, y: 0.5}
        image: <Image> {
            draw_bg: {
                instance radius: 3.
                instance scale: 0.0
                instance down: 0.0
                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    sdf.box(
                        1,
                        1,
                        self.rect_size.x - 2.0,
                        self.rect_size.y - 2.0,
                        max(1.0, self.radius)
                    )
                    let max_scale = vec2(0.9);
                    let scale = mix(vec2(1.0), max_scale, self.scale);
                    let pan = mix(vec2(0.0), (vec2(1.0) - max_scale) * 0.5, self.scale);

                    let color = self.get_color_scale_pan(scale, pan) + mix(vec4(0.0), vec4(0.1), 0);
                    sdf.fill_keep(color);
                    return sdf.result
                }
            }
        }
        animator: {
            zoom = {
                default: off
                off = {
                    from: {
                        ease: OutExp,
                        all: Forward {duration: 0.5}
                    }
                    apply: {
                        image: {draw_bg: {scale: 0.0}}
                    }
                }
                on = {
                    from: {
                        ease: OutExp,
                        all: Forward {duration: 0.5}
                    }
                    apply: {
                        image: { draw_bg: {scale: 1.0} }
                    }
                }
            }
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct GalleryImage {
    #[live]
    #[redraw]
    draw_bg: DrawQuad,
    #[live]
    image: Image,

    #[layout]
    layout: Layout,
    #[walk]
    walk: Walk,

    #[animator]
    animator: Animator,
    #[rust]
    path: String,
    #[rust]
    size: DVec2,
}

#[derive(Clone, Debug, Default, Eq, Hash, Copy, PartialEq, FromLiveId)]
pub struct GalleryImageId(pub LiveId);

impl Widget for GalleryImage {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, _scope: &mut Scope) {
        self.animator_handle_event(cx, event);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, _walk: Walk) -> DrawStep {
        let pos = scope.data.get_mut::<DVec2>();
        self.draw_abs(cx, *pos);

        DrawStep::done()
    }
}

impl GalleryImage {
    pub fn set_path(&mut self, path: String) {
        self.path = path;
    }

    pub fn set_size(&mut self, cx: &mut Cx, size: DVec2) {
        self.size = size;
    }

    pub fn draw_abs(&mut self, cx: &mut Cx2d, pos: DVec2) {
        let bg_width = Size::Fixed(self.size.x);
        let bg_height = Size::Fixed(self.size.y);
        let _ = self.image.load_image_dep_by_path(cx, &self.path);
        _ = self
            .image
            .draw_walk(cx, Walk::size(bg_width, bg_height).with_abs_pos(pos));
    }
}
