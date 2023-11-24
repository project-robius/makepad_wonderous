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

    REGULAR_TEXT = {
        font_size: (10),
        font: {path: dep("crate://self/resources/fonts/Raleway-Regular.ttf")}
    }

    ITALIC_TEXT = {
        font_size: (10),
        font: {path: dep("crate://self/resources/fonts/Raleway-Italic.ttf")}
    }

    REGULAR_ITALIC_TEXT = {
        font_size: (10),
        font: {path: dep("crate://self/resources/fonts/Raleway-MediumItalic.ttf")}
    }

    MONO_TEXT = {
        font_size: (12),
        font: {path: dep("crate://self/resources/fonts/B612Mono-Regular.ttf")}
    }

    DECORATIVE_TEXT = {
        font_size: (16),
        font: {path: dep("crate://self/resources/fonts/CinzelDecorative-Regular.ttf")}
    }

    DECORATIVE_BOLD_TEXT = {
        font_size: (16),
        font: {path: dep("crate://self/resources/fonts/CinzelDecorative-Bold.ttf")}
    }
}
