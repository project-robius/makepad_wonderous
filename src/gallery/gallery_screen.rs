use makepad_widgets::widget::WidgetCache;
use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;

    import crate::shared::styles::*;
    import crate::shared::widgets::*;

    GalleryScreen = {{GalleryScreen}} {
        width: Fill, height: Fill
        flow: Down,

        show_bg: true,
        draw_bg: {
            color: #000
        }

    }
}

#[derive(Live)]
pub struct GalleryScreen {
    #[deref]
    view: View,

    #[animator]
    animator: Animator,
}

impl LiveHook for GalleryScreen {
    fn before_live_design(cx: &mut Cx) {
        register_widget!(cx, GalleryScreen);
    }
}

impl Widget for GalleryScreen {
    fn handle_widget_event_with(
        &mut self,
        cx: &mut Cx,
        event: &Event,
        dispatch_action: &mut dyn FnMut(&mut Cx, WidgetActionItem),
    ) {
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

impl GalleryScreen {}
