use makepad_widgets::widget::WidgetCache;
use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import makepad_draw::shader::std::*;

    import crate::shared::styles::*;
    import crate::shared::curved_label::*;
    import crate::shared::widgets::*;

    IMG_GREAT_WALL = dep("crate://self/resources/images/photo-2.jpg")

    HightlightLabel = <Label> {
        width: Fit,
        draw_text: {
            text_style: <DECORATIVE_BOLD_TEXT>{font_size: 20},
            color: #5d2a2ccc,
        }
    }

    GreatWallHighlight = {{GreatWallHighlight}} {
        flow: Overlay,
        width: Fill,
        height: 450,

        align: {x: 0.5, y: 0.0},

        image = <CenteredOnTop> {
            source: (IMG_GREAT_WALL),
            width: 300,
            height: 600,

            draw_bg: {
                instance opacity: 0.3
                instance image_scale: vec2(0.9, 0.9)
                instance radius: 70.
            }
        }

        label1 = <View> {
            width: Fit,
            height: Fit,
            flow: Down,
            spacing: 10.0
            margin: {top: 20.0}
            align: {x: 0.5, y: 0.0},

            <HightlightLabel> {
                text: "THE LONGEST"
            }
            <HightlightLabel> {
                text: "MAN-MADE"
            }
        }

        label2 = <View> {
            width: Fit,
            height: Fit,
            flow: Down,
            spacing: 10.0
            margin: {top: 310.0}
            align: {x: 0.5, y: 0.0},

            <HightlightLabel> {
                text: "STRUCTURE IN"
            }
            <HightlightLabel> {
                text: "THE WORLD"
            }
        }
    }
}

#[derive(Live)]
pub struct GreatWallHighlight {
    #[deref]
    view: View,
}

impl LiveHook for GreatWallHighlight {
    fn before_live_design(cx: &mut Cx) {
        register_widget!(cx, GreatWallHighlight);
    }
}

impl Widget for GreatWallHighlight {
    fn handle_widget_event_with(
        &mut self,
        cx: &mut Cx,
        event: &Event,
        dispatch_action: &mut dyn FnMut(&mut Cx, WidgetActionItem),
    ) {
        self.view.handle_widget_event_with(cx, event, dispatch_action);
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

impl GreatWallHighlight {
    fn update_values(&mut self, cx: &mut Cx, scroll: f64) {
        if scroll < 500.0 || scroll > 1600.0 {
            return;
        }

        let opacity = 1.0 - (scroll - 500.0) / 4000.0;
        let scale = 1.0 - (scroll - 500.0) / 6000.0;
        let image_scale = dvec2(scale, scale);

        self.image(id!(image)).apply_over(
            cx,
            live! { draw_bg: {
                image_scale: (image_scale),
                opacity: (opacity)
            }},
        );

        let label1_margin = min(200.0, 20.0 + (scroll - 500.0) / 2.5);
        self.label(id!(label1)).apply_over(
            cx,
            live! { margin: { top: (label1_margin) }},
        );

        let label2_margin = max(270.0, 450.0 - (scroll - 500.0) / 2.5);
        self.label(id!(label2)).apply_over(
            cx,
            live! { margin: { top: (label2_margin) }},
        );
    }
}

#[derive(Clone, PartialEq, WidgetRef)]
pub struct GreatWallHighlightRef(WidgetRef);

impl GreatWallHighlightRef {
    pub fn update_values(&mut self, cx: &mut Cx, scroll: f64) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.update_values(cx, scroll);
        }
    }
}
