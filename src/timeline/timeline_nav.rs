use makepad_widgets::*;

use super::wonders_data::{total_years_range, ALL_WONDERS_DATA, GREAT_WALL_DATA, TIMELINE_YEARS_LIMITS};
use crate::timeline::wonders_data::WonderType;

const SELECTOR_H: f64 = 90.;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import makepad_draw::shader::std::*;

    import crate::shared::styles::*;
    import crate::shared::helpers::*;
    import crate::shared::widgets::*;

    NAVIGATOR_W = 320.;
    SELECTOR_W = 75.03;
    SELECTOR_H = 90.;

    ChartItem = <RoundedView> {
        width: 40,
        height: 11,

        draw_bg: {
            border_color: #aaa
            border_width: 1.
            radius: 3.
        }
    }

    ChartRow = <View> {
        width: Fill,
        height: Fit,
        spacing: 2.0
    }

    Chart = {{Chart}}{
        show_bg: true,
        draw_bg: {
            color: #aaa
        }
        width: Fill,
        height: Fit,
        flow: Down,

        show_bg: true,
        draw_bg: {
            color: #222
        }

        item_template: <ChartItem> {}
    }

    SelectorMarker = {{SelectorMarker}} {
        width: 2,
        height: (SELECTOR_H),
        show_bg: true,
        draw_bg: {
            color: #aaa
        }
    }

    Selector = {{Selector}}<RoundedView> {
        show_bg: true,
        draw_bg: {
            border_color: #eee
            border_width: 1.
            radius: 1.
        }

        marker_template: <SelectorMarker> {}
    }

    Navigator = {{Navigator}} {
        show_bg: true,
        draw_bg: {
            color: #272624
        }
        width: Fill,
        height: Fit,
        flow: Overlay,
        show_bg: true,

        selector_template: <Selector>{}

        chart = <Chart> {}
    }

    TimelineNav = <View> {
        show_bg: true,
        draw_bg: {
            color: #333
        }
        height: Fit,
        align: {x: 0.5, y: 1}

        nav = <View> {
            width: Fill,
            height: Fit,
            align: {x: 0.5, y: 0.0}
            flow: Down
            spacing: 10
            padding: {left: 25, right: 25, bottom: 15, top: 10}

            show_bg: true,
            draw_bg: {
                color: #1f1b18
            }

            nav_label = <Label> {
                width: Fit
                height: Fit
                text: "Prehistory"
                draw_text:{
                    color: #fff
                }
            }

            horizontal_timeline = <RoundedView> {
                align: {x: 0.0, y: 0.5}
                width: Fill,
                height: 100
                show_bg: true,
                draw_bg: {
                    color: #272624
                }

                navigator = <Navigator> {}
            }
        }
    }
}

#[derive(Live, Widget)]
pub struct Navigator {
    #[deref]
    view: View,

    #[live]
    selector_template: Option<LivePtr>,
    #[rust]
    selector: Option<Selector>,

    #[rust(-700)]
    current_year: i32,
    #[rust]
    year_drag: Option<i32>,

    /// Number of years that fit in the vertical timeline
    /// for the current display
    #[rust]
    viewport_year_span: f64,
}

impl LiveHook for Navigator {
    fn after_new_from_doc(&mut self, cx: &mut Cx) {
        self.selector = Some(Selector::new_from_ptr(cx, self.selector_template));
        self.current_year = GREAT_WALL_DATA.start_year;
    }
}

impl Widget for Navigator {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let start_pos = cx.turtle().pos();

        let mut chart = self.chart(id!(chart));
        chart.set_viewport_year_span(self.viewport_year_span);

        let _ = self.view.draw_walk(cx, scope, walk);

        let actual_nav_w = self.view.area().rect(cx).size.x;

        let viewport_half = self.viewport_year_span / 2.0;
        let range_start = TIMELINE_YEARS_LIMITS[0] as f64 - viewport_half;
        // The total width of the horziontal timeline represents the total span of years,
        // plus the offset used to center the timeline
        let total_range = total_years_range() as f64 + self.viewport_year_span;

        // Normalize the current year to a value between 0 and 1
        let normalized_pos = (self.current_year as f64 - range_start) / total_range;

        // Scale the normalized position to the navigator width
        let mut x_pos = start_pos.x + normalized_pos * actual_nav_w;

        let selector_w = self.viewport_year_span * actual_nav_w / total_range;

        let timeline_offset_px = (self.viewport_year_span / 2.) * actual_nav_w / total_range;
        x_pos -= timeline_offset_px;

        let selector = self.selector.as_mut().unwrap();
        selector.current_year = self.current_year;
        selector.width = selector_w;
        selector.viewport_year_span = self.viewport_year_span;
        selector.normalized_pos = normalized_pos;
        selector.draw_walk(
            cx,
            &mut Scope::with_data(&mut dvec2(x_pos, start_pos.y)),
            walk,
        )
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, _scope: &mut Scope) {
        match event.hits(cx, self.view.draw_bg.area()) {
            Hit::FingerDown(_fe) => {
                cx.set_key_focus(self.draw_bg.area());
                self.year_drag = Some(self.current_year.clone());
            }
            Hit::FingerMove(fe) => {
                if let Some(year) = &self.year_drag {
                    let moved = fe.abs_start.x - fe.abs.x;

                    // Scale moved distnace to the navigator width
                    let scale = (TIMELINE_YEARS_LIMITS[1] - TIMELINE_YEARS_LIMITS[0]) as f64
                        / fe.rect.size.x;
                    let shifted_amount = -moved * scale;
                    self.current_year = (year + shifted_amount as i32)
                        .clamp(TIMELINE_YEARS_LIMITS[0], TIMELINE_YEARS_LIMITS[1]);
                    self.draw_bg.redraw(cx);

                    cx.action(NavigatorAction::YearChanged(self.current_year));
                }
            }
            _ => (),
        }
    }
}

impl NavigatorRef {
    pub fn set_current_year(&mut self, year: i32) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.current_year = year;
        }
    }

    pub fn set_viewport_year_span(&mut self, span: f64) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.viewport_year_span = span;
        }
    }
}

#[derive(DefaultNone, Debug, Clone)]
pub enum NavigatorAction {
    YearChanged(i32),
    None,
}

#[derive(Live, Widget)]
pub struct Selector {
    #[deref]
    view: View,

    #[live]
    marker_template: Option<LivePtr>,
    #[rust]
    marker: Option<SelectorMarker>,
    #[rust]
    current_year: i32,
    #[rust]
    width: f64,
    #[rust]
    viewport_year_span: f64,
    #[rust]
    normalized_pos: f64,
}

impl LiveHook for Selector {
    fn after_new_from_doc(&mut self, cx: &mut Cx) {
        self.marker = Some(SelectorMarker::new_from_ptr(cx, self.marker_template));
        self.current_year = GREAT_WALL_DATA.start_year;
    }
}

impl Widget for Selector {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let size_w = Size::Fixed(self.width);
        let size_h = Size::Fixed(SELECTOR_H);

        let mut pos = dvec2(0., 0.);

        if let Some(position) = scope.data.get_mut::<DVec2>() {
            pos = *position;
        }

        {
            let _ = self
                .view
                .draw_walk(cx, scope, Walk::size(size_w, size_h).with_abs_pos(pos));
        }

        let marker = self.marker.as_mut().unwrap();
        marker.current_year = self.current_year;
        marker.viewport_year_span = self.viewport_year_span;
        marker.selector_w = self.width;
        marker.normalized_pos = self.normalized_pos;

        marker.draw_walk(cx, &mut Scope::with_data(&mut pos), walk)
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct SelectorMarker {
    #[deref]
    view: View,

    #[rust(-700)]
    current_year: i32,
    #[rust]
    viewport_year_span: f64,
    #[rust]
    selector_w: f64,
    #[rust]
    normalized_pos: f64,
}

impl Widget for SelectorMarker {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, _walk: Walk) -> DrawStep {
        let size_w = Size::Fixed(1.0);
        let size_h = Size::Fixed(SELECTOR_H);

        let mut x_pos = 0.;
        let mut y_pos = 0.;

        {
            let scope_pos = scope.data.get_mut::<DVec2>();

            if let Some(start_pos) = scope_pos {
                x_pos = start_pos.x + self.normalized_pos * self.selector_w;
                y_pos = start_pos.y;
                // TODO: if we want to match exactly the original app,
                // we should have the marker meet the ends of the selector (accounting offset)
                // let timeline_offset_px = (self.viewport_year_span / 2.) * self.selector_w / total_range;
                // x_pos -= timeline_offset_px;
            }
        }

        self.view.draw_walk(
            cx,
            scope,
            Walk::size(size_w, size_h).with_abs_pos(dvec2(x_pos, y_pos)),
        )
    }
}

#[derive(Live, Widget)]
pub struct Chart {
    #[deref]
    view: View,

    #[live]
    item_template: Option<LivePtr>,
    #[rust]
    items: ComponentMap<usize, View>,
    #[rust]
    viewport_year_span: f64,
}

impl ChartRef {
    fn set_viewport_year_span(&mut self, span: f64) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.viewport_year_span = span;
        }
    }
}

impl LiveHook for Chart {
    fn after_new_from_doc(&mut self, cx: &mut Cx) {
        for (index, _wonder) in ALL_WONDERS_DATA.iter().enumerate() {
            let item = View::new_from_ptr(cx, self.item_template);
            self.items.insert(index, item);
        }
    }
}

impl Widget for Chart {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let start_pos = cx.turtle().pos();

        let _ = self.view.draw_walk(cx, scope, walk);
        let range_start = TIMELINE_YEARS_LIMITS[0] as f64;

        // The total width of the horziontal timeline represents the total span of years,
        // plus the offset used to center the timeline
        let total_w_years = total_years_range() as f64 + self.viewport_year_span;
        let total_w_years_minus_offset = total_years_range() as f64;

        let actual_nav_w_px = self.view.area().rect(cx).size.x;

        let offset_yrs = self.viewport_year_span / 2.;
        let offset_px = offset_yrs * actual_nav_w_px / total_w_years;

        let timeline_offset_px = self.viewport_year_span * actual_nav_w_px / total_w_years;

        let actual_nav_w_without_offset_px = actual_nav_w_px - timeline_offset_px;

        for (id, item) in self.items.iter_mut() {
            let wonder_data = ALL_WONDERS_DATA.get(*id).unwrap();

            // Normalize the current year to a value between 0 and 1
            let normalized_pos =
                (wonder_data.start_year as f64 - range_start) / total_w_years_minus_offset;
            // Scale the normalized position to the navigator width
            let mut pos_x = start_pos.x + normalized_pos * actual_nav_w_without_offset_px;
            pos_x += offset_px;

            let column = wonder_data.wonder_type.horizontal_column();
            let pos_y = start_pos.y + (column as f64 * 17.);

            let pos = dvec2(pos_x, pos_y);

            if wonder_data.wonder_type == WonderType::GreatWall {
                item.apply_over(
                    cx,
                    live!(
                        draw_bg: {
                            color: #e2cfba
                            border_color: #e2cfba
                            border_width: 1.
                            radius: 2.
                        }
                    ),
                )
            }

            let wonder_year_range_yrs = wonder_data.end_year - wonder_data.start_year;
            let bar_width = wonder_year_range_yrs as f64 * actual_nav_w_without_offset_px
                / total_w_years_minus_offset;

            let _ = item.draw_walk(
                cx,
                scope,
                Walk::size(Size::Fixed(max(15., bar_width)), Size::Fixed(15.)).with_abs_pos(pos),
            );
        }
        DrawStep::done()
    }
}
