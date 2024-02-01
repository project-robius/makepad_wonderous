use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;

    import crate::shared::styles::*;
    import crate::shared::widgets::*;

    import crate::artifacts::artifacts_carrousel::*;

    BACKGROUND_ITEM_COLOR = #333

    ArtifactsScreen = <View> {
        width: Fill, height: Fill
        flow: Overlay,
        align: {x: 0.5, y: 0.0},

        <Label> {
            margin: { top: 40.0 }
            draw_text:{
                text_style: <SUBTITLE_CAPTION>{font_size: 10},
                color: #fff
            }
            text: "ARTIFACTS"
        }

        <ArtifactsCarrousel> {
            margin: { top: 30.0 }
        }

        <View> {
            height: Fill,
            margin: 20,
            flow: Down,
            
            <View> { height: Fill, width: 1 }

            <Button> {
                width: Fill,
                height: 50,
                text: "BROWSE ALL ARTIFACTS",
                draw_text: {
                    text_style: {
                        font_size: 9.0
                    }

                    fn get_color(self) -> vec4 {
                        return #fff
                    }
                }

                draw_bg: { bodytop: (BACKGROUND_ITEM_COLOR), bodybottom: (BACKGROUND_ITEM_COLOR) }
            }
        }
    }
}
