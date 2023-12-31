use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;

    import crate::shared::styles::*;
    import crate::shared::widgets::*;

    TimelineScreen = <View> {
        width: Fill, height: Fill
        flow: Right,

        show_bg: true,
        draw_bg: {
            color: #fff
        }
    }
}
