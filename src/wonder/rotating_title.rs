use makepad_widgets::widget::WidgetCache;
use makepad_widgets::*;
use crate::wonder::content_sections::ContentSections;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import makepad_draw::shader::std::*;

    import crate::shared::styles::*;
    import crate::shared::curved_label::*;
    import crate::shared::widgets::*;

    IMG_ICON_HISTORY = dep("crate://self/resources/images/history.png")
    IMG_ICON_GEOGRAPHY = dep("crate://self/resources/images/geography.png")
    IMG_ICON_CONSTRUCTION = dep("crate://self/resources/images/construction.png")

    Title = <CurvedLabel> {
        width: 140
        height: Fit

        margin: { left: -2, top: 16 }

        rotation: 0.0
        total_angle: (PI * 0.8)
        draw_bg: {
            color: #0000
        }
        draw_text: {
            color: #e6945c,
            text_style: {font_size: 8},
        }
    }

    Icon = <View> {
        width: Fit,
        height: Fit,
        image = <CenteredScaledImage> {
            margin: { top: 56 }
            width: 26,
            height: 26,

            draw_bg: { instance image_scale: vec2(0.0, 0.0) }
        }
    }

    RotatingTitle = {{RotatingTitle}} {
        flow: Overlay,
        width: Fill,
        height: Fill,

        align: {x: 0.5, y: 0.0}

        history_title = <Title> {
            text: "FACTS AND HISTORY",
            rotation: 0.0
        }

        construction_title = <Title> {
            text: "CONSTRUCTION",
            width: 110,
            total_angle: (PI * 0.56),
            rotation: (-PI)
        }

        geography_title = <Title> {
            text: "LOCATION INFO",
            width: 122,
            total_angle: (PI * 0.62),
            rotation: (PI)
        }

        history_icon = <Icon> {
            image = { source: (IMG_ICON_HISTORY), draw_bg: { instance image_scale: vec2(1.0, 1.0) }},
        }

        construction_icon = <Icon> {
            image = { source: (IMG_ICON_CONSTRUCTION) },
        }

        geography_icon = <Icon> {
            image = { source: (IMG_ICON_GEOGRAPHY) },
        }

        animator: {
            history = {
                default: show,
                hide = {
                    redraw: true,
                    from: {all: Forward {duration: 0.5}}
                    apply: {
                        history_title = { rotation: 3.14159 }
                        history_icon = {
                            image = { draw_bg: {image_scale: vec2(10.0, 10.0)} }
                        }
                    }
                }
                show = {
                    redraw: true,
                    from: {all: Forward {duration: 0.5}}
                    apply: {
                        history_title = { rotation: 0.0 }
                        history_icon = {
                            image = { draw_bg: {image_scale: vec2(1.0, 1.0)} }
                        }
                    }
                }
            },

            construction = {
                default: hide_left,
                hide_right = {
                    redraw: true,
                    from: {all: Forward {duration: 0.5}}
                    apply: {
                        construction_title = { rotation: 3.14159 }
                        construction_icon = {
                            image = { draw_bg: {image_scale: vec2(10.0, 10.0)} }
                        }
                    }
                }
                hide_left = {
                    redraw: true,
                    from: {all: Forward {duration: 0.5}}
                    apply: {
                        construction_title = { rotation: -3.14159 }
                        construction_icon = {
                            image = { draw_bg: {image_scale: vec2(10.0, 10.0)} }
                        }
                    }
                }
                show = {
                    redraw: true,
                    from: {all: Forward {duration: 0.5}}
                    apply: {
                        construction_title = { rotation: 0.0 }
                        construction_icon = {
                            image = { draw_bg: {image_scale: vec2(1.0, 1.0)} }
                        }
                    }
                }
            },
    
            geography = {
                default: hide,
                hide = {
                    redraw: true,
                    from: {all: Forward {duration: 0.5}}
                    apply: {
                        geography_title = { rotation: -3.14159 }
                        geography_icon = {
                            image = { draw_bg: {image_scale: vec2(10.0, 10.0)} }
                        }
                    }
                }
                show = {
                    redraw: true,
                    from: {all: Forward {duration: 0.5}}
                    apply: {
                        geography_title = { rotation: 0.0 }
                        geography_icon = {
                            image = { draw_bg: {image_scale: vec2(1.0, 1.0)} }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Live, LiveHook, Widget)]
pub struct RotatingTitle {
    #[deref]
    view: View,

    #[rust]
    current_section: ContentSections,

    #[animator]
    animator: Animator,

    #[rust]
    scroll_progress: f64,

}

impl Widget for RotatingTitle {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let animatation_state = self.animator_handle_event(cx, event);
        if animatation_state.must_redraw() {
            self.redraw(cx);
        }
        if !animatation_state.is_animating() {
            self.check_state(cx);
        }

        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl RotatingTitle {
    fn check_state(&mut self, cx: &mut Cx) {
        let mut current_section = ContentSections::History;

        if self.scroll_progress > ContentSections::Construction.starts_at() {
            current_section = ContentSections::Construction;
        }

        if self.scroll_progress > ContentSections::Geography.starts_at() {
            current_section = ContentSections::Geography;
        }

        if current_section != self.current_section {
            match self.current_section {
                ContentSections::History => {
                    self.view(id!(history_icon)).set_visible(false);
                    self.animator_play(cx, id!(history.hide));
                }
                ContentSections::Construction => {
                    self.view(id!(construction_icon)).set_visible(false);
                    if current_section == ContentSections::History {
                        self.animator_play(cx, id!(construction.hide_left));
                    } else {
                        self.animator_play(cx, id!(construction.hide_right));
                    }
                }
                ContentSections::Geography => {
                    self.view(id!(geography_icon)).set_visible(false);
                    self.animator_play(cx, id!(geography.hide));
                }
            }

            match current_section {
                ContentSections::History => {
                    self.view(id!(history_icon)).set_visible(true);
                    let scale = dvec2(10.0, 10.0);
                    self.view(id!(history_icon)).apply_over(
                        cx,
                        live!{
                            image = { draw_bg: {image_scale: (scale)}}
                        }
                    );
                    self.animator_play(cx, id!(history.show));
                }
                ContentSections::Construction => {
                    self.view(id!(construction_icon)).set_visible(true);
                    let scale = dvec2(10.0, 10.0);
                    self.view(id!(construction_icon)).apply_over(
                        cx,
                        live!{
                            image = { draw_bg: {image_scale: (scale)}}
                        }
                    );
                    self.animator_play(cx, id!(construction.show));
                }
                ContentSections::Geography => {
                    self.view(id!(geography_icon)).set_visible(true);
                    let scale = dvec2(10.0, 10.0);
                    self.view(id!(geography_icon)).apply_over(
                        cx,
                        live!{
                            image = { draw_bg: {image_scale: (scale)}}
                        }
                    );
                    self.animator_play(cx, id!(geography.show));
                }
            }

            self.current_section = current_section;
        }
    }
}

impl RotatingTitleRef {
    pub fn set_scroll_progress(&mut self, value: f64) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.scroll_progress = value;
        }
    }
}
