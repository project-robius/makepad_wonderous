use crate::{
    gallery::{
        gallery_image_slider::GalleryImageSliderWidgetRefExt,
        gallery_screen::{GalleryGridAction, GalleryWidgetRefExt},
    },
    shared::custom_radio_button::*,
    wonder::wonder_screen::{WonderScreenAction, WonderState},
};
use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import makepad_draw::shader::std::*;

    import crate::shared::styles::*;
    import crate::wonder::wonder_screen::*;
    import crate::gallery::gallery_screen::*;
    import crate::gallery::gallery_image_slider::*;
    import crate::artifacts::artifacts_screen::*;
    import crate::timeline::timeline_screen::*;
    import crate::timeline::global_timeline::*;
    import crate::shared::custom_radio_button::*;

    ICON_WONDER = dep("crate://self/resources/icons/wonder-button.png")

    ICON_MAIN = dep("crate://self/resources/icons/tab-editorial.png")
    ICON_MAIN_ACTIVE = dep("crate://self/resources/icons/tab-editorial-active.png")
    ICON_GALLERY = dep("crate://self/resources/icons/tab-photos.png")
    ICON_GALLERY_ACTIVE = dep("crate://self/resources/icons/tab-photos-active.png")
    ICON_ARTIFACTS = dep("crate://self/resources/icons/tab-artifacts.png")
    ICON_ARTIFACTS_ACTIVE = dep("crate://self/resources/icons/tab-artifacts-active.png")

    ICON_TIMELINE = dep("crate://self/resources/icons/tab-timeline.png")
    ICON_TIMELINE_ACTIVE = dep("crate://self/resources/icons/tab-timeline-active.png")

    AppTab = <CustomRadioButton> {
        width: Fill,
        height: Fill,
        align: {x: 0.0, y: 0.0}
        draw_radio: {
            radio_type: Tab,
            instance inset: vec4(0.0, 0.0, 0.0, 0.0)
            instance radius: 2.5

            fn pixel(self) -> vec4 {
                return vec4(0.0, 0.0, 0.0, 0.0);
            }
        }
        media: Image
        image: <Image> {
            width: 25,
            height: Fit,
            fit: Smallest
        }
        flow: Down, spacing: 5.0, align: {x: 0.5, y: 0.5}
    }

    App = {{App}} {
        menu_images: [
            (ICON_MAIN),
            (ICON_GALLERY),
            (ICON_ARTIFACTS),
            (ICON_TIMELINE),
        ]

        menu_images_active: [
            (ICON_MAIN_ACTIVE),
            (ICON_GALLERY_ACTIVE),
            (ICON_ARTIFACTS_ACTIVE),
            (ICON_TIMELINE_ACTIVE),
        ]

        ui: <Window> {
            window: {position: vec2(0, 0), inner_size: vec2(375, 813)},
            pass: {clear_color: #2A}

            body = {
                navigation = <StackNavigation> {
                    root_view = {
                        width: Fill,
                        height: Fill,
                        padding: 0, align: {x: 0.0, y: 0.0}, spacing: 0., flow: Down
                        show_bg: false

                        application_pages = <View> {
                            margin: 0.0,
                            padding: 0.0

                            tab1_frame = <WonderScreen> {visible: true}
                            tab2_frame = <GalleryScreen> {visible: false}
                            tab3_frame = <ArtifactsScreen> {visible: false}
                            tab4_frame = <TimelineScreen> {visible: false}
                        }

                        mobile_menu = <RoundedView> {
                            visible: false,
                            width: Fill,
                            height: 60,
                            flow: Right, spacing: 6.0, padding: -5
                            draw_bg: {
                                instance radius: 0.0,
                                color: #fff
                            }
                            mobile_modes = <View> {
                                padding: {left: 10.0, right: 10.0}
                                tab_wonder = <AppTab> {
                                    animator: {selected = {default: on}}
                                    image: {
                                        source: (ICON_WONDER)
                                        width: 50,
                                        draw_bg: {
                                            fn get_color(self) -> vec4 {
                                                let color = self.get_color_scale_pan(self.image_scale, self.image_pan);

                                                if color.a < 0.01 {
                                                    return #678551;
                                                }

                                                return color;
                                            }

                                            fn pixel(self) -> vec4 {
                                                let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                                                let c = self.rect_size * 0.5;
                                                sdf.circle(c.x, c.y, c.x - 2.)
                                                sdf.fill_keep(self.get_color());
                                                sdf.stroke(#xfff8ee, 1);
                                                return sdf.result
                                            }
                                        }
                                    }
                                }
                                tab_main = <AppTab> {
                                    animator: {selected = {default: on}}
                                    image: {
                                        source: (ICON_MAIN_ACTIVE)
                                    }
                                }
                                tab_gallery = <AppTab> {
                                    image: {
                                        source: (ICON_GALLERY)
                                    }
                                }
                                tab_artifacts = <AppTab> {
                                    image: {
                                        source: (ICON_ARTIFACTS)
                                    }
                                }
                                tab_timeline = <AppTab> {
                                    image: {
                                        source: (ICON_TIMELINE)
                                    }
                                }
                            }
                        }
                    }

                    // Add stack navigations here
                    gallery_image_slider_stack_view = <StackNavigationView> {
                        show_bg: true
                        draw_bg: {
                            color: #1f1b18
                        }
                        header = {
                            show_bg: false,
                            content = {
                                title_container = {
                                    title = {
                                    // Hack to make sure the button is visible, we render some text in the same
                                    // color as the background
                                        text: "."
                                        draw_text: {
                                            fn get_color(self) -> vec4 {
                                                return #1f1b18;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        body = {
                            show_bg: false,
                            image_slider = <GalleryImageSlider> {}
                        }
                    }
                    global_timeline_stack_view = <StackNavigationView> {
                        show_bg: true
                        draw_bg: {
                            color: #1f1b18
                        }
                        header = {
                            show_bg: false,
                            content = {
                                title_container = {
                                    margin: {left: 125.}
                                    title = {
                                        margin: 0
                                        text: "GLOBAL TIMELINE"
                                        draw_text: {
                                            text_style: {
                                                font_size: 12.0
                                            }

                                            fn get_color(self) -> vec4 {
                                                return #fff
                                            }
                                        }
                                    }
                                }
                                button_container = {
                                    margin: {left: 10.}
                                }
                            }
                        }
                        body = {
                            show_bg: false,
                            global_timeline = <GlobalTimelineScreen> {}
                        }
                    }
                }
            }
        }
    }
}

app_main!(App);

#[derive(Live, LiveHook)]
pub struct App {
    #[live]
    ui: WidgetRef,
    #[live]
    menu_images: Vec<LiveDependency>,
    #[live]
    menu_images_active: Vec<LiveDependency>,
}

impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        makepad_widgets::live_design(cx);

        // Shared
        crate::shared::styles::live_design(cx);
        crate::shared::widgets::live_design(cx);
        crate::shared::curved_label::live_design(cx);
        crate::shared::custom_radio_button::live_design(cx);
        crate::shared::helpers::live_design(cx);

        // Wonder
        crate::wonder::rotating_title::live_design(cx);
        crate::wonder::content::live_design(cx);
        crate::wonder::wonder_screen::live_design(cx);
        crate::wonder::before_content_header::live_design(cx);
        crate::wonder::content_header::live_design(cx);
        crate::wonder::great_wall_highlight::live_design(cx);
        crate::wonder::great_wall_construction_images::live_design(cx);
        crate::wonder::separator::live_design(cx);

        // Gallery
        crate::gallery::gallery_screen::live_design(cx);
        crate::gallery::gallery_image::live_design(cx);
        crate::gallery::gallery_overlay::live_design(cx);
        crate::gallery::gallery_image_slider::live_design(cx);

        // Artifacts
        crate::artifacts::artifacts_screen::live_design(cx);
        crate::artifacts::artifacts_carrousel::live_design(cx);

        // Timeline
        crate::timeline::timeline_screen::live_design(cx);
        crate::timeline::global_timeline::live_design(cx);
        crate::timeline::timeline_nav::live_design(cx);
    }
}

impl MatchEvent for App {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        self.ui
            .custom_radio_button_set(ids!(
                mobile_modes.tab_main,
                mobile_modes.tab_gallery,
                mobile_modes.tab_artifacts,
                mobile_modes.tab_timeline,
            ))
            .selected_to_visible(
                cx,
                &self.ui,
                &actions,
                ids!(
                    application_pages.tab1_frame,
                    application_pages.tab2_frame,
                    application_pages.tab3_frame,
                    application_pages.tab4_frame,
                ),
            );

        let selected_tab = self
            .ui
            .custom_radio_button_set(ids!(
                mobile_modes.tab_main,
                mobile_modes.tab_gallery,
                mobile_modes.tab_artifacts,
                mobile_modes.tab_timeline,
            ))
            .selected(cx, actions);

        if let Some(tab_index) = selected_tab {
            let tab_ref = self.tab_ref_from_index(tab_index);
            let _ = tab_ref.load_image_dep_by_path(cx, self.menu_images_active[tab_index].as_str());

            for inactive_tab in 0..self.menu_images.len() {
                if inactive_tab != tab_index {
                    let tab_ref = self.tab_ref_from_index(inactive_tab);
                    let _ =
                        tab_ref.load_image_dep_by_path(cx, self.menu_images[inactive_tab].as_str());
                }
            }

            self.ui.redraw(cx);
        }

        let mut navigation = self.ui.stack_navigation(id!(navigation));
        navigation.handle_stack_view_actions(cx, &actions);

        self.handle_mobile_menu_visibility(&actions);
        self.handle_selected_gallery_image(cx, &actions);
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.match_event(cx, event);
        self.ui.handle_event(cx, event, &mut Scope::empty());
        let stack_navigation = self.ui.stack_navigation(id!(navigation));
        let mut gallery_ref = stack_navigation.gallery(id!(root_view.application_pages.tab2_frame));
        gallery_ref.set_image_id(0, cx);
    }
}

impl App {
    fn tab_ref_from_index(&self, tab_index: usize) -> CustomRadioButtonRef {
        match tab_index {
            0 => self.ui.custom_radio_button(id!(mobile_modes.tab_main)),
            1 => self.ui.custom_radio_button(id!(mobile_modes.tab_gallery)),
            2 => self.ui.custom_radio_button(id!(mobile_modes.tab_artifacts)),
            3 => self.ui.custom_radio_button(id!(mobile_modes.tab_timeline)),
            _ => unreachable!(),
        }
    }

    fn handle_mobile_menu_visibility(&mut self, actions: &Actions) {
        // hide menu on first page
        let stack_navigation = self.ui.stack_navigation(id!(navigation));
        let mobile_menu = stack_navigation.view(id!(root_view.mobile_menu));
        for action in actions {
            if let WonderScreenAction::StateChange(state) = action.as_widget_action().cast() {
                match state {
                    WonderState::Cover => mobile_menu.set_visible(false),
                    WonderState::Content | WonderState::Title => mobile_menu.set_visible(true),
                }
            }
        }

        // Make background transperent on gallery
    }

    fn handle_selected_gallery_image(&mut self, cx: &mut Cx, actions: &Actions) {
        let stack_navigation = self.ui.stack_navigation(id!(navigation));
        for action in actions {
            if let GalleryGridAction::Selected(id) = action.as_widget_action().cast() {
                let mut slider_ref = stack_navigation
                    .gallery_image_slider(id!(gallery_image_slider_stack_view.image_slider));
                slider_ref.set_image_id(id, cx);
            }
        }
    }
}
