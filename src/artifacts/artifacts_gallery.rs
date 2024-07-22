use makepad_widgets::*;
use std::collections::HashMap;
use crate::shared::{network_images_cache::NetworkImageCache, staggered_grid::{StaggeredGridWidgetRefExt, WidgetAllocationStatus}};

use super::{data::{great_wall_search_data::{SearchData, SEARCH_DATA}, image_search::request_search_images}, grid_image::GridImageWidgetRefExt};
const INITIAL_IMAGE_SEARCH_REQUESTS: usize = 20;
const MAX_ITEMS_IN_TIMEFRAME: usize = 222;

// TODO
//  - Network
//      - improve fetching and add biderectional fetching based on current position and availability (reduce cache size)
//
//  - Performance
//      - fix small visual issues when recycling
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
        width: Fill
        show_bg: true,
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
            
            ImageContainerTall = <FlexibleImageContainer> {
                height: 300.0  // Aspect ratio ~0.6
            }
            ImageContainerSquarish = <FlexibleImageContainer> {
                height: 200.0  // Aspect ratio ~1.0
            }
            ImageContainerWide = <FlexibleImageContainer> {
                height: 150.0  // Aspect ratio ~1.5
            }
            ImageContainerVeryWide = <FlexibleImageContainer> {
                height: 100.0  // Aspect ratio ~2.0
            }
        }
    }

    ArtifactsGallery = {{ArtifactsGallery}} {
        width: Fill, height: Fill
        flow: Down,
        align: {x: 0.5, y: 0.0},
        spacing: 10.0,

        SearchBar = <RoundedView> {
            // TODO: find a better way to override the height and spacing between stack navigation header and body
            margin: {top: 25., left: 15., right: 15.},
            width: Fill,
            height: 40.,
    
            show_bg: true,
            draw_bg: {
                color: #fff
            }
    
            padding: {top: 3, bottom: 3, left: 8}
    
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
                icon_walk: {width: Fit, height: 14}
            }
    
            input = <CustomTextInput> {
                width: 270,
                height: Fit,
                // FIXME: this is a hack to make the text properly align with the icon. 
                // align: {x: 0.0, y: 0.5}, is not enough.
                padding: {top: 13.}
                empty_message: "Search (ex. type or material)"
    
                draw_text: {
                    text_style:<REGULAR_TEXT>{font_size: 10},
                }
            }
        }

        search_results = <Label> {
            margin: { bottom: 5.0, top: 1.0 }
            draw_text: {
                text_style: <SUBTITLE_CAPTION>{font_size: 9},
                color: #e6945c,
            }
            text: "489 artifacts found, 222 in timeframe"
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
        self.last_drawn_items = vec![];

        let search_results = scope.props.get::<ArtifactSearchResults>().unwrap();
        let mut new_scope = Scope::empty();
        if search_results.data.is_empty() {
            return DrawStep::done();
        }
        
        if search_results.changed {
            self.did_initial_image_request = false;
            self.items_artifacts_ids.clear();
            self.items_images_ready.clear();
            self.waiting_for_images = 0;
        }

        if self.did_initial_image_request == false {
            request_search_images(cx, &search_results.data, 0, INITIAL_IMAGE_SEARCH_REQUESTS);
            self.did_initial_image_request = true;
        }

        while let Some(item) = self.view.draw_walk(cx, &mut new_scope, walk).step() {
            if let Some(mut list) = item.as_staggered_grid().borrow_mut() {
                list.set_repurpose_inactive_widgets(true);
                let range_end = (search_results.data.len() as usize - 1).min(MAX_ITEMS_IN_TIMEFRAME);
                if search_results.changed {
                    list.reset_and_scroll_top(cx);
                }

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

                    let template = self.get_template_for_item(item_id);
                    let (item, widget_status) = list.item(cx, item_id, template).unwrap();

                    if !self.items_artifacts_ids.contains_key(&item_id) {
                        let artifact_id = search_results.data[item_id as usize].id.clone();
                        self.items_artifacts_ids.insert(item_id, artifact_id.to_string());
                    }
                    let artifact_id = self.items_artifacts_ids.get(&item_id).unwrap();

                    let cached_image_data = {
                        cx.get_global::<NetworkImageCache>()
                            .get(&LiveId::from_str(&artifact_id))
                    };
    
                    if let Some(image_data) = cached_image_data {
                        let imageref = item.image(id!(image));
                        
                        // If the GridImage is uninitialized or dirty
                        if self.items_images_ready.get(&item_id).is_none() 
                            || widget_status == WidgetAllocationStatus::Repurposed
                            || search_results.changed {

                            let _ = imageref.load_jpg_from_data(cx, &image_data);
                            self.items_images_ready.insert(item_id, true);
                            // item.apply_over(cx,live!{ // comment this out if debugging with labels
                            //     show_bg: false,
                            // });

                            item.as_grid_image().set_animator_play(cx, id!(fade_in.on));
                            imageref.apply_over(cx, live!{
                                draw_bg: {
                                    texture_is_ready: 1.0
                                }
                            });

                            if let Some(dimensions) = get_jpeg_dimensions(&image_data) { 
                                let w = dimensions.0 as f32;
                                let h = dimensions.1 as f32;
                                imageref.apply_over(cx, live!{
                                    draw_bg: {
                                        source_size_w: (w)
                                        source_size_h: (h)
                                    }
                                });
                            }
                        }                            
                    } else {
                        // No image data found, request it
                        if item_id >= INITIAL_IMAGE_SEARCH_REQUESTS && self.items_images_ready.get(&item_id).is_none() && self.waiting_for_images == 0 {
                            let images_to_request = 20;
                            request_search_images(cx, &search_results.data, item_id, images_to_request);
                            self.waiting_for_images = images_to_request;
                        }

                        item.as_grid_image().set_animator_play(cx, id!(fade_in.off));
                        let imageref = item.image(id!(image));
                        imageref.apply_over(cx, live!{
                            draw_bg: {
                                texture_is_ready: 0.0
                            }
                        });

                    }

                    // log!("🎨 🎨 🎨 {}", item_id);
                    // item.label(id!(lbl)).set_text(&format!("{artifact_id}")); // debugging
                    item.draw_all(cx, &mut new_scope);

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

impl ResultsGrid {
    fn get_template_for_item(&self, item_id: usize) -> LiveId {
        // This mimics the Flutter version: (data.id % 10) / 15 + 0.6
        let aspect_ratio = (item_id % 10) as f64 / 15.0 + 0.6;
        
        match aspect_ratio {
            r if r < 0.8 => live_id!(ImageContainerTall),
            r if r < 1.2 => live_id!(ImageContainerSquarish),
            r if r < 1.7 => live_id!(ImageContainerWide),
            _ => live_id!(ImageContainerVeryWide),
        }
    }
}

#[derive(Widget, Live)]
pub struct ArtifactsGallery {
    #[deref]
    view: View,

    #[rust]
    results: Vec<SearchData>,
    #[rust]
    results_changed: bool,
}

impl LiveHook for ArtifactsGallery {
    fn after_new_from_doc(&mut self, _cx:&mut Cx) {
        self.results = SEARCH_DATA.to_vec();
    }
}

impl MatchEvent for ArtifactsGallery {
    fn handle_actions(&mut self, _cx: &mut Cx, actions: &Actions) {
        if let Some(keywords) = self.text_input(id!(input)).changed(actions) {
            self.results = SEARCH_DATA.to_vec();
            
            if !keywords.is_empty() {
                let previous_results_len = self.results.len();
                self.results.retain(|artifact| {
                    artifact.tags.to_lowercase().contains(&keywords)
                        || artifact.title.to_lowercase().contains(&keywords)
                });

                if previous_results_len != self.results.len() {
                    self.results_changed = true;
                }
            } else {
                self.results_changed = true;
            }

            if self.results.is_empty() {
                self.view.label(id!(search_results)).set_text("No artifacts found");
            } else {
                let results_text = &format!("{} artifacts found, {} in timeframe", 
                    self.results.len(),
                    self.results.len().min(MAX_ITEMS_IN_TIMEFRAME),
                );
                self.view.label(id!(search_results)).set_text(&results_text);
            }
        }

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

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        let binding = ArtifactSearchResults {
            data: self.results.clone(),
            changed: self.results_changed,
        };

        let mut scope = Scope::with_props(&binding);
        self.results_changed = false;
        self.view.draw_walk(cx, &mut scope, walk)
    }
}

struct ArtifactSearchResults {
    data: Vec<SearchData>,
    changed: bool,
}


// This function is a simple way to get the dimensions of a JPEG image without decoding the whole image.
// TODO: We should find a way to do this within the Makepad Texture system to avoid having this logic on the application side.
fn get_jpeg_dimensions(data: &[u8]) -> Option<(u16, u16)> {
    if data.len() < 2 || data[0] != 0xFF || data[1] != 0xD8 {
        return None; // Not a JPEG
    }
    
    let mut i = 2;
    while i + 8 < data.len() {
        if data[i] == 0xFF && (0xC0..=0xCF).contains(&data[i + 1]) && data[i + 1] != 0xC4 && data[i + 1] != 0xC8 {
            let height = u16::from_be_bytes([data[i + 5], data[i + 6]]);
            let width = u16::from_be_bytes([data[i + 7], data[i + 8]]);
            return Some((width, height));
        }    
        i += 1;
    }
    None
}
