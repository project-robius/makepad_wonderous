use makepad_widgets::*;

#[derive(Clone, Copy, Debug)]
struct ScrollSample{
    abs: f64,
    time: f64,
}

#[derive(Default, Debug)]
enum ScrollState {
    #[default]
    Stopped,
    Drag{samples:Vec<ScrollSample>},
    Flick {delta: f64, next_frame: NextFrame},
    Pulldown {next_frame: NextFrame},
}

#[derive(Default)]
pub struct SwipeGesture {
    flick_scroll_minimum: f64,
    flick_scroll_maximum: f64,
    flick_scroll_scaling: f64,
    flick_scroll_decay: f64,
    scroll_state: ScrollState,
    
    min_scroll_offset: f64,
    max_scroll_offset: f64,
    pub scroll_offset: f64,

    // TODO resolve this properly
    pub last_finger_move_event: Option<FingerMoveEvent>,
}

impl SwipeGesture {
    pub fn new() -> Self {
        Self {
            flick_scroll_minimum: 0.2,
            flick_scroll_maximum: 80.0,
            flick_scroll_scaling: 0.005,
            flick_scroll_decay: 0.98,
            scroll_state: ScrollState::Stopped,

            scroll_offset: 0.0,
            min_scroll_offset: 0.0,
            max_scroll_offset: f64::MAX,
            last_finger_move_event: None,
        }
    }

    pub fn reset(&mut self, initial_offset: f64, min_offset: f64, max_offset: f64) {
        self.min_scroll_offset = min_offset;
        self.max_scroll_offset = max_offset;
        self.scroll_offset = initial_offset.clamp(self.min_scroll_offset, self.max_scroll_offset);
    }

    pub fn handle_event(&mut self, cx: &mut Cx, event: &Event, area: Area) {
        dbg!(&self.scroll_state);
        //let uid = self.widget_uid();

        match &mut self.scroll_state {
            ScrollState::Flick {delta, next_frame} => {
                if let Some(_) = next_frame.is_event(event) {
                    *delta = *delta * self.flick_scroll_decay;
                    if delta.abs()>self.flick_scroll_minimum {
                        *next_frame = cx.new_next_frame();
                        let delta = *delta;

                        // TODO return or emit action/event to inform delta to parent
                        let new_offset = self.scroll_offset - delta;
                        self.scroll_offset = new_offset.clamp(self.min_scroll_offset, self.max_scroll_offset);

                        //self.delta_top_scroll(cx, delta, true);
                        //dispatch_action(cx, PortalListAction::Scroll.into_action(uid));
                        //self.area.redraw(cx);
                    } else {
                        self.scroll_state = ScrollState::Stopped;
                    }
                }
            }
            // ScrollState::Pulldown {next_frame} => {
            //     if let Some(_) = next_frame.is_event(event) {
            //         // we have to bounce back
            //         if /*self.first_id == self.range_start && */self.scroll_offset > 0.0 {
                        
            //             // TODO return or emit action/event to inform delta to parent

            //             self.scroll_offset *= 0.9;
            //             if self.scroll_offset < 1.0 {
            //                 self.scroll_offset = 0.0;
            //             }
            //             else {
            //                 *next_frame = cx.new_next_frame();
            //                 //dispatch_action(cx, PortalListAction::Scroll.into_action(uid));
            //             }
            //             //self.area.redraw(cx);
            //         }
            //         else {
            //             self.scroll_state = ScrollState::Stopped
            //         }
            //     }
            // }
            _=>()
        }
        // let vi = self.vec_index;
        // let is_scroll = if let Event::Scroll(_) = event {true} else {false};
        // if self.scroll_bar.is_area_captured(cx){
        //     self.scroll_state = ScrollState::Stopped;
        // }
        //if /*!self.scroll_bar.is_area_captured(cx) ||*/ is_scroll{ 
        match event.hits_with_capture_overload(cx, area, true/*self.capture_overload*/) {
            // Hit::FingerScroll(e) => {
            //     if self.tail_range {
            //         self.tail_range = false;
            //     }
            //     self.detect_tail_in_draw = true;
            //     self.scroll_state = ScrollState::Stopped;
            //     self.delta_top_scroll(cx, -e.scroll.index(vi), true);
            //     dispatch_action(cx, PortalListAction::Scroll.into_action(uid));
            //     self.area.redraw(cx);
            // },
            
            Hit::KeyDown(ke) => match ke.key_code {
                KeyCode::Home => {
                    // self.first_id = 0;
                    self.scroll_offset = 0.0;
                    // self.tail_range = false;
                    // self.update_scroll_bar(cx);
                    // self.area.redraw(cx);
                },
                KeyCode::End => {
                    //self.first_id = self.range_end.max(1) - 1;
                    self.scroll_offset = self.max_scroll_offset;
                    // if self.auto_tail {
                    //     self.tail_range = true;
                    // }
                    // self.update_scroll_bar(cx);
                    // self.area.redraw(cx);
                },
                // KeyCode::PageUp => {
                //     self.first_id = self.first_id.max(self.view_window) - self.view_window;
                //     self.scroll_offset = 0.0;
                //     self.tail_range = false;
                //     self.update_scroll_bar(cx);
                //     self.area.redraw(cx);
                // },
                // KeyCode::PageDown => {
                //     self.first_id += self.view_window;
                //     self.scroll_offset = 0.0;
                //     if self.first_id >= self.range_end.max(1) {
                //         self.first_id = self.range_end.max(1) - 1;
                //     }
                //     self.detect_tail_in_draw = true;
                //     self.update_scroll_bar(cx);
                //     self.area.redraw(cx);
                // },
                // KeyCode::ArrowDown => {
                //     self.first_id += 1;
                //     if self.first_id >= self.range_end.max(1) {
                //         self.first_id = self.range_end.max(1) - 1;
                //     }
                //     self.detect_tail_in_draw = true;
                //     self.scroll_offset = 0.0;
                //     self.update_scroll_bar(cx);
                //     self.area.redraw(cx);
                // },
                // KeyCode::ArrowUp => {
                //     if self.first_id > 0 {
                //         self.first_id -= 1;
                //         if self.first_id < self.range_start {
                //             self.first_id = self.range_start;
                //         }
                //         self.scroll_offset = 0.0;
                //         self.area.redraw(cx);
                //         self.tail_range = false;
                //         self.update_scroll_bar(cx);
                //     }
                // },
                _ => ()
            }
            Hit::FingerDown(e) => {
                // if self.grab_key_focus {
                //     cx.set_key_focus(self.area);
                // }
                // if self.tail_range {
                //     self.tail_range = false;
                // }
                //if self.drag_scrolling{
                    dbg!("into dragging in motion");
                    self.scroll_state = ScrollState::Drag {
                        //samples: vec![ScrollSample{abs:e.abs.index(vi),time:e.time}]
                        samples: vec![ScrollSample{abs: e.abs.y, time: e.time}]
                    };
                //}
            }
            Hit::FingerMove(e) => {
                cx.set_cursor(MouseCursor::Default);
                match &mut self.scroll_state {
                    ScrollState::Drag {samples}=>{
                        //let new_abs = e.abs.index(vi);
                        let new_abs = e.abs.y;
                        let old_sample = *samples.last().unwrap();
                        samples.push(ScrollSample{abs: new_abs, time: e.time});
                        if samples.len() > 4 {
                            samples.remove(0);
                        }
                        let new_offset = self.scroll_offset + old_sample.abs - new_abs;
                        self.scroll_offset = new_offset.clamp(self.min_scroll_offset, self.max_scroll_offset);

                        //self.scroll_offset += new_abs - old_sample.abs;
                        //self.area.redraw(cx);

                        self.last_finger_move_event = Some(e);
                    }
                    _=>()
                }
            }
            Hit::FingerUp(e) => {
                match &mut self.scroll_state {
                    ScrollState::Drag {samples}=>{
                        // alright so we need to see if in the last couple of samples
                        // we have a certain distance per time
                        let mut last = None;
                        let mut scaled_delta = 0.0;
                        let mut total_delta = 0.0;
                        for sample in samples.iter().rev(){
                            if last.is_none(){
                                last = Some(sample);
                            }
                            else{
                                total_delta += last.unwrap().abs - sample.abs;
                                scaled_delta += (last.unwrap().abs - sample.abs)/ (last.unwrap().time - sample.time)
                            }
                        }
                        scaled_delta *= self.flick_scroll_scaling;
                        // if /*self.first_id == self.range_start && */self.scroll_offset > 0.0 {
                        //     self.scroll_state = ScrollState::Pulldown {next_frame: cx.new_next_frame()};
                        // }
                        // else
                        if total_delta.abs() > 10.0 && scaled_delta.abs() > self.flick_scroll_minimum{
                            self.scroll_state = ScrollState::Flick {
                                delta: scaled_delta.min(self.flick_scroll_maximum).max(-self.flick_scroll_maximum),
                                next_frame: cx.new_next_frame()
                            };
                        }
                        else{
                            self.scroll_state = ScrollState::Stopped;
                            self.last_finger_move_event = None;
                        }
                    }
                    _=>()
                }
            }
            Hit::KeyFocus(_) => {
            }
            Hit::KeyFocusLost(_) => {
            }
            _ => ()
        }
    }
}