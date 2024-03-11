const YEAR_TO_POSITION_RATIO: f64 = 0.5;
const TIMELINE_YEARS_LIMITS: [i32; 2] = [-3000, 2200];

fn years_to_pixels(year: i32) -> f64 {
    year as f64 * YEAR_TO_POSITION_RATIO
}

use makepad_widgets::{
    touch_gesture::{ScrollMode, TouchGesture},
    *,
};
use std::fmt::Write;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import makepad_draw::shader::std::*;

    import crate::shared::styles::*;
    import crate::shared::helpers::*;
    import crate::shared::widgets::*;

    IMG_GREAT_WALL = dep("crate://self/resources/images/great-wall-flattened.jpg")

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
                    color: #BFABA2
                    border_color: #BFABA2
                    border_width: 1.
                    radius: 2.
                }
            }
            <View> { width: 10, height: 1 }
            <ChartItem> {}
        }

        <ChartBottom> {}
    }


    TimelineSlider = {{TimelineSlider}} {
        width: Fill,
        height: Fill,
        flow: Overlay,

        viewer_box = <View> {
            width: 50,
            height: Fill,

            show_bg: true
            draw_bg: {
                color: #fff
               // TODO: Make the box only square
               // fn pixel(self) -> vec4 {
               //     let border_width = 5.0; // Width of the space between stripes

               //     let norm_pos = pos * 2.0 - vec2(1.0, 1.0); // Normalize to -1 to 1

               //     // Determine the distance from the center (assuming square is centered)
               //     let dist_x = abs(norm_pos.x);
               //     let dist_y = abs(norm_pos.y);

               //     // Calculate the distance from the edges to determine if it's within the rectangle size
               //     let dist_from_left_right = self. / 2.0 - abs(norm_pos.x);
               //     let dist_from_top_bottom = rectangle_height / 2.0 - abs(norm_pos.y);

               //     // Use max to indirectly determine if the pixel is inside the rectangle
               //     let is_inside_rectangle_x = max(0.0, dist_from_left_right);
               //     let is_inside_rectangle_y = max(0.0, dist_from_top_bottom);

               //     // Determine the minimum distance to the edge, to identify border region without comparisons
               //     let distance_to_edge = min(is_inside_rectangle_x, is_inside_rectangle_y);
               //     let is_in_border = self.border_width - distance_to_edge;

               //     // Return color based on location
               //     if in_border {
               //         return vec4(1.0, 1.0, 1.0, 1.0);
               //     } else {
               //         return vec4(0.0, 0.0, 0.0, 0.0);
               //     }

               // }
            }
        }
    }

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

    TimelineWonderEntry = <View> {
        show_bg: true,
        draw_bg: {
            instance border_width: 0.0
            instance border_color: #0000
            instance inset: vec4(0.0, 0.0, 0.0, 0.0)
            instance radius: 18.0,

            fn get_color(self) -> vec4 {
                return self.color
            }

            fn get_border_color(self) -> vec4 {
                return self.border_color
            }

            fn pixel(self) -> vec4 {
                let sdf = Sdf2d::viewport(self.pos * self.rect_size)
                sdf.box(
                    self.inset.x + self.border_width,
                    self.inset.y + self.border_width,
                    self.rect_size.x - (self.inset.x + self.inset.z + self.border_width * 2.0),
                    self.rect_size.y - (self.inset.y + self.inset.w + self.border_width * 2.0),
                    max(1.0, self.radius)
                )
                sdf.fill_keep(self.get_color())
                if self.border_width > 0.0 {
                    sdf.stroke(self.get_border_color(), self.border_width)
                }
                return sdf.result;
            }
        }
        align: {x: 0.5, y: 0.0}

        image = <Image> {
            margin: {top: 5, bottom: 5}
            source: (IMG_GREAT_WALL)
            width: 60
            height: 75
            draw_bg: {
                instance radius: 13.
                instance scale: 0.0
                instance down: 0.0

                uniform tint_color_x: 0.0
                uniform tint_color_y: 0.0
                uniform tint_color_z: 0.0
                uniform should_tint: 0.0

                fn get_color(self) -> vec4 {
                    let source = self.get_color_scale_pan(self.image_scale, self.image_pan);

                    if self.should_tint < 0.5 {
                        return source
                    }

                    let gray = dot(source.xyz, vec3(0.6, 0.6, 0.6));
                    let tint_color = vec4(self.tint_color_x, self.tint_color_y, self.tint_color_z, 1.0);
                    let tinted = gray * tint_color;
                    let color = vec4(tinted.xyz, 0.8);
                    return color;

//                    let source = self.get_color_scale_pan(self.image_scale, self.image_pan);
//                    let tint_color = vec4(self.tint_color_x, self.tint_color_y, self.tint_color_z, 1.0);
//                    let brightness_factor = 0.5;
//                    let gray = clamp(dot(source.xyz, vec3(0.7, 0.7, 0.7)) * brightness_factor, 0.0, 1.0);
//                    let tinted = gray * tint_color;
//                    let color = vec4(tinted.xyz, source.w);
//                    return color;
                }
                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    sdf.box(
                        1,
                        1,
                        self.rect_size.x - 2.0,
                        self.rect_size.y - 2.0,
                        max(1.0, self.radius)
                    )
                    sdf.fill_keep(self.get_color());
                    return sdf.result
                }
            }
        }
    }

    TimelineWonderEntries = {{TimelineWonderEntries}} {
        width: Fill,
        height: Fit,
        flow: Overlay,

        images_deps: [
            dep("crate://self/resources/images/great-wall-flattened.jpg"),
            dep("crate://self/resources/images/petra-flattened.jpg"),
            dep("crate://self/resources/images/colosseum-flattened.jpg"),
            dep("crate://self/resources/images/chichen-itza-flattened.jpg"),
            dep("crate://self/resources/images/machu-picchu-flattened.jpg"),
            dep("crate://self/resources/images/taj-mahal-flattened.jpg"),
            dep("crate://self/resources/images/christ-the-redeemer-flattened.jpg"),
            dep("crate://self/resources/images/pyramids-of-giza-flattened.jpg"),
        ]

        wonder_entry_template: <TimelineWonderEntry> {}
    }

    GlobalTimeline = {{GlobalTimeline}} {
        flow: Down,
        width: Fill,
        height: Fill,

        initial_offset: 0.0;

        flow: Overlay,
        width: Fill,
        height: Fill,
        panel = <View> {
            flow: Right,
            width: Fill,
            height: Fit,
            align: { x: 0.5, y: 0 }
            spacing: 0,

            years = <View> {
                flow: Overlay,
                width: 80,
                height: Fit,
                padding: {right: 20, left: 20},
                spacing: 40,

                <TimelineYears> {}
            }

            wonder_entries = <TimelineWonderEntries> {}
        }

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
                    fn pixel(self) -> vec4 {
                        let line_width = 10.0; // Width of the space between stripes

                        let m = self.pos.x - ( line_width * (self.pos.x / line_width));

                        if m == 0 {
                            return vec4(0.0, 0.0, 0.0, 0.0); // Transparent
                        } else {
                            return #fff
                        }
                    }
                }
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

// TODO: This should probably be abstracted to the whole app
#[derive(Default, Clone, PartialEq)]
enum WonderType {
    #[default]
    GreatWall,
    Petra,
    Colosseum,
    ChichenItza,
    MachuPichu,
    TajMahal,
    ChristTheRedeemer,
    PyramidsOfGiza,
}

impl WonderType {
    fn bg_color(&self) -> Vec4 {
        match *self {
            Self::GreatWall => vec4(0.404, 0.522, 0.32, 1.0), // #678551
            Self::Petra => vec4(0.102, 0.102, 0.384, 1.0),    // #1A1A62
            Self::Colosseum => vec4(0.286, 0.643, 0.624, 1.0), // #49A49F
            Self::ChichenItza => vec4(0.882, 0.812, 0.733, 1.0), // #E1CFBB
            Self::MachuPichu => vec4(0.765, 0.855, 0.824, 1.0), // #C3DAD2
            Self::TajMahal => vec4(0.392, 0.157, 0.153, 1.0), // #642827
            Self::ChristTheRedeemer => vec4(0.957, 0.463, 0.404, 1.0), // #F47667
            Self::PyramidsOfGiza => vec4(0.271, 0.29, 0.612, 1.0), // #454A9C
        }
    }
}
/*
16184D + vec4(0.086, 0.094, 0.302, 1.0)
642828 + vec4(0.392, 0.157, 0.157, 1.0)
444B9B + vec4(0.267, 0.294, 0.608, 1.0)
1E736D + vec4(0.118, 0.451, 0.427, 1.0)
164F2A + vec4(0.086, 0.310, 0.165, 1.0)
0E4064 + vec4(0.055, 0.251, 0.392, 1.0)
C96454 + vec4(0.788, 0.392, 0.329, 1.0)
1C4D46 + vec4(0.110, 0.302, 0.275, 1.0)
*/
struct WonderData {
    wonder_type: WonderType,
    start_year: i32,
    end_year: i32,
    image_path: &'static str,
    // fg_color: &str,
}
// dep("crate://self/resources/images/great-wall-flattened.jpg"),
// dep("crate://self/resources/images/petra-flattened.jpg"),
// dep("crate://self/resources/images/colosseum-flattened.jpg"),
// dep("crate://self/resources/images/chichen-itza-flattened.jpg"),
// dep("crate://self/resources/images/machu-picchu-flattened.jpg"),
// dep("crate://self/resources/images/taj-mahal-flattened.jpg"),
// dep("crate://self/resources/images/christ-the-redeemer-flattened.jpg"),
// dep("crate://self/resources/images/pyramids-of-giza-flattened.jpg"),

const GREAT_WALL_DATA: WonderData = WonderData {
    wonder_type: WonderType::GreatWall,
    start_year: -700,
    end_year: 1644,
    image_path: "crate://self/resources/images/great-wall-flattened.jpg",
};
const PETRA_DATA: WonderData = WonderData {
    wonder_type: WonderType::Petra,
    start_year: -312,
    end_year: 100,
    image_path: "crate://self/resources/images/petra-flattened.jpg",
};
const COLOSSEUM_DATA: WonderData = WonderData {
    wonder_type: WonderType::Colosseum,
    start_year: 70,
    end_year: 80,
    image_path: "crate://self/resources/images/colosseum-flattened.jpg",
};
const CHICHENITZA_DATA: WonderData = WonderData {
    wonder_type: WonderType::ChichenItza,
    start_year: 550,
    end_year: 1550,
    image_path: "crate://self/resources/images/chichen-itza-flattened.jpg",
};
const MACHU_PICHU_DATA: WonderData = WonderData {
    wonder_type: WonderType::MachuPichu,
    start_year: 1450,
    end_year: 1572,
    image_path: "crate://self/resources/images/machu-picchu-flattened.jpg",
};
const TAJ_MAHAL_DATA: WonderData = WonderData {
    wonder_type: WonderType::TajMahal,
    start_year: 1632,
    end_year: 1653,
    image_path: "crate://self/resources/images/taj-mahal-flattened.jpg",
};
const CHRIST_THE_REDEEMER_DATA: WonderData = WonderData {
    wonder_type: WonderType::ChristTheRedeemer,
    start_year: 1922,
    end_year: 1931,
    image_path: "crate://self/resources/images/christ-the-redeemer-flattened.jpg",
};
const PYRAMIDS_OF_GIZA_DATA: WonderData = WonderData {
    wonder_type: WonderType::PyramidsOfGiza,
    start_year: -2600,
    end_year: -2500,
    image_path: "crate://self/resources/images/pyramids-of-giza-flattened.jpg",
};
// This array keeps the same order as the image dependency ones
const ALL_WONDERS_DATA: [WonderData; 8] = [
    GREAT_WALL_DATA,
    PETRA_DATA,
    COLOSSEUM_DATA,
    CHICHENITZA_DATA,
    MACHU_PICHU_DATA,
    TAJ_MAHAL_DATA,
    CHRIST_THE_REDEEMER_DATA,
    PYRAMIDS_OF_GIZA_DATA,
];

#[derive(Live, Widget)]
pub struct GlobalTimeline {
    #[deref]
    view: View,

    #[live]
    initial_offset: f64,
    #[rust]
    touch_gesture: Option<TouchGesture>,

    #[rust]
    current_wonder: WonderType,

    #[rust]
    current_year: i32,
    #[rust(true)]
    first_render: bool,
}

impl LiveHook for GlobalTimeline {
    fn after_new_from_doc(&mut self, _cx: &mut Cx) {
        self.current_year = GREAT_WALL_DATA.start_year;
        self.current_wonder = WonderType::GreatWall;
        self.first_render = true;

        self.timeline_wonder_entries(id!(panel.wonder_entries))
            .update_current_wonder(self.current_wonder.clone());
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

        let initial_wonder_offset =
            (GREAT_WALL_DATA.start_year - TIMELINE_YEARS_LIMITS[0]) as f64 * YEAR_TO_POSITION_RATIO;
        let initial_wonder_year = GREAT_WALL_DATA.start_year;

        if let Some(touch_gesture) = self.touch_gesture.as_mut() {
            if self.first_render {
                let scrolled_at = touch_gesture.scrolled_at + initial_wonder_offset;
                let panel_margin = self.initial_offset - scrolled_at;

                self.apply_over(
                    cx,
                    live! {
                        panel = { margin: { top: (panel_margin) }}
                    },
                );
            }
        }
        if self.first_render {
            let rounded_current_year = (initial_wonder_year.abs() / 10) * 10;
            let year_subfix = if initial_wonder_year < 0 { "BCE" } else { "CE" };

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
        }
        if let Some(touch_gesture) = self.touch_gesture.as_mut() {
            if self.first_render {
                touch_gesture.scrolled_at = ((GREAT_WALL_DATA.start_year - TIMELINE_YEARS_LIMITS[0])
                    as f64
                    * YEAR_TO_POSITION_RATIO);
                self.first_render = false;
            }
        }

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

                self.timeline_wonder_entries(id!(panel.wonder_entries))
                    .update_current_year(self.current_year);

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
pub struct TimelineWonderEntries {
    #[walk]
    walk: Walk,
    #[layout]
    layout: Layout,
    #[rust]
    area: Area,

    #[rust]
    current_year: i32,
    #[rust]
    current_wonder: WonderType,

    #[live]
    images_deps: Vec<LiveDependency>,

    #[live]
    wonder_entry_template: Option<LivePtr>,
    #[rust]
    items: ComponentMap<LiveId, WidgetRef>,
}

// TODO: Remove this trait
impl WidgetNode for TimelineWonderEntries {
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

impl Widget for TimelineWonderEntries {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        for (_id, item) in self.items.iter_mut() {
            item.handle_event(cx, event, scope);
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        cx.begin_turtle(walk, self.layout);

        self.draw_wonders_entries(cx, walk);

        cx.end_turtle_with_area(&mut self.area);

        DrawStep::done()
    }
}

impl TimelineWonderEntries {
    fn draw_wonders_entries(&mut self, cx: &mut Cx2d, walk: Walk) {
        for (i, wonder_data) in ALL_WONDERS_DATA.iter().enumerate() {
            let column = match wonder_data.wonder_type {
                WonderType::GreatWall => 0,
                WonderType::Petra => 1,
                WonderType::Colosseum => 2,
                WonderType::ChichenItza => 2,
                WonderType::MachuPichu => 1,
                WonderType::TajMahal => 2,
                WonderType::ChristTheRedeemer => 0,
                WonderType::PyramidsOfGiza => 0,
            };

            let wonder_min_height = 85.0;
            let mut wonder_height = years_to_pixels(wonder_data.end_year - wonder_data.start_year);
            let image_min_height = 75.0;
            let mut image_height = image_min_height;

            if wonder_height < wonder_min_height {
                wonder_height = wonder_min_height;
            } else if wonder_height > wonder_min_height * 1.66 {
                image_height = image_min_height * 1.66;
            }

            let width = 70.0;
            let margin = Margin {
                left: (width + 30.0) * column as f64,
                top: years_to_pixels(wonder_data.start_year - TIMELINE_YEARS_LIMITS[0]),
                right: 0.0,
                bottom: 0.0,
            };

            let item_id = LiveId(i as u64);
            let item_widget = self.items.get_or_insert(cx, item_id, |cx| {
                WidgetRef::new_from_ptr(cx, self.wonder_entry_template)
            });

            item_widget.apply_over(
                cx,
                live! {
                    draw_bg: {
                        color: (wonder_data.wonder_type.bg_color())
                    }
                },
            );

            if self.current_wonder != wonder_data.wonder_type {
                item_widget
                    .image(id!(image))
                    .set_uniform(cx, id!(should_tint), &[1.0]);

                item_widget.image(id!(image)).set_uniform(
                    cx,
                    id!(tint_color_x),
                    &[wonder_data.wonder_type.bg_color().x],
                );
                item_widget.image(id!(image)).set_uniform(
                    cx,
                    id!(tint_color_y),
                    &[wonder_data.wonder_type.bg_color().y],
                );
                item_widget.image(id!(image)).set_uniform(
                    cx,
                    id!(tint_color_z),
                    &[wonder_data.wonder_type.bg_color().z],
                );
            } else {
                item_widget
                    .image(id!(image))
                    .set_uniform(cx, id!(should_tint), &[0.0]);
            }

            item_widget.widget(id!(image)).apply_over(
                cx,
                live! {
                    height: (image_height)
                },
            );

            let image_path = self.images_deps[i].as_str();
            let _ = item_widget
                .image(id!(image))
                .load_image_dep_by_path(cx, image_path);

            let walk = Walk {
                abs_pos: walk.abs_pos,
                margin,
                width: Size::Fixed(width),
                height: Size::Fixed(wonder_height),
            };

            let _ = item_widget.draw_walk(cx, &mut Scope::empty(), walk);

            // move images
            // get a fraction from 0 - 1 based on selected yr and start/end yr of the wonder
            if self.current_year < wonder_data.end_year
                && self.current_year > wonder_data.start_year
            {
                // Make the animation smoother
                let fraction = (self.current_year as f64 - wonder_data.start_year as f64)
                    / (wonder_data.end_year as f64 - wonder_data.start_year as f64);

                item_widget.apply_over(
                    cx,
                    live! {
                        align: { y: (fraction) }
                    },
                );
            }
        }
    }
}

impl TimelineWonderEntriesRef {
    fn update_current_year(&mut self, year: i32) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.current_year = year;
        }
    }
    fn update_current_wonder(&mut self, wonder: WonderType) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.current_wonder = wonder;
        }
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

// Slider

#[derive(Live, LiveHook, Widget)]
pub struct TimelineSlider {
    #[deref]
    view: View,
    #[rust(true)]
    ready_to_swipe: bool,
}

impl Widget for TimelineSlider {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        self.animator_handle_event(cx, event);

        match event.hits(cx, self.view.area()) {
            Hit::FingerMove(fe) => {
                if !self.ready_to_swipe {
                    return;
                }

                let mut swipe_vector = fe.abs - fe.abs_start;
                // Negate y values because makepad's y axis grows to the south
                swipe_vector.y = -swipe_vector.y;

                // only trigger swipe if it is larger than some pixels
                let swipe_trigger_value = 40.;
                let diagonal_trigger_value = swipe_trigger_value / 2.;

                if (swipe_vector.x.abs() > swipe_trigger_value)
                    || (swipe_vector.y.abs() > swipe_trigger_value)
                {
                    if !self.ready_to_swipe {
                        return;
                    }
                    // compensate diagonal swipe case (both trigger the diagonal value)
                    if swipe_vector.x.abs() > diagonal_trigger_value {
                        // play animations (shrink overlay)
                        self.animator_play(cx, id!(shrink_horizontally.on));
                    }
                    if swipe_vector.y.abs() > diagonal_trigger_value {
                        self.animator_play(cx, id!(shrink_vertically.on));
                        // play animations (shrink overlay)
                    }

                    self.ready_to_swipe = false;
                }
            }
            Hit::FingerUp(_fe) => self.ready_to_swipe = true,
            _ => {}
        }

        self.update_animation(cx);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl TimelineSlider {
    fn update_animation(&mut self, cx: &mut Cx) {}
}
