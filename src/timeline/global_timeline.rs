// TODO:
//   - Make initial offset half the timeline height on load

const YEAR_TO_POSITION_RATIO: f64 = 0.5;
const TIMELINE_YEARS_LIMITS: [i32; 2] = [-3000, 2200];

use makepad_widgets::{
    touch_gesture::{ScrollMode, TouchGesture},
    *,
};
use std::fmt::Write;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;

    import crate::shared::styles::*;
    import crate::shared::helpers::*;
    import crate::shared::widgets::*;

    IMG_GREAT_WALL = dep("crate://self/resources/images/great-wall.png")

    TimelineYear = <View> {
        width: Fit,
        height: Fit,
        label = <Label> {
            height: 0
            width: Fit
            text: "100"
            margin: { bottom: 0 }
            draw_text: {
                text_style: <MONO_TEXT>{font_size: 9},
                color: #fff,
                wrap: Word,
            }
        }
    }


    TimelineYears = {{TimelineYears}} {
        width: Fill,
        height: Fit,
        flow: Down,
        spacing: 40,

        timeline_year_template: <TimelineYear> {}
    }

    TimelineWonder = <RoundedView> {
        width: 50.0, height: 400.0,
        margin: {bottom: 0}
        show_bg: true,
        draw_bg: {
            instance radius: 12.0,
            color: #0cb328
        }
    }

    GlobalTimeline = {{GlobalTimeline}} {
        flow: Overlay,
        width: Fill,
        height: Fill,

        initial_offset: 0.0;

        year_info = <View> {
            flow: Right,
            width: Fill,
            height: Fill,
            align: { y: 0.5, x: 1.}
            margin: { bottom: 30, right: 10}
            spacing: 5

            year = <Label> {
                width: Fit
                text: "2000"
                align: { y: 0.5}
                draw_text: {
                    text_style: <MONO_TEXT>{font_size: 16},
                    color: #fff,
                    wrap: Word,
                }
            }

            year_subfix = <Label> {
                width: Fit
                text: "BCE"
                align: { y: 0.5}
                draw_text: {
                    text_style: <MONO_TEXT>{font_size: 10},
                    color: #fff,
                    wrap: Word,
                }
            }

        }

        center_line = <View> {
            flow: Overlay,
            width: Fill,
            height: Fill,
            align: { y: 0.5 }


            line = <View> {
                flow: Overlay,
                width: Fill,
                height: 1,
                margin: 0.0,
                padding: 0.0, spacing: 0.0
                show_bg: true
                draw_bg: {
                    color: #fff
                    // TODO: Make the line striped
                   // fn pixel(self) -> vec4 {
                   //     // Define the width of the stripped line
                   //     let strip_width = 5.0; // Adjust as needed

                   //     // Check if the pixel is within the strip width
                   //     let strip_position = self.pos.x - self.pos.x / strip_width * strip_width;
                   //     let strip_a = strip_position < 1.0;

                   //     // If within the strip width, make the pixel transparent
                   //     if mod(self.pos.x, strip_width) != 0 {
                   //         return vec4(0.0, 0.0, 0.0, 0.0); // Transparent
                   //     } else {
                   //         return #fff
                   //     }
                   // }
                }
            }
        }

        panel = <View> {
            flow: Down,
            width: Fill,
            height: Fit,
            align: { x: 0.5, y: 0 }
            spacing: 10,


            years = <View> {
                flow: Down,
                width: Fill,
                height: Fit,
                padding: {right: 20, left: 20},
                spacing: 40,

                <TimelineYears> {}
            }

        }
    }

    GlobalTimelineScreen = <View> {
        width: Fill, height: Fill
        flow: Right,

        show_bg: true,
        draw_bg: {
            color: #1f1b18
        }

        <GlobalTimeline> {}
    }
}

#[derive(Live, Widget)]
pub struct GlobalTimeline {
    #[deref]
    view: View,
    #[animator]
    animator: Animator,

    #[live]
    initial_offset: f64,
    #[rust]
    touch_gesture: Option<TouchGesture>,

    #[rust]
    current_year: i32,
}

impl LiveHook for GlobalTimeline {
    fn after_new_from_doc(&mut self, _cx: &mut Cx) {
        self.current_year = 0;
    }

    fn after_apply_from(&mut self, cx: &mut Cx, apply: &mut Apply) {
        if apply.from.is_from_doc() {
            self.apply_over(
                cx,
                live! {
                    panel = { margin: { top: (self.initial_offset) }}
                },
            );
        }
    }
}

impl Widget for GlobalTimeline {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let timeline_height = self.area().rect(cx).size.y;
        self.initial_offset = timeline_height / 2.;
        self.view.handle_event(cx, event, scope);

        if let Some(touch_gesture) = self.touch_gesture.as_mut() {
            if touch_gesture
                .handle_event(cx, event, self.view.area())
                .has_changed()
            {
                let scrolled_at = touch_gesture.scrolled_at;
                let panel_margin = self.initial_offset - scrolled_at;

                self.apply_over(
                    cx,
                    live! {
                        panel = { margin: { top: (panel_margin) }}
                    },
                );

                // update current year
                self.current_year = ((scrolled_at) / YEAR_TO_POSITION_RATIO) as i32;
                // convert to the negative limits
                self.current_year += TIMELINE_YEARS_LIMITS[0];
                // keep the year within the limits
                self.current_year = self
                    .current_year
                    .clamp(TIMELINE_YEARS_LIMITS[0], TIMELINE_YEARS_LIMITS[1]);

                // TODO: Round up the displayed year
                let rounded_current_year = (self.current_year.abs() / 10) * 10;
                let year_subfix = if self.current_year < 0 { "BCE" } else { "CE" };

                self.apply_over(
                    cx,
                    live! {
                        year_info = {
                            year = {
                                text: (rounded_current_year.to_string())
                            }
                            year_subfix = {
                                text: (year_subfix)
                            }
                        }
                    },
                );

                self.redraw(cx);
            }
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let result = self.view.draw_walk(cx, scope, walk);

        if self.touch_gesture.is_none() {
            let mut touch_gesture = TouchGesture::new();
            touch_gesture.set_mode(ScrollMode::Swipe);

            // Limit the amount of dragging allowed for the panel
            let panel_height = self.view(id!(panel)).area().rect(cx).size.y;
            touch_gesture.set_range(0.0, panel_height - self.initial_offset);

            touch_gesture.reset_scrolled_at();
            self.touch_gesture = Some(touch_gesture);
        }

        result
    }
}

#[derive(Live, LiveHook, LiveRegisterWidget, WidgetRef)]
pub struct TimelineYears {
    #[walk]
    walk: Walk,
    #[layout]
    layout: Layout,
    #[rust]
    area: Area,

    #[live]
    timeline_year_template: Option<LivePtr>,
    #[rust]
    items: ComponentMap<LiveId, WidgetRef>,
}

// TODO: Remove this trait
impl WidgetNode for TimelineYears {
    fn walk(&mut self, _cx: &mut Cx) -> Walk {
        self.walk
    }

    fn redraw(&mut self, cx: &mut Cx) {
        self.area.redraw(cx)
    }

    fn find_widgets(&mut self, path: &[LiveId], cached: WidgetCache, results: &mut WidgetSet) {
        for item in self.items.values_mut() {
            item.find_widgets(path, cached, results);
        }
    }
}

impl Widget for TimelineYears {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        for (_id, item) in self.items.iter_mut() {
            item.handle_event(cx, event, scope);
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        cx.begin_turtle(walk, self.layout);

        self.draw_years(cx, walk);

        // draw wonders

        cx.end_turtle_with_area(&mut self.area);

        DrawStep::done()
    }
}

impl TimelineYears {
    fn draw_years(&mut self, cx: &mut Cx2d, walk: Walk) {
        let year_spacing = YEAR_TO_POSITION_RATIO * 100.0;
        let year_height = 10.0;

        let mut year_string_buffer = String::new();
        for year in (TIMELINE_YEARS_LIMITS[0]..=TIMELINE_YEARS_LIMITS[1]).step_by(100) {
            let item_id = LiveId(year as u64);
            let item_widget = self.items.get_or_insert(cx, item_id, |cx| {
                WidgetRef::new_from_ptr(cx, self.timeline_year_template)
            });

            year_string_buffer.clear();
            write!(&mut year_string_buffer, "{}", year.abs()).expect("Failed to write");

            item_widget.widget(id!(label)).set_text(&year_string_buffer);

            let walk = Walk {
                abs_pos: walk.abs_pos,
                margin: walk.margin,
                width: walk.width,
                height: Size::Fixed(year_height),
            };

            let _ = item_widget.draw_walk(cx, &mut Scope::empty(), walk);
        }

        // We subtract the height of the year view so we have a true spacing of the declared one
        self.apply_over(
            cx,
            live! {
                spacing: (year_spacing - year_height)
            },
        );
    }
}
