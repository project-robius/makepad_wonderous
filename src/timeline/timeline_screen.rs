use makepad_widgets::*;

const CONTENT_LENGTH: f64 = 800.;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;

    import crate::shared::styles::*;
    import crate::shared::widgets::*;

    import crate::shared::expandable_panel::*;

    IMG_HEADER = dep("crate://self/resources/images/great-wall-flattened.jpg")

    BACKGROUND_COLOR = #222
    BACKGROUND_ITEM_COLOR = #333


    Header = <FadeView> {
        flow: Overlay,
        width: Fill,
        height: 254,

        align: { x: 0.5, y: 0 }

        draw_bg: {
            opacity: 1.0,
        }

        <CenteredOnTop> {
            source: (IMG_HEADER),
            width: 200,
            height: 430,

            draw_bg: {
                instance radius: 48.

                fn get_opacity(self) -> float {
                    return clamp((1.0 - self.pos.y * 1.6) * 2.0, 0.0, 1.0);
                }
            }
        }

        <View> {
            flow: Down,
            width: Fill,
            height: Fit,

            abs_pos: vec2(0, 240.0),
            align: { x: 0.5, y: 0 }

            <Label> {
                draw_text:{
                    text_style: <INTRO_TITLE>{font_size: 14},
                    color: #fff
                }
                text: "the"
            }
            <Label> {
                draw_text:{
                    text_style: <INTRO_TITLE>{font_size: 40},
                    color: #fff
                }
                text: "Great Wall"
            }
        }
    }

    ContentItem = <View> {
        width: Fill,
        height: 140,

        show_bg: true,
        draw_bg: {
            color: (BACKGROUND_ITEM_COLOR)
        }

        spacing: 10.0
        padding: 10.0

        year_wrapper = <View> {
            width: 100,
            height: Fit,

            flow: Down,
            spacing: 5.0

            year_label = <Label> {
                draw_text: {
                    text_style: <INTRO_SUBTITLE>{font_size: 12},
                    color: #fff
                }
            }
            year_label_2 = <Label> {
                draw_text: {
                    text_style: <REGULAR_TEXT>{font_size: 8},
                    color: #fff
                }
                text: "BCE"
            }
        }
        <VerticalLine> {
            height: Fill,
            draw_bg: {
                color: #fff
            }
        }
        content_wrapper = <View> {
            width: Fill,
            height: Fit,

            content_label = <Label> {
                width: Fill,

                draw_text: {
                    text_style: <REGULAR_TEXT>{font_size: 9},
                    color: #fff,
                    wrap: Word,
                }
            }
        }
    }

    Content = <View> {
        flow: Down,
        spacing: 20,

        width: Fill,
        height: 2000,

        <ContentItem> {
            year_wrapper = { year_label = { text: "700" }}
            content_wrapper = { content_label = {
                text: "First landmark of the Great Wall began originally as a square wall surrounding the state of Chu. Over the years, additional walls would be built and added to it to expand and connect territory."
            }}
        }
        <ContentItem> {
            year_wrapper = { year_label = { text: "214" }}
            content_wrapper = { content_label = {
                text: "The first Qin Emperor unifies China and links the wall of the surrounding states of Qin, Yan, and Zhao into the Great Wall of China, taking 10 years to build with hundreds of thousands of laborers."
            }}
        }
        <ContentItem> {
            year_wrapper = { year_label = { text: "121" }}
            content_wrapper = { content_label = {
                text: "A 20-year construction project was started by the Han emperor to build east and west sections of the wall, including beacons, towers, and castles. Not just for defense, but also to control trade routes like the Silk Road."
            }}
        }
        <ContentItem> {
            year_wrapper = { year_label = { text: "556" }, year_label_2 = { text: "CE" }}
            content_wrapper = { content_label = {
                text: "The Bei Qi kingdom also launched several construction projects, utilizing over 1.8 million workers to repair and extend sections of the wall, adding to its length and even building a second inner wall around Shanxi.."
            }}
        }
        <ContentItem> {
            year_wrapper = { year_label = { text: "618" }, year_label_2 = { text: "CE" }}
            content_wrapper = { content_label = {
                text: "First landmark of the Great Wall began originally as a square wall surrounding the state of Chu. Over the years, additional walls would be built and added to it to expand and connect territory."
            }}
        }
        <ContentItem> {
            year_wrapper = { year_label = { text: "1487" }, year_label_2 = { text: "CE" }}
            content_wrapper = { content_label = {
                text: "Hongzhi Emperor split the walls into north and south lines, eventually shaping it into how it is today. Since then, it has gradually fallen into disrepair and remains mostly unused."
            }}
        }
    }

    ChartItem = <RoundedView> {
        width: 10,
        height: 11,

        draw_bg: {
            border_color: #aaa
            border_width: 1.
            radius: 2.
        }
    }

    ChartRow = <View> {
        width: Fill,
        height: Fit,
        spacing: 2.0
    }

    ChartBottom = <View> {
        width: Fill,
        height: Fit,
        margin: { top: 10. }
        align: { x: 0.5, y: 0 }

        <Label> {
            draw_text: {
                text_style: <REGULAR_TEXT>{font_size: 10},
                color: #ccc
            }
            text: "700 BCE to 1487   •  CE Prehistory"
        }
    }

    Chart = <View> {
        width: Fill,
        height: Fit,
        flow: Down,
        padding: 10,
        spacing: 5,

        <ChartRow> {
            <View> { width: 170, height: 1 }
            <ChartItem> {}
            <View> { width: 30, height: 1 }
            <ChartItem> { width: 80 }
            <ChartItem> {}
        }
        <ChartRow> {
            <View> { width: 148, height: 1 }
            <ChartItem> { width: 30 }
            <View> { width: 110, height: 1 }
            <ChartItem> {}
        }
        <ChartRow> {
            <View> { width: 10, height: 1 }
            <ChartItem> {}
            <View> { width: 130, height: 1 }
            <ChartItem> {
                width: 170
                draw_bg: {
                    color: #6dc26e
                    border_color: #6dc26e
                    border_width: 1.
                    radius: 2.
                }
            }
            <View> { width: 10, height: 1 }
            <ChartItem> {}
        }

        <ChartBottom> {}
    }

    TimelineScreenInner = {{TimelineScreenInner}} {
        width: Fill, height: Fill
        flow: Overlay,

        show_bg: true,
        draw_bg: {
            color: (BACKGROUND_COLOR)
        }

       expandable_panel = <ExpandablePanel> {
            body = {
                flow: Down,
                spacing: 10,
                header = <Header> { margin: { top: 50. } }
                <Chart> {}
            }

            panel = {
                draw_bg: {
                    color: (BACKGROUND_COLOR)
                }

                scroll_handler = {
                    draw_bg: {
                        color: #aaa
                        radius: 2.
                    }
                }

                <Content> {}
            }
        }

    }

    TimelineScreenWrapper = {{TimelineScreenWrapper}} {
        width: Fill, height: Fill
        flow: Down,

        show_bg: true,
        draw_bg: {
            color: (BACKGROUND_COLOR)
        }

        <TimelineScreenInner> {}
        <View> {
            height: Fit,
            margin: 20,

            open_global_timeline_button = <Button> {
                width: Fill,
                height: 50,
                text: "OPEN GLOBAL TIMELINE",
                draw_text: {
                    text_style: {
                        font_size: 9.0
                    }

                    fn get_color(self) -> vec4 {
                        return #fff
                    }
                }

                draw_bg: { bodytop: (BACKGROUND_ITEM_COLOR), bodybottom: (BACKGROUND_ITEM_COLOR) }
            }
        }
    }

    TimelineScreen = <View> {
        width: Fill, height: Fill
        flow: Down,

        <TimelineScreenWrapper> {}
    }
}

#[derive(Live, Widget)]
pub struct TimelineScreenInner {
    #[deref]
    view: View,

    #[rust]
    touch_gesture: TouchGesture,
}

impl LiveHook for TimelineScreenInner {
    fn after_apply_from(&mut self, _cx: &mut Cx, apply: &mut Apply) {
        if apply.from.is_from_doc() {
            self.touch_gesture = TouchGesture::new();
            self.touch_gesture
                .reset(0.0, 0.0, CONTENT_LENGTH, ScrollMode::Swipe);
        }
    }
}

impl Widget for TimelineScreenInner {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        self.widget_match_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl WidgetMatchEvent for TimelineScreenInner {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions, _scope: &mut Scope) {
        for action in actions {
            match action.as_widget_action().cast() {
                ExpandablePanelAction::ScrolledAt(scroll_offset) => {
                    let header_opacity = clamp(1.0 - scroll_offset / 200.0, 0.5, 1.0);
                    let panel = self.expandable_panel(id!(expandable_panel));
                    panel.apply_over(cx, live! {
                        body = { header = { draw_bg: { opacity: (header_opacity) }}}
                    });

                    self.redraw(cx);
                }
                _ => ()
            }
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct TimelineScreenWrapper {
    #[deref]
    view: View,
}
impl Widget for TimelineScreenWrapper {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        self.widget_match_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}
impl WidgetMatchEvent for TimelineScreenWrapper {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions, scope: &mut Scope) {
        if self
            .button(id!(open_global_timeline_button))
            .clicked(&actions)
        {
            let widget_uid = self.widget_uid();
            cx.widget_action(
                widget_uid,
                &scope.path,
                StackNavigationAction::NavigateTo(live_id!(global_timeline_stack_view))
            );
        }
    }
}
