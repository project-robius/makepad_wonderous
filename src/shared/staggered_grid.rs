use makepad_widgets::{scroll_bar::{ScrollAxis, ScrollBarAction}, *};
use std::collections::HashMap;

// TODO:
// - fix all items being called on next_visible_item
// - fix snapping at end
// - fix scroll_bar not moving correctly on mouse wheel
// - fix range not setting the right limits

live_design!{
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;

    StaggeredGrid = {{StaggeredGrid}} {
        width: Fill
        height: Fill
        capture_overload: true
        scroll_bar: <ScrollBar> {}
        flow: Down
    }
}

#[derive(Clone,Copy)]
struct ScrollSample{
    abs: f64,
    time: f64,
}

enum ScrollState {
    Stopped,
    Drag{samples:Vec<ScrollSample>},
    Flick {delta: f64, next_frame: NextFrame},
    Pulldown {next_frame: NextFrame},
}

#[derive(Clone)]
enum ListDrawState {
    Begin,
    Down {index: usize, pos: f64, viewport: Rect},
    Up {index: usize, pos: f64, hit_bottom: bool, viewport: Rect},
    DownAgain {index: usize, pos: f64, viewport: Rect},
    End {viewport: Rect}
}

#[derive(Clone, Debug, DefaultNone)]
pub enum StaggeredGridAction {
    Scroll,
    None
}
impl ListDrawState {
    fn is_down_again(&self) -> bool {
        match self {
            Self::DownAgain {..} => true,
            _ => false
        }
    }
}
#[derive(Live, Widget)]
pub struct StaggeredGrid {
    #[redraw] #[rust] area: Area,
    #[walk] walk: Walk,
    #[layout] layout: Layout,
    
    #[rust] range_start: usize,
    #[rust(usize::MAX)] range_end: usize,
    #[rust(0usize)] view_window: usize,
    #[live(0.2)] flick_scroll_minimum: f64,
    #[live(80.0)] flick_scroll_maximum: f64,
    #[live(0.005)] flick_scroll_scaling: f64,
    #[live(0.98)] flick_scroll_decay: f64,
    #[live(100.0)] max_pull_down: f64,
    #[live(true)] align_top_when_empty: bool,
    #[live(false)] grab_key_focus: bool,
    #[live(true)] drag_scrolling: bool,
    #[rust] first_id: usize,
    #[rust] first_scroll: f64,
    #[rust(Vec2Index::X)] vec_index: Vec2Index,
    #[live] scroll_bar: ScrollBar,
    #[live] capture_overload: bool,
    #[live(false)] keep_invisible: bool,
    #[rust] draw_state: DrawStateWrap<ListDrawState>,
    #[rust] draw_align_list: Vec<AlignItem>,
    #[rust] detect_tail_in_draw: bool,
    #[live(false)] auto_tail: bool,
    #[rust(false)] tail_range: bool,
    
    #[rust] templates: ComponentMap<LiveId, LivePtr>,
    #[rust] items: ComponentMap<(usize, LiveId), WidgetRef>,
    //#[rust(DragState::None)] drag_state: DragState,
    #[rust(ScrollState::Stopped)] scroll_state: ScrollState,
    #[live] columns_number: usize,
    #[live] column_spacing: f64,
    #[rust(1usize)] last_drawn_column: usize,

    /// List of columns on the grid, indexed by column number, left to right.
    #[rust] columns: Vec<Column>,

    /// Maps item indices to their assigned columns, anchoring items to fixed positions
    /// within the grid when first drawn. It prevents reordering when scrolling or resizing the viewport.
    #[rust] item_columns: HashMap<usize, usize>
}

#[derive(Default, Clone, Debug)]
struct Column {
    pub last_item_index: usize,
    pub height: f64,
    pub exceeds_viewport: bool, 
}

struct AlignItem {
    align_range: TurtleAlignRange,
    size: DVec2,
    shift: f64,
    index: usize
}

impl LiveHook for StaggeredGrid {
    fn before_apply(&mut self, _cx: &mut Cx, apply: &mut Apply, _index: usize, _nodes: &[LiveNode]) {
        if let ApplyFrom::UpdateFromDoc {..} = apply.from {
            self.templates.clear();
        }
    }
    
    // hook the apply flow to collect our templates and apply to instanced childnodes
    fn apply_value_instance(&mut self, cx: &mut Cx, apply: &mut Apply, index: usize, nodes: &[LiveNode]) -> usize {
        let id = nodes[index].id;
        match apply.from {
            ApplyFrom::NewFromDoc {file_id} | ApplyFrom::UpdateFromDoc {file_id} => {
                if nodes[index].origin.has_prop_type(LivePropType::Instance) {
                    let live_ptr = cx.live_registry.borrow().file_id_index_to_live_ptr(file_id, index);
                    self.templates.insert(id, live_ptr);
                    // lets apply this thing over all our childnodes with that template
                    for ((_, templ_id), node) in self.items.iter_mut() {
                        if *templ_id == id {
                            node.apply(cx, apply, index, nodes);
                        }
                    }
                }
                else {
                    cx.apply_error_no_matching_field(live_error_origin!(), index, nodes);
                }
            }
            _ => ()
        }
        nodes.skip_node(index)
    }
    
    fn after_apply(&mut self, _cx: &mut Cx, _applyl: &mut Apply, _index: usize, _nodes: &[LiveNode]) {
        if let Flow::Down = self.layout.flow {
            self.vec_index = Vec2Index::Y
        }
        else {
            self.vec_index = Vec2Index::X
        }
        if self.auto_tail{
            self.tail_range = true;
        }
    }
}

impl StaggeredGrid {
    fn begin(&mut self, cx: &mut Cx2d, walk: Walk) {
        cx.begin_turtle(walk, self.layout);
        log!("▶️▶️▶️▶️▶️▶️ BEGIN");
        
        // The columns number in the DSL has changed so we need to recompute item ordering
        if self.columns.len() != self.columns_number {
            self.item_columns.clear();
        }

        self.columns = vec![Column::default(); self.columns_number];


        self.draw_align_list.clear();
    }

    // Positions all the drawn items 
    fn end(&mut self, cx: &mut Cx2d) {
        let vi = self.vec_index;
        let mut at_end = false;
        let mut visible_items = 0;

        if let Some(ListDrawState::End {viewport}) = self.draw_state.get() {
            let list = &mut self.draw_align_list;
            if list.len()>0 {
                list.sort_by( | a, b | a.index.cmp(&b.index));
                let first_index = list.iter().position( | v | v.index == self.first_id).unwrap();
                
                // Find the position of the first item in our set
                let mut first_pos = self.first_scroll;
                for i in (0..first_index).rev() {
                    let item = &list[i];
                    first_pos -= item.size.index(vi);
                }
                
                // Find the position of the last item in the range
                // note that the grid requests items beyond the range so you can pad out the grid 
                // when there is not enough data
                let mut last_pos = self.first_scroll;
                let mut last_item_pos = None;
                for i in first_index..list.len() {
                    let item = &list[i];
                    last_pos += item.size.index(vi);
                    if item.index < self.range_end {
                        last_item_pos = Some(last_pos);
                    }
                    else {
                        break;
                    }
                }
                
                // Viewport filling check
                // later used to trigger a stick-to-top if needed
                let mut not_filling_viewport = false;
                if list[0].index == self.range_start {
                    let mut total = 0.0;
                    for item in list.iter() {
                        if item.index >= self.range_end {
                            break;
                        }
                        total += item.size.index(vi);
                    }
                    not_filling_viewport = total < viewport.size.index(vi);
                }
                
                // 'Pull Down' scenario - when we are at the top
                if list.first().unwrap().index == self.range_start && first_pos > 0.0 {
                    let min = if let ScrollState::Stopped = self.scroll_state {
                        0.0
                    }
                    else {
                        self.max_pull_down
                    };
                    
                    let mut pos = first_pos.min(min); // maximum for first scroll
                    for item in list {
                        let shift = DVec2::from_index_pair(vi, pos, 0.0);
                        cx.shift_align_range(&item.align_range, shift - DVec2::from_index_pair(vi, item.shift, 0.0));
                        pos += item.size.index(vi);
                        visible_items += 1;
                    }
                    self.first_scroll = first_pos.min(min);
                    self.first_id = self.range_start;
                    log!("::end - first_id: {}", self.first_id);
                }
                else {
                    // Compute the 'stick to bottom' case 
                    let shift = if let Some(last_item_pos) = last_item_pos {
                        if self.align_top_when_empty && not_filling_viewport {
                            -first_pos
                        }
                        else {
                            let ret = (viewport.size.index(vi) - last_item_pos).max(0.0);
                            if ret > 0.0 {
                                at_end = true;
                            }
                            ret
                        }
                    }
                    else {
                        0.0
                    };

                    // Regular scenario - align to current scroll position
                    // Scan upwards and move items in place
                    let mut first_id_changed = false;
                    let start_pos = self.first_scroll + shift;
                    let mut pos = start_pos;
                    for i in (0..first_index).rev() {
                        let item = &list[i];
                        let visible = pos > 0.0;
                        pos -= item.size.index(vi);
                        let shift = DVec2::from_index_pair(vi, pos, 0.0);
                        cx.shift_align_range(&item.align_range, shift - DVec2::from_index_pair(vi, item.shift, 0.0));
                        if visible { // move up
                            self.first_scroll = pos;
                            self.first_id = item.index;
                            log!("::end 2 - first_id: {}", self.first_id);
                            first_id_changed = true;
                            if item.index < self.range_end {
                                visible_items += 1;
                            }
                        }
                    }
                    // Scan downwards
                    let mut pos = start_pos;
                    for i in first_index..list.len() {
                        let item = &list[i];
                        let shift = DVec2::from_index_pair(vi, pos, 0.0);
                        cx.shift_align_range(&item.align_range, shift - DVec2::from_index_pair(vi, item.shift, 0.0));
                        pos += item.size.index(vi);
                        let invisible = pos < 0.0;
                        if invisible { // move down
                            self.first_scroll = pos - item.size.index(vi);
                            self.first_id = item.index;
                            first_id_changed = true;
                            // TODO: fix this, if we don't break the second invisible item will cause a wierd jump in first_scroll
                            //dgb
                            break;
                        }
                        else if item.index < self.range_end {
                            visible_items += 1;
                        }
                    }
                    // Overwrite first scroll for top/bottom aligns if we havent updated already
                    if !first_id_changed {
                        self.first_scroll = start_pos;
                    }
                }
                if !self.scroll_bar.animator_in_state(cx, id!(hover.pressed)){
                    self.update_scroll_bar(cx);
                }
            }
        }
        else {
            log!("Draw state not at end in StaggeredGrid, please review your next_visible_item loop")
        }
        let rect = cx.turtle().rect();
        if at_end || self.view_window == 0 || self.view_window > visible_items{
            self.view_window = visible_items.max(4) - 3;
        }
        if self.detect_tail_in_draw{
            self.detect_tail_in_draw = false;
            if self.auto_tail && at_end{
                self.tail_range = true;
            }
        }
        let total_views = (self.range_end - self.range_start) as f64 / self.view_window as f64;
        self.scroll_bar.draw_scroll_bar(cx, ScrollAxis::Vertical, rect, dvec2(100.0, rect.size.y * total_views));
        if !self.keep_invisible{
            self.items.retain_visible();
        }
        cx.end_turtle_with_area(&mut self.area);
    }
    
    pub fn next_visible_item(&mut self, cx: &mut Cx2d) -> Option<usize> {
        let vi = self.vec_index;
        if let Some(draw_state) = self.draw_state.get() {
            match draw_state {
                ListDrawState::Begin => {
                    let viewport = cx.turtle().padded_rect();
                    self.draw_state.set(ListDrawState::Down {
                        index: self.first_id,
                        pos: self.first_scroll,
                        viewport,
                    });

                    log!("ListDrawState::Begin - first_id {} - first_scroll {}", self.first_id, self.first_scroll);

                    let abs_pos = dvec2(viewport.pos.x, viewport.pos.y + self.first_scroll);
                    log!("FIRST ITEM {:?}", abs_pos);
                    
                    self.last_drawn_column = 0;
                    // When beginning draw state, set the column heights to the first item's position
                    // in relationship to the viewport.
                    for column in self.columns.iter_mut() {
                        column.height = viewport.pos.y + self.first_scroll;
                    }

                    cx.begin_turtle(Walk {
                        abs_pos: Some(abs_pos),
                        margin: Default::default(),
                        width: Size::Fixed(self.column_width(viewport)),
                        height: Size::Fit
                    }, self.layout_with_spacing());
                    return Some(self.first_id)
                }
                ListDrawState::Down {index, pos, viewport} | ListDrawState::DownAgain {index, pos, viewport} => {
                    // log!("POS: {}", pos);
                    let is_down_again = draw_state.is_down_again();
                    let did_draw = cx.turtle_has_align_items();
                    let align_range = cx.get_turtle_align_range();
                    // Computed width and height of the last item drawn
                    let prev_item_rect = cx.end_turtle();
                    self.draw_align_list.push(AlignItem {
                        align_range,
                        shift: pos,
                        size: prev_item_rect.size,
                        index
                    });

                    let current_column = self.find_column_for_item(index);
                    // let current_column = self.find_next_available_column().unwrap_or(0);

                    self.record_previous_column(index, prev_item_rect, viewport, vi);

                    if !did_draw || self.columns[current_column].exceeds_viewport {
                        // lets scan upwards
                        if self.first_id > 0 && !is_down_again {
                            self.draw_state.set(ListDrawState::Up {
                                index: self.first_id - 1,
                                pos: self.first_scroll,
                                hit_bottom: index >= self.range_end,
                                viewport
                            });
                            cx.begin_turtle(Walk {
                                abs_pos: Some(dvec2(viewport.pos.x, viewport.pos.y)),
                                margin: Default::default(),
                                width: Size::Fixed(self.column_width(viewport)),
                                height: Size::Fit
                            }, Layout::flow_down());
                            return Some(self.first_id - 1);
                        }
                        else {
                            self.draw_state.set(ListDrawState::End {viewport});
                            log!("Set draw state to end");
                            return None
                        }
                    }
                    if is_down_again {
                        self.draw_state.set(ListDrawState::DownAgain {
                            index: index + 1,
                            pos: pos + prev_item_rect.size.index(vi),
                            viewport
                        });
                    }
                    else {
                        self.draw_state.set(ListDrawState::Down {
                            index: index + 1,
                            pos: pos + prev_item_rect.size.index(vi),
                            viewport
                        });
                    }

                    // Currently all columns are the same width
                    let column_width = viewport.size.x / self.columns.len() as f64;
                    let new_item_abs_pos = dvec2(
                        viewport.pos.x + (current_column as f64 * column_width), // x position based on current column
                        self.columns[current_column].height,    // y position based on accumulated height
                    );

                    log!("ABS POS -> X: {:.0}, Y: {:.0}", new_item_abs_pos.x, new_item_abs_pos.y);

                    self.last_drawn_column = current_column;
                    self.item_columns.insert(index, current_column);

                    cx.begin_turtle(Walk {
                        abs_pos: Some(new_item_abs_pos),
                        margin: Default::default(),
                        width: Size::Fixed(self.column_width(viewport)),
                        height: Size::Fit
                    }, self.layout_with_spacing());
                    return Some(index + 1)
                }
                ListDrawState::Up {index, pos, hit_bottom, viewport} => {
                    log!("UP");
                    let did_draw = cx.turtle_has_align_items();
                    let align_range = cx.get_turtle_align_range();
                    let rect = cx.end_turtle();
                    self.draw_align_list.push(AlignItem {
                        align_range,
                        size: rect.size,
                        shift: 0.0,
                        index
                    });
                    if index == self.range_start {
                        // we are at range start, but if we snap to top, we might need to walk further down as well
                        // therefore we now go 'down again' to make sure we have enough visible items
                        // if we snap to the top
                        if pos - rect.size.index(vi) > 0.0 {
                            // scan the tail
                            if let Some(last_index) = self.draw_align_list.iter().map( | v | v.index).max() {
                                // lets sum up all the items
                                let total_height: f64 = self.draw_align_list.iter().map( | v | v.size.index(vi)).sum();
                                self.draw_state.set(ListDrawState::DownAgain {
                                    index: last_index + 1,
                                    pos: total_height,
                                    viewport
                                });
                                log!("UP 1");
                                cx.begin_turtle(Walk {
                                    abs_pos: Some(dvec2(viewport.pos.x, viewport.pos.y + total_height)),
                                    margin: Default::default(),
                                    width: Size::Fixed(self.column_width(viewport)),
                                    height: Size::Fit
                                }, Layout::flow_down());
                                return Some(last_index + 1);
                            }
                        }
                        self.draw_state.set(ListDrawState::End {viewport});
                        return None
                    }
                    
                    if !did_draw || pos < if hit_bottom {-viewport.size.index(vi)} else {0.0} {
                        self.draw_state.set(ListDrawState::End {viewport});
                        return None
                    }
                    
                    self.draw_state.set(ListDrawState::Up {
                        index: index - 1,
                        hit_bottom,
                        pos: pos - rect.size.index(vi),
                        viewport
                    });
                    log!("UP 2");                   
                    cx.begin_turtle(Walk {
                        abs_pos: Some(dvec2(viewport.pos.x, viewport.pos.y)),
                        margin: Default::default(),
                        width: Size::Fixed(self.column_width(viewport)),
                        height: Size::Fit
                    }, Layout::flow_down());
                    
                    return Some(index - 1);
                }
                _ => ()
            }
        }
        None
    }
    
    pub fn item(&mut self, cx: &mut Cx, entry_id: usize, template: LiveId) -> Option<WidgetRef> {
        if let Some(ptr) = self.templates.get(&template) {
            let entry = self.items.get_or_insert(cx, (entry_id, template), | cx | {
                WidgetRef::new_from_ptr(cx, Some(*ptr))
            });
            return Some(entry.clone())
        }
        None
    }
    
    pub fn set_item_range(&mut self, cx: &mut Cx, range_start: usize, range_end: usize) {
        self.range_start = range_start;
        if self.range_end != range_end {
            self.range_end = range_end;
            if self.tail_range{
                self.first_id = self.range_end.max(1) - 1;
                self.first_scroll = 0.0;
            }
            self.update_scroll_bar(cx);
        }
    }
    
    pub fn update_scroll_bar(&mut self, cx: &mut Cx) {
        let scroll_pos = ((self.first_id - self.range_start) as f64 / ((self.range_end - self.range_start).max(self.view_window + 1) - self.view_window) as f64) * self.scroll_bar.get_scroll_view_total();
        // move the scrollbar to the right 'top' position
        self.scroll_bar.set_scroll_pos_no_action(cx, scroll_pos);
    }
    
    fn delta_top_scroll(&mut self, cx: &mut Cx, delta: f64, clip_top: bool) {
        self.first_scroll += delta;
        if self.first_id == self.range_start {
            self.first_scroll = self.first_scroll.min(self.max_pull_down);
        }
        if self.first_id == self.range_start && self.first_scroll > 0.0 && clip_top {
            self.first_scroll = 0.0;
        }
        self.update_scroll_bar(cx);
        log!("delta_top_scroll: first_scroll: {:.0}", self.first_scroll);
    }

    /// Updates the state of the previously drawn column
    fn record_previous_column(&mut self, index: usize, prev_item_rect: Rect, viewport: Rect, vi: Vec2Index) {
        let column = &mut self.columns[self.last_drawn_column];

        column.last_item_index = index + 1;
        column.height =  column.height + prev_item_rect.size.index(vi);

        // log!("Stored - COLUMN {} INDEX {} POS {:.0}", self.last_drawn_column, index + 1, column.height);

        // Wether we have finished drawing on the column (surpassed the viewport height or width)
        column.exceeds_viewport = column.height - viewport.pos.y > viewport.size.index(vi);
        // log!("COLUMN: {} EXCEEDS VIEWPORT: {}", self.last_drawn_column, column.exceeds_viewport)
    }

    fn find_column_for_item(&mut self, index: usize) -> usize {
        match self.item_columns.get(&index) {
            Some(column) => {
                *column
            },
            None => {
                let column = if let Some(col) = self.find_next_available_column() {
                    col
                } else {
                    (self.last_drawn_column + 1) % self.columns.len()
                };
                column
            }
        }
    }

    /// Returns the index of the next column to draw on.
    ///
    /// Starts searching from the column after the last one drawn and wraps around
    /// to the beginning as needed. If a column has already filled the viewport,
    /// it skips to the next one with available space.
    fn find_next_available_column(&self) -> Option<usize> {
        // Determine the starting index for the search, which is the next column after the last drawn column
        let start_index = (self.last_drawn_column + 1) % self.columns.len();

        // Iterator that represents the column indices in the order they should be checked
        let mut column_order = (start_index..self.columns.len()).chain(0..start_index);

        // Find the index of the next column that hasn't exceeded the viewport
        let next_column = column_order
            .find(|&index| !self.columns[index].exceeds_viewport);
            // .unwrap_or(start_index); // If all columns are full

        next_column
    }

    fn layout_with_spacing(&mut self) -> Layout{
        Layout::flow_down().with_padding(Padding {
            top: self.column_spacing,
            right: self.column_spacing,
            bottom: self.column_spacing,
            left: self.column_spacing
        })
    }

    fn column_width(&self, viewport: Rect) -> f64 {
        viewport.size.x / self.columns.len() as f64
    }
}


impl Widget for StaggeredGrid {

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        let uid = self.widget_uid();
        
        let mut scroll_to = None;
        self.scroll_bar.handle_event_with(cx, event, &mut | _cx, action | {
            // snap the scrollbar to a top-index with scroll_pos 0
            if let ScrollBarAction::Scroll {scroll_pos, view_total, view_visible} = action {
                scroll_to = Some((scroll_pos, scroll_pos+0.5 >= view_total - view_visible))
            }
        });
        if let Some((scroll_to, at_end)) = scroll_to {
            if at_end && self.auto_tail{
                self.first_id = self.range_end.max(1) - 1;
                self.first_scroll = 0.0;
                self.tail_range = true;
            }
            else if self.tail_range {
                self.tail_range = false;
            }

            self.first_id = ((scroll_to / self.scroll_bar.get_scroll_view_visible()) * self.view_window as f64) as usize;
            self.first_scroll = 0.0;
            cx.widget_action(uid, &scope.path, StaggeredGridAction::Scroll);
            self.area.redraw(cx);
        }
        
        for item in self.items.values_mut() {
            let item_uid = item.widget_uid();
            cx.group_widget_actions(uid, item_uid, |cx|{
                item.handle_event(cx, event, scope)
            });
        }
        
        match &mut self.scroll_state {
            ScrollState::Flick {delta, next_frame} => {
                if let Some(_) = next_frame.is_event(event) {
                    *delta = *delta * self.flick_scroll_decay;
                    if delta.abs()>self.flick_scroll_minimum {
                        *next_frame = cx.new_next_frame();
                        let delta = *delta;
                        self.delta_top_scroll(cx, delta, true);
                        cx.widget_action(uid, &scope.path, StaggeredGridAction::Scroll);
                        self.area.redraw(cx);
                    } else {
                        self.scroll_state = ScrollState::Stopped;
                    }
                }
            }
            ScrollState::Pulldown {next_frame} => {
                if let Some(_) = next_frame.is_event(event) {
                    // we have to bounce back
                    if self.first_id == self.range_start && self.first_scroll > 0.0 {
                        self.first_scroll *= 0.9;
                        if self.first_scroll < 1.0 {
                            self.first_scroll = 0.0;
                        }
                        else {
                            *next_frame = cx.new_next_frame();
                            cx.widget_action(uid, &scope.path, StaggeredGridAction::Scroll);
                        }
                        self.area.redraw(cx);
                    }
                    else {
                        self.scroll_state = ScrollState::Stopped
                    }
                }
            }
            _=>()
        }
        let vi = self.vec_index;
        let is_scroll = if let Event::Scroll(_) = event {true} else {false};
        if self.scroll_bar.is_area_captured(cx){
            self.scroll_state = ScrollState::Stopped;
        }
        if !self.scroll_bar.is_area_captured(cx) || is_scroll{ 
            match event.hits_with_capture_overload(cx, self.area, self.capture_overload) {
                Hit::FingerScroll(e) => {
                    if self.tail_range {
                        self.tail_range = false;
                    }
                    self.detect_tail_in_draw = true;
                    self.scroll_state = ScrollState::Stopped;
                    self.delta_top_scroll(cx, -e.scroll.index(vi), true);
                    cx.widget_action(uid, &scope.path, StaggeredGridAction::Scroll);
                    self.area.redraw(cx);
                },
                
                Hit::KeyDown(ke) => match ke.key_code {
                    KeyCode::Home => {
                        self.first_id = 0;
                        self.first_scroll = 0.0;
                        self.tail_range = false;
                        self.update_scroll_bar(cx);
                        self.area.redraw(cx);
                    },
                    KeyCode::End => {
                        self.first_id = self.range_end.max(1) - 1;
                        self.first_scroll = 0.0;
                        if self.auto_tail {
                            self.tail_range = true;
                        }
                        self.update_scroll_bar(cx);
                        self.area.redraw(cx);
                    },
                    KeyCode::PageUp => {
                        self.first_id = self.first_id.max(self.view_window) - self.view_window;
                        self.first_scroll = 0.0;
                        self.tail_range = false;
                        self.update_scroll_bar(cx);
                        self.area.redraw(cx);
                    },
                    KeyCode::PageDown => {
                        self.first_id += self.view_window;
                        self.first_scroll = 0.0;
                        if self.first_id >= self.range_end.max(1) {
                            self.first_id = self.range_end.max(1) - 1;
                        }
                        self.detect_tail_in_draw = true;
                        self.update_scroll_bar(cx);
                        self.area.redraw(cx);
                    },
                    KeyCode::ArrowDown => {
                        self.first_id += 1;
                        if self.first_id >= self.range_end.max(1) {
                            self.first_id = self.range_end.max(1) - 1;
                        }
                        self.detect_tail_in_draw = true;
                        self.first_scroll = 0.0;
                        self.update_scroll_bar(cx);
                        self.area.redraw(cx);
                    },
                    KeyCode::ArrowUp => {
                        if self.first_id > 0 {
                            self.first_id -= 1;
                            if self.first_id < self.range_start {
                                self.first_id = self.range_start;
                            }
                            self.first_scroll = 0.0;
                            self.area.redraw(cx);
                            self.tail_range = false;
                            self.update_scroll_bar(cx);
                        }
                    },
                    _ => ()
                }
                Hit::FingerDown(e) => {
                    //log!("F inger down {} {}", e.time, e.abs);
                    if self.grab_key_focus {
                        cx.set_key_focus(self.area);
                    }
                    // ok so fingerdown eh.
                    if self.tail_range {
                        self.tail_range = false;
                    }
                    if self.drag_scrolling{
                        self.scroll_state = ScrollState::Drag {
                            samples: vec![ScrollSample{abs:e.abs.index(vi),time:e.time}]
                        };
                    }
                }
                Hit::FingerMove(e) => {
                    //log!("Finger move {} {}", e.time, e.abs);
                    cx.set_cursor(MouseCursor::Default);
                    match &mut self.scroll_state {
                        ScrollState::Drag {samples}=>{
                            let new_abs = e.abs.index(vi);
                            let old_sample = *samples.last().unwrap();
                            samples.push(ScrollSample{abs:new_abs, time:e.time});
                            if samples.len()>4{
                                samples.remove(0);
                            }
                            self.delta_top_scroll(cx, new_abs - old_sample.abs, false);
                            self.area.redraw(cx);
                        }
                        _=>()
                    }
                }
                Hit::FingerUp(_e) => {
                    //log!("Finger up {} {}", e.time, e.abs);
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
                            if self.first_id == self.range_start && self.first_scroll > 0.0 {
                                self.scroll_state = ScrollState::Pulldown {next_frame: cx.new_next_frame()};
                            }
                            else if total_delta.abs() > 10.0 && scaled_delta.abs() > self.flick_scroll_minimum{
                                
                                self.scroll_state = ScrollState::Flick {
                                    delta: scaled_delta.min(self.flick_scroll_maximum).max(-self.flick_scroll_maximum),
                                    next_frame: cx.new_next_frame()
                                };
                            }
                            else{
                                self.scroll_state = ScrollState::Stopped;
                            }
                        }
                        _=>()
                    }
                    // ok so. lets check our gap from 'drag'
                    // here we kinda have to take our last delta and animate it
                }
                Hit::KeyFocus(_) => {
                }
                Hit::KeyFocusLost(_) => {
                }
                _ => ()
            }
        }
    }
    
    fn draw_walk(&mut self, cx: &mut Cx2d, _scope:&mut Scope, walk: Walk) -> DrawStep {
        if self.draw_state.begin(cx, ListDrawState::Begin) {
            self.begin(cx, walk);
            return DrawStep::make_step()
        }

        if let Some(_) = self.draw_state.get() {
            log!("🏁🏁🏁🏁🏁");
            self.end(cx);
            self.draw_state.end();
            log!("🏁🏁🏁🏁🏁 END\n\n");
        }
        DrawStep::done()
    }
}

impl StaggeredGridRef {
    pub fn set_first_id_and_scroll(&self, id: usize, s: f64) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.first_id = id;
            inner.first_scroll = s;
        }
    }
    
    pub fn set_first_id(&self, id: usize) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.first_id = id;
        }
    }
    
    pub fn first_id(&self) -> usize {
        if let Some(inner) = self.borrow() {
            inner.first_id
        }
        else {
            0
        }
    }
    
    pub fn set_tail_range(&self, tail_range: bool) {
        if let Some(mut inner) = self.borrow_mut() {
            inner.tail_range = tail_range
        }
    }
    
    pub fn item(&self, cx: &mut Cx, entry_id: usize, template: LiveId) -> Option<WidgetRef> {
        if let Some(mut inner) = self.borrow_mut() {
            inner.item(cx, entry_id, template)
        }
        else {
            None
        }
    }
    
    pub fn items_with_actions(&self, actions: &Actions) -> ItemsWithActions {
        let mut set = Vec::new();
        self.items_with_actions_vec(actions, &mut set);
        set
    }
    
    fn items_with_actions_vec(&self, actions: &Actions, set: &mut ItemsWithActions) {
        let uid = self.widget_uid();
        for action in actions {
            if let Some(action) = action.as_widget_action(){
                if let Some(group) = &action.group{
                    if group.group_uid == uid{
                        if let Some(inner) = self.borrow() {
                            for ((item_id, _), item) in inner.items.iter() {
                                if group.item_uid == item.widget_uid(){
                                    set.push((*item_id, item.clone()))
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

type ItemsWithActions = Vec<(usize, WidgetRef)>;

impl StaggeredGridSet {
    pub fn set_first_id(&self, id: usize) {
        for list in self.iter() {
            list.set_first_id(id)
        }
    }
    
    
    pub fn items_with_actions(&self, actions: &Actions) -> ItemsWithActions {
        let mut set = Vec::new();
        for list in self.iter() {
            list.items_with_actions_vec(actions, &mut set)
        }
        set
    }
}
