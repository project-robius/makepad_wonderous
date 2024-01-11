use makepad_widgets::widget::WidgetCache;
use makepad_widgets::*;

use crate::shared::stack_view_action::StackViewAction;

use super::gallery_image::{GalleryImage, GalleryImageId};

pub const IMAGE_WIDTH: f64 = 250.;
pub const IMAGE_HEIGHT: f64 = 400.;
pub const IMAGE_PADDING: f64 = 20.;

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

        gallery_image_template: <GalleryImage> {}

        // This values are hardcoded from the width and height of the image
        image_offset = dvec2(270., 420.)

        animator: {
            swipe = {
                default: idle,
                idle = {
                    from: {all: Snap}
                    apply: {image_offset: vec2(0., 0.)}
                }

                horizontal = {
                    from: {all: Forward {duration: 0.1}}
                    apply: {image_offset: vec2(270., 0.)}
                }
                vertical = {
                    from: {all: Forward {duration: 0.1}}
                    apply: {image_offset: vec2(0., 420.)}
                }
                diagonal = {
                    from: {all: Forward {duration: 0.1}}
                    apply: {image_offset: vec2(270., 420.)}
                }

                reset = {
                    from: {all: Snap}
                    apply: {image_offset: vec2(0., 0.)}
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
    images_deps: Vec<LiveDependency>,
    #[live]
    gallery_image_template: Option<LivePtr>,

    #[animator]
    animator: Animator,

    #[rust]
    #[redraw]
    area: Area,
    #[live]
    image_offset: DVec2,

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

        for i in 0..self.grid_size.pow(2) {
            let image_id = LiveId(i as u64).into();
            let new_image = GalleryImage::new_from_ptr(cx, self.gallery_image_template);

            self.images.insert(image_id, new_image);
        }
    }
}

impl Widget for Gallery {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.animator_handle_event(cx, event);

        if !self.animator.is_track_animating(cx, id!(swipe)) {
            if self.animator.animator_in_state(cx, id!(swipe.vertical))
                || self.animator.animator_in_state(cx, id!(swipe.horizontal))
                || self.animator.animator_in_state(cx, id!(swipe.diagonal))
            {
                self.animator_play(cx, id!(swipe.idle));
            }
        }

        self.handle_click_and_swipe(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
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

            if let Some(image_path) = match image_idu64 {
                24 => Some(self.images_deps[0].as_str()),
                _ => Some(self.images_deps[image_idu64 as usize].as_str()),
            } {
                gallery_image.set_path(image_path.to_owned());
                gallery_image.set_size(dvec2(IMAGE_WIDTH, IMAGE_HEIGHT));
            }

            gallery_image.draw_all(cx, &mut Scope::with_data(&mut pos));
        }
        cx.end_turtle_with_area(&mut self.area);

        DrawStep::done()
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

        let padded_size = dvec2(
            (IMAGE_WIDTH + IMAGE_PADDING),
            (IMAGE_HEIGHT + IMAGE_PADDING),
        );

        let progress = self.image_offset / padded_size;

        // Check if the animation is complete
        if !self.animator.is_track_animating(cx, id!(swipe)) {
            if self.animator.animator_in_state(cx, id!(swipe.vertical))
                || self.animator.animator_in_state(cx, id!(swipe.horizontal))
                || self.animator.animator_in_state(cx, id!(swipe.diagonal))
            {
                self.previous_index = self.current_index;
                return current_offset;
            }
        }

        // Interpolate between the previous and current offsets
        let interpolated_offset = dvec2(
            previous_offset.x + (current_offset.x - previous_offset.x) * progress.x,
            previous_offset.y + (current_offset.y - previous_offset.y) * progress.y,
        );

        interpolated_offset
    }

    // TODO: Abstract this in a wrapper, so we keep the logic in one place for this and the overlay
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

                    self.set_index(new_index);

                    self.ready_to_swipe = false;
                }
            }
            Hit::FingerUp(fe) => {
                let mut swipe_vector = fe.abs - fe.abs_start;
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

                if swipe_vector.x.abs() < swipe_trigger_value
                    && swipe_vector.y.abs() < swipe_trigger_value
                    && is_clicking_center_image
                {
                    let widget_uid = self.widget_uid();
                    cx.widget_action(
                        widget_uid,
                        &scope.path,
                        GalleryAction::Selected(self.current_index),
                    );
                    cx.widget_action(
                        widget_uid,
                        &scope.path,
                        StackViewAction::ShowGalleryImageSlider,
                    );
                }
                // Reset variable for swiping
                self.ready_to_swipe = true;
            }

            _ => {}
        }
    }

    fn set_index(&mut self, value: i64) {
        if value < 0 || value >= self.image_count {
            return;
        }
        self.current_index = value;
    }
}

#[derive(Clone, DefaultNone, Debug)]
pub enum GalleryAction {
    None,
    Selected(i64),
}
