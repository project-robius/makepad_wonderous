use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;

    import crate::shared::styles::*;
    import crate::shared::widgets::*;

    IMG_HEADER = dep("crate://self/resources/images/great-wall-flattened.jpg")

    Header = <View> {
        flow: Overlay,
        width: Fill,
        height: 340,

        align: { x: 0.5, y: 0 }

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
        width: Fill,
        height: 2000,

        flow: Down,
        spacing: 20.

        margin: { top: 400. }
        padding: 20.

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

    TimelineScreen = <View> {
        width: Fill, height: Fill
        flow: Overlay,

        show_bg: true,
        draw_bg: {
            color: #222
        }

        <Header> {
            margin: { top: 50. }
        }
        <Content> {}
    }
}
