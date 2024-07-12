use makepad_widgets::*;
use std::collections::HashMap;
use crate::shared::{network_images_cache::NetworkImageCache, staggered_grid::{StaggeredGridWidgetRefExt, WidgetAllocationStatus}};

use super::data::{great_wall_search_data::SEARCH_DATA, image_search::request_search_images};
const INITIAL_IMAGE_SEARCH_REQUESTS: usize = 20;

// TODO
//  - Network
//      - ✓ make sure there's always a buffer of images loaded so that scrolling is smooth.     
//      - ✓ make sure responses correspond to their widget, or at least make sure the images are only loaded once into the cache.  
//
//  - Performance
//      - ✓ recycle widgets instances in StaggeredGrid instead of creating new ones endlessly
//      - ✓ add a spinner images loading on slow networks
//      - android is leaking graphics memory
//      - fix small visual issues when recycling
//
//  - Platforms
//      - ✓ random text bug in wasm and android
//
//  - Timeline navigator
//      - the grid range represents the timeline of artifacts
// 
//  - Search
//
//  - Artifact detail view: 
//      - items should be clickable and a sub-page should slide-in with more information about the artifact 

// Maybe
//  - pre-allocate item sizes and then load the images into the items depending on their dimensions.
//  - currently we cache the raw image data, but we could also cache the textures and re-use them.


live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;

    import crate::shared::styles::*;
    import crate::shared::widgets::*;
    import crate::shared::icon::*;
    import makepad_draw::shader::std::*;

    import crate::shared::staggered_grid::*;
    import crate::artifacts::grid_image::*;

    CALENDAR_ICON = dep("crate://self/resources/icons/calendar.svg")
    SEARCH_ICON = dep("crate://self/resources/icons/search.svg")

    FlexibleImageContainer = <GridImage> {
        height: Fit
        show_bg: true,
        draw_bg: {
            // color: #0f0e0c
            instance radius: 4.0,
        }
        // lbl = <Label> { // debugging
        //     draw_text:{
        //         text_style: <SUBTITLE_CAPTION>{font_size: 12},
        //         color: #fff
        //     }
        // }
        align: {x: 0.5, y: 0.5}
    }

    ResultsGrid = {{ResultsGrid}} {
        list = <StaggeredGrid>{
            columns_number: 2
            column_spacing: 5.0
            ImageContainer = <FlexibleImageContainer> {}
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

    #[rust]
    last_drawn_items: Vec<ItemId>,
    #[rust]
    all_drawn_items: Vec<(WidgetRef, ItemId)>,
}

type ItemId = usize;

impl MatchEvent for ResultsGrid {
    fn handle_network_responses(&mut self, cx: &mut Cx, responses: &NetworkResponsesEvent) {
        for event in responses {
            if event.request_id == live_id!(image_search) {
                match &event.response {
                    NetworkResponse::HttpResponse(response) => {
                        if response.status_code == 200 {
                            if let Some(body) = response.get_body() {
                                cx.get_global::<NetworkImageCache>()
                                    .insert(response.metadata_id, body);
                                if self.waiting_for_images > 0 {
                                    self.waiting_for_images -= 1;
                                }
                                self.redraw(cx);
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
}

impl Widget for ResultsGrid {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        if self.did_initial_image_request == false {
            request_search_images(cx, 0, INITIAL_IMAGE_SEARCH_REQUESTS);
            self.did_initial_image_request = true;
        }

        self.last_drawn_items = vec![];

        while let Some(item) = self.view.draw_walk(cx, scope, walk).step() {
            if let Some(mut list) = item.as_staggered_grid().borrow_mut() {
                list.set_repurpose_inactive_widgets(true);
                let range_end = SEARCH_DATA.len() as usize - 1;

                let mut first_drawn_item = None;
                list.set_item_range(cx, 0, range_end);
                while let Some(item_id) = list.next_visible_item(cx) {
                    // break early if the item_id is lower than the previous item_ids
                    // if item_id < last_drawn_item {
                        // break;
                    // }

                    if first_drawn_item.is_none() {
                        first_drawn_item = Some(item_id);
                    }

                    // let template = match item_id {
                    // // let template = match random_number() {
                    //     x if x % 4 == 0 => live_id!(ShortElement),
                    //     x if x % 4 == 1 => live_id!(MediumElement),
                    //     x if x % 4 == 2 => live_id!(MediumElement_2),
                    //     _ => live_id!(LongElement),
                    // };

                    let template = live_id!(ImageContainer);
                    let (item, widget_status) = list.item(cx, item_id, template).unwrap();

                    if !self.items_artifacts_ids.contains_key(&item_id) {
                        let artifact_id = SEARCH_DATA[item_id as usize].id.clone();
                        self.items_artifacts_ids.insert(item_id, artifact_id.to_string());
                    }
                    let artifact_id = self.items_artifacts_ids.get(&item_id).unwrap();

                    let cached_image_data = {
                        cx.get_global::<NetworkImageCache>()
                            .get(&LiveId::from_str(&artifact_id))
                    };
    
                    if let Some(image_data) = cached_image_data {
                        let imageref = item.image(id!(image));
                        if self.items_images_ready.get(&item_id).is_none() || widget_status == WidgetAllocationStatus::Repurposed {
                            let _ = imageref.load_jpg_from_data(cx, &image_data);
                            self.items_images_ready.insert(item_id, true);
                            // item.apply_over(cx,live!{ // comment this out if debugging with labels
                            //     show_bg: false,
                            // });

                            imageref.apply_over(cx, live!{
                                draw_bg: {
                                    texture_is_ready: 1.0
                                }
                            });
                        }                            
                    } else {
                        // No image data found, request it
                        if item_id >= INITIAL_IMAGE_SEARCH_REQUESTS && self.items_images_ready.get(&item_id).is_none() && self.waiting_for_images == 0 {
                            let images_to_request = 20;
                            request_search_images(cx, item_id, images_to_request);
                            self.waiting_for_images = images_to_request;
                        }

                        let imageref = item.image(id!(image));
                        imageref.apply_over(cx, live!{
                            draw_bg: {
                                texture_is_ready: 0.0
                            }
                        });

                    }

                    // log!("🎨 🎨 🎨 {}", item_id);
                    // item.label(id!(lbl)).set_text(&format!("{item_id}")); // debugging
                    item.draw_all(cx, scope);

                    self.last_drawn_items.push(item_id);

                    if self.all_drawn_items.iter().find(|(_, id)| *id == item_id).is_none() {
                        self.all_drawn_items.push((item, item_id));
                    }
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
