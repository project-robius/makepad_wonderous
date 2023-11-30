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

    // GalleryImageGrid = <View> {
    //     width: Fit
    //     height: Fit
    // }

    Gallery = {{Gallery}} {
        width: 1000, height: 1000

        gallery_image_template = <GalleryImage> {
            image: <RotatedImage> {
                source: (IMAGE_CONTENT)
            }
        }

        show_bg: true,
        draw_bg: {
            color: #123
        }


    }

    GalleryScreen = <View> {
        width: 800, height: 1000

        show_bg: true,
        draw_bg: {
            color: #000
        }
        // view = <View> {

        //     width: 80, height: 100
        //     show_bg: true,
        //     draw_bg: {
        //         color: #732
        //     }
        //     }
        <Gallery> {}
    }
}

#[derive(Live)]
pub struct Gallery {
    // #[deref]
    // view: View,
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
    grid_size: i32,
    #[rust]
    index: i32,
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
        // Define values
        self.grid_size = 5;
        self.index = (self.grid_size.pow(2) + 1) / 2;

        // Draw grid
        for i in 0..(self.grid_size.pow(2)) {
            let image_id = LiveId(i as u64 * 100 as u64).into();

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
    }

    fn walk(&mut self, cx: &mut Cx) -> Walk {
        // self.view.walk(cx)
        self.walk
    }

    fn redraw(&mut self, cx: &mut Cx) {
        // self.view.redraw(cx);
    }

    // fn find_widgets(&mut self, path: &[LiveId], cached: WidgetCache, results: &mut WidgetSet) {
    //     self.view.find_widgets(path, cached, results);
    // }

    fn draw_walk_widget(&mut self, cx: &mut Cx2d, walk: Walk) -> WidgetDraw {
        // let _ = self.view.draw_walk_widget(cx, walk);
        self.draw_walk(cx, walk);
        WidgetDraw::done()
    }
}

impl Gallery {
    pub fn draw_walk(&mut self, cx: &mut Cx2d, _walk: Walk) {
        let start_pos = cx.turtle().pos();
        for (image_id, gallery_image) in self.images.iter_mut() {
            let image_idu64 = image_id.0.get_value();
            let image_offset = ((IMAGE_WIDTH * IMAGE_WIDTH * 2.0).sqrt() - IMAGE_WIDTH) / 2.0;
            let pos = start_pos
                + dvec2(
                    (image_idu64 / 100) as f64 * IMAGE_WIDTH - image_offset,
                    (image_idu64 % 100) as f64 * IMAGE_WIDTH - image_offset,
                );
            let pos = dvec2(100., 100.);
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

    // pub fn draw_walk(&mut self, cx: &mut Cx2d, walk: Walk) {
    //     cx.begin_turtle(walk, self.layout);

    //     // Create Grid
    //     for x in 1..(self.grid_size * self.grid_size + 1) {
    //         let image_id = LiveId((x) as u64).into();
    //         let current_image = self.images.get_or_insert(cx, image_id, |cx| {
    //             WidgetRef::new_from_ptr(cx, self.gallery_image)
    //         });
    //         let image_width = current_image.walk(cx).width.fixed_or_zero();

    //         // image_walk = Walk::with_abs_pos( as f64, 0.));

    //         current_image
    //             .walk(cx)
    //             .with_abs_pos(dvec2((x * 100) as f64, 0.));

    //         dbg!(current_image.walk(cx));

    //         // let _ = current_image.draw_walk_widget(
    //         //     cx,
    //         //     walk.with_abs_pos(DVec2 {
    //         //         x: (x * 100) as f64,
    //         //         y: 0.,
    //         //     }),
    //         // );
    //     }
    //     cx.end_turtle();
    // }
}

#[derive(Clone, WidgetAction)]
pub enum GalleryAction {
    None,
}
