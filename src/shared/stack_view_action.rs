use makepad_widgets::*;

#[derive(Clone, WidgetAction, Eq, Hash, PartialEq)]
pub enum StackViewAction {
    None,
    ShowWonder,
    ShowArtifacts,
    ShowGallery,
    ShowTimeline,
}
