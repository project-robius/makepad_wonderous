use makepad_widgets::widget::WidgetCache;
use makepad_widgets::*;
use std::rc::Rc;

use crate::shared::network_images_cache::NetworkImageCache;

use super::{
    gallery_image::{GalleryImage, GalleryImageId},
    GALLERY_IMAGE_URLS,
};

pub const IMAGE_WIDTH: f64 = 270.;
pub const IMAGE_HEIGHT: f64 = 430.;
pub const IMAGE_PADDING: f64 = 20.;
pub const ASSETS_BASE_URL: &str = "https://project-robius.github.io/makepad_wonderous_assets/images";

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import makepad_draw::shader::std::*;

    import crate::shared::styles::*;
    import crate::shared::widgets::*;
    import crate::gallery::gallery_image::*;
    import crate::gallery::gallery_overlay::*;

    IMG_CONTENT = dep("crate://self/resources/images/great-wall-content-1.jpg")

    Gallery = {{Gallery}} {
        width: Fill, height: Fill

        gallery_image_template: <GalleryImage> {
            image: <Image> {
                fit: Biggest
                draw_bg: {
                    instance radius: 3.
                    instance scale: 0.0
                    instance down: 0.0
                    instance size: vec2(270., 430.)
                    instance opacity: 1.0

                    fn pixel(self) -> vec4 {
                        let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                        sdf.box(
                            1,
                            1,
                            self.size.x - 2.0,
                            self.size.y - 2.0,
                            max(1.0, self.radius)
                        )
                        let max_scale = vec2(0.9);
                        let scale = mix(vec2(1.0), max_scale, self.scale);
                        let pan = mix(vec2(0.0), (vec2(1.0) - max_scale) * 0.5, self.scale);

                        let color = self.get_color_scale_pan(scale, pan) + mix(vec4(0.0), vec4(0.1), 0);

                        let final_color = Pal::premul(vec4(color.xyz, color.w * self.opacity))

                        sdf.fill_keep(final_color);
                        return sdf.result;
                    }
                }
            }
        }

        swipe_progress: vec2(1., 1.)

        animator: {
            swipe = {
                default: idle,
                idle = {
                    from: {all: Snap}
                    apply: {swipe_progress: vec2(0., 0.)}
                }

                horizontal = {
                    from: {all: Forward {duration: 0.15}}
                    apply: {swipe_progress: vec2(1., 0.)}
                }
                vertical = {
                    from: {all: Forward {duration: 0.15}}
                    apply: {swipe_progress: vec2(0., 1.)}
                }
                diagonal = {
                    from: {all: Forward {duration: 0.15}}
                    apply: {swipe_progress: vec2(1., 1.)}
                }

                reset = {
                    from: {all: Snap}
                    apply: {swipe_progress: vec2(0., 0.)}
                }
            }
        }
    }

    GalleryScreen = <View> {
        width: Fill, height: Fill
        flow: Overlay
        show_bg: true,
        draw_bg: {
            color: #000
        }

        <Gallery> {}
        black_overlay = <GalleryOverlay> {}
    }
}

#[derive(Live, Widget)]
pub struct Gallery {
    #[walk]
    walk: Walk,
    #[layout]
    layout: Layout,
    #[deref]
    view: View,

    #[live]
    gallery_image_template: Option<LivePtr>,

    #[animator]
    animator: Animator,

    #[rust]
    #[redraw]
    area: Area,
    #[live]
    swipe_progress: DVec2,

    #[rust]
    images: ComponentMap<GalleryImageId, GalleryImage>,

    #[rust]
    grid_size: i64,
    #[rust]
    current_index: i64,
    #[rust]
    previous_index: i64,
    #[rust]
    image_count: i64,
    #[rust]
    ready_to_swipe: bool,
    #[rust]
    requested_images: bool,
}

impl LiveHook for Gallery {
    fn after_apply(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) {
        for gallery_image in self.images.values_mut() {
            if let Some(index) = nodes.child_by_name(index, live_id!(gallery_image).as_field()) {
                gallery_image.apply(cx, apply, index, nodes);
            }
        }
    }

    fn after_new_from_doc(&mut self, cx: &mut Cx) {
        self.grid_size = 5;
        self.image_count = self.grid_size.pow(2);
        self.current_index = self.grid_size.pow(2) / 2;
        self.previous_index = self.current_index;
        self.ready_to_swipe = true;

        if !cx.has_global::<NetworkImageCache>() {
            cx.set_global(NetworkImageCache::new());
        }

        for i in 0..self.grid_size.pow(2) {
            let image_id = LiveId(i as u64).into();
            let new_image = GalleryImage::new_from_ptr(cx, self.gallery_image_template);

            self.images.insert(image_id, new_image);
        }
    }
}

impl Widget for Gallery {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.match_event(cx, event);
        for image in self.images.values_mut() {
            image.handle_event(cx, event, scope);
        }
        self.view.handle_event(cx, event, scope);
        if self.animator_handle_event(cx, event).is_animating() {
            self.redraw(cx);
        }
        if !self.animator.is_track_animating(cx, id!(swipe))
            && (self.animator.animator_in_state(cx, id!(swipe.vertical))
                || self.animator.animator_in_state(cx, id!(swipe.horizontal))
                || self.animator.animator_in_state(cx, id!(swipe.diagonal)))
        {
            self.animator_play(cx, id!(swipe.idle));
        }

        self.handle_click_and_swipe(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        if !self.requested_images {
            self.request_images(cx);
            self.requested_images = true;
        }

        cx.begin_turtle(walk, self.layout);

        let start_pos = cx.turtle().size() / dvec2(2., 2.);
        let padding = 20.;
        let indexed_offset = self.calculate_indexed_offset(cx);
        let padded_image_width = IMAGE_WIDTH + padding;
        let padded_image_height = IMAGE_HEIGHT + padding;

        for (image_id, gallery_image) in self.images.iter_mut() {
            let image_idu64 = image_id.0.get_value();
            let col = (image_idu64 % self.grid_size as u64) as f64;
            let row = (image_idu64 / self.grid_size as u64) as f64;

            let mut pos = start_pos
                + dvec2(
                    (col * padded_image_width + indexed_offset.x) - IMAGE_WIDTH / 2.,
                    (row * (padded_image_height) + indexed_offset.y) - IMAGE_HEIGHT / 2.,
                );

            if let Some(image_id) = match image_idu64 {
                24 => Some(GALLERY_IMAGE_URLS[0]),
                _ => Some(GALLERY_IMAGE_URLS[image_idu64 as usize]),
            } {
                let is_center_image = image_idu64 == (GALLERY_IMAGE_URLS.len() / 2) as u64;

                let cached_image_data = {
                    cx.get_global::<NetworkImageCache>()
                        .get(&LiveId::from_str(&image_id))
                };

                if let Some(image_data) = cached_image_data {
                    if !gallery_image.is_image_ready() {
                        let _ = gallery_image.load_jpg_from_data(cx, &image_data);

                        // Make the center image immediately visible
                        if is_center_image {
                            gallery_image.animator_cut(cx, id!(fade_in.on));
                        } else {
                            gallery_image.animator_play(cx, id!(fade_in.on));
                        }
                    }
                }

                gallery_image.set_size(cx, dvec2(IMAGE_WIDTH, IMAGE_HEIGHT));
            }

            gallery_image.draw_all(cx, &mut Scope::with_data(&mut pos));
        }
        cx.end_turtle_with_area(&mut self.area);

        DrawStep::done()
    }
}

impl MatchEvent for Gallery {
    fn handle_network_responses(&mut self, cx: &mut Cx, responses: &NetworkResponsesEvent) {
        for event in responses {
            if event.request_id == live_id!(gallery_image) {
                match &event.response {
                    NetworkResponse::HttpResponse(response) => {
                        if response.status_code == 200 {
                            if let Some(body) = response.get_body() {
                                cx.get_global::<NetworkImageCache>()
                                    .insert(response.metadata_id, body);
                                self.redraw(cx);
                            }
                        } else {
                            error!("Error fetching gallery image: {:?}", response);
                        }
                    }
                    NetworkResponse::HttpRequestError(error) => {
                        println!("Error fetching gallery image: {:?}", error);
                    }
                    _ => (),
                }
            }
        }
    }
}

impl Gallery {
    fn calculate_indexed_offset(&mut self, cx: &mut Cx) -> DVec2 {
        let current_col = (self.current_index % self.grid_size) as f64;
        let current_row = (self.current_index / self.grid_size) as f64;

        let previous_col = (self.previous_index % self.grid_size) as f64;
        let previous_row = (self.previous_index / self.grid_size) as f64;

        let current_offset = dvec2(
            -(IMAGE_WIDTH + IMAGE_PADDING) * current_col,
            -(IMAGE_HEIGHT + IMAGE_PADDING) * current_row,
        );
        let previous_offset = dvec2(
            -(IMAGE_WIDTH + IMAGE_PADDING) * previous_col,
            -(IMAGE_HEIGHT + IMAGE_PADDING) * previous_row,
        );

        // Check if the animation is complete
        if !self.animator.is_track_animating(cx, id!(swipe))
            && (self.animator.animator_in_state(cx, id!(swipe.vertical))
                || self.animator.animator_in_state(cx, id!(swipe.horizontal))
                || self.animator.animator_in_state(cx, id!(swipe.diagonal)))
        {
            self.previous_index = self.current_index;
            return current_offset;
        }

        // Interpolate between the previous and current offsets

        dvec2(
            previous_offset.x + (current_offset.x - previous_offset.x) * self.swipe_progress.x,
            previous_offset.y + (current_offset.y - previous_offset.y) * self.swipe_progress.y,
        )
    }

    // TODO: Abstract this in a wrapper, so we keep the logic in one place for this and the overlay
    fn handle_click_and_swipe(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        match event.hits_with_capture_overload(cx, self.area, true) {
            Hit::FingerMove(fe) => {
                let swipe_trigger_value = 40.;
                let diagonal_trigger_value = swipe_trigger_value / 2.;
                let mut swipe_vector = fe.abs - fe.abs_start;
                // Negate y values because makepad's y axis grows to the south
                swipe_vector.y = -swipe_vector.y;

                // only trigger swipe if it is larger than some pixels

                if (swipe_vector.x.abs() > swipe_trigger_value)
                    || (swipe_vector.y.abs() > swipe_trigger_value)
                {
                    if !self.ready_to_swipe {
                        return;
                    }

                    let mut new_index = self.current_index;

                    // compensate diagonal swipe case (both trigger the diagonal value)
                    if swipe_vector.x.abs() > diagonal_trigger_value {
                        new_index += if swipe_vector.x > 0. { -1 } else { 1 };
                        // play animations (shrink overlay)
                        self.animator_play(cx, id!(swipe.horizontal));
                    }
                    if swipe_vector.y.abs() > diagonal_trigger_value {
                        new_index += self.grid_size * if swipe_vector.y > 0. { 1 } else { -1 };
                        // play animations (shrink overlay)
                        // self.animator_play(cx, id!(swipe.reset));

                        self.animator_play(cx, id!(swipe.vertical));
                    }
                    // play animation on diagonal swipe
                    if swipe_vector.y.abs() > diagonal_trigger_value
                        && swipe_vector.x.abs() > diagonal_trigger_value
                    {
                        self.animator_play(cx, id!(swipe.diagonal));
                    }

                    // Handle prohibited swipe cases
                    // keep the index in range
                    if new_index < 0 || new_index > self.grid_size.pow(2) - 1 {
                        return;
                    }
                    // hitting right limit
                    if swipe_vector.x < 0. && new_index % self.grid_size == 0 {
                        return;
                    }
                    // hitting left limit
                    if swipe_vector.x > 0. && new_index % self.grid_size == self.grid_size - 1 {
                        return;
                    }

                    // Play animations
                    if let Some(previous_image) = self
                        .images
                        .get_mut(&LiveId(self.current_index as u64).into())
                    {
                        previous_image.animator_play(cx, id!(zoom.off));
                    }
                    if let Some(new_image) = self.images.get_mut(&LiveId(new_index as u64).into()) {
                        new_image.animator_play(cx, id!(zoom.on));
                    }

                    self.set_index(new_index, cx);

                    self.ready_to_swipe = false;
                }
            }
            Hit::FingerUp(fe) => {
                let mut swipe_vector = fe.abs - fe.abs_start;
                let click_trigger_value = 10.;
                // Negate y values because makepad's y axis grows to the south
                swipe_vector.y = -swipe_vector.y;

                // Clicked image if its the one in the center
                let center_image = dvec2(
                    (fe.rect.size.x - IMAGE_WIDTH) / 2.,
                    (fe.rect.size.y - IMAGE_HEIGHT) / 2.,
                );

                let is_clicking_center_image = center_image.x <= fe.abs_start.x
                    && fe.abs_start.x <= center_image.x + IMAGE_WIDTH
                    && center_image.y <= fe.abs_start.y
                    && fe.abs_start.y <= center_image.y + IMAGE_HEIGHT;

                if swipe_vector.x.abs() < click_trigger_value
                    && swipe_vector.y.abs() < click_trigger_value
                    && is_clicking_center_image
                {
                    let widget_uid = self.widget_uid();
                    cx.widget_action(
                        widget_uid,
                        &scope.path,
                        GalleryGridAction::Selected(self.current_index),
                    );
                    cx.widget_action(
                        widget_uid,
                        &scope.path,
                        StackNavigationAction::NavigateTo(live_id!(
                            gallery_image_slider_stack_view
                        )),
                    );
                }
                // Reset variable for swiping
                self.ready_to_swipe = true;
            }

            _ => {}
        }
    }

    fn set_index(&mut self, value: i64, cx: &mut Cx) {
        if value < 0 || value >= self.image_count {
            return;
        }
        self.current_index = value;
        self.redraw(cx);
    }

    fn request_images(&self, cx: &mut Cx) {
        let middle = GALLERY_IMAGE_URLS.len() / 2;
    
        // Helper function to create and send an HTTP request
        let mut fetch_image = |url: &str| {
            let full_url = format!(
                "{}/{}-{}.jpg",
                ASSETS_BASE_URL, url, 800
            );
            let request_id = live_id!(gallery_image);
            let mut request = HttpRequest::new(full_url, HttpMethod::GET);
            request.metadata_id = LiveId::from_str(url);
            cx.http_request(request_id, request);
        };
    
        // Iterate outwards from the middle
        for offset in 0..=middle {
            if let Some(right_url) = GALLERY_IMAGE_URLS.get(middle + offset) {
                fetch_image(right_url);
            }
    
            if offset != 0 {
                if let Some(left_url) = GALLERY_IMAGE_URLS.get(middle - offset) {
                    fetch_image(left_url);
                }
            }
        }
    }
}

impl GalleryRef {
    pub fn set_image_id(&mut self, id: i64, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_index(id, cx);
        }
    }
}

#[derive(Clone, DefaultNone, Debug)]
pub enum GalleryGridAction {
    None,
    Selected(i64),
}
