use makepad_widgets::widget::WidgetCache;
use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import makepad_draw::shader::std::*;

    import crate::shared::styles::*;
    import crate::shared::widgets::*;

    IMG_GREAT_WALL_CONSTRUCTION_1 = dep("crate://self/resources/images/photo-3.jpg")
    IMG_GREAT_WALL_CONSTRUCTION_2 = dep("crate://self/resources/images/photo-4.jpg")

    GreatWallConstructionImages = {{GreatWallConstructionImages}} {
        flow: Overlay,
        width: Fill,
        height: 700,

        image1 = <View> { 
            width: 300,
            height: 400,

            margin: {left: 50.0},

            <CenteredOnTop> {
                source: (IMG_GREAT_WALL_CONSTRUCTION_1),
                width: 300,
                height: 510,

                draw_bg: {
                    instance radius: 70.
                }
            }
        }

        image2 = <CenteredOnBottom> {
            source: (IMG_GREAT_WALL_CONSTRUCTION_2),
            width: 180,
            height: 279,

            margin: {left: 20.0, top: 600.0},

            draw_bg: {
                instance radius: 42.
            }
        }
    }
}

#[derive(Live)]
pub struct GreatWallConstructionImages {
    #[deref]
    view: View,
}

impl LiveHook for GreatWallConstructionImages {
    fn before_live_design(cx: &mut Cx) {
        register_widget!(cx, GreatWallConstructionImages);
    }
}

impl Widget for GreatWallConstructionImages {
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

impl GreatWallConstructionImages {
    fn update_values(&mut self, cx: &mut Cx, scroll: f64) {
        if scroll < 1900.0 {
            return;
        }

        let margin_image1 = (scroll - 1900.0) / 5.0;
        self.view(id!(image1)).apply_over(
            cx,
            live! { margin: { top: (margin_image1)}},
        );

        let margin_image2 = 600.0 - (scroll - 1900.0) / 3.5;
        self.image(id!(image2)).apply_over(
            cx,
            live! { margin: { top: (margin_image2)}},
        );
    }
}

#[derive(Clone, PartialEq, WidgetRef)]
pub struct GreatWallConstructionImagesRef(WidgetRef);

impl GreatWallConstructionImagesRef {
    pub fn update_values(&mut self, cx: &mut Cx, scroll: f64) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.update_values(cx, scroll);
        }
    }
}
