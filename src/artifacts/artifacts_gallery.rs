use makepad_widgets::*;
use std::collections::HashMap;
use crate::shared::{network_images_cache::NetworkImageCache, staggered_grid::StaggeredGridWidgetRefExt};

use super::data::{great_wall_search_data::SEARCH_DATA, image_search::request_search_images};
const INITIAL_IMAGE_SEARCH_NUMBER: usize = 20;

// TODO
//  - network images
//      - make sure responses correspond to their widget, or at least make sure the images are only loaded once into the cache.
//      - make sure there's always a buffer of images loaded so that scrolling is smooth.
//  - timeline navigator 
//      - the grid range represents the timeline of artifacts
//  - search
//  - artifact detail view: 
//      - items should be clickable and a sub-page should slide-in with more information about the artifact 

// Maybe
//  - pre-allocate item sizes and then load the images into the items depending on their dimensions.
//  - re-use a set of let's say 30 Image instances for the grid and swap the images, so we don't over-use textures.
//  - currently we cache the raw image data, but we could also cache the textures and re-use them.


live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;

    import crate::shared::styles::*;
    import crate::shared::widgets::*;
    import crate::shared::icon::*;
    import makepad_draw::shader::std::*;

    import crate::shared::staggered_grid::*;

    CALENDAR_ICON = dep("crate://self/resources/icons/calendar.svg")
    SEARCH_ICON = dep("crate://self/resources/icons/search.svg")

    GridImage = <Image> {
        width: Fill,
        height: Fill
        min_width: 100,
        min_height: 100,
        fit: Horizontal,
        draw_bg: {
            instance hover: 0.0
            instance down: 0.0
            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size)
                sdf.box(1, 1, self.rect_size.x - 2, self.rect_size.y - 2, 4.0)
                let max_scale = vec2(0.92);
                let scale = mix(vec2(1.0), max_scale, self.hover);
                let pan = mix(vec2(0.0), (vec2(1.0) - max_scale) * 0.5, self.hover);
                let color = self.get_color_scale_pan(scale, pan) + mix(vec4(0.0), vec4(0.1), self.down);
                sdf.fill_keep(color);
                sdf.stroke(
                    mix(mix(#x0000, #x0006, self.hover), #xfff2, self.down),
                    1.0
                )
                
                return sdf.result
            }
        }
    }

    ImageView = <View> {
        image = <GridImage> {}
        align: {x: 0.5, y: 0.5}
    }

    FlexibleImageContainer = <ImageView> {
        height: Fit
    }
    // Medium = <ImageView> {
    //     height: 200
    // }
    // Medium_2 = <ImageView> {
    //     height: 250
    // }
    // Long = <ImageView> {
    //     height: 300
    // }

    // Debugging
    // Short = <View> {
    //     height: 100
    //     show_bg: true
    //     draw_bg: {
    //         color: #38ada9
    //     }

    //     align: {x: 0.5, y: 0.5}
    //     lbl = <Label> {
    //         draw_text:{
    //             text_style: <SUBTITLE_CAPTION>{font_size: 12},
    //             color: #fff
    //         }
    //     }
    // }
    // Medium = <View> {
    //     height: 200
    //     show_bg: true
    //     draw_bg: {
    //         color: #4a69bd
    //     }
    //     align: {x: 0.5, y: 0.5}

    //     lbl = <Label> {
    //         draw_text:{
    //             text_style: <SUBTITLE_CAPTION>{font_size: 12},
    //             color: #fff
    //         }
    //     }
    // }
    // Medium_2 = <View> {
    //     height: 250
    //     show_bg: true
    //     draw_bg: {
    //         color: #6a89cc
    //     }
    //     align: {x: 0.5, y: 0.5}

    //     lbl = <Label> {
    //         draw_text:{
    //             text_style: <SUBTITLE_CAPTION>{font_size: 12},
    //             color: #fff
    //         }
    //     }
    // }
    // Long = <View> {
    //     height: 300
    //     show_bg: true
    //     draw_bg: {
    //         color: #0a3d62
    //     }
    //     align: {x: 0.5, y: 0.5}

    //     lbl = <Label> {
    //         draw_text:{
    //             text_style: <SUBTITLE_CAPTION>{font_size: 12},
    //             color: #fff
    //         }
    //     }
    // }

    ResultsGrid = {{ResultsGrid}} {
        list = <StaggeredGrid>{
            columns_number: 2
            column_spacing: 3.0
            ImageContainer = <FlexibleImageContainer> {}
            // ShortElement = <Short> {}
            // MediumElement = <Medium> {}
            // MediumElement_2 = <Medium_2> {} 
            // LongElement = <Long> {}
        }
    }

    ArtifactsGallery = {{ArtifactsGallery}} {
        width: Fill, height: Fill
        flow: Down,
        align: {x: 0.5, y: 0.0},
        spacing: 10.0,

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
        
        SearchBar = <RoundedView> {
            width: Fit,
            height: Fit,
    
            show_bg: true,
            draw_bg: {
                color: #fff
            }
    
            padding: {top: 3, bottom: 3, left: 20, right: 20}
    
            spacing: 4,
            align: {x: 0.0, y: 0.5},
    
            draw_bg: {
                radius: 4.0,
                border_color: #D0D5DD,
                border_width: 1.0,
            }
    
            <Icon> {
                draw_icon: {
                    svg_file: (SEARCH_ICON),
                    fn get_color(self) -> vec4 {
                        return #666;
                    }
                }
                icon_walk: {width: 14, height: Fit}
            }
    
            input = <CustomTextInput> {
                width: 270,
                height: Fit,
    
                empty_message: "Search (ex. type or material)"
    
                draw_text: {
                    text_style:<REGULAR_TEXT>{font_size: 10},
                }
            }
            text: "Search (ex. type or material)"
        }

        search_results = <Label> {
            margin: { bottom: 5.0 }
            draw_text: {
                text_style: <SUBTITLE_CAPTION>{font_size: 9},
                color: #e6945c,
            }
            text: "489 artifacts found, 277 in timeframe"
        }

        results_grid = <ResultsGrid> {}
    }
}

#[derive(Live, LiveHook, Widget)]
struct ResultsGrid {
    #[deref]
    view: View,

    #[rust]
    did_initial_image_request: bool,
    #[rust]
    items_artifacts_ids: HashMap<usize, String>,
    #[rust]
    items_images_ready: HashMap<usize, bool>,

    #[rust]
    waiting_for_images: usize,
}

impl MatchEvent for ResultsGrid {
    fn handle_network_responses(&mut self, cx: &mut Cx, responses: &NetworkResponsesEvent) {
        for event in responses {
            match &event.response {
                NetworkResponse::HttpResponse(response) => {
                    if response.status_code == 200 {
                        // TODO: we should make sure that the response corresponds to a request made by this widget.
                        if let Some(body) = response.get_body() {
                            cx.get_global::<NetworkImageCache>()
                                .insert(event.request_id, body.clone());
                            self.redraw(cx);
                            if self.waiting_for_images > 0 {
                                self.waiting_for_images -= 1;
                            }
                        }
                    } else {
                        error!("Error fetching gallery image: {:?}", response);
                    }
                }
                NetworkResponse::HttpRequestError(error) => {
                    error!("Error fetching gallery image: {:?}", error);
                }
                _ => (),
            }
        }
    }
}

impl Widget for ResultsGrid {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        if self.did_initial_image_request == false {
            request_search_images(cx, 0, INITIAL_IMAGE_SEARCH_NUMBER);
            self.did_initial_image_request = true;
        }
        while let Some(item) = self.view.draw_walk(cx, scope, walk).step() {
            if let Some(mut list) = item.as_staggered_grid().borrow_mut() {
                let range_end = SEARCH_DATA.len() as usize - 1;

                let mut last_drawn_item = 0;
                list.set_item_range(cx, 0, range_end);
                while let Some(item_id) = list.next_visible_item(cx) {
                    // break early if the item_id is lower than the previous item_ids
                    if item_id < last_drawn_item {
                        // break;
                    }

                    // let template = match item_id {
                    // // let template = match random_number() {
                    //     x if x % 4 == 0 => live_id!(ShortElement),
                    //     x if x % 4 == 1 => live_id!(MediumElement),
                    //     x if x % 4 == 2 => live_id!(MediumElement_2),
                    //     _ => live_id!(LongElement),
                    // };

                    let template = live_id!(ImageContainer);
                    let item = list.item(cx, item_id, template).unwrap();

                    if !self.items_artifacts_ids.contains_key(&item_id) {
                        let artifact_id = SEARCH_DATA[item_id as usize].id.clone();
                        self.items_artifacts_ids.insert(item_id, artifact_id.to_string());
                    }
                    let artifact_id = self.items_artifacts_ids.get(&item_id).unwrap();

                    let blob = {
                        cx.get_global::<NetworkImageCache>()
                            .get(&LiveId::from_str(&artifact_id))
                    };
    
                    if let Some(blob) = blob {
                        let image_data = blob.clone();
                        let imageref = item.image(id!(image));
                        if self.items_images_ready.get(&item_id).is_none() {
                            let _ = imageref.load_jpg_from_data(cx, &image_data);
                            self.items_images_ready.insert(item_id, true);
                            log!("updating item {item_id} with artifact image {artifact_id}");
                        }                            
                    } else {
                        // No image data found, request it
                        if item_id > 20 && self.items_images_ready.get(&item_id).is_none() && self.waiting_for_images == 0 {
                            let images_to_request = 15;
                            request_search_images(cx, item_id, images_to_request);
                            self.waiting_for_images = images_to_request;

                            // let texture_format = TextureFormat::VecBGRAu8_32 {
                            //     width: 4,
                            //     height: 4,
                            //     data: vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
                            // };
                    
                            // let default_texture = Texture::new_with_format(cx, texture_format);
                            // let imageref = item.image(id!(image));

                            // imageref.set_texture(cx, Some(default_texture));
                        }
                    }

                    // log!("🎨 🎨 🎨 {}", item_id);
                    item.draw_all(cx, scope);
                    last_drawn_item = item_id;
                }
            }
        }
        DrawStep::done()
    }
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.match_event(cx, event);
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
