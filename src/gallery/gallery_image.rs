use makepad_widgets::*;
pub const IMAGE_WIDTH: f64 = 250.;
pub const IMAGE_HEIGHT: f64 = 400.;

live_design! {
    import makepad_draw::shader::std::*;
    import makepad_widgets::base::*;

    GalleryImage = {{GalleryImage}} {
        image: <Image> {}
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
        _ = self
            .image
            .draw_walk_widget(cx, Walk::size(bg_width, bg_height).with_abs_pos(pos));
    }
}

pub enum GalleryImageAction {}
