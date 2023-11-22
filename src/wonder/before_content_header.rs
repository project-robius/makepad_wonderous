use makepad_widgets::widget::WidgetCache;
use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import makepad_draw::shader::std::*;

    import crate::shared::styles::*;
    import crate::shared::curved_label::*;
    import crate::shared::widgets::*;

    IMG_GREAT_WALL_CONTENT_1 = dep("crate://self/resources/images/great-wall-content-1.jpg")

    BeforeContentHeader = {{BeforeContentHeader}} {
        width: Fit,
        height: Fit,

        image = <Image> {
            // Override to have the upper corners rounded
            draw_bg: {
                instance radius: 90.
                instance opacity: 0.3
                instance image_scale: vec2(0.9, 0.9)
                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    sdf.box(
                        1,
                        1,
                        self.rect_size.x - 2.0,
                        // This calculation is to make sure the bottom part is not rounded
                        self.rect_size.y + self.radius * 2.0,
                        max(1.0, self.radius)
                    );

                    let color = self.get_color();
                    sdf.fill_keep(Pal::premul(vec4(color.xyz, color.w * self.opacity)));
                    return sdf.result
                }
            }

            source: (IMG_GREAT_WALL_CONTENT_1),
            width: 375,
            height: 430,
        }

        animator: {
            header = {
                default: show,
                hide = {
                    redraw: true,
                    from: {all: Forward {duration: 0.5}}
                    apply: {
                        image = { draw_bg: { opacity: 0.0 } }
                    }
                }
                show = {
                    redraw: true,
                    from: {all: Snap}
                    apply: {
                        image = { draw_bg: { opacity: 1.0 } }
                    }
                }
            },
        }
    }
}

#[derive(Live)]
pub struct BeforeContentHeader {
    #[deref]
    view: View,

    #[animator]
    animator: Animator,
}

impl LiveHook for BeforeContentHeader {
    fn before_live_design(cx: &mut Cx) {
        register_widget!(cx, BeforeContentHeader);
    }
}

impl Widget for BeforeContentHeader {
    fn handle_widget_event_with(
        &mut self,
        cx: &mut Cx,
        event: &Event,
        dispatch_action: &mut dyn FnMut(&mut Cx, WidgetActionItem),
    ) {
        if self.animator_handle_event(cx, event).must_redraw() {
            self.redraw(cx);
        }

        self.view.handle_widget_event_with(cx, event, dispatch_action);
    }

    fn walk(&mut self, cx: &mut Cx) -> Walk {
        self.view.walk(cx)
    }

    fn redraw(&mut self, cx: &mut Cx) {
        self.view.redraw(cx);
    }

    fn find_widgets(&mut self, path: &[LiveId], cached: WidgetCache, results: &mut WidgetSet) {
        self.view.find_widgets(path, cached, results);
    }

    fn draw_walk_widget(&mut self, cx: &mut Cx2d, walk: Walk) -> WidgetDraw {
        let _ = self.view.draw_walk_widget(cx, walk);
        WidgetDraw::done()
    }
}

impl BeforeContentHeader {
    fn update_values(&mut self, cx: &mut Cx, scale: f64, vertical_pan: f64, opacity: f64) {
        self.animator_play(cx, id!(header.show));
        self.image(id!(image)).apply_over(
            cx,
            live! {
                draw_bg: {
                    image_scale: (dvec2(scale, scale)),
                    image_pan: (dvec2(0.0, vertical_pan)),
                    opacity: (opacity)
                }
            },
        );
    }

    fn hide(&mut self, cx: &mut Cx) {
        self.animator_play(cx, id!(header.hide));
    }
}

#[derive(Clone, PartialEq, WidgetRef)]
pub struct BeforeContentHeaderRef(WidgetRef);

impl BeforeContentHeaderRef {
    pub fn hide(&mut self, cx: &mut Cx) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.hide(cx);
            dbg!("hide");
        }
    }

    pub fn show(&mut self, cx: &mut Cx, scale: f64, vertical_pan: f64, opacity: f64) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.update_values(cx, scale, vertical_pan, opacity);
            dbg!(opacity);
        }
    }
}
