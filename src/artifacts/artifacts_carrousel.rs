use makepad_widgets::widget::WidgetCache;
use makepad_widgets::*;

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import makepad_draw::shader::std::*;

    import crate::shared::styles::*;
    import crate::shared::widgets::*;

    CarrouselItem = <RoundedView> {
        width: 200,
        height: 280,
        padding: 4.0,

        draw_bg: {
            radius: 46.
            border_width: 1.0
            border_color: #fff
        }

        image = <Image> {
            width: Fill,
            height: Fill,

            draw_bg: {
                instance radius: 46.
                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    sdf.box(
                        1,
                        1,
                        self.rect_size.x - 2.0,
                        self.rect_size.y - 2.0,
                        max(1.0, self.radius)
                    )
                    sdf.fill_keep(self.get_color())
                    return sdf.result
                }
            }
        }
    }

    IMG_CONTENT = dep("crate://self/resources/images/great-wall-content-1.jpg")

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

        background = <FadeView> {
            width: Fill,
            height: Fill,

            draw_bg: {
                opacity: 0.3,
            }

            image = <Image> {
                width: Fill,
                height: Fill,

                draw_bg: {
                    image_scale: 0.7
                    image_pan: vec2(0.15, 0.2)
                }
            }
        }

        <RoundedView> {
            margin: { top: 300, bottom: -200, left: -200, right: -200 }

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
            padding: { top: 100.0 }
            align: {x: 0.5, y: 0.0},
            
            container = <View> {
                flow: Overlay,
                width: Fill,
                height: 360,

                main_item = <CarrouselItem> {
                    margin: { left: 75.0 }
                }

                previous_item = <CarrouselItem> {
                    margin: { left: -90.0, top: 170.0 }
                    width: 160,
                    height: 160,

                    draw_bg: {
                        radius: 36.
                    }

                    image = {
                        draw_bg: {
                            radius: 36.
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

                    image = {
                        draw_bg: {
                            radius: 36.
                        }
                    }
                }

                aux_item = <CarrouselItem> {
                    visible: false
                    width: 160,
                    height: 160,

                    draw_bg: {
                        radius: 36.
                    }

                    image = {
                        draw_bg: {
                            radius: 36.
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
                                margin: {left: 75.0, top: 0.0}
                                width: 200,
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
                                margin: {left: 75.0, top: 0.0}
                                width: 200,
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
                                margin: {left: 75.0, top: 0.0}
                                width: 200,
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
                                margin: {left: 75.0, top: 0.0}
                                width: 200,
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
                                margin: {left: 75.0, top: 0.0}
                                width: 200,
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

        let mut dep_path = self.items[index].as_str();
        let mut image = self.view.image(id!(container.main_item.image));
        let _ = image.load_image_dep_by_path(cx, dep_path);

        image = self.view.image(id!(background.image));
        let _ = image.load_image_dep_by_path(cx, dep_path);

        let previous_index = (index as i8 - 1).rem_euclid(self.items.len() as i8) as usize;
        dep_path = self.items[previous_index].as_str();
        image = self.view.image(id!(container.previous_item.image));
        let _ = image.load_image_dep_by_path(cx, dep_path);

        let next_index = (index as i8 + 1).rem_euclid(self.items.len() as i8) as usize;
        dep_path = self.items[next_index].as_str();
        image = self.view.image(id!(container.next_item.image));
        let _ = image.load_image_dep_by_path(cx, dep_path);

        if move_next {
            let aux_index = (index as i8 - 2).rem_euclid(self.items.len() as i8) as usize;
            dep_path = self.items[aux_index].as_str();
        } else {
            let aux_index = (index as i8 + 2).rem_euclid(self.items.len() as i8) as usize;
            dep_path = self.items[aux_index].as_str();
        }
        image = self.view.image(id!(container.aux_item.image));
        let _ = image.load_image_dep_by_path(cx, dep_path);

        let carrousel_item = self.view.view(id!(container.aux_item));
        carrousel_item.set_visible(false);

        self.view.redraw(cx);
    }
}