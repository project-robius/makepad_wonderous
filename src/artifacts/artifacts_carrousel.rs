use makepad_widgets::widget::WidgetCache;
use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import makepad_draw::shader::std::*;

    import crate::shared::styles::*;
    import crate::shared::widgets::*;

    CarrouselItem = <RoundedView> {
        width: 160,
        height: 280,
        padding: 4.0,

        flow: Overlay

        draw_bg: {
            instance radius: 40.0
            instance border_width: 1.0
            instance border_color: #fff
        }

        image_wrapper = <CachedRoundedView> {
            draw_bg: {
                instance radius: 40.0
            }

            image = <Image> {
                fit: Vertical,
                width: Fill,
                height: Fill,

                draw_bg: {
                    image_pan: vec2(0.5, 0.5)
                }
            }
        }
    }

    IMG_CONTENT = dep("crate://self/resources/images/artifacts/great-wall-1.jpg")

    ArtifactsCarrousel = {{ArtifactsCarrousel}} {
        flow: Overlay,
        width: Fill,
        height: Fill,

        items: [
            dep("crate://self/resources/images/artifacts/great-wall-1.jpg"),
            dep("crate://self/resources/images/artifacts/great-wall-2.jpg"),
            dep("crate://self/resources/images/artifacts/great-wall-3.jpg"),
            dep("crate://self/resources/images/artifacts/great-wall-4.jpg"),
            dep("crate://self/resources/images/artifacts/great-wall-5.jpg"),
        ]

        item_pan: [
            vec2(0.15, 0.0),
            vec2(0.15, 0.0),
            vec2(0.28, 0.0),
            vec2(0.29, 0.0),
            vec2(0.12, 0.0)
        ]

        background = <BlurStage>{
            width: Fill,
            height: Fill,
            draw_bg:{blury: 0.0, blurx: 10.0, blursize: 0.1}
            step3 = <BlurStage>{
                width: Fill,
                height: Fill,
                draw_bg:{blury: 10.0, blurx: 0.0, blursize: 0.1}
                step2 = <BlurStage>{
                    width: Fill,
                    height: Fill,
                    draw_bg:{blury: 7.07, blurx: 7.07, blursize: 0.1}
                    step1 = <BlurStage>{
                        width: Fill,
                        height: Fill,
                        draw_bg:{blury: -7.07, blurx: 7.07, blursize: 0.1}

                        image = <Image> {
                            width: Fill,
                            height: Fill,
                            fit: Biggest,

                            source: (IMG_CONTENT)

                            draw_bg: {
                                image_scale: 15.0
                                image_pan: vec2(0.2, 0.2)
                            }
                        }
                    }
                }
            }  
        }

        <RoundedView> {
            margin: { top: 350, bottom: -200, left: -200, right: -200 }

            width: Fill,
            height: Fill,
            draw_bg: {
                color: #fffd
                radius: 160.
            }
        }

        body = <View> {
            width: Fill,
            height: Fit,

            flow: Down,
            spacing: 10.0,
            padding: { top: 125.0 }
            align: {x: 0.5, y: 0.0},
            
            container = <View> {
                flow: Overlay,
                width: Fill,
                height: 360,

                main_item = <CarrouselItem> {
                    margin: { left: 95.0 }
                }

                previous_item = <CarrouselItem> {
                    margin: { left: -90.0, top: 170.0 }
                    width: 160,
                    height: 160,

                    draw_bg: {
                        radius: 36.
                    }

                    image_wrapper = {
                        draw_bg: {
                            radius: 36.
                        }

                        image = {
                            fit: Biggest,
                        }
                    }
                }

                next_item = <CarrouselItem> {
                    margin: { left: 280.0, top: 170.0 }
                    width: 160,
                    height: 160,

                    draw_bg: {
                        radius: 36.
                    }

                    image_wrapper = {
                        draw_bg: {
                            radius: 36.
                        }

                        image = {
                            fit: Biggest,
                        }
                    }
                }

                aux_item = <CarrouselItem> {
                    visible: false

                    draw_bg: {
                        radius: 36.
                    }

                    image_wrapper = {
                        draw_bg: {
                            radius: 36.
                        }

                        image = {
                            fit: Biggest,
                        }
                    }
                }
            }

            <Label> {
                draw_text:{
                    text_style: <INTRO_SUBTITLE>{font_size: 20},
                    color: #333
                }
                text: "Cape"
            }

            <Label> {
                draw_text:{
                    text_style: <INTRO_SUBTITLE>{font_size: 10},
                    color: #333
                }
                text: "second half 16th century"
            }
        }

        animator: {
            transition = {
                default: init
                init = {
                    from: {all: Snap}
                    apply: {
                        body = { container = {
                            main_item = {
                                margin: {left: 95.0, top: 0.0}
                                width: 160,
                                height: 280,
                            }
                            next_item = {
                                margin: {left: 280.0, top: 170.0}
                                width: 160,
                                height: 160,
                            }
                            previous_item = {
                                margin: {left: -90.0, top: 170.0}
                                width: 160,
                                height: 160,
                            }
                            aux_item = {
                                margin: {left: -295.0, top: 170.0}
                                width: 160,
                                height: 160,
                            }
                        }
                    }}
                }
                before_next = {
                    from: {all: Snap}
                    redraw: true
                    apply: {
                        body = { container = {
                            main_item = {
                                margin: {left: 280.0, top: 170.0}
                                width: 160,
                                height: 160,
                            }
                            next_item = {
                                margin: {left: 485.0, top: 170.0}
                                width: 160,
                                height: 160,
                            }
                            previous_item = {
                                margin: {left: 95.0, top: 0.0}
                                width: 160,
                                height: 280,
                            }
                            aux_item = {
                                margin: {left: -90.0, top: 170.0}
                                width: 160,
                                height: 160,
                            }
                        }
                    }}
                }
                go_next = {
                    from: {all: Forward {duration: 0.3}}
                    redraw: true
                    apply: {
                        body = { container = {
                            main_item = {
                                margin: {left: 95.0, top: 0.0}
                                width: 160,
                                height: 280,
                            }
                            next_item = {
                                margin: {left: 280.0, top: 170.0}
                                width: 160,
                                height: 160,
                            }
                            previous_item = {
                                margin: {left: -90.0, top: 170.0}
                                width: 160,
                                height: 160,
                            }
                            aux_item = {
                                margin: {left: -295.0, top: 170.0}
                                width: 160,
                                height: 160,
                            }
                        }
                    }}
                }
                before_previous = {
                    from: {all: Snap}
                    redraw: true
                    apply: {
                        body = { container = {
                            main_item = {
                                margin: {left: -90.0, top: 170.0}
                                width: 160,
                                height: 160,
                            }
                            next_item = {
                                margin: {left: 95.0, top: 0.0}
                                width: 160,
                                height: 280,
                            }
                            previous_item = {
                                margin: {left: -295.0, top: 170.0}
                                width: 160,
                                height: 160,
                            }
                            aux_item = {
                                margin: {left: 280.0, top: 170.0}
                                width: 160,
                                height: 160,
                            }
                        }
                    }}
                }
                go_previous = {
                    from: {all: Forward {duration: 0.3}}
                    redraw: true
                    apply: {
                        body = { container = {
                            main_item = {
                                margin: {left: 95.0, top: 0.0}
                                width: 160,
                                height: 280,
                            }
                            next_item = {
                                margin: {left: 280.0, top: 170.0}
                                width: 160,
                                height: 160,
                            }
                            previous_item = {
                                margin: {left: -90.0, top: 170.0}
                                width: 160,
                                height: 160,
                            }
                            aux_item = {
                                margin: {left: 485.0, top: 170.0}
                                width: 160,
                                height: 160,
                            }
                        }
                    }}
                }
            }
        }
    }
}

#[derive(Live, Widget)]
pub struct ArtifactsCarrousel {
    #[deref]
    view: View,

    #[live]
    items: Vec<LiveDependency>,

    #[live]
    item_pan: Vec<Vec2>,

    #[rust(0)]
    current_index: i8,

    #[rust(true)]
    ready_to_swipe: bool,

    #[animator]
    animator: Animator,
}

impl LiveHook for ArtifactsCarrousel {
    fn after_apply_from(&mut self, cx: &mut Cx, apply: &mut Apply) {
        if apply.from.is_from_doc() {
            self.preload_images(cx);
            self.update_images(cx, true);
        }
    }
}

impl Widget for ArtifactsCarrousel {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
        if self.animator_handle_event(cx, event).is_animating() {
            self.redraw(cx);
        }
        if self.animator.animator_in_state(cx, id!(transition.before_next)) {
            self.animator_play(cx, id!(transition.go_next));
            let carrousel_item = self.view.view(id!(container.aux_item));
            carrousel_item.set_visible(true);

        } else if self.animator.animator_in_state(cx, id!(transition.before_previous)) {
            self.animator_play(cx, id!(transition.go_previous));
            let carrousel_item = self.view.view(id!(container.aux_item));
            carrousel_item.set_visible(true);
        }

        match event.hits(cx, self.view.area()) {
            Hit::FingerMove(fe) => {
                if !self.ready_to_swipe {
                    return;
                }

                let swipe_vector = fe.abs - fe.abs_start;

                // only trigger swipe if it is larger than some pixels
                let swipe_trigger_value = 40.;

                if swipe_vector.x.abs() > swipe_trigger_value {
                    if swipe_vector.x > 0. {
                        self.current_index = (self.current_index - 1).rem_euclid(self.items.len() as i8);
                        self.animator_play(cx, id!(transition.before_previous));
                    } else {
                        self.current_index = (self.current_index + 1).rem_euclid(self.items.len() as i8);
                        self.animator_play(cx, id!(transition.before_next));
                    };

                    self.update_images(cx, swipe_vector.x < 0.);
                    self.ready_to_swipe = false;
                }
            }
            Hit::FingerUp(_fe) => self.ready_to_swipe = true,
            _ => {}
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        self.view.draw_walk(cx, scope, walk)
    }
}

impl ArtifactsCarrousel {
    fn update_images(&mut self, cx: &mut Cx, move_next: bool) {
        let index = self.current_index as usize;
        self.set_image(cx, index, id!(container.main_item.image_wrapper.image));

        let background_image = self.view.image(id!(background.step3.step2.step1.image));
        let dep_path = self.items[index].as_str();
        let _ = background_image.load_image_dep_by_path(cx, dep_path);

        let previous_index = (index as i8 - 1).rem_euclid(self.items.len() as i8) as usize;
        self.set_image(cx, previous_index, id!(container.previous_item.image_wrapper.image));

        let next_index = (index as i8 + 1).rem_euclid(self.items.len() as i8) as usize;
        self.set_image(cx, next_index, id!(container.next_item.image_wrapper.image));

        let aux_index;
        if move_next {
            aux_index = (index as i8 - 2).rem_euclid(self.items.len() as i8) as usize;
        } else {
            aux_index = (index as i8 + 2).rem_euclid(self.items.len() as i8) as usize;
        }
        self.set_image(cx, aux_index, id!(container.aux_item.image_wrapper.image));

        let carrousel_item = self.view.view(id!(container.aux_item));
        carrousel_item.set_visible(false);

        self.view.redraw(cx);
    }

    fn set_image(&mut self, cx: &mut Cx, index: usize, live_id: &[LiveId]) {
        let dep_path = self.items[index].as_str();
        let image = self.view.image(live_id);
        let _ = image.load_image_dep_by_path(cx, dep_path);
        image.apply_over(cx, live!{draw_bg: {image_pan: (self.item_pan[index])}});
    }

    // This is not meant to be displayed at all, it is just to force images to load in advanced
    // Otherwise, the animation could be choppy
    fn preload_images(&mut self, cx: &mut Cx) {
        for index in 0..self.items.len() {
            self.set_image(cx, index, id!(container.main_item.image_wrapper.image));
        }
    }
}