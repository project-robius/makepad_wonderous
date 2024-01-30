use makepad_widgets::widget::WidgetCache;
use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import makepad_draw::shader::std::*;

    import crate::shared::styles::*;
    import crate::shared::widgets::*;

    IMG_GREAT_WALL_ARTIFACT_1 = dep("crate://self/resources/images/artifacts/great-wall-1.jpg")

    CarrouselItem = <RoundedView> {
        width: 200,
        height: 280,
        padding: 4.0,

        draw_bg: {
            radius: 46.
            border_width: 1.0
            border_color: #fff
        }

        image = <Image> {
            source: (IMG_GREAT_WALL_ARTIFACT_1),
            width: Fill,
            height: Fill,

            draw_bg: {
                instance radius: 46.
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

    ArtifactsCarrousel = {{ArtifactsCarrousel}} {
        width: Fill,
        height: Fit,

        flow: Down,
        spacing: 10.0,
        padding: 10.0,
        align: {x: 0.5, y: 0.0},
        
        <View> {
            flow: Overlay,
            width: Fill,
            height: 360,

            main_item = <CarrouselItem> {
                margin: { left: 75.0 }
            }

            previous_item = <CarrouselItem> {
                margin: { left: -90.0, top: 170.0 }
                width: 160,
                height: 160,

                draw_bg: {
                    radius: 36.
                }

                image = {
                    draw_bg: {
                        radius: 36.
                    }
                }
            }

            next_item = <CarrouselItem> {
                margin: { left: 280.0, top: 170.0 }
                width: 160,
                height: 160,

                draw_bg: {
                    radius: 36.
                }

                image = {
                    draw_bg: {
                        radius: 36.
                    }
                }
            }
        }

        <Label> {
            draw_text:{
                text_style: <SUBTITLE_CAPTION>{font_size: 16},
                color: #fff
            }
            text: "Cape"
        }

        <Label> {
            draw_text:{
                text_style: <SUBTITLE_CAPTION>{font_size: 12},
                color: #fff
            }
            text: "second half 16th century"
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct ArtifactsCarrousel {
    #[deref]
    view: View,
}

impl Widget for ArtifactsCarrousel {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope)
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl ArtifactsCarrousel {
}