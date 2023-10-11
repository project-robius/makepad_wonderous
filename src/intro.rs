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

    Intro = <View> {
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
}