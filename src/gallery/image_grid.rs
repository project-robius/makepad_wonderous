use crate::gallery::image_box::{Animation, ImageBox, ImageBoxId, IMAGE_WIDTH};
use makepad_widgets::widget::WidgetCache;
use makepad_widgets::*;

const ROWS: i32 = 40;
const COLS: i32 = 20;

live_design! {
    import crate::image_box::*;
    import makepad_widgets::base::*;

    IMG_1 = dep("crate://self/resources/images/great-wall-content-1.jpg")
    IMG_2 = dep("crate://self/resources/images/great-wall-content-1.jpg")
    IMG_3 = dep("crate://self/resources/images/great-wall-content-1.jpg")
    // IMG_1 = dep("crate://self/resources/image_1_200x200.jpg")
    // IMG_2 = dep("crate://self/resources/image_2_200x200.jpg")
    // IMG_3 = dep("crate://self/resources/image_3_200x200.jpg")

    ImageGrid= {{ImageGrid}} {

        show_bg: true,
        draw_bg: {
            color: #411
        }

        fading_image_box: <ImageBox> {
            image: <RotatedImage> {
                source: (IMG_1)
            }
        }
        scaling_image_box: <ImageBox> {
            image: <RotatedImage> {
                source: (IMG_2)
            }
        }
        rotating_image_box: <ImageBox> {
            image: <RotatedImage> {
                source: (IMG_3)
            }
        }
        width: Fill,
        height: Fill
    }
}

#[derive(Live)]
pub struct ImageGrid {
    #[walk]
    walk: Walk,
    #[layout]
    layout: Layout,
    #[live]
    fading_image_box: Option<LivePtr>,
    #[live]
    scaling_image_box: Option<LivePtr>,
    #[live]
    rotating_image_box: Option<LivePtr>,
    #[rust]
    image_boxes: ComponentMap<ImageBoxId, ImageBox>,
}

impl Widget for ImageGrid {
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
    }

    fn walk(&mut self, cx: &mut Cx) -> Walk {
        self.walk
    }

    fn redraw(&mut self, _cx: &mut Cx) {}

    fn draw_walk_widget(&mut self, cx: &mut Cx2d, walk: Walk) -> WidgetDraw {
        self.draw_walk(cx, walk);
        WidgetDraw::done()
    }
}

impl LiveHook for ImageGrid {
    fn before_live_design(cx: &mut Cx) {
        register_widget!(cx, ImageGrid);
    }

    fn after_new_from_doc(&mut self, cx: &mut Cx) {
        for y in 0..ROWS {
            for x in 0..COLS {
                let box_id = LiveId(x as u64 * 100 + y as u64).into();

                let mut new_box;
                let pattern_index = ((x as i64 - y as i64).rem_euclid(3) + 3) % 3;
                let animation = Animation::from_index(pattern_index as usize);

                match animation {
                    Animation::Fade => {
                        new_box = ImageBox::new_from_ptr(cx, self.fading_image_box);
                    }
                    Animation::Scale => {
                        new_box = ImageBox::new_from_ptr(cx, self.scaling_image_box);
                    }
                    Animation::Rotate => {
                        new_box = ImageBox::new_from_ptr(cx, self.rotating_image_box);
                    }
                }

                new_box.animation = animation;
                self.image_boxes.insert(box_id, new_box);
            }
        }
    }

    fn after_apply(&mut self, cx: &mut Cx, from: ApplyFrom, index: usize, nodes: &[LiveNode]) {
        for image_box in self.image_boxes.values_mut() {
            if let Some(index) = nodes.child_by_name(index, live_id!(image_box).as_field()) {
                image_box.apply(cx, from, index, nodes);
            }
        }
    }
}

impl ImageGrid {
    pub fn draw_walk(&mut self, cx: &mut Cx2d, _walk: Walk) {
        let start_pos = cx.turtle().pos();
        for (box_id, image_box) in self.image_boxes.iter_mut() {
            let box_idu64 = box_id.0.get_value();
            let image_offset = ((IMAGE_WIDTH * IMAGE_WIDTH * 2.0).sqrt() - IMAGE_WIDTH) / 2.0;
            let pos = start_pos
                + dvec2(
                    (box_idu64 / 100) as f64 * IMAGE_WIDTH - image_offset,
                    (box_idu64 % 100) as f64 * IMAGE_WIDTH - image_offset,
                );
            image_box.draw_abs(cx, pos);
            dbg!(box_id);
        }
    }

    pub fn handle_event_with(
        &mut self,
        cx: &mut Cx,
        event: &Event,
        _dispatch_action: &mut dyn FnMut(&mut Cx, ImageGridAction),
    ) {
        let mut actions = Vec::new();
        for (box_id, image_box) in self.image_boxes.iter_mut() {
            image_box
                .handle_event_with(cx, event, &mut |_, action| actions.push((*box_id, action)));
        }
    }
}

#[derive(Clone, WidgetAction)]
pub enum ImageGridAction {
    None,
}
