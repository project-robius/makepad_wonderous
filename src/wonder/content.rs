use makepad_widgets::widget::WidgetCache;
use makepad_widgets::*;

const HEADER_REACHES_TOP_OFFSET: f64 = 570.0;
const SCROLL_LENGHT_FOR_HEADER: f64 = 380.0;
const SCROLL_LENGHT_FOR_MAIN_CONTENT: f64 = 430.0;
const CONTENT_PANEL_REACHES_TOP_OFFSET: f64 =
    SCROLL_LENGHT_FOR_HEADER + HEADER_REACHES_TOP_OFFSET - 80.0;

pub const MAIN_CONTENT_LENGTH: f64 = 2600.0;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import makepad_draw::shader::std::*;

    import crate::shared::styles::*;
    import crate::shared::curved_label::*;
    import crate::shared::widgets::*;

    HEADER_REACHES_TOP_OFFSET = 570.0
    SCROLL_LENGHT_FOR_HEADER = 380.0
    SCROLL_LENGHT_FOR_MAIN_CONTENT = 430.0
    CONTENT_PANEL_REACHES_TOP_OFFSET = (SCROLL_LENGHT_FOR_HEADER + HEADER_REACHES_TOP_OFFSET - 80.0);
    MAIN_CONTENT_LENGTH = 2000.0;

    IMG_GREAT_WALL_CONTENT_1 = dep("crate://self/resources/images/great-wall-content-1.jpg")
    IMG_GREAT_WALL_LOCATION = dep("crate://self/resources/images/great-wall-location.jpg")
    IMG_GREAT_WALL_VIDEO = dep("crate://self/resources/images/great-wall-video.jpg")

    ContentLabel = <Label> {
        padding: 20.
        width: Fill
        draw_text: {
            text_style: <REGULAR_TEXT>{font_size: 9},
            color: #333,
            wrap: Word,
        }
    }

    ContentCallout = <View> { 
        flow: Right
        width: Fill
        
        //height: Fit is not working?
        padding: 20.
        spacing: 10.

        <VerticalLine> {
            draw_bg: {
                color: #e6945c
            }
        }
        label = <Label> {
            width: Fill
            margin: {top: 6}
            draw_text: {
                text_style: <ITALIC_TEXT>{font_size: 10},
                color: #333,
                wrap: Word,
            }
        }
    }

    PictureWithCaption = <View> {
        flow: Down,
        width: Fill,
        height: Fit,
        spacing: 8,

        align: {x: 0.5, y: 0.0}

        image = <Image> {
            width: 375,
        }

        caption = <Label> {
            width: Fill,
            padding: {left: 20, right: 20}
            draw_text: {
                text_style: <REGULAR_ITALIC_TEXT>{font_size: 8},
                color: #666,
            }
        }
    }

    WonderContent = {{WonderContent}} {
        flow: Overlay

        margin: { top: (HEADER_REACHES_TOP_OFFSET) }

        <View> {
            flow: Overlay
            width: Fill
            height: Fit

            header_before_full_content = <Image> {
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

            header_for_full_content = <View> {
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

            margin: {top: (SCROLL_LENGHT_FOR_HEADER)}
            align: {x: 0.5, y: 0.0}

            <View> {
                width: Fill
                height: 70

                margin: {top: 60.0}

                show_bg: true
                draw_bg: {
                    color: #f8eee5
                }
            }

            <CircleView> {
                width: 200
                height: 200

                show_bg: true
                draw_bg: {
                    color: #f8eee5
                    radius: 80.0
                }

                <CurvedLabel> {
                    width: 140
                    height: Fit
                    text: "FACTS AND HISTORY",

                    total_angle: (PI * 0.8)

                    margin: { left: 18, top: 16 }

                    draw_bg: {
                        color: #0000
                    }

                    draw_text: {
                        color: #e6945c,
                        text_style: {font_size: 8},
                    }
                }
            }
        }

        main_content = <View> {
            flow: Overlay
            width: Fill
            height: (MAIN_CONTENT_LENGTH)

            show_bg: true
            draw_bg: {
                color: #f8eee5
            }

            margin: {top: (SCROLL_LENGHT_FOR_MAIN_CONTENT)}

            main_content_inner = <View> {
                flow: Down
                width: Fill
                height: Fill

                show_bg: true
                draw_bg: {
                    color: #f8eee5
                }

                spacing: 20.

                <ContentLabel> {
                    text: "The Great Wall of China is a series of fortifications that were built across the historical northern borders of ancient Chinese states and Imperial China as protection against various nomadic groups from the Eurasian Steppe. The total length of all sections ever built is over 13,000 miles."
                }

                <ContentCallout> {
                    height: 80.
                    label = {
                        text: "The best-known sections of the wall were built by the Ming dynasty (1368-1644)"
                    }
                }

                <ContentLabel> {
                    text: "Several walls were built from as early as the 7th century BCE, with selective stretches later joined together by Qin Shi Huang (220-206  BCE), the first emperor of China. Little of the Qin wall remains."
                }

                <ContentLabel> {
                    text: "Later on, many successive dynasties built and maintained multiple stretches of border walls."
                }

                <ContentLabel> {
                    text: "Transporting the large quantity of materials required for construction was difficult, so builders always tried to use local resources. Stones from the mountains were used over mountain ranges, while rammed earth was used for construction in the plains. Most of the ancient walls have eroded away over the centuries."
                }

                <PictureWithCaption> {
                    image = {
                        source: (IMG_GREAT_WALL_VIDEO),
                        height: 280,
                    }

                    caption = {
                        text: "“See China’s Iconic Great Wall From Above | National Geographic.” Youtube, uploaded by National Geographic."
                    }
                }

                <ContentCallout> {
                    height: 96.
                    label = {
                        text: "During the Ming dynasty, however, bricks were heavily used in many areas of the wall, as were materials such as tiles, lime, and stone."
                    }
                }

                <ContentLabel> {
                    text: "Stones cut into rectangular shapes were used for the foundation, inner and outer brims, and gateways of the wall."
                }

                <ContentLabel> {
                    text: "Under the rule of the Qing dynasty, China's borders extended beyond the walls and Mongolia was annexed into the empire, so construction was discontinued."
                }

                <ContentLabel> {
                    text: "The frontier walls built by different dynasties have multiple courses. Collectively, they stretch from Liaodong in the east to Lop Lake in the west, from the present-day Sino-Russian border in the north to Tao River in the south; along an arc that roughly delineates the edge of the Mongolian steppe."
                }

                <View> {
                    flow: Down
                    width: Fill
                    height: Fit
                    spacing: 8.0
                    align: {x: 0.5, y: 0.0}

                    <Label> {
                        text: "“",
                        draw_text: {
                            text_style: <MONO_TEXT>{font_size: 70},
                            color: #e6945c,
                        }
                    }

                    <Label> {
                        margin: {top: -40}
                        text: "Its historic and strategic",
                        draw_text: {
                            text_style: <DECORATIVE_TEXT>{font_size: 13},
                            color: #333,
                            wrap: Word,
                        }
                    }
                    <Label> {
                        text: "importance is matched only",
                        draw_text: {
                            text_style: <DECORATIVE_TEXT>{font_size: 13},
                            color: #333,
                            wrap: Word,
                        }
                    }
                    <Label> {
                        text: "by its architectural",
                        draw_text: {
                            text_style: <DECORATIVE_TEXT>{font_size: 13},
                            color: #333,
                            wrap: Word,
                        }
                    }
                    <Label> {
                        text: "significance.",
                        draw_text: {
                            text_style: <DECORATIVE_TEXT>{font_size: 13},
                            color: #333,
                            wrap: Word,
                        }
                    }

                    <Label> {
                        text: "- UNESCO",
                        margin: {top: 20}
                        draw_text: {
                            text_style: <REGULAR_TEXT>{font_size: 10},
                            color: #e6945c,
                        }
                    }
                }

                <ContentLabel> {
                    text: "Apart from defense, other purposes of the Great Wall have included border controls, allowing the imposition of duties on goods transported along the Silk Road, regulation or encouragement of trade and the control of immigration and emigration."
                }

                <PictureWithCaption> {
                    image = {
                        source: (IMG_GREAT_WALL_LOCATION),
                        height: 178,
                    }

                    caption = {
                        text: "Map showing location of Great Wall of China in northern China."
                    }
                }
            }
        }
    }
}

#[derive(Debug, Clone, WidgetAction)]
pub enum WonderContentAction {
    Scrolling,
    Closed,
    None,
}

#[derive(PartialEq)]
enum WonderContentState {
    BeforeFullContent,
    FullContent,
}

#[derive(Live)]
pub struct WonderContent {
    #[deref]
    view: View,

    #[rust(0.0)]
    current_scroll_offset: f64,

    #[rust(WonderContentState::BeforeFullContent)]
    state: WonderContentState,

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

        self.view
            .handle_widget_event_with(cx, event, dispatch_action);
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
    fn update_state(&mut self, cx: &mut Cx, offset: f64) {
        match self.state {
            WonderContentState::BeforeFullContent => {
                if offset > CONTENT_PANEL_REACHES_TOP_OFFSET {
                    self.state = WonderContentState::FullContent;
                }

                self.update_top_margin(cx, offset);
                self.update_header_section(cx, offset);
                self.update_content_position(cx, offset);
            }
            WonderContentState::FullContent => {
                if offset < CONTENT_PANEL_REACHES_TOP_OFFSET {
                    self.state = WonderContentState::BeforeFullContent;

                    self.update_top_margin(cx, offset);
                    self.update_header_section(cx, offset);
                }

                self.update_content_position(cx, offset);
            }
        }
    }

    fn update_top_margin(&mut self, cx: &mut Cx, offset: f64) {
        let margin = max(0.0, HEADER_REACHES_TOP_OFFSET - offset);
        self.apply_over(
            cx,
            live! {
                margin: {top: (margin)}
            },
        );
    }

    fn update_header_section(&mut self, cx: &mut Cx, offset: f64) {
        match self.state {
            WonderContentState::BeforeFullContent => {
                self.view(id!(header_for_full_content)).set_visible(false);

                let opacity = min(1.0, 0.5 + offset / HEADER_REACHES_TOP_OFFSET);
                let scale = 0.9 + min(0.1, offset / (HEADER_REACHES_TOP_OFFSET * 10.));
                let pan_factor = HEADER_REACHES_TOP_OFFSET * 3.0;
                let vertical_pan = max(0.0, (offset - HEADER_REACHES_TOP_OFFSET) / pan_factor);

                self.image(id!(header_before_full_content)).apply_over(
                    cx,
                    live! {
                        draw_bg: {
                            radius: 90.0,
                            image_scale: (dvec2(scale, scale)),
                            image_pan: (dvec2(0.0, vertical_pan)),
                            opacity: (opacity)
                        }
                    },
                );
            }
            WonderContentState::FullContent => {
                self.view(id!(header_for_full_content)).set_visible(true);
                self.image(id!(header_before_full_content)).apply_over(
                    cx,
                    live! {
                        draw_bg: { opacity: (0.0) }
                    },
                );
            }
        }

        let header_bottom_margin_base = if offset > HEADER_REACHES_TOP_OFFSET {
            max(
                SCROLL_LENGHT_FOR_HEADER - (offset - HEADER_REACHES_TOP_OFFSET),
                80.0,
            )
        } else {
            SCROLL_LENGHT_FOR_HEADER
        };
        let header_bottom_margin = header_bottom_margin_base - 50.0;

        self.view(id!(header_bottom)).apply_over(
            cx,
            live! {
                margin: {top: (header_bottom_margin)}
            },
        );
    }

    fn update_content_position(&mut self, cx: &mut Cx, offset: f64) {
        let main_content_margin = if offset > CONTENT_PANEL_REACHES_TOP_OFFSET {
            SCROLL_LENGHT_FOR_MAIN_CONTENT - (SCROLL_LENGHT_FOR_HEADER - 80.)
        } else if offset > HEADER_REACHES_TOP_OFFSET {
            SCROLL_LENGHT_FOR_MAIN_CONTENT - (offset - HEADER_REACHES_TOP_OFFSET)
        } else {
            SCROLL_LENGHT_FOR_MAIN_CONTENT
        };

        self.view(id!(main_content)).apply_over(
            cx,
            live! {
                margin: {top: (main_content_margin)}
            },
        );

        let main_content_inner_offset =
            SCROLL_LENGHT_FOR_MAIN_CONTENT + HEADER_REACHES_TOP_OFFSET + 20.0;
        self.view(id!(main_content_inner)).apply_over(
            cx,
            live! {
                abs_pos: (dvec2(0.0, main_content_inner_offset - offset))
            },
        );
    }
}

#[derive(Clone, PartialEq, WidgetRef)]
pub struct WonderContentRef(WidgetRef);

impl WonderContentRef {
    pub fn scroll(&mut self, cx: &mut Cx, delta: f64, is_dragging: bool) -> WonderContentAction {
        if let Some(mut inner) = self.borrow_mut() {
            inner.current_scroll_offset = delta;

            if is_dragging {
                // 60 is pulldown maximum offset in touch_gesture.rs
                if inner.current_scroll_offset >= -59.0 {
                    inner.update_state(cx, max(delta, -59.0));
                    WonderContentAction::Scrolling
                } else {
                    inner.update_state(cx, 0.0);
                    inner.current_scroll_offset = 0.0;
                    WonderContentAction::Closed
                }
            } else {
                inner.update_state(cx, delta);
                WonderContentAction::Scrolling
            }
        } else {
            WonderContentAction::None
        }
    }
}
