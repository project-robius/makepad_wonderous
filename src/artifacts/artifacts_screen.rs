use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;

    import crate::shared::styles::*;
    import crate::shared::widgets::*;

    import crate::artifacts::artifacts_carrousel::*;

    ArtifactsScreen = <View> {
        width: Fill, height: Fill
        flow: Down,
        spacing: 10.0,
        align: {x: 0.5, y: 0.0},

        show_bg: true,
        draw_bg: {
            color: #004
        }

        <Label> {
            margin: { top: 30.0 }
            draw_text:{
                text_style: <SUBTITLE_CAPTION>{font_size: 16},
                color: #fff
            }
            text: "ARTIFACTS"
        }

        <ArtifactsCarrousel> {
            margin: { top: 30.0 }
        }
    }
}
