use makepad_widgets::*;

#[derive(Clone, Copy, Debug)]
struct ScrollSample{
    abs: f64,
    time: f64,
}

#[derive(Default, Debug)]
pub enum ScrollMode {
    #[default]
    DragAndDrop,
    Swipe,
}

#[derive(Default, Debug)]
enum ScrollState {
    #[default]
    Stopped,
    Drag{samples:Vec<ScrollSample>},
    Flick {delta: f64, next_frame: NextFrame},
    Pulldown {next_frame: NextFrame},
}

#[derive(Default, PartialEq)]
pub enum TouchMotionChange {
    #[default]
    None,
    ScrollStateChanged,
    ScrollOffsetChanged,
}

#[derive(Default)]
pub struct TouchGesture {
    flick_scroll_minimum: f64,
    flick_scroll_maximum: f64,
    flick_scroll_scaling: f64,
    flick_scroll_decay: f64,

    scroll_mode: ScrollMode,
    scroll_state: ScrollState,

    min_scroll_offset: f64,
    max_scroll_offset: f64,
    pulldown_maximum: f64,

    pub scroll_offset: f64,
}

impl TouchGesture {
    pub fn new() -> Self {
        Self {
            flick_scroll_minimum: 0.2,
            flick_scroll_maximum: 80.0,
            flick_scroll_scaling: 0.005,
            flick_scroll_decay: 0.98,

            scroll_state: ScrollState::Stopped,
            scroll_mode: ScrollMode::DragAndDrop,

            scroll_offset: 0.0,
            min_scroll_offset: f64::MIN,
            max_scroll_offset: f64::MAX,
            pulldown_maximum: 60.0,
        }
    }

    pub fn reset(&mut self, initial_offset: f64, min_offset: f64, max_offset: f64, scroll_mode: ScrollMode) {
        self.min_scroll_offset = min_offset;
        self.max_scroll_offset = max_offset;
        self.scroll_offset = initial_offset.clamp(
            self.min_scroll_offset - self.pulldown_maximum,
            self.max_scroll_offset + self.pulldown_maximum
        );
        self.scroll_mode = scroll_mode;
    }

    // Current motion is stopped so dragging is from now ignored, until a new finger down event
    pub fn stop(&mut self) {
        self.scroll_offset = 0.0;
        self.scroll_state = ScrollState::Stopped;
    }

    pub fn is_stopped(&self) -> bool {
        match self.scroll_state {
            ScrollState::Stopped => true,
            _ => false
        }
    }

    pub fn is_dragging(&self) -> bool {
        match self.scroll_state {
            ScrollState::Drag {..} => true,
            _ => false
        }
    }

    pub fn handle_event(&mut self, cx: &mut Cx, event: &Event, area: Area) -> TouchMotionChange {
        let needs_pulldown_when_flicking = self.needs_pulldown_when_flicking();
        let needs_pulldown = self.needs_pulldown();

        match &mut self.scroll_state {
            ScrollState::Flick {delta, next_frame} => {
                if let Some(_) = next_frame.is_event(event) {
                    *delta = *delta * self.flick_scroll_decay;
                    if needs_pulldown_when_flicking {
                        self.scroll_state = ScrollState::Pulldown {next_frame: cx.new_next_frame()};
                        return TouchMotionChange::ScrollStateChanged
                    } else if delta.abs() > self.flick_scroll_minimum {
                        *next_frame = cx.new_next_frame();
                        let delta = *delta;

                        let new_offset = self.scroll_offset - delta;
                        self.scroll_offset = new_offset.clamp(
                            self.min_scroll_offset - self.pulldown_maximum,
                            self.max_scroll_offset + self.pulldown_maximum
                        );

                        return TouchMotionChange::ScrollOffsetChanged
                    } else {
                        if needs_pulldown {
                            self.scroll_state = ScrollState::Pulldown {next_frame: cx.new_next_frame()};
                        } else {
                            self.scroll_state = ScrollState::Stopped;
                        }

                        return TouchMotionChange::ScrollStateChanged
                    }
                }
            }
            ScrollState::Pulldown {next_frame} => {
                if let Some(_) = next_frame.is_event(event) {
                    if self.scroll_offset < self.min_scroll_offset {
                        self.scroll_offset += (self.min_scroll_offset - self.scroll_offset) * 0.1;
                        if self.min_scroll_offset - self.scroll_offset < 1.0 {
                            self.scroll_offset = self.min_scroll_offset + 0.5;
                        }
                        else {
                            *next_frame = cx.new_next_frame();
                        }

                        return TouchMotionChange::ScrollOffsetChanged
                    }
                    else if self.scroll_offset > self.max_scroll_offset {
                        self.scroll_offset -= (self.scroll_offset - self.max_scroll_offset) * 0.1;
                        if self.scroll_offset - self.max_scroll_offset < 1.0 {
                            self.scroll_offset = self.max_scroll_offset - 0.5;

                            return TouchMotionChange::ScrollOffsetChanged
                        }
                        else {
                            *next_frame = cx.new_next_frame();
                        }

                        return TouchMotionChange::ScrollOffsetChanged
                    }
                    else {
                        self.scroll_state = ScrollState::Stopped;
                        return TouchMotionChange::ScrollStateChanged
                    }
                }
            }
            _=>()
        }

        match event.hits_with_capture_overload(cx, area, true) {            
            Hit::FingerDown(e) => {
                self.scroll_state = ScrollState::Drag {
                    samples: vec![ScrollSample{abs: e.abs.y, time: e.time}]
                };

                return TouchMotionChange::ScrollStateChanged
            }
            Hit::FingerMove(e) => {
                cx.set_cursor(MouseCursor::Default);
                match &mut self.scroll_state {
                    ScrollState::Drag {samples}=>{
                        let new_abs = e.abs.y;
                        let old_sample = *samples.last().unwrap();
                        samples.push(ScrollSample{abs: new_abs, time: e.time});
                        if samples.len() > 4 {
                            samples.remove(0);
                        }
                        let new_offset = self.scroll_offset + old_sample.abs - new_abs;
                        self.scroll_offset = new_offset.clamp(
                            self.min_scroll_offset - self.pulldown_maximum,
                            self.max_scroll_offset + self.pulldown_maximum
                        );

                        return TouchMotionChange::ScrollOffsetChanged
                    }
                    _=>()
                }
            }
            Hit::FingerUp(_e) => {
                match &mut self.scroll_state {
                    ScrollState::Drag {samples} => {
                        match self.scroll_mode {
                            ScrollMode::Swipe => {
                                let mut last = None;
                                let mut scaled_delta = 0.0;
                                let mut total_delta = 0.0;
                                for sample in samples.iter().rev() {
                                    if last.is_none() {
                                        last = Some(sample);
                                    }
                                    else {
                                        total_delta += last.unwrap().abs - sample.abs;
                                        scaled_delta += (last.unwrap().abs - sample.abs)/ (last.unwrap().time - sample.time)
                                    }
                                }
                                scaled_delta *= self.flick_scroll_scaling;

                                if self.needs_pulldown() {
                                    self.scroll_state = ScrollState::Pulldown {next_frame: cx.new_next_frame()};
                                }
                                else if total_delta.abs() > 10.0 && scaled_delta.abs() > self.flick_scroll_minimum {
                                    self.scroll_state = ScrollState::Flick {
                                        delta: scaled_delta.min(self.flick_scroll_maximum).max(-self.flick_scroll_maximum),
                                        next_frame: cx.new_next_frame()
                                    };
                                } else {
                                    self.scroll_state = ScrollState::Stopped;
                                }

                                return TouchMotionChange::ScrollStateChanged
                            }
                            ScrollMode::DragAndDrop => {
                                self.scroll_state = ScrollState::Stopped;
                                return TouchMotionChange::ScrollStateChanged
                            }
                        }
                    }
                    _=>()
                }
            }
            _ => ()
        }

        TouchMotionChange::None
    }

    fn needs_pulldown(&self) -> bool {
        self.scroll_offset < self.min_scroll_offset || self.scroll_offset > self.max_scroll_offset
    }

    fn needs_pulldown_when_flicking(&self) -> bool {
        self.scroll_offset - 0.5 < self.min_scroll_offset - self.pulldown_maximum ||
            self.scroll_offset + 0.5 > self.max_scroll_offset + self.pulldown_maximum
    }
}

impl TouchMotionChange {
    pub fn has_changed(&self) -> bool {
        match self {
            TouchMotionChange::None => false,
            _ => true
        }
    }
}