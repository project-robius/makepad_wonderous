use makepad_widgets::{image_cache::ImageCacheImpl, *};
pub const IMAGE_WIDTH: f64 = 250.;
pub const IMAGE_HEIGHT: f64 = 400.;

live_design! {
    import makepad_draw::shader::std::*;
    import makepad_widgets::base::*;

    GalleryImage = {{GalleryImage}} {
        image: <Image> {
            draw_bg: {
                instance radius: 70.0
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
    }
}

#[derive(Live)]
pub struct GalleryImage {
    #[live]
    draw_bg: DrawQuad,
    #[live]
    image: Image,
    #[layout]
    layout: Layout,
    #[animator]
    animator: Animator,
    #[rust]
    path: String,
}

#[derive(Clone, Debug, Default, Eq, Hash, Copy, PartialEq, FromLiveId)]
pub struct GalleryImageId(pub LiveId);

impl LiveHook for GalleryImage {
    fn before_apply(
        &mut self,
        _cx: &mut Cx,
        _apply_from: ApplyFrom,
        _index: usize,
        _nodes: &[LiveNode],
    ) {
    }
}

impl GalleryImage {
    pub fn handle_event_with(
        &mut self,
        cx: &mut Cx,
        event: &Event,
        _dispatch_action: &mut dyn FnMut(&mut Cx, GalleryImageAction),
    ) {
        self.animator_handle_event(cx, event);
    }

    pub fn draw_abs(&mut self, cx: &mut Cx2d, pos: DVec2) {
        // play animator here
        let bg_width = Size::Fixed(IMAGE_WIDTH);
        let bg_height = Size::Fixed(IMAGE_HEIGHT);
        self.image.load_image_dep_by_path(cx, &self.path);
        _ = self
            .image
            .draw_walk_widget(cx, Walk::size(bg_width, bg_height).with_abs_pos(pos));
    }

    pub fn set_path(&mut self, path: String) {
        self.path = path;
    }
}

pub enum GalleryImageAction {}
