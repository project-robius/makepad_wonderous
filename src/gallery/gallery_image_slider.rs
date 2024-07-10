use makepad_widgets::widget::WidgetCache;
use makepad_widgets::*;

use crate::shared::network_images_cache::NetworkImageCache;

use super::{
    gallery_image::{GalleryImage, GalleryImageId},
    GALLERY_IMAGE_URLS,
};

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import makepad_draw::shader::std::*;

    import crate::shared::styles::*;
    import crate::shared::widgets::*;
    import crate::gallery::gallery_image::*;

    GalleryImageSlider = {{GalleryImageSlider}} {
        width: Fill, height: Fill
        align: {y: 0.5}

        offset: 0.

        gallery_image_template: <GalleryImage> {
            image: {
                fit: Horizontal,
                draw_bg: {
                    instance radius: 0.
                }
            }
        }

        animator: {
            swipe = {
                default: reset,
                reset = {
                    from: {all: Snap}
                    apply: {offset: 0.}
                }
                horizontal = {
                    from: {all: Forward {duration: 0.2}}
                    apply: {offset: 1.}
                }

            }
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
    #[animator]
    animator: Animator,

    #[live]
    gallery_image_template: Option<LivePtr>,
    #[live]
    offset: f64,
    #[rust]
    is_dragging: bool,
    #[rust]
    swipe_vector_x: f64,
    #[rust]
    last_swipe_vector_x: f64,
    #[rust]
    #[redraw]
    area: Area,
    #[rust]
    images: ComponentMap<GalleryImageId, GalleryImage>,

    #[rust]
    current_index: i64,
    #[rust]
    previous_index: i64,
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
        self.image_count = GALLERY_IMAGE_URLS.len() as i64;
        self.ready_to_swipe = true;
        self.is_dragging = true;
        self.swipe_vector_x = 0.;
        self.last_swipe_vector_x = 0.;
        for i in 0..self.image_count {
            let image_id = LiveId(i as u64).into();
            let new_image = GalleryImage::new_from_ptr(cx, self.gallery_image_template);

            self.images.insert(image_id, new_image);
        }
    }
}

impl Widget for GalleryImageSlider {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, _scope: &mut Scope) {
        if self.animator_handle_event(cx, event).is_animating() {
            self.redraw(cx);
        }
        if !self.animator.is_track_animating(cx, id!(swipe))
            && self.animator.animator_in_state(cx, id!(swipe.horizontal))
        {
            self.animator_play(cx, id!(swipe.reset));
        }
        self.handle_swipe(cx, event);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        cx.begin_turtle(walk, self.layout);
        let start_pos = cx.turtle().size() / dvec2(2., 2.);

        // Set image size to fit screen if it small enough, else set it fixed

        let image_width = if cx.turtle().size().x < 450. {
            cx.turtle().size().x
        } else {
            450.
        };
        let image_height = if cx.turtle().size().y * 0.7 < 520. {
            cx.turtle().size().y * 0.7
        } else {
            520.
        };

        let padding = 0.;
        let image_offset = self.calculate_current_offset(padding, image_width, cx);
        let padded_image_width = image_width + padding;
        for (image_id, gallery_image) in self.images.iter_mut() {
            let image_idu64 = image_id.0.get_value();

            let mut pos = start_pos
                + dvec2(
                    (image_idu64 as f64 * padded_image_width + image_offset.x) - image_width / 2.,
                    -(image_height / 2.) - 80.,
                );

            if let Some(image_id) = match image_idu64 {
                24 => Some(GALLERY_IMAGE_URLS[0]),
                _ => Some(GALLERY_IMAGE_URLS[image_idu64 as usize]),
            } {
                let blob = {
                    cx.cx
                        .get_global::<NetworkImageCache>()
                        .get(&LiveId::from_str(&image_id))
                };

                if let Some(blob) = blob {
                    let image_data = blob.clone();
                    if !gallery_image.is_image_ready() {
                        let _ = gallery_image.load_jpg_from_data(cx, &image_data);
                    }
                }
                gallery_image.set_size(cx, dvec2(image_width, image_height));
            }

            gallery_image.draw_all(cx, &mut Scope::with_data(&mut pos));
        }
        cx.end_turtle_with_area(&mut self.area);

        DrawStep::done()
    }
}

impl GalleryImageSlider {
    fn calculate_current_offset(&mut self, padding: f64, width: f64, cx: &mut Cx) -> DVec2 {
        let padded_image_width = width + padding;

        if self.is_dragging {
            let current_offset =
                (-padded_image_width) * self.current_index as f64 + self.swipe_vector_x;
            return dvec2(current_offset, 0.);
        }
        // Stays in same index
        if !self.animator.is_track_animating(cx, id!(swipe))
            && self.animator.animator_in_state(cx, id!(swipe.horizontal))
        {
            self.last_swipe_vector_x = 0.;
            self.previous_index = self.current_index;
            self.redraw(cx);
        }
        if self.current_index == self.previous_index {
            let last_offset =
                (-padded_image_width) * self.current_index as f64 + self.last_swipe_vector_x;

            let current_offset = (-padded_image_width) * self.current_index as f64;

            dvec2(
                last_offset
                    + (current_offset - last_offset)
                    + self.last_swipe_vector_x * (1. - self.offset),
                0.,
            )
        } else {
            let last_offset =
                (-padded_image_width) * self.previous_index as f64 + self.last_swipe_vector_x;

            let current_offset = (-padded_image_width) * self.current_index as f64;

            dvec2(
                last_offset + (current_offset - last_offset) * self.offset,
                0.,
            )
        }
    }

    fn handle_swipe(&mut self, cx: &mut Cx, event: &Event) {
        let swipe_trigger_value = 60.;
        match event.hits_with_capture_overload(cx, self.area, true) {
            Hit::FingerMove(fe) => {
                self.swipe_vector_x = fe.abs.x - fe.abs_start.x;
                self.last_swipe_vector_x = self.swipe_vector_x;
                self.redraw(cx);
                self.is_dragging = true;
                // only trigger swipe if it is larger than some pixels
            }
            Hit::FingerUp(_fe) => {
                // Reset variable for swiping
                self.ready_to_swipe = true;
                self.is_dragging = false;

                self.last_swipe_vector_x = self.swipe_vector_x;
                if self.swipe_vector_x.abs() > swipe_trigger_value {
                    if !self.ready_to_swipe {
                        return;
                    }

                    // self.last_swipe_direction = if self.swipe_vector_x > 0. { 1. } else { -1. };

                    let mut new_index = self.current_index;
                    // Handle prohibited swipe cases
                    // keep the index in range
                    if new_index < 0 || new_index > self.image_count - 1 {
                        return;
                    }

                    if self.swipe_vector_x.abs() > swipe_trigger_value {
                        new_index += if self.swipe_vector_x > 0. { -1 } else { 1 };
                        self.offset = self.swipe_vector_x.abs();
                    }

                    self.set_index(new_index, cx);

                    self.ready_to_swipe = false;
                }
                self.animator_play(cx, id!(swipe.horizontal));
                self.last_swipe_vector_x = self.swipe_vector_x;
                self.redraw(cx);
                self.swipe_vector_x = 0.;
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
    pub fn set_image_id(&mut self, id: i64, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.set_index(id, cx);
            inner.previous_index = id;
        }
    }
}
