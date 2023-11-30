use makepad_widgets::*;
pub const IMAGE_WIDTH: f64 = 20.0;

live_design! {
    import makepad_draw::shader::std::*;
    import makepad_widgets::base::*;

    ImageBox= {{ImageBox}} {
        image: <RotatedImage> {}

        animator: {
            fade = {
                default: off
                off = {
                    from: {all: Snap}
                    apply: {
                        image: { draw_bg: {opacity: 1.0} }
                    }
                }
                on = {
                    from: {all: Loop {duration: 5, end: 1.0}}
                    apply: {
                        image: { draw_bg: {opacity: 0.0} }
                    }
                }
            }

            scale = {
                default: off
                off = {
                    from: {all: Snap}
                    apply: {
                        image: { draw_bg: {scale: 1.0} }
                    }
                }
                on = {
                    from: {all: Loop {duration: 5, end: 1.0}}
                    apply: {
                        image: { draw_bg: {scale: 0.0} }
                    }
                }
            }

            rotate = {
                default: off
                off = {
                    from: {all: Snap}
                    apply: {
                        image: { draw_bg: {rotation: 0.0} }
                    }
                }
                on = {
                    from: {all: Loop {duration: 5, end: 1.0}}
                    apply: {
                        image: { draw_bg: {rotation: 6.28318}}
                    }
                }
            }
        }
    }
}

#[derive(Live)]
pub struct ImageBox {
    #[live]
    draw_bg: DrawQuad,
    #[live]
    image: Image,
    #[layout]
    layout: Layout,
    #[animator]
    animator: Animator,

    #[rust]
    pub animation: Animation,
}

#[derive(Clone, Debug, Default, Eq, Hash, Copy, PartialEq, FromLiveId)]
pub struct ImageBoxId(pub LiveId);

impl LiveHook for ImageBox {
    fn before_apply(
        &mut self,
        _cx: &mut Cx,
        _apply_from: ApplyFrom,
        _index: usize,
        _nodes: &[LiveNode],
    ) {
    }
}

impl ImageBox {
    pub fn handle_event_with(
        &mut self,
        cx: &mut Cx,
        event: &Event,
        _dispatch_action: &mut dyn FnMut(&mut Cx, ImageBoxAction),
    ) {
        self.animator_handle_event(cx, event);
    }

    pub fn draw_abs(&mut self, cx: &mut Cx2d, pos: DVec2) {
        if self.animator.need_init() {
            match self.animation {
                Animation::Fade => self.animator_play(cx, id!(fade.on)),
                Animation::Scale => self.animator_play(cx, id!(scale.on)),
                Animation::Rotate => self.animator_play(cx, id!(rotate.on)),
            }
        }

        let bg_size = Size::Fixed(IMAGE_WIDTH);
        _ = self
            .image
            .draw_walk_widget(cx, Walk::size(bg_size, bg_size).with_abs_pos(pos));
    }
}

pub enum ImageBoxAction {}

#[derive(Default)]
pub enum Animation {
    #[default]
    Rotate,
    Scale,
    Fade,
}

impl Animation {
    pub fn from_index(index: usize) -> Self {
        match index {
            0 => Animation::Fade,
            1 => Animation::Scale,
            2 => Animation::Rotate,
            _ => Animation::Rotate,
        }
    }
}
