use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;

    import crate::shared::styles::*;

    IMG_SUN = dep("crate://self/resources/sun.png")
    IMG_CLOUD = dep("crate://self/resources/cloud-white.png")
    IMG_GREAT_WALL = dep("crate://self/resources/great-wall.png")
    IMG_FG_LEFT_GREAT_WALL = dep("crate://self/resources/foreground_left_great_wall.png")
    IMG_FG_RIGHT_GREAT_WALL = dep("crate://self/resources/foreground_right_great_wall.png")
    IMG_BACKGROUND_ROLLER = dep("crate://self/resources/roller-1-black.png")

    Intro = <View> {
        flow: Overlay,

        show_bg: true
        draw_bg: {
            color: #8b9e77
        }

        <View> {
            flow: Overlay,
            width: Fill,
            height: Fill,

            <Image> {
                source: (IMG_BACKGROUND_ROLLER),

                width: (1476 * 0.6),
                height: (1371 * 0.6),

                draw_bg: {
                    instance opacity: 0.2
                }
            }
            <Image> {
                source: (IMG_SUN),
                abs_pos: vec2(30, 35),
                width: 200,
                height: 202,
            }
            <Image> {
                source: (IMG_CLOUD),
                abs_pos: vec2(-5, 130),
                width: 280,
                height: 45,

                draw_bg: {
                    instance opacity: 0.5
                }
            }
            <Image> {
                source: (IMG_CLOUD),
                abs_pos: vec2(165, 55),
                width: 280,
                height: 45,

                draw_bg: {
                    instance opacity: 0.5
                }
            }
            <Image> {
                source: (IMG_GREAT_WALL),
                abs_pos: vec2(-100, 48),

                width: (1476 * 0.4),
                height: (1371 * 0.4),
            }
            <Image> {
                source: (IMG_FG_LEFT_GREAT_WALL),
                abs_pos: vec2(-260, 440),

                width: (1386 * 0.35),
                height: (1764 * 0.35),
            }
            <Image> {
                source: (IMG_FG_RIGHT_GREAT_WALL),
                abs_pos: vec2(130, 270),

                width: (1386 * 0.45),
                height: (1764 * 0.45),
            }
        }

        <View> {
            flow: Down,
            width: Fill,
            height: Fill,

            align: { x: 0.5, y: 0.85 }
 
            <Label> {
                draw_text:{
                    text_style: <INTRO_TEXT>{font_size: 14},
                    color: #fff
                }
                text: "the"
            }
            <Label> {
                draw_text:{
                    text_style: <INTRO_TEXT>{font_size: 40},
                    color: #fff
                }
                text: "Great Wall"
            }
        }
    }
}