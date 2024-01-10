use makepad_widgets::widget::WidgetCache;
use makepad_widgets::*;

use crate::shared::stack_view_action::StackViewAction;

use super::{
    gallery_image::{GalleryImage, GalleryImageId, IMAGE_HEIGHT, IMAGE_WIDTH},
    slider_image::{SliderImage, SliderImageId},
};

pub const SLIDER_IMAGE_WIDTH: f64 = 374.;
pub const SLIDER_IMAGE_HEIGHT: f64 = 412.2;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import makepad_draw::shader::std::*;

    import crate::shared::styles::*;
    import crate::shared::widgets::*;
    // import crate::gallery::gallery_image::*;
    import crate::gallery::slider_image::*;
    import crate::gallery::gallery_overlay::*;

    IMG_CONTENT = dep("crate://self/resources/images/great-wall-content-1.jpg")

    GalleryImageSlider = {{GalleryImageSlider}} {
        width: Fill, height: Fill

        images_deps: [
            dep("crate://self/resources/images/gallery/great-wall/gallery-great-wall-0.jpg"),
            dep("crate://self/resources/images/gallery/great-wall/gallery-great-wall-1.jpg"),
            dep("crate://self/resources/images/gallery/great-wall/gallery-great-wall-2.jpg"),
            dep("crate://self/resources/images/gallery/great-wall/gallery-great-wall-3.jpg"),
            dep("crate://self/resources/images/gallery/great-wall/gallery-great-wall-4.jpg"),
            dep("crate://self/resources/images/gallery/great-wall/gallery-great-wall-5.jpg"),
            dep("crate://self/resources/images/gallery/great-wall/gallery-great-wall-6.jpg"),
            dep("crate://self/resources/images/gallery/great-wall/gallery-great-wall-7.jpg"),
            dep("crate://self/resources/images/gallery/great-wall/gallery-great-wall-8.jpg"),
            dep("crate://self/resources/images/gallery/great-wall/gallery-great-wall-9.jpg"),
            dep("crate://self/resources/images/gallery/great-wall/gallery-great-wall-10.jpg"),
            dep("crate://self/resources/images/gallery/great-wall/gallery-great-wall-11.jpg"),
            dep("crate://self/resources/images/gallery/great-wall/gallery-great-wall-12.jpg"),
            dep("crate://self/resources/images/gallery/great-wall/gallery-great-wall-13.jpg"),
            dep("crate://self/resources/images/gallery/great-wall/gallery-great-wall-14.jpg"),
            dep("crate://self/resources/images/gallery/great-wall/gallery-great-wall-15.jpg"),
            dep("crate://self/resources/images/gallery/great-wall/gallery-great-wall-16.jpg"),
            dep("crate://self/resources/images/gallery/great-wall/gallery-great-wall-17.jpg"),
            dep("crate://self/resources/images/gallery/great-wall/gallery-great-wall-18.jpg"),
            dep("crate://self/resources/images/gallery/great-wall/gallery-great-wall-19.jpg"),
            dep("crate://self/resources/images/gallery/great-wall/gallery-great-wall-20.jpg"),
            dep("crate://self/resources/images/gallery/great-wall/gallery-great-wall-21.jpg"),
            dep("crate://self/resources/images/gallery/great-wall/gallery-great-wall-22.jpg"),
            dep("crate://self/resources/images/gallery/great-wall/gallery-great-wall-23.jpg"),
        ]

        gallery_image_template: <SliderImage> {

        }

    }
}

#[derive(Live, Widget)]
pub struct GalleryImageSlider {
    #[walk]
    walk: Walk,
    #[layout]
    layout: Layout,
    #[deref]
    view: View,

    #[live]
    images_deps: Vec<LiveDependency>,
    #[live]
    gallery_image_template: Option<LivePtr>,

    #[rust]
    #[redraw]
    area: Area,
    #[rust]
    images: ComponentMap<SliderImageId, SliderImage>,

    #[rust]
    grid_size: i64,
    #[rust]
    current_index: i64,
    #[rust]
    image_count: i64,
    #[rust]
    ready_to_swipe: bool,
}

impl LiveHook for GalleryImageSlider {
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
        self.ready_to_swipe = true;
        for i in 0..self.grid_size.pow(2) {
            let image_id = LiveId(i as u64).into();
            let new_image = SliderImage::new_from_ptr(cx, self.gallery_image_template);

            self.images.insert(image_id, new_image);
        }
    }
}

impl Widget for GalleryImageSlider {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.handle_click_and_swipe(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        cx.begin_turtle(walk, self.layout);
        let start_pos = cx.turtle().size() / dvec2(2., 2.);
        let image_width = cx.turtle().size().x;
        let image_height = cx.turtle().size().y * 0.7;
        let padding = 20.;
        let image_offset = self.calculate_current_offset(padding, image_width, image_height);
        let padded_image_width = image_width + padding;
        let padded_image_height = image_height + padding;
        for (image_id, gallery_image) in self.images.iter_mut() {
            let image_idu64 = image_id.0.get_value();
            let col = (image_idu64 % self.grid_size as u64) as f64;
            let row = (image_idu64 / self.grid_size as u64) as f64;

            let mut pos = start_pos
                + dvec2(
                    (image_idu64 as f64 * padded_image_width + image_offset.x) - image_width / 2.,
                    -(image_height / 2.) + 20.,
                );

            if let Some(image_path) = match image_idu64 {
                24 => Some(self.images_deps[0].as_str()),
                _ => Some(self.images_deps[image_idu64 as usize].as_str()),
            } {
                gallery_image.set_path(image_path.to_owned());
            }

            gallery_image.draw_all(cx, &mut Scope::with_data(&mut pos));
        }
        cx.end_turtle_with_area(&mut self.area);

        DrawStep::done()
    }
}

impl GalleryImageSlider {
    fn calculate_current_offset(&self, padding: f64, width: f64, height: f64) -> DVec2 {
        let padded_image_width = width + padding;

        let col = (self.current_index % self.grid_size) as f64;
        let row = (self.current_index / self.grid_size) as f64;
        let indexed_offset = dvec2((-padded_image_width) * self.current_index as f64, 0.);

        return indexed_offset;
    }

    fn handle_click_and_swipe(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let swipe_trigger_value = 60.;
        let diagonal_trigger_value = swipe_trigger_value / 2.;
        match event.hits_with_capture_overload(cx, self.area, true) {
            Hit::FingerMove(fe) => {
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
                    }
                    // Handle prohibited swipe cases
                    // keep the index in range
                    if new_index < 0 || new_index > self.grid_size.pow(2) - 1 {
                        return;
                    }
                    self.set_index(new_index, cx);

                    self.ready_to_swipe = false;
                }
            }
            Hit::FingerUp(fe) => {
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
}

impl GalleryImageSliderRef {
    pub fn set_image_id(&mut self, cx: &mut Cx, id: i64) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_index(id, cx);
        }
    }
}
