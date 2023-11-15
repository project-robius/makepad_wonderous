use makepad_widgets::widget::WidgetCache;
use makepad_widgets::*;

const HEADER_REACHES_TOP_OFFSET: f64 = 570.0;
const SCROLL_LENGHT_FOR_HEADER: f64 = 380.0;
const SCROLL_LENGHT_FOR_MAIN_CONTENT: f64 = 430.0;
const CONTENT_PANEL_REACHES_TOP_OFFSET: f64 =
    SCROLL_LENGHT_FOR_HEADER + HEADER_REACHES_TOP_OFFSET - 80.0;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import makepad_draw::shader::std::*;

    import crate::shared::styles::*;
    import crate::shared::widgets::*;

    HEADER_REACHES_TOP_OFFSET = 570.0
    SCROLL_LENGHT_FOR_HEADER = 380.0
    SCROLL_LENGHT_FOR_MAIN_CONTENT = 430.0
    CONTENT_PANEL_REACHES_TOP_OFFSET = (SCROLL_LENGHT_FOR_HEADER + HEADER_REACHES_TOP_OFFSET - 80.0);
    MAIN_CONTENT_LENGTH = 3000.0;

    IMG_GREAT_WALL_CONTENT_1 = dep("crate://self/resources/images/great-wall-content-1.jpg")

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
                    color: #fff
                    radius: 80.0
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
