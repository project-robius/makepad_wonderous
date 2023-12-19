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

#[derive(Live, LiveHook, Widget)]
pub struct GreatWallConstructionImages {
    #[deref]
    view: View,
}

impl Widget for GreatWallConstructionImages {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope)
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
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

impl GreatWallConstructionImagesRef {
    pub fn update_values(&mut self, cx: &mut Cx, scroll: f64) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.update_values(cx, scroll);
        }
    }
}
