use makepad_widgets::*;
use makepad_widgets::widget::WidgetCache;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import makepad_draw::shader::std::*;

    import crate::shared::styles::*;
    import crate::shared::widgets::*;

    IMG_GREAT_WALL_CONTENT_1 = dep("crate://self/resources/great-wall-content-1.jpg")

    WonderContent = {{WonderContent}} {
        flow: Overlay

        margin: { top: 570.0 }

        <View> {
            flow: Overlay
            width: Fill
            height: Fit

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

            header_img_2 = <View> {
                flow: Down
                visible: false
                show_bg: true
                draw_bg: {
                    color: #5d2a2c
                }
                width: 375,
                height: 430,

                margin: { top: -86 }

                <FadeView> {
                    width: 375,
                    height: 430,
                    draw_bg: { instance opacity: 0.3 }

                    <Image> {
                        source: (IMG_GREAT_WALL_CONTENT_1),
                        width: 375,
                        height: 430,
                    }
                } 
            }
        }

        header_bottom = <View> {
            flow: Overlay
            width: Fill
            height: 130

            margin: {top: 380.0}
            align: {x: 0.5, y: 0.0}

            <View> {
                width: Fill
                height: 70

                margin: {top: 60.0}

                show_bg: true
                draw_bg: {
                    color: #fff
                }
            }

            <CircleView> {
                width: 200
                height: 200

                show_bg: true
                draw_bg: {
                    color: #fff
                    radius: 80.0
                }
            }
        }

        main_content = <View> {
            flow: Overlay
            width: Fill
            height: 3000.0

            show_bg: true
            draw_bg: {
                color: #fff
            }

            margin: { top: 470. }

            main_content_inner = <View> {
                flow: Down
                width: Fill
                height: Fill

                show_bg: true
                draw_bg: {
                    color: #fff
                }

                spacing: 20.

                <Label> {
                    padding: 20.
                    width: Fill
                    draw_text: {
                        text_style: <INTRO_SUBTITLE>{font_size: 10},
                        color: #333,
                        wrap: Word,
                    }
                    text: "The Great Wall of China is a series of fortifications that were built across the historical northern borders of ancient Chinese states and Imperial China as protection against various nomadic groups from the Eurasian Steppe. The total length of all sections ever built is over 13,000 miles."
                }

                <Label> {
                    padding: 20.
                    width: Fill
                    draw_text: {
                        text_style: <INTRO_SUBTITLE>{font_size: 10},
                        color: #333,
                        wrap: Word,
                    }
                    text: "Several walls were built from as early as the 7th century BCE, with selective stretches later joined together by Qin Shi Huang (220-206  BCE), the first emperor of China. Little of the Qin wall remains."
                }

                <Label> {
                    padding: 20.
                    width: Fill
                    draw_text: {
                        text_style: <INTRO_SUBTITLE>{font_size: 10},
                        color: #333,
                        wrap: Word,
                    }
                    text: "Later on, many successive dynasties built and maintained multiple stretches of border walls."
                }

                <Label> {
                    padding: 20.
                    width: Fill
                    draw_text: {
                        text_style: <INTRO_SUBTITLE>{font_size: 10},
                        color: #333,
                        wrap: Word,
                    }
                    text: "Transporting the large quantity of materials required for construction was difficult, so builders always tried to use local resources. Stones from the mountains were used over mountain ranges, while rammed earth was used for construction in the plains. Most of the ancient walls have eroded away over the centuries."
                }

                <Label> {
                    padding: 20.
                    width: Fill
                    draw_text: {
                        text_style: <INTRO_SUBTITLE>{font_size: 10},
                        color: #333,
                        wrap: Word,
                    }
                    text: "Stones cut into rectangular shapes were used for the foundation, inner and outer brims, and gateways of the wall."
                }

                <Label> {
                    padding: 20.
                    width: Fill
                    draw_text:{
                        text_style: <INTRO_SUBTITLE>{font_size: 10},
                        color: #333,
                        wrap: Word,
                    }
                    text: "Under the rule of the Qing dynasty, China's borders extended beyond the walls and Mongolia was annexed into the empire, so construction was discontinued."
                }

                <Label> {
                    draw_text:{
                        text_style: <INTRO_SUBTITLE>{font_size: 20},
                        color: #000
                    }
                    text: "LONGEST STRUCTURE ON EARTH"
                }

                <Label> {
                    draw_text:{
                        text_style: <INTRO_SUBTITLE>{font_size: 20},
                        color: #000
                    }
                    text: "LONGEST STRUCTURE ON EARTH 2"
                }

                <Label> {
                    draw_text:{
                        text_style: <INTRO_SUBTITLE>{font_size: 20},
                        color: #000
                    }
                    text: "LONGEST STRUCTURE ON EARTH"
                }

                <Label> {
                    draw_text:{
                        text_style: <INTRO_SUBTITLE>{font_size: 20},
                        color: #000
                    }
                    text: "LONGEST STRUCTURE ON EARTH 2"
                }

                <Label> {
                    draw_text:{
                        text_style: <INTRO_SUBTITLE>{font_size: 20},
                        color: #000
                    }
                    text: "LONGEST STRUCTURE ON EARTH"
                }

                <Label> {
                    draw_text:{
                        text_style: <INTRO_SUBTITLE>{font_size: 20},
                        color: #000
                    }
                    text: "LONGEST STRUCTURE ON EARTH 2"
                }

                <Label> {
                    draw_text:{
                        text_style: <INTRO_SUBTITLE>{font_size: 20},
                        color: #000
                    }
                    text: "LONGEST STRUCTURE ON EARTH"
                }

                <Label> {
                    draw_text:{
                        text_style: <INTRO_SUBTITLE>{font_size: 20},
                        color: #000
                    }
                    text: "LONGEST STRUCTURE ON EARTH 8"
                }
            }
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

        let opacity = if offset > 380. + 570. - 80. {
            0.0
        } else {
            min(1.0, 0.5 + offset / 570.)
        };

        let scale = 0.9 + min(0.1, offset / (570. * 10.));

        let vertical_pan = if offset > 380. + 570. - 80. {
            // TODO Add constants to better communicate these calculations
            (380. - 80.) / (570. * 3.)
        } else if offset > 570. {
            (offset - 570.) / (570. * 3.)
        } else {
            0.0
        };

        self.image(id!(header_img)).apply_over(cx, live!{
            draw_bg: {
                radius: 90.0,
                image_scale: (dvec2(scale, scale)),
                image_pan: (dvec2(0.0, vertical_pan)),
                opacity: (opacity)
            }
        });

        if offset > 380. + 570. - 80. {
            self.view(id!(main_content)).apply_over(cx, live!{
                margin: {top: (470.0 - (380. - 80.))}
            });

            let inner_offset = offset - (380. + 570. - 80.);
            self.view(id!(main_content_inner)).apply_over(cx, live!{
                abs_pos: (dvec2(0.0, -inner_offset))
            });
        } else if offset > 570.0 {
            self.view(id!(main_content)).apply_over(cx, live!{
                margin: {top: (470.0 - (offset - 570.0))}
            });
            self.view(id!(main_content_inner)).apply_over(cx, live!{
                abs_pos: (dvec2(0.0, 837.0 - offset))
            });
        } else {
            self.view(id!(main_content_inner)).apply_over(cx, live!{
                abs_pos: (dvec2(0.0, 837.0 - offset))
            });
        }

        let header_bottom_margin = if offset > 570.0 {
            max(370.0 - (offset - 570.0), 70.0)
        } else {
            370.0 // 430. - 80.
        };

        self.view(id!(header_bottom)).apply_over(cx, live!{
            margin: {top: (header_bottom_margin)}
        });

        // Visibility toggle for permanent header
        if offset > 380. + 570. - 80. {
            self.view(id!(header_img_2)).set_visible(true);
        } else {
            self.view(id!(header_img_2)).set_visible(false);
        }
    }
}


#[derive(Clone, PartialEq, WidgetRef)]
pub struct WonderContentRef(WidgetRef);

impl WonderContentRef {
    pub fn scroll(&mut self, cx: &mut Cx, delta: f64, is_up: bool) -> WonderContentAction {
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