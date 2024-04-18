use makepad_widgets::*;

use crate::shared::staggered_grid::StaggeredGridWidgetRefExt;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;

    import crate::shared::styles::*;
    import crate::shared::widgets::*;
    import crate::shared::icon::*;
    import makepad_draw::shader::std::*;

    import crate::shared::staggered_grid::*;

    CALENDAR_ICON = dep("crate://self/resources/icons/calendar.svg")

    ResultsGrid = {{ResultsGrid}} {
        list = <StaggeredGrid>{
            columns_number: 2
            column_spacing: 4.0
            ShortElement = <View> {
                height: 100
                show_bg: true
                draw_bg: {
                    // color: #0a3d62
                    color: #38ada9
                }
                align: {x: 0.5, y: 0.5}
                lbl = <Label> {
                    draw_text:{
                        text_style: <SUBTITLE_CAPTION>{font_size: 12},
                        color: #fff
                    }
                    // text: "Short element"
                }
            }
            MediumElement = <View> {
                height: 200
                show_bg: true
                draw_bg: {
                    // color: #0c2461
                    color: #4a69bd
                }
                align: {x: 0.5, y: 0.5}
                lbl = <Label> {
                    draw_text:{
                        text_style: <SUBTITLE_CAPTION>{font_size: 12},
                        color: #fff
                    }
                    // text: "Tall element"
                }
            }
            LongElement = <View> {
                height: 300
                show_bg: true
                draw_bg: {
                    // color: #0c2461
                    color: #0a3d62
                }
                align: {x: 0.5, y: 0.5}
                lbl = <Label> {
                    draw_text:{
                        text_style: <SUBTITLE_CAPTION>{font_size: 12},
                        color: #fff
                    }
                    // text: "Tall element"
                }
            }
        }
    }

    ArtifactsGallery = {{ArtifactsGallery}} {
        width: Fill, height: Fill
        flow: Down,
        align: {x: 0.5, y: 0.0},

        header = <Label> {
            draw_text:{
                text_style: <SUBTITLE_CAPTION>{font_size: 10},
                color: #fff
            }
            text: "BROWSE ARTIFACTS"
        }
        wonder_name = <Label> {
            draw_text:{
                text_style: <SUBTITLE_CAPTION>{font_size: 12},
                color: #e6945c,
            }
            text: "THE GREAT WALL"
        }

        search_bar = <TextInput> {
            margin: { top: 10.0, left: 10.0, right: 10.0}
            width: Fill
            draw_bg: {
                color: #fff
                // border_width: 1.0
                // border_color: #x00000044
            }
            draw_text: {
                text_style: <REGULAR_TEXT>{font_size: 10},
                fn get_color(self) -> vec4 {
                    return vec4(0.0, 0.0, 0.0, 1.0);
                }
            }
            text: "Search (ex. type or material)"
        }

        search_results = <Label> {
            margin: { top: 8.0, bottom: 20.0 }
            draw_text: {
                text_style: <SUBTITLE_CAPTION>{font_size: 9},
                color: #e6945c,
            }
            text: "489 artifacts found, 277 in timeframe"
        }

        results_grid = <ResultsGrid> {}

        // timeframe_selector = <View> {
        //     flow: Right
        //     width: Fit, height: Fit
        //     align: {x: 0.5, y: 0.0},
        //     margin: { top: 580.0 }

        //     show_bg: true
        //     draw_bg: {
        //         instance border_width: 0.0
        //         // instance border_color: #0000
        //         instance inset: vec4(0.0, 0.0, 0.0, 0.0)
        //         instance radius: 2.5

        //         fn pixel(self) -> vec4 {
        //             let sdf = Sdf2d::viewport(self.pos * self.rect_size)
        //             sdf.box(
        //                 self.inset.x + self.border_width,
        //                 self.inset.y + self.border_width,
        //                 self.rect_size.x - (self.inset.x + self.inset.z + self.border_width * 2.0),
        //                 self.rect_size.y - (self.inset.y + self.inset.w + self.border_width * 2.0),
        //                 max(1.0, self.radius)
        //             )
        //             sdf.fill_keep(vec4(0.0, 0.0, 0.0, 0.4))
        //             // if self.border_width > 0.0 {
        //                 // sdf.stroke(self.border_color, self.border_width)
        //             // }
        //             return sdf.result;
        //         }
        //     }
        //     padding: 8.0

        //     timeframe = <Label> {
        //         draw_text:{
        //             text_style: <SUBTITLE_CAPTION>{font_size: 10},
        //             color: #fff
        //         }
        //         text: "700 BCE - 1650 CE"
        //     }
        //     <Icon> {
        //         draw_icon: {
        //             svg_file: (CALENDAR_ICON), color: #e6945c,
        //         }
        //         icon_walk: {width: 20, height: Fit}
        //     }
        // }
    }
}

#[derive(Live, LiveHook, Widget)]
struct ResultsGrid {
    #[deref]
    view: View,
}

impl Widget for ResultsGrid {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        while let Some(item) = self.view.draw_walk(cx, scope, walk).step() {
            if let Some(mut list) = item.as_staggered_grid().borrow_mut() {
                // TODO: this creates a total of range_end x number of columns items
                list.set_item_range(cx, 0, 100);
                while let Some(item_id) = list.next_visible_item(cx) {
                    let template = match item_id {
                    // let template = match random_number() {
                        x if x % 3 == 0 => live_id!(ShortElement),
                        x if x % 3 == 1 => live_id!(MediumElement),
                        _ => live_id!(LongElement),
                    };
                    let item = list.item(cx, item_id, template).unwrap();
                    // let text = match item_id % 4 {
                    //     1 => format!("Hello! {}", item_id),
                    //     2 => format!("Hello GOSIM\n With lines {}", item_id),
                    //     3 => format!("Random numbers {}", item_id),
                    //     _ => format!("Text body 4 id {}", item_id),
                    // };
                    item.label(id!(lbl)).set_text(&format!("{}", item_id + 1));
                    // item.button(id!(likes)).set_text(&format!("{}", item_id % 23));
                    // item.button(id!(comments)).set_text(&format!("{}", item_id % 6));
                    log!("****************** DRAWING ITEM {}", item_id + 2);
                    item.draw_all(cx, scope);
                }
            }
        }
        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope)
    }
}

#[derive(Widget, Live)]
pub struct ArtifactsGallery {
    #[deref]
    view: View,
}

impl LiveHook for ArtifactsGallery {}

impl MatchEvent for ArtifactsGallery {
    fn handle_actions(&mut self, _cx: &mut Cx, actions: &Actions) {
        let results_grid = self.view.portal_list_set(ids!(results_grid.list));
        for (item_id, item) in results_grid.items_with_actions(&actions) {
            if item.button(id!(likes)).clicked(&actions) {
                log!("hello {}", item_id);
            }
        }
    }
}

impl Widget for ArtifactsGallery {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.match_event(cx, event);
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}
