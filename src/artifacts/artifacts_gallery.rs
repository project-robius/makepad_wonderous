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

    IMG_A = dep("crate://self/resources/images/artifacts/test_1.jpg")
    IMG_B = dep("crate://self/resources/images/artifacts/test_2.jpg")
    IMG_C = dep("crate://self/resources/images/artifacts/test_3.jpg")
    IMG_D = dep("crate://self/resources/images/artifacts/test_4.jpg")

    // GridImage = <Image> {
    //     source: (IMG_A)
    //     width: Fill,
    //     height: Fill
    //     min_width: 100,
    //     min_height: 100,
    //     fit: Horizontal,
    //     draw_bg: {
    //         instance hover: 0.0
    //         instance down: 0.0
    //         fn pixel(self) -> vec4 {
    //             let sdf = Sdf2d::viewport(self.pos * self.rect_size)
    //             sdf.box(1, 1, self.rect_size.x - 2, self.rect_size.y - 2, 4.0)
    //             let max_scale = vec2(0.92);
    //             let scale = mix(vec2(1.0), max_scale, self.hover);
    //             let pan = mix(vec2(0.0), (vec2(1.0) - max_scale) * 0.5, self.hover);
    //             let color = self.get_color_scale_pan(scale, pan) + mix(vec4(0.0), vec4(0.1), self.down);
    //             sdf.fill_keep(color);
    //             sdf.stroke(
    //                 mix(mix(#x0000, #x0006, self.hover), #xfff2, self.down),
    //                 1.0
    //             )
                
    //             return sdf.result
    //         }
    //     }
    // }

    // Short = <View> {
    //     height: 100
    //     show_bg: true
    //     draw_bg: {
    //         color: #38ada9
    //     }

    //     <GridImage> { source: (IMG_A) }
    //     align: {x: 0.5, y: 0.5}
    // }
    // Medium = <View> {
    //     height: 200
    //     show_bg: true
    //     draw_bg: {
    //         color: #4a69bd
    //     }
    //     align: {x: 0.5, y: 0.5}

    //     <GridImage> { source: (IMG_B) }
    // }
    // Medium_2 = <View> {
    //     height: 250
    //     show_bg: true
    //     draw_bg: {
    //         color: #0c2461
    //     }
    //     align: {x: 0.5, y: 0.5}

    //     <GridImage> { source: (IMG_D) }
    // }
    // Long = <View> {
    //     height: 300
    //     show_bg: true
    //     draw_bg: {
    //         color: #0a3d62
    //     }
    //     align: {x: 0.5, y: 0.5}
    //     <GridImage> { source: (IMG_C) }
    // }

    Short = <View> {
        height: 100
        show_bg: true
        draw_bg: {
            color: #38ada9
        }

        align: {x: 0.5, y: 0.5}
        lbl = <Label> {
            draw_text:{
                text_style: <SUBTITLE_CAPTION>{font_size: 12},
                color: #fff
            }
        }
    }
    Medium = <View> {
        height: 200
        show_bg: true
        draw_bg: {
            color: #4a69bd
        }
        align: {x: 0.5, y: 0.5}

        lbl = <Label> {
            draw_text:{
                text_style: <SUBTITLE_CAPTION>{font_size: 12},
                color: #fff
            }
        }
    }
    Medium_2 = <View> {
        height: 250
        show_bg: true
        draw_bg: {
            color: #6a89cc
        }
        align: {x: 0.5, y: 0.5}

        lbl = <Label> {
            draw_text:{
                text_style: <SUBTITLE_CAPTION>{font_size: 12},
                color: #fff
            }
        }
    }
    Long = <View> {
        height: 300
        show_bg: true
        draw_bg: {
            color: #0a3d62
        }
        align: {x: 0.5, y: 0.5}

        lbl = <Label> {
            draw_text:{
                text_style: <SUBTITLE_CAPTION>{font_size: 12},
                color: #fff
            }
        }
    }

    ResultsGrid = {{ResultsGrid}} {
        list = <StaggeredGrid>{
            columns_number: 2
            column_spacing: 4.0
            ShortElement = <Short> {}
            MediumElement = <Medium> {}
            MediumElement_2 = <Medium_2> {} 
            LongElement = <Long> {}
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

                let mut last_drawn_item = 0;
                list.set_item_range(cx, 0, 20);
                while let Some(item_id) = list.next_visible_item(cx) {
                    // break early if the item_id is lower than the previous item_ids
                    if item_id < last_drawn_item {
                        log!("SOMETHING WHACKY HAPPENED OMG");
                        // break;
                    }

                    let template = match item_id {
                    // let template = match random_number() {
                        x if x % 4 == 0 => live_id!(ShortElement),
                        x if x % 4 == 1 => live_id!(MediumElement),
                        x if x % 4 == 2 => live_id!(MediumElement_2),
                        _ => live_id!(LongElement),
                    };
                    let item = list.item(cx, item_id, template).unwrap();
                    item.label(id!(lbl)).set_text(&format!("{}", item_id));
                    log!("****************** DRAWING ITEM {}", item_id);
                    item.draw_all(cx, scope);
                    last_drawn_item = item_id;
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
