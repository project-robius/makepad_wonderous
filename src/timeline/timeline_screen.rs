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

    TimelineScreen = <View> {
        width: Fill, height: Fill
        flow: Right,

        show_bg: true,
        draw_bg: {
            color: #222
        }

        <Header> {
            margin: { top: 50. }
        }
    }
}
