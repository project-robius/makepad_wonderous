use makepad_widgets::widget::WidgetCache;
use makepad_widgets::*;

use super::gallery_image::{GalleryImage, GalleryImageId, IMAGE_HEIGHT, IMAGE_WIDTH};

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;

    import crate::shared::styles::*;
    import crate::shared::widgets::*;
    import crate::gallery::gallery_image::*;

    IMG_CONTENT = dep("crate://self/resources/images/great-wall-content-1.jpg")

    Gallery = {{Gallery}} {
        width: Fill, height: Fill

        gallery_image_template: <GalleryImage> {
            image: <RotatedImage> {
                source: (IMG_CONTENT)
            }
        }

    }

    GalleryScreen = <View> {
        width: Fill, height: Fill

        show_bg: true,
        draw_bg: {
            color: #000
        }

        <Gallery> {}
    }

}

#[derive(Live)]
pub struct Gallery {
    #[deref]
    view: View,
    #[walk]
    walk: Walk,
    #[layout]
    layout: Layout,

    // #[animator]
    // animator: Animator,
    #[live]
    gallery_image_template: Option<LivePtr>,
    #[rust]
    images: ComponentMap<GalleryImageId, GalleryImage>,

    #[rust]
    last_finger_abs_pos: DVec2,
    #[rust]
    grid_size: i64,
    #[rust]
    current_index: i64,
    #[rust]
    image_count: i64,
    #[rust]
    ready_to_swipe: bool,
}

impl LiveHook for Gallery {
    fn before_live_design(cx: &mut Cx) {
        register_widget!(cx, Gallery);
    }

    fn after_apply(&mut self, cx: &mut Cx, from: ApplyFrom, index: usize, nodes: &[LiveNode]) {
        for gallery_image in self.images.values_mut() {
            if let Some(index) = nodes.child_by_name(index, live_id!(gallery_image).as_field()) {
                gallery_image.apply(cx, from, index, nodes);
            }
        }
    }

    fn after_new_from_doc(&mut self, cx: &mut Cx) {
        self.grid_size = 5;
        self.image_count = self.grid_size.pow(2);
        self.current_index = self.grid_size.pow(2) / 2;
        self.ready_to_swipe = true;
        for i in 0..self.grid_size.pow(2) {
            let image_id = LiveId(i as u64).into();
            let new_image = GalleryImage::new_from_ptr(cx, self.gallery_image_template);

            self.images.insert(image_id, new_image);
        }
    }
}

impl Widget for Gallery {
    fn handle_widget_event_with(
        &mut self,
        cx: &mut Cx,
        event: &Event,
        dispatch_action: &mut dyn FnMut(&mut Cx, WidgetActionItem),
    ) {
        let uid = self.widget_uid();
        self.handle_event_with(cx, event, &mut |cx, action| {
            dispatch_action(cx, WidgetActionItem::new(action.into(), uid));
        });
        self.handle_swipe(cx, event);
    }

    fn walk(&mut self, cx: &mut Cx) -> Walk {
        self.view.walk(cx)
        // self.walk
    }

    fn redraw(&mut self, cx: &mut Cx) {
        self.view.redraw(cx);
    }

    // fn find_widgets(&mut self, path: &[LiveId], cached: WidgetCache, results: &mut WidgetSet) {
    //     self.view.find_widgets(path, cached, results);
    // }

    fn draw_walk_widget(&mut self, cx: &mut Cx2d, walk: Walk) -> WidgetDraw {
        let _ = self.view.draw_walk_widget(cx, walk);
        self.draw_walk(cx, walk);
        WidgetDraw::done()
    }
}

impl Gallery {
    pub fn draw_walk(&mut self, cx: &mut Cx2d, _walk: Walk) {
        let start_pos = cx.turtle().size() / dvec2(2., 2.);
        let padding = 20.;
        let image_offset = self.calculate_current_offset(padding, IMAGE_WIDTH, IMAGE_HEIGHT);
        let padded_image_width = IMAGE_WIDTH + padding;
        let padded_image_height = IMAGE_HEIGHT + padding;

        for (image_id, gallery_image) in self.images.iter_mut() {
            let image_idu64 = image_id.0.get_value();
            let col = (image_idu64 % self.grid_size as u64) as f64;
            let row = (image_idu64 / self.grid_size as u64) as f64;

            let pos = start_pos
                + dvec2(
                    (col * padded_image_width + image_offset.x) - IMAGE_WIDTH / 2.,
                    (row * (padded_image_height) + image_offset.y) - IMAGE_HEIGHT / 2.,
                );
            gallery_image.draw_abs(cx, pos);
        }
    }

    pub fn handle_event_with(
        &mut self,
        cx: &mut Cx,
        event: &Event,
        _dispatch_action: &mut dyn FnMut(&mut Cx, GalleryAction),
    ) {
        let mut actions = Vec::new();
        for (image_id, gallery_image) in self.images.iter_mut() {
            gallery_image.handle_event_with(cx, event, &mut |_, action| {
                actions.push((*image_id, action))
            });
        }
    }

    fn calculate_current_offset(&self, padding: f64, width: f64, height: f64) -> DVec2 {
        let padded_image_width = width + padding;
        let padded_image_height = height + padding;

        let col = (self.current_index % self.grid_size) as f64;
        let row = (self.current_index / self.grid_size) as f64;
        let indexed_offset = dvec2((-padded_image_width) * col, (-padded_image_height) * row);

        return indexed_offset;
    }

    fn handle_swipe(&mut self, cx: &mut Cx, event: &Event) {
        match event.hits(cx, self.view.area()) {
            Hit::FingerMove(fe) => {
                let mut swipe_vector = fe.abs - fe.abs_start;
                // Negate y values because makepad's y axis grows to the south
                swipe_vector.y = -swipe_vector.y;

                // only trigger swipe if it is larger than some pixels
                let swipe_trigger_value = 60.;
                let diagonal_trigger_value = swipe_trigger_value / 2.;
                if (swipe_vector.x.abs() > swipe_trigger_value)
                    || (swipe_vector.y.abs() > swipe_trigger_value)
                {
                    let mut new_index = self.current_index;

                    // compensate diagonal swipe case (both trigger the diagonal value)
                    if swipe_vector.x.abs() > diagonal_trigger_value {
                        new_index += if swipe_vector.x > 0. { -1 } else { 1 };
                    }
                    if swipe_vector.y.abs() > diagonal_trigger_value {
                        new_index += self.grid_size * if swipe_vector.y > 0. { 1 } else { -1 };
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

                    // finally update the index if we didnt do it recently
                    if self.ready_to_swipe {
                        self.set_index(new_index);
                    }
                    self.ready_to_swipe = false;
                }
            }
            Hit::FingerUp(_fe) => self.ready_to_swipe = true,
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

#[derive(Clone, WidgetAction)]
pub enum GalleryAction {
    None,
}

#[derive(Debug, Clone, PartialEq, WidgetRef)]
pub struct GalleryRef(WidgetRef);
