use makepad_widgets::widget::WidgetCache;
use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import makepad_draw::shader::std::*;

    import crate::shared::styles::*;
    import crate::shared::widgets::*;

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

        items: [
            dep("crate://self/resources/images/artifacts/great-wall-1.jpg"),
            dep("crate://self/resources/images/artifacts/great-wall-2.jpg"),
            dep("crate://self/resources/images/artifacts/great-wall-3.jpg"),
            dep("crate://self/resources/images/artifacts/great-wall-4.jpg"),
            dep("crate://self/resources/images/artifacts/great-wall-5.jpg"),
        ]
        
        container = <View> {
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

#[derive(Live, Widget)]
pub struct ArtifactsCarrousel {
    #[deref]
    view: View,

    #[live]
    items: Vec<LiveDependency>
}

impl LiveHook for ArtifactsCarrousel {
    fn after_apply_from(&mut self, cx: &mut Cx, apply: &mut Apply) {
        if apply.from.is_from_doc() {
            self.set_current_image(cx, 0);
        }
    }
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
    fn set_current_image(&mut self, cx: &mut Cx, index: usize) {
        let mut dep_path = self.items[index].as_str();
        let mut image = self.view.image(id!(container.main_item.image));
        let _ = image.load_image_dep_by_path(cx, dep_path);

        let previous_index = (index as i8 - 1).rem_euclid(self.items.len() as i8) as usize;
        dep_path = self.items[previous_index].as_str();
        image = self.view.image(id!(container.previous_item.image));
        let _ = image.load_image_dep_by_path(cx, dep_path);

        let next_index = (index as i8 + 1).rem_euclid(self.items.len() as i8) as usize;
        dep_path = self.items[next_index].as_str();
        image = self.view.image(id!(container.next_item.image));
        let _ = image.load_image_dep_by_path(cx, dep_path);
    }
}