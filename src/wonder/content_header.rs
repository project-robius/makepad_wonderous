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

    ContentHeader = {{ContentHeader}} {
        flow: Down
        visible: false
        show_bg: true
        draw_bg: {
            color: #5d2a2c
        }
        width: Fill,
        height: 430,

        margin: { top: -86 }

        view = <FadeView> {
            width: Fill,
            height: 430,
            draw_bg: { instance opacity: 0.3 }

            <Image> {
                source: (IMG_GREAT_WALL_CONTENT_1),
                width: Fill,
                height: 430,
            }
        }

        animator: {
            header = {
                default: show,
                hide = {
                    redraw: true,
                    from: {all: Snap}
                    apply: {
                        view = { draw_bg: { opacity: 0.0 } }
                    }
                }
                wait_to_show = {
                    redraw: true,
                    from: {all: Forward {duration: 0.5}}
                    apply: {
                        view = { draw_bg: { opacity: 0.0 } }
                    }
                }
                show = {
                    redraw: true,
                    from: {all: Forward {duration: 0.5}}
                    apply: {
                        view = { draw_bg: { opacity: 1.0 } }
                    }
                }
            },
        }
    }
}

#[derive(Live, Widget)]
pub struct ContentHeader {
    #[deref]
    view: View,

    #[animator]
    animator: Animator,

    #[rust]
    next_frame: NextFrame,
}

impl LiveHook for ContentHeader {
    fn after_apply_from(&mut self, cx: &mut Cx, apply: &mut Apply) {
        if apply.from.is_from_doc() {
            self.next_frame = cx.new_next_frame();
        }
    }
}

impl Widget for ContentHeader {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        if self.animator_handle_event(cx, event).must_redraw() {
            self.redraw(cx);
        }

        if let Some(_ne) = self.next_frame.is_event(event) {
            if !self.animator.is_track_animating(cx, id!(header))
                && self.animator_in_state(cx, id!(header.wait_to_show))
            {
                self.view.apply_over(cx, live! {visible: true});
                self.animator_play(cx, id!(header.show));
            }
            self.next_frame = cx.new_next_frame();
        }

        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl ContentHeader {
    fn show(&mut self, cx: &mut Cx) {
        self.animator_play(cx, id!(header.wait_to_show));
    }

    fn hide(&mut self, cx: &mut Cx) {
        self.view.apply_over(cx, live! {visible: false});
        self.animator_play(cx, id!(header.hide));
    }
}

impl ContentHeaderRef {
    pub fn show(&mut self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.show(cx);
        }
    }

    pub fn hide(&mut self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.hide(cx);
        }
    }
}
