use crate::wonder::content::*;
use crate::shared::touch_gesture::*;
use makepad_widgets::widget::WidgetCache;
use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;

    import crate::shared::styles::*;
    import crate::shared::widgets::*;
    import crate::wonder::content::*;

    IMG_SUN = dep("crate://self/resources/images/sun.png")
    IMG_CLOUD = dep("crate://self/resources/images/cloud-white.png")
    IMG_GREAT_WALL = dep("crate://self/resources/images/great-wall.png")
    IMG_FG_LEFT_GREAT_WALL = dep("crate://self/resources/images/foreground_left_great_wall.png")
    IMG_FG_RIGHT_GREAT_WALL = dep("crate://self/resources/images/foreground_right_great_wall.png")
    IMG_BACKGROUND_ROLLER = dep("crate://self/resources/images/roller-1-black.png")
    IMG_COMPASS = dep("crate://self/resources/images/compass-icon.png")

    WonderScreen = {{WonderScreen}} {
        flow: Overlay,

        show_bg: true,
        draw_bg: {
            color: #5d2a2c
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

        content = <WonderContent> {
            visible: false,
            width: Fill,
            height: Fit,
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

                <View> {
                    width: Fill,
                    height: Fit,

                    align: { x: 0.5, y: 0.5 }

                    <Label> {
                        draw_text:{
                            text_style: <SUBTITLE_CAPTION>{font_size: 10},
                            color: #fff
                        }
                        text: "700 BCE to 1644 CE"
                    }
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
            great_wall_sun = <Image> {
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
            great_wall = <View> {
                width: (1476 * 0.4),
                height: (1371 * 0.4),

                image = <Image> {
                    abs_pos: vec2(-100, 48),

                    source: (IMG_GREAT_WALL),
                    width: (1476 * 0.4),
                    height: (1371 * 0.4),
                }

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

        title = <FadeView> {
            flow: Down,
            width: Fill,
            height: Fit,

            abs_pos: vec2(0, 600.0),
            align: { x: 0.5, y: 0 }

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
                    from: {all: Snap}
                    apply: {
                        intro = { draw_bg: { instance opacity: 1.0 }}
                        header = { draw_bg: { instance opacity: 0.0 }}
                    }
                }
            },
            great_wall_leaves = {
                default: show,
                show = {
                    redraw: true,
                    from: {all: Forward {duration: 0.3}}
                    apply: {
                        intro = {
                            left_great_wall = { draw_bg: { instance opacity: 1.0 } }
                            right_great_wall = { draw_bg: { instance opacity: 1.0 } }
                        }
                    }
                }
                will_show = {
                    from: {all: Snap}
                    apply: {
                        intro = {
                            left_great_wall = { draw_bg: { instance opacity: 0.0 } }
                            right_great_wall = { draw_bg: { instance opacity: 0.0 } }
                        }
                    }
                }
            },
            great_wall_scale = {
                default: show,
                show = {
                    redraw: true,
                    from: {all: Forward {duration: 0.3}}
                    apply: {
                        intro = {
                            great_wall = { image = {
                                draw_bg: { image_scale: vec2(1.0, 1.0) }
                            }},
                            great_wall_sun = {
                                draw_bg: { image_scale: vec2(1.0, 1.0) }
                            }
                        }
                    }
                }
                will_show = {
                    from: {all: Snap}
                    apply: {
                        intro = {
                            great_wall = { image = {
                                draw_bg: { image_scale: vec2(1.5, 1.5) }
                            }},
                            great_wall_sun = {
                                draw_bg: { image_scale: vec2(1.5, 1.5) }
                            }
                        }
                    }
                }
            },
            great_wall_padding = {
                default: show,
                show = {
                    redraw: true,
                    ease: InQuad
                    from: {all: Forward {duration: 0.3}}
                    apply: {
                        intro = {
                            great_wall = { image = {
                                margin: {top: 0.0, left: 0.0}
                            }},
                            great_wall_sun = {
                                margin: {top: 0.0}
                            }
                        }
                    }
                }
                will_show = {
                    from: {all: Snap}
                    apply: {
                        intro = {
                            great_wall = { image = {
                                margin: {top: -100.0, left: 60.0}
                            }},
                            great_wall_sun = {
                                margin: {top: -100.0}
                            }
                        }
                    }
                }
            },
            content_sun = {
                default: hide,
                show = {
                    redraw: true,
                    ease: OutExp
                    from: {all: Forward {duration: 0.5}}
                    apply: {
                        header = { sun = { abs_pos: vec2(100, 0) }}
                    }
                }
                hide = {
                    redraw: true,
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
                    redraw: true,
                    ease: InExp
                    from: {all: Forward {duration: 0.3}}
                    apply: {
                        title = { abs_pos: vec2(0, 370.0) }
                    }
                }
                intro = {
                    redraw: true,
                    ease: InExp
                    from: {all: Forward {duration: 0.3}}
                    apply: {
                        title = { abs_pos: vec2(0, 600.0) }
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

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum WonderState {
    #[default]
    Cover,
    Title,
    Content,
}

#[derive(Live)]
pub struct WonderScreen {
    #[deref]
    view: View,

    #[rust]
    state: WonderState,
    #[rust]
    previous_state: WonderState,

    #[animator]
    animator: Animator,

    #[rust]
    next_frame: NextFrame,

    #[rust]
    touch_gesture: TouchGesture,
}

impl LiveHook for WonderScreen {
    fn before_live_design(cx: &mut Cx) {
        register_widget!(cx, WonderScreen);
    }

    fn after_apply(&mut self, cx: &mut Cx, from: ApplyFrom, _index: usize, _nodes: &[LiveNode]) {
        if from.is_from_doc() {
            self.state = WonderState::Cover;
            self.touch_gesture = TouchGesture::new();
            self.next_frame = cx.new_next_frame();
        }
    }
}

impl Widget for WonderScreen {
    fn handle_widget_event_with(
        &mut self,
        cx: &mut Cx,
        event: &Event,
        dispatch_action: &mut dyn FnMut(&mut Cx, WidgetActionItem),
    ) {
        self.handle_event_with(cx, event, &mut |cx, action| {
            dispatch_action(cx, action);
        });

        if self.animator_handle_event(cx, event).must_redraw() {
            self.redraw(cx);
        }

        self.orchestrate_animations(cx, event);

        self.view
            .handle_widget_event_with(cx, event, dispatch_action);

        match self.state {
            WonderState::Cover => {
                self.handle_event_in_cover_state(cx, event);
            }
            WonderState::Title => {
                self.handle_event_in_title_state(cx, event);
            }
            WonderState::Content => {
                self.handle_event_in_content_state(cx, event);
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

impl WonderScreen {
    fn handle_event_with(
        &mut self,
        cx: &mut Cx,
        _event: &Event,
        dispatch_action: &mut dyn FnMut(&mut Cx, WidgetActionItem),
    ) {
        let widget_uid = self.widget_uid();
        if self.previous_state != self.state {
            dispatch_action(
                cx,
                WidgetActionItem::new(
                    Box::new(WonderScreenAction::StateChange(self.state)),
                    widget_uid,
                ),
            );
            self.previous_state = self.state;
        }
    }

    fn handle_event_in_cover_state(&mut self, cx: &mut Cx, event: &Event) {
        self.touch_gesture.handle_event(cx, event, self.view.area());
        
        if self.touch_gesture.is_stopped() {
            self.reset_all_positions(cx);
            self.touch_gesture.scroll_offset = 0.0;
            return;
        }
        
        const COVER_STATE_SCROLL_FACTOR: f64 = 0.6;
        let delta = self.touch_gesture.scroll_offset * COVER_STATE_SCROLL_FACTOR;

        const SHOW_TITLE_THRESHOLD: f64 = 60.0;
        if delta > SHOW_TITLE_THRESHOLD {
            self.view(id!(header)).set_visible(true);
            self.view(id!(subtitle_group)).set_visible(true);

            // TODO it is hard to access to set_visible in the "view parent" of the custom widget
            self.wonder_content(id!(content)).apply_over(cx, live!{
                visible: true
            });

            self.state = WonderState::Title;
            self.reset_all_positions(cx);

            self.animator_play(cx, id!(intro.hide));
            self.animator_play(cx, id!(content_sun.show));
            self.animator_play(cx, id!(title.content));
            self.animator_play(cx, id!(subtitle_on_content.will_show));
            self.animator_play(cx, id!(compass.show));

            self.touch_gesture.stop();
            self.touch_gesture.scroll_offset = 0.0;
        } else if delta > 0. {
            // Animate the great wall left and right images

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

    fn handle_event_in_title_state(&mut self, cx: &mut Cx, event: &Event) {
        self.touch_gesture.handle_event(cx, event, self.view.area());

        if self.touch_gesture.is_stopped() {
            // TODO rename this!
            self.reset_all_positions(cx);
            self.touch_gesture.scroll_offset = 0.0;
            return;
        }

        let delta = self.touch_gesture.scroll_offset;

        const SHOW_COVER_THRESHOLD: f64 = -60.0;
        const SHOW_CONTENT_THRESHOLD: f64 = 20.0;

        if delta < SHOW_COVER_THRESHOLD {
            self.state = WonderState::Cover;
            self.reset_all_positions(cx);

            self.animator_play(cx, id!(intro.show));
            self.animator_play(cx, id!(great_wall_scale.will_show));
            self.animator_play(cx, id!(great_wall_padding.will_show));
            self.animator_play(cx, id!(great_wall_leaves.will_show));

            self.animator_play(cx, id!(content_sun.hide));
            self.animator_play(cx, id!(title.intro));
            self.animator_play(cx, id!(subtitle_on_intro.will_show));
            self.animator_play(cx, id!(compass.hide));

            // TODO it is hard to access to set_visible in the "view parent" of the custom widget
            self.wonder_content(id!(content)).apply_over(cx, live!{
                visible: false
            });

            self.touch_gesture.stop();
            self.touch_gesture.scroll_offset = 0.0;
        } else if delta < 0. {
            let subtitle_group = self.view(id!(subtitle_group));
            subtitle_group.apply_over(cx, live!{
                margin: {top: (-delta)},
            });
            subtitle_group.redraw(cx);

            let title = self.view(id!(title));
            title.apply_over(cx, live!{
                margin: {top: (-delta)},
            });
            title.redraw(cx);
        } else if delta > SHOW_CONTENT_THRESHOLD {
            self.state = WonderState::Content;

            // Note this is NOT reseting current dragging state
            self.touch_gesture.reset(0.0, 0.0, MAIN_CONTENT_LENGTH, ScrollMode::Swipe);
        }
    }

    fn handle_event_in_content_state(&mut self, cx: &mut Cx, event: &Event) {
        self.touch_gesture.handle_event(cx, event, self.view.area());
        let delta = self.touch_gesture.scroll_offset;
        let is_dragging = self.touch_gesture.is_dragging();

        let action = self.wonder_content(id!(content)).scroll(cx, delta, is_dragging);
        match action {
            WonderContentAction::Scrolling => {
                self.update_title_position_on_into_content(cx, delta);
            }
            WonderContentAction::Closed => {
                self.state = WonderState::Title;
                self.update_title_position_on_into_content(cx, 0.0);

                self.touch_gesture.reset(0.0, f64::MIN, f64::MAX, ScrollMode::DragAndDrop);
            }
            _ => {}
        }
    }

    fn update_title_position_on_into_content(&mut self, cx: &mut Cx, offset: f64) {
        let opacity = min(1.0, 1.0 - offset / 570.0);

        let subtitle_group = self.view(id!(subtitle_group));
        subtitle_group.apply_over(
            cx,
            live! {
                margin: {top: (-offset * 0.6)},
                draw_bg: { opacity: (opacity) }
            },
        );
        subtitle_group.redraw(cx);

        let title = self.view(id!(title));
        title.apply_over(
            cx,
            live! {
                margin: {top: (-offset * 0.6)},
                draw_bg: { opacity: (opacity) }
            },
        );
        title.redraw(cx);

        let header = self.view(id!(header));
        header.apply_over(
            cx,
            live! {
                draw_bg: { opacity: (opacity) }
            },
        );
        header.redraw(cx);
    }

    fn reset_all_positions(&mut self, cx: &mut Cx) {
        match self.state {
            WonderState::Cover => {
                let left_image = self.view(id!(left_great_wall));
                left_image.apply_over(
                    cx,
                    live! {
                        margin: {top: 0, left: 0},
                        width: (1386. * 0.35),
                        height: (1764. * 0.35)
                    },
                );
                left_image.redraw(cx);

                let right_image = self.view(id!(right_great_wall));
                right_image.apply_over(
                    cx,
                    live! {
                        margin: {top: 0, left: 0},
                        width: (1386. * 0.45),
                        height: (1764. * 0.45)
                    },
                );
                right_image.redraw(cx);
            }
            WonderState::Title => {
                let subtitle_group = self.view(id!(subtitle_group));
                subtitle_group.apply_over(
                    cx,
                    live! {
                        margin: {top: 0}
                    },
                );
                subtitle_group.redraw(cx);

                let title = self.view(id!(title));
                title.apply_over(
                    cx,
                    live! {
                        margin: {top: 0}
                    },
                );
                title.redraw(cx);

                let content = self.wonder_content(id!(content));
                content.redraw(cx);
            }
            WonderState::Content => {}
        }
    }

    fn orchestrate_animations(&mut self, cx: &mut Cx, event: &Event) {
        if let Some(_ne) = self.next_frame.is_event(event) {
            if self
                .animator
                .is_track_animating(cx, id!(subtitle_on_content))
            {
                if self
                    .animator
                    .animator_in_state(cx, id!(subtitle_on_content.will_show))
                {
                    // Make sure the subtitle is visible
                    self.animator_play(cx, id!(subtitle_on_intro.reset));

                    self.animator_play(cx, id!(subtitle_on_content.show));
                }
            }
            if self.animator.is_track_animating(cx, id!(subtitle_on_intro)) {
                if self
                    .animator
                    .animator_in_state(cx, id!(subtitle_on_intro.will_show))
                {
                    self.animator_play(cx, id!(subtitle_on_intro.show));
                }
            }
            if self.animator.is_track_animating(cx, id!(great_wall_scale)) {
                if self
                    .animator
                    .animator_in_state(cx, id!(great_wall_scale.will_show))
                {
                    self.animator_play(cx, id!(great_wall_scale.show));
                }
            }
            if self
                .animator
                .is_track_animating(cx, id!(great_wall_padding))
            {
                if self
                    .animator
                    .animator_in_state(cx, id!(great_wall_padding.will_show))
                {
                    self.animator_play(cx, id!(great_wall_padding.show));
                }
            }
            if self.animator.is_track_animating(cx, id!(great_wall_leaves)) {
                if self
                    .animator
                    .animator_in_state(cx, id!(great_wall_leaves.will_show))
                {
                    self.animator_play(cx, id!(great_wall_leaves.show));
                }
            }

            self.next_frame = cx.new_next_frame();
        }
    }
}

#[derive(Debug, Clone, WidgetAction)]
pub enum WonderScreenAction {
    StateChange(WonderState),
    None,
}
