use makepad_widgets::*;
use crate::shared::touch_gesture::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;

    ExpandablePanel = {{ExpandablePanel}} {
        flow: Overlay,
        width: Fill,
        height: Fill,

        scroll_max: 1000.0,

        body = <View> {}

        panel = <View> {
            flow: Down,
            width: Fill,
            height: Fit,

            show_bg: true,
            draw_bg: {
                color: #FFF
            }

            align: { x: 0.5, y: 0 }
            padding: 20,
            spacing: 10,

            margin: { top: 400.0 }

            scroll_handler = <RoundedView> {
                width: 40,
                height: 6,  

                show_bg: true,
                draw_bg: {
                    color: #333
                    radius: 2.
                }
            }
        }
    }
}

#[derive(Clone, DefaultNone, Debug)]
pub enum ExpandablePanelAction {
    ScrolledAt(f64),
    None,
}

#[derive(Live, Widget)]
pub struct ExpandablePanel {
    #[deref] view: View,
    #[rust] touch_gesture: TouchGesture,
    #[live] scroll_max: f64,
}

impl LiveHook for ExpandablePanel {
    fn after_apply_from(&mut self, _cx: &mut Cx, apply: &mut Apply) {
        if apply.from.is_from_doc() {
            self.touch_gesture = TouchGesture::new();
            self.touch_gesture.reset(0.0, 0.0, self.scroll_max, ScrollMode::Swipe);
        }
    }
}

impl Widget for ExpandablePanel {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);

        if self.touch_gesture.handle_event(cx, event, self.view.area()).has_changed() {
            let panel_margin = 400. - self.touch_gesture.scroll_offset;
            self.apply_over(cx, live! {
                panel = { margin: { top: (panel_margin) }}
            });
            self.redraw(cx);

            cx.widget_action(
                self.widget_uid(),
                &scope.path,
                ExpandablePanelAction::ScrolledAt(self.touch_gesture.scroll_offset),
            );
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}