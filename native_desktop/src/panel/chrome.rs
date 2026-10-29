//! Secondary GTK windows keep native close-only title controls.
//! No global window-manager policy, main-card decoration or daemon state changes.
use super::*;

/// Restrict only Settings and About to a close title button on KDE and GTK themes.
pub(super) fn close_only_titlebar(window: &gtk::Window) {
    let header = gtk::HeaderBar::new();
    header.set_decoration_layout(Some(":close"));
    header.set_show_title_buttons(true);
    window.set_titlebar(Some(&header));
}

#[cfg(test)]
pub(super) fn assert_close_only_titlebar(window: &gtk::Window) {
    let titlebar = window
        .titlebar()
        .expect("secondary window must have a titlebar")
        .downcast::<gtk::HeaderBar>()
        .expect("secondary titlebar must have native GTK controls");
    assert!(titlebar.shows_title_buttons());
    assert_eq!(titlebar.decoration_layout().as_deref(), Some(":close"));
}
