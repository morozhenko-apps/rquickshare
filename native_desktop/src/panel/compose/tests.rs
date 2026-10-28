use super::*;
#[test]
fn given_editor_byte_boundaries_when_inserted_then_no_overflow_nul_or_silent_truncation_is_allowed()
{
    // Arrange / Act / Assert
    for (current, inserted, expected) in [
        (0, "", true),
        (MAX_TEXT_BYTES, "", true),
        (MAX_TEXT_BYTES - 1, "x", true),
        (MAX_TEXT_BYTES, "x", false),
        (MAX_TEXT_BYTES - 2, "é", true),
        (MAX_TEXT_BYTES - 1, "é", false),
        (0, "a\0b", false),
        (usize::MAX, "x", false),
    ] {
        assert_eq!(insertion_allowed(current, inserted), expected);
    }
}
fn finish(composer: &Composer) {
    let task = composer.task.borrow_mut().take().unwrap();
    glib::MainContext::default().block_on(task).unwrap();
}
pub(in crate::panel) fn verify_composer_smoke(panel: &Rc<Panel>, application: &gtk::Application) {
    // Arrange: isolated GTK display; controller domain is exercised on its real private broker separately.
    use std::cell::Cell;
    let clipboard = gtk::prelude::WidgetExt::display(&panel.window).clipboard();
    clipboard.set_text("<b>explicit clipboard</b> 🌍");
    let primary = gtk::prelude::WidgetExt::display(&panel.window).primary_clipboard();
    primary.set_text("untouched selection");
    let guard = Rc::new(Cell::new(true));
    let attempts = Rc::new(Cell::new(0));
    let check = guard.clone();
    let count = attempts.clone();
    let composer = Composer::new(
        Some(application),
        1,
        panel.ingress.gate.try_acquire().unwrap(),
        move || check.get(),
        move |text| {
            assert_eq!(text.text(), "<b>explicit clipboard</b> 🌍");
            count.set(count.get() + 1);
            Err(ControllerError::Busy)
        },
        |_| panic!("Rejected adapter admission cannot complete preparation"),
    );
    // Act / Assert: no clipboard capture without a paste action.
    composer.window.present();
    assert!(composer.body().is_none());
    assert!(!composer.prepare.is_sensitive());
    assert_eq!(attempts.get(), 0);
    assert!(*panel.ingress.gate.subscribe().borrow());
    assert_eq!(
        glib::MainContext::default()
            .block_on(clipboard.read_text_future())
            .unwrap()
            .unwrap(),
        "<b>explicit clipboard</b> 🌍"
    );
    assert_eq!(
        glib::MainContext::default()
            .block_on(primary.read_text_future())
            .unwrap()
            .unwrap(),
        "untouched selection"
    );
    composer.paste.emit_clicked();
    finish(&composer);
    assert_eq!(
        composer.body().unwrap().text(),
        "<b>explicit clipboard</b> 🌍"
    );
    assert_eq!(
        glib::MainContext::default()
            .block_on(clipboard.read_text_future())
            .unwrap()
            .unwrap(),
        "<b>explicit clipboard</b> 🌍"
    );
    assert_eq!(
        glib::MainContext::default()
            .block_on(primary.read_text_future())
            .unwrap()
            .unwrap(),
        "untouched selection"
    );
    assert!(composer.prepare.is_sensitive());
    assert_eq!(attempts.get(), 0); // Paste cannot prepare/send.
    guard.set(false);
    composer.prepare.emit_clicked();
    assert_eq!(attempts.get(), 0);
    guard.set(true);
    composer.prepare.emit_clicked();
    assert_eq!(attempts.get(), 1);
    assert!(composer.body().is_some()); // Rejected admission preserves user text.
                                        // Normal keyboard paste is routed through the same bounded reader.
    clipboard.set_text("edited");
    composer.editor.emit_by_name::<()>("paste-clipboard", &[]);
    finish(&composer);
    assert_eq!(composer.body().unwrap().text(), "edited");
    for value in ["x".repeat(MAX_TEXT_BYTES + 1), "".to_owned()] {
        clipboard.set_text(&value);
        composer.paste.emit_clicked();
        finish(&composer);
        assert_eq!(composer.body().unwrap().text(), "edited");
    }
    composer.buffer.set_text(&"x".repeat(MAX_TEXT_BYTES));
    composer.buffer.insert_at_cursor("é");
    assert_eq!(composer.body().unwrap().text().len(), MAX_TEXT_BYTES);
    composer.buffer.set_text("edited");
    // Controlled memory stream exercises UTF8/NUL, short read/EOF and exact byte limit without clipboard provider assumptions.
    for (bytes, valid) in [
        (vec![], false),
        (vec![255], false),
        (b"a\0b".to_vec(), false),
        ("é".repeat(65536).into_bytes(), true),
        (vec![b'x'; MAX_TEXT_BYTES + 1], false),
    ] {
        let stream = gio::MemoryInputStream::from_bytes(&glib::Bytes::from_owned(bytes));
        let result = glib::MainContext::default().block_on(read_bounded(stream.upcast_ref()));
        assert_eq!(result.is_ok(), valid);
        glib::MainContext::default()
            .block_on(stream.close_future(glib::Priority::DEFAULT))
            .unwrap();
    }
    panel.composer.replace(Some(composer.clone()));
    clipboard.set_text("late clipboard");
    composer.paste.emit_clicked();
    // Lock cancels the admitted GLib wait and clears private editor state/gate.
    panel.safety_update(crate::desktop_guard::SafetyState {
        locked: Some(true),
        inhibited: Some(false),
        closed: false,
    });
    assert!(composer.closed.get() && composer.buffer.char_count() == 0);
    assert!(!composer.window.is_visible() && composer.task.borrow().is_none());
    assert!(!*panel.ingress.gate.subscribe().borrow());
    assert_eq!(attempts.get(), 1);
}

#[test]
fn given_prepared_source_metadata_when_summarized_then_text_is_never_mislabeled_as_files_or_exposes_body(
) {
    // The visible summary describes local preparation, never a sent payload.
    use rqs_lib::sessions::details::PayloadKind;
    let text = source_summary(PayloadKind::Text, u32::MAX, 12);
    let link = source_summary(PayloadKind::Url, u32::MAX, 7);
    let files = source_summary(PayloadKind::Files, 3, u64::MAX);
    assert_eq!(text, crate::i18n::prepared_content(false, 12));
    assert_eq!(link, crate::i18n::prepared_content(true, 7));
    assert_eq!(files, crate::i18n::prepared_files(3));
    assert!(text.starts_with(crate::i18n::label("Text")));
    assert!(link.starts_with(crate::i18n::label("Link")));
    assert!(files.starts_with(&crate::i18n::file_count(3)));
    assert!(text.contains(&crate::i18n::byte_count(12)));
    assert!(link.contains(&crate::i18n::byte_count(7)));
    assert!(!text.contains(&crate::i18n::file_count(u32::MAX as usize)));
    assert_ne!(text, link);
    for count in [0, 1, 2, u32::MAX] {
        assert_eq!(
            source_summary(PayloadKind::Files, count, 0),
            crate::i18n::prepared_files(count)
        );
    }
    for bytes in [0, 1, 1024, u64::MAX] {
        assert_eq!(
            source_summary(PayloadKind::Text, 0, bytes),
            crate::i18n::prepared_content(false, bytes)
        );
        assert_eq!(
            source_summary(PayloadKind::Url, 0, bytes),
            crate::i18n::prepared_content(true, bytes)
        );
    }
}
