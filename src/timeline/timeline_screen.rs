use makepad_widgets::*;
use crate::shared::touch_gesture::*;

const CONTENT_LENGTH: f64 = 800.;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import makepad_draw::shader::std::*;

    import crate::shared::styles::*;
    import crate::shared::widgets::*;

    IMG_HEADER = dep("crate://self/resources/images/great-wall-flattened.jpg")

    Header = <CachedView> {
        flow: Overlay,
        width: Fill,
        height: 340,

        align: { x: 0.5, y: 0 }

        draw_bg: {
            instance blur_radius: 0.0

            varying radius: float
            varying initial_offset: vec2
            varying step_delta: vec2

            fn get_color_pos(self, pos: vec2) -> vec4 {
                let color = sample2d_rt(self.image, pos);
                return Pal::premul(vec4(color.xyz, color.w))
            }

            fn coeffs(i: float) -> float {
                if i - 0.1 < 0.0 { return 0.0369543295325685 }
                else if i - 0.1 < 1.0 { return 0.05707225834646017 }
                else if i - 0.1 < 2.0 { return 0.08056214824901288 }
                else if i - 0.1 < 3.0 { return 0.10394046353380644 }
                else if i - 0.1 < 4.0 { return 0.1279319631769404 }
                else if i - 0.1 < 5.0 { return 0.14953785857076586 }
                else if i - 0.1 < 6.0 { return 0.13014234492770832 }
                else if i - 0.1 < 7.0 { return 0.11718098483901983 }
                else if i - 0.1 < 8.0 { return 0.10394046353380644 }
                else if i - 0.1 < 9.0 { return 0.0885838874611158 }
                else if i - 0.1 < 10.0 { return 0.07253844158670234 }
                else if i - 0.1 < 11.0 { return 0.04987059658718462 }
                else { return 0.0; }
            }

            fn vertex(self) -> vec4 {
                self.radius = self.blur_radius / 10000.0 * self.rect_size.x;
                self.initial_offset = vec2(-16.0 / self.rect_size.x, -16.0 / self.rect_size.y);
                self.step_delta = vec2(3.0 / self.rect_size.x, 3.0 / self.rect_size.y);

                return self.clip_and_transform_vertex(self.rect_pos, self.rect_size * 2.0)
            }

            fn pixel(self) -> vec4 {
                let color = vec4(0.0, 0.0, 0.0, 0.0);
                let pos = self.pos;
                let i = 0.0;
                let j = 0.0;

                let posx = self.initial_offset.x;
                let posy = self.initial_offset.y;
                for step in 0..11 {
                    j = 0.0;
                    posy = self.initial_offset.y;
                    for b in 0..11 {
                        pos = self.pos + vec2(posx, posy) * self.radius;
                        color += self.get_color_pos(pos) * coeffs(i) * coeffs(j);

                        posy += self.step_delta.x;
                        j += 1.0;
                    }
                    posx += self.step_delta.y;
                    i += 1.0;
                }

                return color;
            }
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
        height: 120,

        show_bg: true,
        draw_bg: {
            color: #333
        }

        spacing: 10.0
        padding: 10.0

        year_wrapper = <View> {
            width: 70,
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
        width: Fill,
        height: 2000,

        flow: Down,
        spacing: 20.

        margin: { top: 400. }
        padding: 20.

        show_bg: true,
        draw_bg: {
            color: #222
        }

        align: { x: 0.5, y: 0 }

        <RoundedView> {
            width: 40,
            height: 6,

            draw_bg: {
                color: #aaa
                radius: 2.
            }
        }

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

    TimelineScreen = {{TimelineScreen}} {
        width: Fill, height: Fill
        flow: Overlay,

        show_bg: true,
        draw_bg: {
            color: #222
        }

        header = <Header> { margin: { top: 50. }}
        content = <Content> {}
    }
}

#[derive(Live, Widget)]
pub struct TimelineScreen {
    #[deref]
    view: View,

    #[rust]
    touch_gesture: TouchGesture,
}

impl LiveHook for TimelineScreen {
    fn after_apply_from(&mut self, _cx: &mut Cx, apply: &mut Apply) {
        if apply.from.is_from_doc() {
            self.touch_gesture = TouchGesture::new();
            self.touch_gesture.reset(0.0, 0.0, CONTENT_LENGTH, ScrollMode::Swipe);
        }
    }
}

impl Widget for TimelineScreen {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);

        self.touch_gesture.handle_event(cx, event, self.view.area());

        let header_blur = min(6.0, self.touch_gesture.scroll_offset / 30.);
        let content_margin = 400. - self.touch_gesture.scroll_offset;

        self.apply_over(cx, live! {
            header = { draw_bg: { blur_radius: (header_blur)} }
            content = { margin: { top: (content_margin) }}
        });

        // TODO avoid calling redraw all the time
        self.redraw(cx);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}