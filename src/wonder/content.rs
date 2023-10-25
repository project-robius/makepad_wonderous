use makepad_widgets::*;
use makepad_widgets::widget::WidgetCache;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import makepad_draw::shader::std::*;

    IMG_GREAT_WALL_CONTENT_1 = dep("crate://self/resources/great-wall-content-1.jpg")

    WonderContent = {{WonderContent}} {
        flow: Down

        margin: { top: 570.0 }

        header_img = <Image> {
             // Override to have the upper corners rounded
             draw_bg: {
                instance radius: 90.
                instance opacity: 0.5
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
    }
}

#[derive(Debug, Clone, WidgetAction)]
pub enum WonderContentAction {
    Scrolling(f64),
    Closed,
    None,
}

#[derive(Live)]
pub struct WonderContent {
    #[deref]
    view: View,

    #[rust(0.0)]
    current_scroll_offset: f64,

    #[animator]
    animator: Animator,
    
    #[rust]
    next_frame: NextFrame,
}

impl LiveHook for WonderContent {
    fn before_live_design(cx: &mut Cx) {
        register_widget!(cx, WonderContent);
    }

    fn after_apply(&mut self, cx: &mut Cx, from: ApplyFrom, _index: usize, _nodes: &[LiveNode]) {
        if from.is_from_doc() {
            self.next_frame = cx.new_next_frame();
        }
    }
}

impl Widget for WonderContent {
    fn handle_widget_event_with(
        &mut self,
        cx: &mut Cx,
        event: &Event,
        dispatch_action: &mut dyn FnMut(&mut Cx, WidgetActionItem),
    ) {
        // if self.animator_handle_event(cx, event).must_redraw() {
        //     self.redraw(cx);
        // }

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

impl WonderContent {
    fn update_header_section(&mut self, cx: &mut Cx, offset: f64) {
        let margin = max(0.0, 570.0 - offset);
        self.apply_over(cx, live!{
            margin: {top: (margin)}
        });

        let opacity = min(1.0, 0.5 + offset / 570.);
        let scale = 0.9 + min(0.1, offset / 5700.);
        dbg!(scale);
        self.image(id!(header_img)).apply_over(cx, live!{
            draw_bg: {
                radius: 90.0,
                image_scale: (dvec2(scale, scale)),
                opacity: (opacity)
            }
        });
    }
}

#[derive(Clone, PartialEq, WidgetRef)]
pub struct WonderContentRef(WidgetRef);

impl WonderContentRef {
    pub fn process_dragging(&mut self, cx: &mut Cx, delta: f64, is_up: bool) -> WonderContentAction {
        if let Some(mut inner) = self.borrow_mut() {
            let new_delta = inner.current_scroll_offset + delta;
            if is_up {
                inner.current_scroll_offset += delta;
            }

            if new_delta >= 0.0 {
                inner.update_header_section(cx, new_delta);
                
                WonderContentAction::Scrolling(new_delta)
            } else {
                inner.update_header_section(cx, 0.0);
                inner.current_scroll_offset = 0.0;

                WonderContentAction::Closed
            }
        } else {
            WonderContentAction::None
        }
    }
}