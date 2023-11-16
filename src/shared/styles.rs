use makepad_widgets::*;

live_design! {
    INTRO_TITLE = {
        font_size: (12),
        font: {path: dep("crate://self/resources/fonts/YesevaOne-Regular.ttf")}
    }

    INTRO_SUBTITLE = {
        font_size: (10),
        font: {path: dep("crate://self/resources/fonts/TenorSans-Regular.ttf")}
    }

    SUBTITLE_CAPTION = {
        font_size: (10),
        font: {path: dep("crate://self/resources/fonts/Raleway-Bold.ttf")}
    }

    MONO_TEXT = {
        font_size: (12),
        font: {path: dep("crate://self/resources/fonts/B612Mono-Regular.ttf")}
    }
}
