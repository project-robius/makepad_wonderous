use makepad_widgets::*;
use makepad_widgets::widget::WidgetCache;

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

    Wonder = {{Wonder}} {
        flow: Overlay,

        intro = <View> {
            //visible: false,

            flow: Overlay,
            width: Fill,
            height: Fill,

            show_bg: true
            draw_bg: {
                color: #8b9e77
            }

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
            left_great_wall = <Image> {
                source: (IMG_FG_LEFT_GREAT_WALL),
                abs_pos: vec2(-260, 440),

                width: (1386 * 0.35),
                height: (1764 * 0.35),
            }
            right_great_wall = <Image> {
                source: (IMG_FG_RIGHT_GREAT_WALL),
                abs_pos: vec2(130, 270),

                width: (1386 * 0.45),
                height: (1764 * 0.45),
            }
        }

        title = <View> {
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

        header = <View> {
            visible: false,
            flow: Overlay,
            width: Fill,
            height: Fill,

            <View> {
                show_bg: true,
                draw_bg: {
                    color: #8b9e77
                }

                width: Fill,
                height: 250,

                <Image> {
                    source: (IMG_BACKGROUND_ROLLER),
                    abs_pos: vec2(-60, -20),

                    width: (1476 * 0.6),
                    height: (1371 * 0.6),

                    draw_bg: {
                        instance opacity: 0.2
                    }
                }
            }
            <Image> {
                source: (IMG_SUN),
                abs_pos: vec2(100, 0),
                width: (200 * 0.6),
                height: (202 * 0.6),
            }
            <Image> {
                source: (IMG_GREAT_WALL),
                abs_pos: vec2(60, 30),

                width: (1476 * 0.185),
                height: (1371 * 0.185),
            }
        }
    }
}

enum WonderState {
    Intro,
    Content,
}

#[derive(Live)]
pub struct Wonder {
    #[deref]
    view: View,

    #[rust(WonderState::Intro)]
    state: WonderState,

    #[rust]
    dragging: bool,
    #[rust]
    last_abs: DVec2,
    #[rust]
    init_drag_time: f64,
}

impl LiveHook for Wonder {
    fn before_live_design(cx: &mut Cx) {
        register_widget!(cx, Wonder);
    }
}

impl Widget for Wonder {
    fn handle_widget_event_with(
        &mut self,
        cx: &mut Cx,
        event: &Event,
        dispatch_action: &mut dyn FnMut(&mut Cx, WidgetActionItem),
    ) {
        self.view.handle_widget_event_with(cx, event, dispatch_action);

        match self.state {
            WonderState::Intro => {
                self.handle_intro_event(cx, event);
            }
            WonderState::Content => {
                //self.handle_content_event(cx, event);
            }
        }
    }

    fn walk(&mut self, cx: &mut Cx) -> Walk {
        self.view.walk(cx)
    }

    fn redraw(&mut self, cx: &mut Cx) {
        self.view.redraw(cx);
    }

    fn find_widgets(&mut self, path: &[LiveId], cached: WidgetCache, results: &mut WidgetSet) {
        self.view.find_widgets(path, cached, results);
    }

    fn draw_walk_widget(&mut self, cx: &mut Cx2d, walk: Walk) -> WidgetDraw {
        let _ = self.view.draw_walk_widget(cx, walk);
        WidgetDraw::done()
    }
}

impl Wonder {
    fn handle_intro_event(&mut self, cx: &mut Cx, event: &Event) {
        match event.hits_with_capture_overload(cx, self.view.area(), true) {
            Hit::FingerDown(fe) => {
                self.last_abs = fe.abs;
                self.init_drag_time = fe.time;
            }
            Hit::FingerMove(fe) => {
                let time_elapsed = fe.time - self.init_drag_time;
                if time_elapsed > 0.15 {
                    self.dragging = true;
                    let delta = (self.last_abs.y - fe.abs.y) * 0.6;

                    if delta > 60. {
                        self.state = WonderState::Content;
                        self.view(id!(intro)).set_visible(false);
                        self.view(id!(header)).set_visible(true);

                        self.reset_intro_dragging(cx);
                    } else if delta > 0. {
                        let left_image = self.view(id!(left_great_wall));
                        left_image.apply_over(cx, live!{
                            margin: {top: (-delta), left: (-delta / 2.)},
                            width: (1386. * 0.35 + delta),
                            height: (1764. * 0.35 + delta * (1764. / 1386.))
                        });
                        left_image.redraw(cx);

                        let right_image = self.view(id!(right_great_wall));
                        right_image.apply_over(cx, live!{
                            margin: {top: (-delta), left: (-delta / 2.)},
                            width: (1386. * 0.45 + delta),
                            height: (1764. * 0.45 + delta * (1764. / 1386.))
                        });
                        right_image.redraw(cx);
                    }
                }
            }
            Hit::FingerUp(fe) => {
                self.reset_intro_dragging(cx);
            }
            _ => {}
        }
    }

    fn reset_intro_dragging(&mut self, cx: &mut Cx) {
        self.dragging = false;

        let left_image = self.view(id!(left_great_wall));
        left_image.apply_over(cx, live!{
            margin: {top: 0, left: 0},
            width: (1386. * 0.35),
            height: (1764. * 0.35)
        });
        left_image.redraw(cx);

        let right_image = self.view(id!(right_great_wall));
        right_image.apply_over(cx, live!{
            margin: {top: 0, left: 0},
            width: (1386. * 0.45),
            height: (1764. * 0.45)
        });
        right_image.redraw(cx);
    }
}
