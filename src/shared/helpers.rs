use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;

    Divider = <View> {
        width: Fill, height: Fit
        flow: Down
        <RoundedView> {
            width: Fill,
            height: 1.,
            draw_bg: {color: (#ddd)}
        }
    }

    FillerX = <View> { width: Fill, height: Fit }
    FillerY = <View> { width: Fit, height: Fill }
}
