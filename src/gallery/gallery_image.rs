use makepad_widgets::{image_cache::ImageCacheImpl, *};
pub const IMAGE_WIDTH: f64 = 250.;
pub const IMAGE_HEIGHT: f64 = 400.;

live_design! {
    import makepad_draw::shader::std::*;
    import makepad_widgets::base::*;

    GalleryImage = {{GalleryImage}} {
        image: <RotatedImage> {
            draw_bg: {
                instance radius: 3.
                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    sdf.box(
                        1,
                        1,
                        self.rect_size.x - 2.0,
                        self.rect_size.y - 2.0,
                        max(1.0, self.radius)
                    )
                    sdf.fill_keep(self.get_color())
                    return sdf.result
                }
            }
        }

        animator: {
            zoom = {
                default: off
                on = {
                    from: {all: Forward {duration: 0.3}}
                    apply: {
                        image: { draw_bg: {scale: 1.1} }
                    }
                }
                off = {
                    from: {all: Forward {duration: 0.3}}
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
    #[live] #[redraw]
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

    pub fn draw_abs(&mut self, cx: &mut Cx2d, pos: DVec2) {
        let bg_width = Size::Fixed(IMAGE_WIDTH);
        let bg_height = Size::Fixed(IMAGE_HEIGHT);
        self.image.load_image_dep_by_path(cx, &self.path);
        _ = self
            .image
            .draw_walk(cx, Walk::size(bg_width, bg_height).with_abs_pos(pos));
    }
}
