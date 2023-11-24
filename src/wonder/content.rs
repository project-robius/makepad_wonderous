use makepad_widgets::widget::WidgetCache;
use makepad_widgets::*;
use crate::wonder::rotating_title::RotatingTitleWidgetExt;
use crate::wonder::before_content_header::BeforeContentHeaderWidgetExt;
use crate::wonder::content_header::ContentHeaderWidgetExt;
use crate::wonder::great_wall_highligth::GreatWallHighlightWidgetExt;

const HEADER_REACHES_TOP_OFFSET: f64 = 570.0;
const SCROLL_LENGHT_FOR_HEADER: f64 = 380.0;
const SCROLL_LENGHT_FOR_MAIN_CONTENT: f64 = 430.0;
const CONTENT_PANEL_REACHES_TOP_OFFSET: f64 =
    SCROLL_LENGHT_FOR_HEADER + HEADER_REACHES_TOP_OFFSET - 80.0;

pub const MAIN_CONTENT_LENGTH: f64 = 2800.0;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import makepad_draw::shader::std::*;

    import crate::shared::styles::*;
    import crate::shared::widgets::*;
    import crate::wonder::rotating_title::*;
    import crate::wonder::before_content_header::*;
    import crate::wonder::content_header::*;
    import crate::wonder::great_wall_highligth::*;

    HEADER_REACHES_TOP_OFFSET = 570.0
    SCROLL_LENGHT_FOR_HEADER = 380.0
    SCROLL_LENGHT_FOR_MAIN_CONTENT = 430.0
    CONTENT_PANEL_REACHES_TOP_OFFSET = (SCROLL_LENGHT_FOR_HEADER + HEADER_REACHES_TOP_OFFSET - 80.0);

    IMG_GREAT_WALL_LOCATION = dep("crate://self/resources/images/great-wall-location.jpg")
    IMG_GREAT_WALL_VIDEO = dep("crate://self/resources/images/great-wall-video.jpg")

    IMG_ICON_HISTORY = dep("crate://self/resources/images/history.png")
    IMG_ICON_GEOGRAPHY = dep("crate://self/resources/images/geography.png")
    IMG_ICON_CONSTRUCTION = dep("crate://self/resources/images/construction.png")

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

            header_before_full_content = <BeforeContentHeader> {}

            header_for_full_content = <ContentHeader> {}
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
            }

            rotating_title = <RotatingTitle> {
                width: 140
            }
        }

        main_content = <View> {
            flow: Overlay
            width: Fill
            height: Fit

            show_bg: true
            draw_bg: {
                color: #f8eee5
            }

            margin: {top: (SCROLL_LENGHT_FOR_MAIN_CONTENT)}

            main_content_inner = <View> {
                flow: Down
                width: Fill
                height: Fit

                show_bg: true
                draw_bg: {
                    color: #f8eee5
                }

                spacing: 20.

                <ContentLabel> {
                    text: "The Great Wall of China is a series of fortifications that were built across the historical northern borders of ancient Chinese states and Imperial China as protection against various nomadic groups from the Eurasian Steppe. The total length of all sections ever built is over 13,000 miles."
                }

                great_wall_highlight = <GreatWallHighlight> {}

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

                // Gap space
                <View> {
                    width: Fill
                    height: 300
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
}

impl LiveHook for WonderContent {
    fn before_live_design(cx: &mut Cx) {
        register_widget!(cx, WonderContent);
    }
}

impl Widget for WonderContent {
    fn handle_widget_event_with(
        &mut self,
        cx: &mut Cx,
        event: &Event,
        dispatch_action: &mut dyn FnMut(&mut Cx, WidgetActionItem),
    ) {
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
        let mut before_content_header = self.before_content_header(id!(header_before_full_content));
        let mut full_content_header = self.content_header(id!(header_for_full_content));
        match self.state {
            WonderContentState::BeforeFullContent => {
                full_content_header.hide(cx);

                let opacity = min(1.0, 0.3 + offset / (HEADER_REACHES_TOP_OFFSET * 2.0));
                let scale = 0.9 + min(0.1, offset / (HEADER_REACHES_TOP_OFFSET * 10.));
                let pan_factor = HEADER_REACHES_TOP_OFFSET * 3.0;
                let vertical_pan = max(0.0, (offset - HEADER_REACHES_TOP_OFFSET) / pan_factor);

                before_content_header.show(
                    cx,
                    scale,
                    vertical_pan,
                    opacity,
                )
            }
            WonderContentState::FullContent => {
                full_content_header.show(cx);
                before_content_header.hide(cx);
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

        if self.state == WonderContentState::FullContent {
            let mut rotating_title = self.rotating_title(id!(rotating_title));
            rotating_title.set_scroll_progress(offset);
        }

        let mut great_wall_highlight = self.great_wall_highlight(id!(great_wall_highlight));
        great_wall_highlight.update_values(cx, offset);
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
