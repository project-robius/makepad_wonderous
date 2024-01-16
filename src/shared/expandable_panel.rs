use makepad_widgets::*;
use crate::shared::touch_gesture::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;

    ExpandablePanel = {{ExpandablePanel}} {
        flow: Overlay,
        width: Fill,
        height: Fill,

        initial_offset: 400.0;

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
    #[rust] touch_gesture: Option<TouchGesture>,
    #[live] initial_offset: f64,
}

impl LiveHook for ExpandablePanel {
    fn after_apply_from(&mut self, cx: &mut Cx, apply: &mut Apply) {
        if apply.from.is_from_doc() {
            self.apply_over(cx, live! {
                panel = { margin: { top: (self.initial_offset) }}
            });
        }
    }
}

impl Widget for ExpandablePanel {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);

        if let Some(touch_gesture) = self.touch_gesture.as_mut() {
            if touch_gesture.handle_event(cx, event, self.view.area()).has_changed() {
                let scroll_offset = touch_gesture.scroll_offset;
                let panel_margin = self.initial_offset - scroll_offset;
                self.apply_over(cx, live! {
                    panel = { margin: { top: (panel_margin) }}
                });
                self.redraw(cx);

                cx.widget_action(
                    self.widget_uid(),
                    &scope.path,
                    ExpandablePanelAction::ScrolledAt(scroll_offset),
                );
            }
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let result = self.view.draw_walk(cx, scope, walk);

        if self.touch_gesture.is_none() {
            let mut touch_gesture = TouchGesture::new();
            touch_gesture.set_scroll_mode(ScrollMode::Swipe);

            // Limit the amount of dragging allowed for the panel
            let panel_height = self.view(id!(panel)).area().rect(cx).size.y;
            touch_gesture.set_scroll_range(0.0, panel_height - self.initial_offset);

            self.touch_gesture = Some(touch_gesture);
        }

        result
    }
}