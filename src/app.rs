use std::collections::HashMap;

use crate::{
    gallery::{
        gallery_image_slider::{GalleryImageSliderWidgetRefExt, GallerySliderAction},
        gallery_screen::{GalleryGridAction, GalleryWidgetRefExt},
    },
    shared::{stack_navigation::StackNavigationWidgetRefExt, stack_view_action::StackViewAction},
    wonder::wonder_screen::{WonderScreenAction, WonderState},
};
use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;

    import crate::shared::styles::*;
    import crate::wonder::wonder_screen::*;
    import crate::gallery::gallery_screen::*;
    import crate::gallery::gallery_image_slider::*;
    import crate::artifacts::artifacts_screen::*;
    import crate::timeline::timeline_screen::*;
    import crate::shared::stack_navigation::*;

    ICON_WONDER = dep("crate://self/resources/icons/test.svg")
    ICON_GALLERY = dep("crate://self/resources/icons/test.svg")
    ICON_ARTIFACTS = dep("crate://self/resources/icons/test.svg")
    ICON_TIMELINE = dep("crate://self/resources/icons/test.svg")

    AppTab = <RadioButton> {
        width: Fit,
        height: Fill,
        align: {x: 0.0, y: 0.0}
        draw_radio: {
            radio_type: Tab,
            color_active: #fff,
            color_inactive: #fff,
        }
        draw_text: {
            color_selected: #0b0,
            color_unselected: #000,
            color_unselected_hover: #111,
        }
    }

    App = {{App}} {
        ui: <Window> {
            window: {position: vec2(0, 0), inner_size: vec2(375, 813)},
            pass: {clear_color: #2A}

            body = {
                navigation = <StackNavigation> {
                    root_view = {
                        width: Fill,
                        height: Fill,
                        padding: 0, align: {x: 0.0, y: 0.0}, spacing: 0., flow: Down

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
                                tab1 = <AppTab> {
                                    animator: {selected = {default: on}}
                                    draw_icon: {
                                        svg_file: (ICON_WONDER),
                                        fn get_color(self) -> vec4 {
                                            return mix(
                                                #000,
                                                #e6945c,
                                                self.selected
                                            )
                                        }
                                    }
                                    width: Fill,
                                    icon_walk: {width: 20, height: 20}
                                    flow: Down, spacing: 5.0, align: {x: 0.5, y: 0.5}
                                }
                                tab2 = <AppTab> {
                                    draw_icon: {
                                        svg_file: (ICON_GALLERY),
                                        fn get_color(self) -> vec4 {
                                            return mix(
                                                #000,
                                                #e6945c,
                                                self.selected
                                            )
                                        }
                                    }
                                    width: Fill
                                    icon_walk: {width: 20, height: 20}
                                    flow: Down, spacing: 5.0, align: {x: 0.5, y: 0.5}
                                }
                                tab3 = <AppTab> {
                                    draw_icon: {
                                        svg_file: (ICON_ARTIFACTS),
                                        fn get_color(self) -> vec4 {
                                            return mix(
                                                #000,
                                                #e6945c,
                                                self.selected
                                            )
                                        }
                                    }
                                    width: Fill
                                    icon_walk: {width: 20, height: 20}
                                    flow: Down, spacing: 5.0, align: {x: 0.5, y: 0.5}
                                }
                                tab4 = <AppTab> {
                                    draw_icon: {
                                        svg_file: (ICON_TIMELINE),
                                        fn get_color(self) -> vec4 {
                                            return mix(
                                                #000,
                                                #e6945c,
                                                self.selected
                                            )
                                        }
                                    }
                                    width: Fill
                                    icon_walk: {width: 20, height: 20}
                                    flow: Down, spacing: 5.0, align: {x: 0.5, y: 0.5}
                                }
                            }
                        }
                    }

                    // Add stack navigations here
                    gallery_image_slider_stack_view = <StackNavigationView> {
                        image_slider = <GalleryImageSlider> {}
                    }
                }
            }
        }
    }
}

app_main!(App);

#[derive(Live)]
pub struct App {
    #[live]
    ui: WidgetRef,
    #[rust]
    navigation_destinations: HashMap<StackViewAction, LiveId>,
}

impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        makepad_widgets::live_design(cx);

        // Shared
        crate::shared::styles::live_design(cx);
        crate::shared::widgets::live_design(cx);
        crate::shared::stack_navigation::live_design(cx);
        crate::shared::curved_label::live_design(cx);
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

        // Timeline
        crate::timeline::timeline_screen::live_design(cx);
    }
}

impl LiveHook for App {
    fn after_new_from_doc(&mut self, _cx: &mut Cx) {
        self.init_navigation_destinations();
    }
}

impl MatchEvent for App {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        self.ui
            .radio_button_set(ids!(
                mobile_modes.tab1,
                mobile_modes.tab2,
                mobile_modes.tab3,
                mobile_modes.tab4,
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

        let mut navigation = self.ui.stack_navigation(id!(navigation));
        navigation.handle_stack_view_actions(cx, &actions, &self.navigation_destinations);

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
    fn init_navigation_destinations(&mut self) {
        self.navigation_destinations = HashMap::new();
        self.navigation_destinations.insert(
            StackViewAction::ShowGalleryImageSlider,
            live_id!(gallery_image_slider_stack_view),
        );
        // Add stack view actions here
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
            if let GallerySliderAction::Selected(id) = action.as_widget_action().cast() {
                let mut gallery_ref =
                    stack_navigation.gallery(id!(root_view.application_pages.tab2_frame));
                gallery_ref.set_image_id(id, cx);
            }
            if let GalleryGridAction::Selected(id) = action.as_widget_action().cast() {
                let mut slider_ref = stack_navigation
                    .gallery_image_slider(id!(gallery_image_slider_stack_view.image_slider));
                slider_ref.set_image_id(id, cx);
            }
        }
    }
}
