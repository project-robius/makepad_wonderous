use makepad_widgets::*;
use makepad_widgets::widget::WidgetCache;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;

    import crate::shared::styles::*;
    import makepad_draw::shader::std::*;

    IMG_SUN = dep("crate://self/resources/sun.png")
    IMG_CLOUD = dep("crate://self/resources/cloud-white.png")
    IMG_GREAT_WALL = dep("crate://self/resources/great-wall.png")
    IMG_FG_LEFT_GREAT_WALL = dep("crate://self/resources/foreground_left_great_wall.png")
    IMG_FG_RIGHT_GREAT_WALL = dep("crate://self/resources/foreground_right_great_wall.png")
    IMG_BACKGROUND_ROLLER = dep("crate://self/resources/roller-1-black.png")
    IMG_COMPASS = dep("crate://self/resources/compass-icon.png")

    FadeView = <CachedView> {
        dpi_factor: 2.0,

        draw_bg: {
            instance opacity: 1.0

            fn pixel(self) -> vec4 {
                let color = sample2d_rt(self.image, self.pos);
                return Pal::premul(vec4(color.xyz, color.w * self.opacity))
            }
        }
    }

    Line = <View> {
        width: Fill,
        height: 1,
        show_bg: true,
        draw_bg: {
            color: #8b9e77
        }
    }

    Wonder = {{Wonder}} {
        flow: Overlay,

        show_bg: true,
        draw_bg: {
            color: #5d2a2c
        }

        subtitle_group = <FadeView> {
            visible: false,
            flow: Down,
            width: Fill,
            height: Fit,

            abs_pos: vec2(0, 340.0),
            spacing: 100.0

            subtitle = <View> {
                width: Fill,
                height: Fit,

                align: { x: 0.5, y: 0.5 }
                spacing: 35.0
                margin: {left: 35.0, right: 35.0}

                <Line> {}
                <Label> {
                    draw_text:{
                        text_style: <INTRO_SUBTITLE>{font_size: 9},
                        color: #fff
                    }
                    text: "LONGEST STRUCTURE ON EARTH"
                }
                <Line> {}
            }

            subtitle_bottom = <View> {
                flow: Down,
                width: Fill,
                height: Fit,

                spacing: 20.0

                <View> {
                    width: Fill,
                    height: Fit,

                    align: { x: 0.5, y: 0.5 }

                    <Label> {
                        draw_text:{
                            text_style: <INTRO_SUBTITLE>{font_size: 10},
                            color: #fff
                        }
                        text: "CHINA"
                    }
                }

                footer = <View> {
                    width: Fill,
                    height: Fit,

                    align: { x: 0.5, y: 0.5 }
                    spacing: 35.0
                    margin: {left: 35.0, right: 35.0}

                    <Line> {}
                    compass = <RotatedImage> {
                        width: 30,
                        height: 30,
                        source: (IMG_COMPASS),

                        draw_bg: {
                            instance rotation: -0.5
                        }
                    }
                    <Line> {}
                }
            }
        }

        intro = <FadeView> {
            flow: Overlay,
            width: Fill,
            height: Fill,

            <View> {
                width: Fill,
                height: Fill,
                show_bg: true,
                draw_bg: {
                    color: #8b9e77
                }
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

        header = <FadeView> {
            visible: false,
            flow: Overlay,
            width: Fill,
            height: 280, // Issue: height: Fit, doesn't work as expected

            draw_bg: { instance opacity: 0.0 }

            <View> {
                show_bg: true,
                draw_bg: {
                    color: #8b9e77
                }

                width: Fill,
                height: 230,

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
            sun = <Image> {
                source: (IMG_SUN),
                abs_pos: vec2(100, 60),
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

        animator: {
            intro = {
                default: show,
                hide = {
                    redraw: true,
                    from: {all: Forward {duration: 0.2}}
                    apply: {
                        intro = { draw_bg: { instance opacity: 0.0 }}
                        header = { draw_bg: { instance opacity: 1.0 }}
                    }
                }
                show = {
                    redraw: true,
                    from: {all: Forward {duration: 0.2}}
                    apply: {
                        intro = { draw_bg: { instance opacity: 1.0 }}
                        header = { draw_bg: { instance opacity: 0.0 }}
                    }
                }
            },
            sun = {
                default: hide,
                show = {
                    ease: OutExp
                    from: {all: Forward {duration: 0.5}}
                    apply: {
                        header = { sun = { abs_pos: vec2(100, 0) }}
                    }
                }
                hide = {
                    ease: OutExp
                    from: {all: Forward {duration: 0.5}}
                    apply: {
                        header = { sun = { abs_pos: vec2(100, 60) }}
                    }
                }
            },
            title = {
                default: intro,
                content = {
                    ease: InExp
                    from: {all: Forward {duration: 0.3}}
                    apply: {
                        title = { align: { x: 0.5, y: 0.5 } }
                    }
                }
                intro = {
                    ease: InExp
                    from: {all: Forward {duration: 0.3}}
                    apply: {
                        title = { align: { x: 0.5, y: 0.85 } }
                    }
                }
            }
            subtitle_on_content = {
                default: will_show,
                will_show = {
                    from: {all: Snap}
                    apply: {
                        subtitle_group = {
                            subtitle = {
                                spacing: 35.0
                                margin: {left: 35.0, right: 35.0}
                            }
                            subtitle_bottom = { footer = {
                                spacing: 80.0
                                margin: {left: 80.0, right: 80.0}
                            }}
                        }
                    }
                }
                show = {
                    from: {all: Forward {duration: 1.0}}
                    apply: {
                        subtitle_group = {
                            subtitle = {
                                spacing: 15.0
                                margin: {left: 15.0, right: 15.0}
                            }
                            subtitle_bottom = { footer = {
                                spacing: 15.0
                                margin: {left: 15.0, right: 15.0}
                            }}
                        }
                    }
                }
            }
            subtitle_on_intro = {
                default: reset,
                reset = {
                    from: {all: Snap}
                    apply: {
                        subtitle_group = {
                            abs_pos: vec2(0, 340),
                            draw_bg: { instance opacity: 1.0 }
                        }
                    }
                }
                will_show = {
                    from: {all: Snap}
                    apply: {
                        subtitle_group = {
                            abs_pos: vec2(0, 340),
                            draw_bg: { instance opacity: 1.0 }
                        }
                    }
                }
                show = {
                    from: {all: Forward {duration: 1.0}}
                    apply: {
                        subtitle_group = {
                            abs_pos: vec2(0, 0),
                            draw_bg: { instance opacity: 0.0 }
                        }
                    }
                }
            }
            compass = {
                default: hide,
                show = {
                    ease: OutBack
                    from: {all: Forward {duration: 2.0}}
                    apply: {
                        subtitle_group = { subtitle_bottom = { footer =
                            { compass = { draw_bg: { instance rotation: 0 }}}
                        }}
                    }
                }
                hide = {
                    from: {all: Snap}
                    apply: {
                        subtitle_group = { subtitle_bottom = { footer =
                            { compass = { draw_bg: { instance rotation: -3.0 }}}
                        }}
                    }
                }
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

    #[animator]
    animator: Animator,
    
    #[rust]
    next_frame: NextFrame,
}

impl LiveHook for Wonder {
    fn before_live_design(cx: &mut Cx) {
        register_widget!(cx, Wonder);
    }

    fn after_apply(&mut self, cx: &mut Cx, from: ApplyFrom, _index: usize, _nodes: &[LiveNode]) {
        if from.is_from_doc() {
            self.state = WonderState::Intro;
            self.next_frame = cx.new_next_frame();
        }
    }
}

impl Widget for Wonder {
    fn handle_widget_event_with(
        &mut self,
        cx: &mut Cx,
        event: &Event,
        dispatch_action: &mut dyn FnMut(&mut Cx, WidgetActionItem),
    ) {
        if self.animator_handle_event(cx, event).must_redraw() {
            self.redraw(cx);
        }

        self.orchestrate_animations(cx, event);

        self.view.handle_widget_event_with(cx, event, dispatch_action);

        match self.state {
            WonderState::Intro => {
                self.handle_intro_event(cx, event);
            }
            WonderState::Content => {
                self.handle_content_event(cx, event);
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
                        self.view(id!(header)).set_visible(true);
                        self.view(id!(subtitle_group)).set_visible(true);

                        self.reset_intro_dragging(cx);
                        self.animator_play(cx, id!(intro.hide));
                        self.animator_play(cx, id!(sun.show));
                        self.animator_play(cx, id!(title.content));
                        self.animator_play(cx, id!(subtitle_on_content.will_show));
                        self.animator_play(cx, id!(compass.show));
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

    fn handle_content_event(&mut self, cx: &mut Cx, event: &Event) {
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

                    if delta < -60. {
                        self.state = WonderState::Intro;

                        self.reset_intro_dragging(cx);
                        self.animator_play(cx, id!(intro.show));
                        self.animator_play(cx, id!(sun.hide));
                        self.animator_play(cx, id!(title.intro));
                        self.animator_play(cx, id!(subtitle_on_intro.will_show));
                        self.animator_play(cx, id!(compass.hide));
                    } else if delta < 0. {
                        let subtitle_group = self.view(id!(subtitle_group));
                        subtitle_group.apply_over(cx, live!{
                            margin: {top: (-delta)},
                        });
                        subtitle_group.redraw(cx);

                        let title = self.view(id!(title));
                        title.apply_over(cx, live!{
                            margin: {top: (-delta * 2.0)},
                        });
                        title.redraw(cx);
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

        match self.state {
            WonderState::Intro => {
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
            },
            WonderState::Content => {
                let subtitle_group = self.view(id!(subtitle_group));
                subtitle_group.apply_over(cx, live!{
                    margin: {top: 0}
                });
                subtitle_group.redraw(cx);

                let title = self.view(id!(title));
                title.apply_over(cx, live!{
                    margin: {top: 0}
                });
                title.redraw(cx);
            }
        }
    }

    fn orchestrate_animations(
        &mut self,
        cx: &mut Cx,
        event: &Event
    ){
        if let Some(ne) = self.next_frame.is_event(event) {
            if self.animator.is_track_animating(cx, id!(subtitle_on_content)) {
                if self.animator.animator_in_state(cx, id!(subtitle_on_content.will_show)) {
                    // Make sure the subtitle is visible
                    self.animator_play(cx, id!(subtitle_on_intro.reset));

                    self.animator_play(cx, id!(subtitle_on_content.show));
                }
            }
            if self.animator.is_track_animating(cx, id!(subtitle_on_intro)) {
                if self.animator.animator_in_state(cx, id!(subtitle_on_intro.will_show)) {
                    self.animator_play(cx, id!(subtitle_on_intro.show));
                }
            }

            self.next_frame = cx.new_next_frame();
        }
    }
}
