use super::*;
use rqs_lib::service::{
    client::state::{FileDetail, StateView},
    ResourceUsage,
};

#[test]
fn prepared_source_summary_uses_exact_localized_counts_without_mutating_source_kind() {
    use rqs_lib::sessions::details::PayloadKind;
    for count in [0, 1, 2, u32::MAX] {
        assert_eq!(
            source_summary(PayloadKind::Files, count, u64::MAX),
            crate::i18n::prepared_files(count)
        );
    }
    for bytes in [0, 1, 1024, u64::MAX] {
        assert_eq!(
            source_summary(PayloadKind::Text, u32::MAX, bytes),
            crate::i18n::prepared_content(false, bytes)
        );
        assert_eq!(
            source_summary(PayloadKind::Url, u32::MAX, bytes),
            crate::i18n::prepared_content(true, bytes)
        );
    }
}

#[test]
fn given_complete_view_and_busy_flags_when_controls_are_projected_then_only_synchronized_live_idle_state_is_mutable(
) {
    // Arrange
    let mut state = PanelState {
        view: Some(Arc::new(StateView {
            sources: vec![],
            epoch: 1,
            revision: 1,
            lifecycle: Lifecycle::Running,
            synchronized: true,
            resources: ResourceUsage::default(),
            endpoints: vec![],
            selections: vec![],
            transfers: vec![],
            warnings: vec![],
            notices: vec![],
        })),
        ..Default::default()
    };
    // Act / Assert
    assert!(controls_ready(&state, false));
    assert!(!controls_ready(&state, true));
    for flag in 0..4 {
        state.busy = flag == 0;
        state.pending = flag == 1;
        state.disconnected = flag == 2;
        state.text_pending = flag == 3;
        assert!(!controls_ready(&state, false));
    }
    state.busy = false;
    state.pending = false;
    state.disconnected = false;
    state.text_pending = false;
    for lifecycle in [Lifecycle::Draining, Lifecycle::Stopped] {
        Arc::make_mut(state.view.as_mut().unwrap()).lifecycle = lifecycle;
        assert!(!controls_ready(&state, false));
    }
    Arc::make_mut(state.view.as_mut().unwrap()).lifecycle = Lifecycle::Running;
    Arc::make_mut(state.view.as_mut().unwrap()).synchronized = false;
    assert!(!controls_ready(&state, false));
    state.view = None;
    assert!(!controls_ready(&state, false));
}

#[test]
fn given_authoritative_incoming_revisions_when_devices_attention_is_reconciled_then_only_unreviewed_requests_pulse_and_new_requests_focus(
) {
    let mut attention = DeviceAttention::default();
    assert!(!attention.has_unreviewed());
    assert_eq!(
        attention.observe(Some(9), [("phone-a".into(), 1)], false),
        None
    );
    assert!(attention.has_unreviewed());
    assert_eq!(attention.review_pending().as_deref(), Some("phone-a"));
    assert!(!attention.has_unreviewed());

    // Re-rendering the same immutable view never retriggers attention.
    assert_eq!(
        attention.observe(Some(9), [("phone-a".into(), 1)], true),
        None
    );
    assert!(!attention.has_unreviewed());

    // A second device is focused while the drawer is already open.
    assert_eq!(
        attention.observe(
            Some(9),
            [("phone-a".into(), 1), ("phone-b".into(), 2)],
            true,
        ),
        Some("phone-b".into())
    );
    assert!(!attention.has_unreviewed());
    // A new consent round is a new request even when the transfer ID is reused.
    assert_eq!(
        attention.observe(
            Some(9),
            [("phone-a".into(), 3), ("phone-b".into(), 2)],
            false,
        ),
        None
    );
    assert!(attention.has_unreviewed());
    assert_eq!(attention.review_pending().as_deref(), Some("phone-a"));
    assert!(!attention.has_unreviewed());

    // Resolved requests, disconnection and daemon epoch replacement retire all
    // presentation rights rather than leaking a stale device focus.
    assert_eq!(
        attention.observe(Some(9), Vec::<(String, u64)>::new(), false),
        None
    );
    assert!(!attention.has_unreviewed());
    assert_eq!(attention.review_pending(), None);
    attention.observe(Some(9), [("old".into(), 1)], false);
    assert!(attention.has_unreviewed());
    attention.observe(None, Vec::<(String, u64)>::new(), false);
    assert!(!attention.has_unreviewed());
    assert_eq!(attention.review_pending(), None);
    attention.observe(Some(10), [("new".into(), 1)], false);
    assert_eq!(attention.review_pending().as_deref(), Some("new"));
}

fn labels(widget: &impl IsA<gtk::Widget>) -> Vec<gtk::Label> {
    let widget = widget.as_ref();
    let mut found = Vec::new();
    if let Some(label) = widget.downcast_ref::<gtk::Label>() {
        found.push(label.clone());
    }
    let mut next = widget.first_child();
    while let Some(child) = next {
        found.extend(labels(&child));
        next = child.next_sibling();
    }
    found
}

fn expand_details(widget: &impl IsA<gtk::Widget>) {
    let widget = widget.as_ref();
    if let Some(expander) = widget.downcast_ref::<gtk::Expander>() {
        expander.set_expanded(true);
    }
    let mut next = widget.first_child();
    while let Some(child) = next {
        expand_details(&child);
        next = child.next_sibling();
    }
}

#[test]
fn given_actual_builder_and_untrusted_details_when_rendered_then_preview_is_inert_manifest_is_complete_plain_text_and_close_hides(
) {
    // Arrange: this single GTK test runs on an isolated display/session bus.
    gtk::init().unwrap();
    let css_errors = Rc::new(RefCell::new(Vec::new()));
    let observed = css_errors.clone();
    let css = gtk::CssProvider::new();
    css.connect_parsing_error(move |_, _, error| observed.borrow_mut().push(error.to_string()));
    css.load_from_string(include_str!("../../ui/panel.css"));
    assert!(css_errors.borrow().is_empty(), "{:?}", css_errors.borrow());
    rows::verify_keyed_layout();
    transfers::verify_columns();
    movement::verify_targets();
    let child = gtk::Button::with_label("Behind curtain");
    let backdrop = backdrop::Backdrop::new(&child);
    backdrop.set_blurred(true);
    assert!(!child.is_sensitive());
    backdrop.set_blurred(false);
    assert!(child.is_sensitive());
    reveal::tests::verify_reveal_smoke();
    let runtime = tokio::runtime::Runtime::new().unwrap();
    let application = gtk::Application::new(
        Some("io.github.morozhenko_apps.RQuickShare.WidgetTest"),
        gio::ApplicationFlags::NON_UNIQUE,
    );
    application.register(gio::Cancellable::NONE).unwrap();
    let panel = Panel::new(
        &application,
        None,
        runtime.handle().clone(),
        Arc::new(Ingress::new()),
        false,
    );
    // Act / Assert
    assert_eq!(panel.model.borrow().drawer, Drawer::Closed);
    assert_eq!(
        panel.button("quick-devices").tooltip_text().as_deref(),
        Some(crate::i18n::label("Show devices and transfers"))
    );
    assert!(panel.box_widget("compact").get_visible());
    assert!(!panel.pages.get_visible());
    assert!(!panel.box_widget("source").get_visible());
    for id in ["menu", "badge", "visibility", "tabs"] {
        assert!(panel.builder.object::<gtk::Widget>(id).is_none(), "{id}");
    }
    assert!(panel.window.has_css_class("rqs-card-window"));
    assert!(panel.box_widget("root").has_css_class("rqs-card-body"));
    assert_eq!(panel.box_widget("root").overflow(), gtk::Overflow::Hidden);
    assert_eq!(
        panel.button("quick-hide").action_name().as_deref(),
        Some("panel.hide")
    );
    for text in labels(&panel.window) {
        assert!(!["rQuickShare", "Ready", "Ready to share"].contains(&text.text().as_str()));
        assert!(!text.text().starts_with("Choose files"));
    }
    let permit = panel.ingress.gate.try_acquire().unwrap();
    panel.window.present();
    panel.button("quick-hide").emit_clicked();
    assert!(!panel.window.is_visible() && !panel.model.borrow().closing);
    assert!(*panel.ingress.gate.subscribe().borrow());
    panel.button("quick-devices").emit_clicked();
    assert_eq!(panel.model.borrow().drawer, Drawer::Open);
    assert!(panel.pages.get_visible());
    assert_eq!(
        panel.pages.next_sibling(),
        Some(panel.box_widget("source").upcast())
    );
    assert_eq!(
        panel.box_widget("source").next_sibling(),
        Some(panel.box_widget("receive-indicator").upcast())
    );
    assert_eq!(
        panel.curtain.action_name().as_deref(),
        Some("panel.discard")
    );
    assert_eq!(
        panel.reconnect_button.label().as_deref(),
        Some(crate::i18n::label("Retry connection"))
    );
    let expected_curtain = format!(
        "{}\n{}",
        crate::i18n::label("Choose target devices"),
        crate::i18n::label("Clear prepared files")
    );
    assert_eq!(
        panel.curtain.label().as_deref(),
        Some(expected_curtain.as_str())
    );
    assert_eq!(
        panel.button("quick-devices").tooltip_text().as_deref(),
        Some(crate::i18n::label("Hide devices and transfers"))
    );
    assert_eq!(
        panel.button("quick-receive").tooltip_text().as_deref(),
        Some(crate::i18n::label(
            "Review an incoming request or become visible for one minute"
        ))
    );
    assert!(!panel.box_widget("source").get_visible());
    for removed in ["header", "mode", "settings-button", "close", "send-entry"] {
        assert!(panel.builder.object::<gtk::Widget>(removed).is_none());
    }
    assert!(panel.box_widget("compact").get_visible());
    assert_eq!(
        panel.box_widget("device-drawer").next_sibling(),
        panel.backdrop.parent()
    );
    let scroll = panel
        .builder
        .object::<gtk::ScrolledWindow>("nearby-scroll")
        .unwrap();
    assert_eq!(scroll.min_content_height(), 0);
    assert_eq!(scroll.max_content_height(), 300);
    panel.resize_to_content();
    if let Some(monitor) = panel.window.monitor() {
        let expected = monitor
            .geometry()
            .height()
            .saturating_sub(220)
            .clamp(120, 760);
        assert_eq!(scroll.max_content_height(), expected);
    }
    assert!(panel.box_widget("compact").get_visible());
    assert!(*panel.ingress.gate.subscribe().borrow());
    panel.button("quick-devices").emit_clicked();
    assert_eq!(panel.drawer(), Drawer::Closed);
    assert!(!panel.box_widget("device-drawer").get_visible());
    assert_eq!(
        panel.button("quick-devices").tooltip_text().as_deref(),
        Some(crate::i18n::label("Show devices and transfers"))
    );
    assert_eq!(
        panel.curtain.action_name().as_deref(),
        Some("panel.discard")
    );
    panel.button("quick-devices").emit_clicked();
    assert_eq!(panel.drawer(), Drawer::Open);
    assert_eq!(
        panel.button("quick-devices").tooltip_text().as_deref(),
        Some(crate::i18n::label("Hide devices and transfers"))
    );
    assert_eq!(
        panel.curtain.action_name().as_deref(),
        Some("panel.discard")
    );
    panel.show_sending();
    assert_eq!(panel.pages.visible_child_name().as_deref(), Some("nearby"));
    assert_eq!(panel.drawer(), Drawer::Open);
    assert!(
        !panel.sending.is_visible(),
        "Source choice must stay closed while preparation is busy and no service view exists"
    );
    drop(permit);
    assert!(!*panel.ingress.gate.subscribe().borrow());
    panel.actions.activate_action("files", None);
    assert!(panel.model.borrow().dialog.is_none());
    for name in ["files", "folder", "discard", "recover", "abort"] {
        assert!(!panel.button(name).is_sensitive(), "{name}");
    }
    let unsupported = TransferDetails {
        status: DetailsStatus::Unsupported,
        ..Default::default()
    };
    for phase in [
        Phase::Finished,
        Phase::Rejected,
        Phase::Cancelled,
        Phase::Failed,
        Phase::Disconnected,
    ] {
        let details = safe_transfer_details(&unsupported, phase, true, None);
        let messages: Vec<_> = labels(&details)
            .iter()
            .map(|item| item.text().to_string())
            .collect();
        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0], "Content details are unavailable for review.");
    }
    let pending = safe_transfer_details(&unsupported, Phase::AwaitingConsent, true, None);
    assert!(labels(&pending)
        .iter()
        .any(|item| item.text().contains("Decline or cancel")));
    for status in [DetailsStatus::Unavailable, DetailsStatus::Unsupported] {
        let row = details_card(&TransferDetails {
            status,
            ..Default::default()
        });
        let text = labels(&row)
            .iter()
            .map(|label| label.text().to_string())
            .collect::<Vec<_>>()
            .join(" ");
        assert!(text.contains(if status == DetailsStatus::Unavailable {
            "Waiting"
        } else {
            "cannot be reviewed safely"
        }));
        assert!(!text.contains("Code:"));
    }
    let details = TransferDetails {
        status: DetailsStatus::Ready,
        sender: "<b>Unverified phone</b>".into(),
        pin: "0123".into(),
        total_bytes: 256,
        files: (0..256)
            .map(|index| FileDetail {
                name: format!("<file-{index}>.txt"),
                size: 1,
            })
            .collect::<Vec<_>>()
            .into(),
        ..Default::default()
    };
    for kind in [
        rqs_lib::sessions::details::PayloadKind::Text,
        rqs_lib::sessions::details::PayloadKind::Url,
    ] {
        let text_details = TransferDetails {
            kind,
            status: DetailsStatus::Ready,
            sender: "phone".into(),
            pin: "0123".into(),
            total_bytes: 3,
            ..Default::default()
        };
        for full in [false, true] {
            let row = details_card_mode(&text_details, full);
            let texts = labels(&row)
                .iter()
                .map(|label| label.text().to_string())
                .collect::<Vec<_>>()
                .join(" ");
            assert!(
                texts.contains(if kind == rqs_lib::sessions::details::PayloadKind::Url {
                    "Link · 3 B"
                } else {
                    "Text · 3 B"
                })
            );
            assert!(texts.contains(&crate::i18n::pin_code("0123")));
            assert!(!texts.contains(&crate::i18n::file_count(1)));
            assert!(!texts.contains(crate::i18n::label("Review all files")));
            assert!(labels(&row).iter().all(|label| !label.uses_markup()));
        }
    }
    let invalid_pin = TransferDetails {
        pin: "01\u{202e}23".into(),
        ..details.clone()
    };
    let invalid = details_card(&invalid_pin);
    assert!(!labels(&invalid)
        .iter()
        .any(|label| label.widget_name() == "detail-code"));
    assert_eq!(invalid_pin.pin, "01\u{202e}23");
    assert_eq!(details.pin, "0123");
    let row = details_card(&details);
    assert!(labels(&row).iter().any(|label| {
        label.text()
            == format!(
                "{} · {}",
                crate::i18n::isolate_untrusted(&details.sender),
                crate::i18n::label("advertised device")
            )
    }));
    assert!(labels(&row).iter().all(|label| !label.uses_markup()));
    assert!(labels(&row)
        .iter()
        .any(|label| label.text() == crate::i18n::pin_code("0123")));
    let expander = row
        .last_child()
        .unwrap()
        .downcast::<gtk::Expander>()
        .unwrap();
    assert_eq!(
        expander.label().as_deref(),
        Some(crate::i18n::label("Review all files"))
    );
    assert!(expander.child().is_none());
    expander.set_expanded(true);
    let file_labels = labels(&expander.child().unwrap());
    assert_eq!(file_labels.len(), 256);
    for (index, label) in file_labels.iter().enumerate() {
        assert_eq!(
            label.text(),
            format!(
                "{} · {}",
                crate::i18n::isolate_untrusted(&format!("<file-{index}>.txt")),
                crate::i18n::byte_count(1)
            )
        );
        assert!(!label.uses_markup());
    }
    expander.set_expanded(false);
    assert!(expander.child().is_none());
    panel.set_drawer(Drawer::Closed);
    panel.preview("incoming");
    let compact_text = labels(&panel.box_widget("compact"))
        .iter()
        .map(|label| label.text().to_string())
        .collect::<Vec<_>>()
        .join(" ");
    assert_eq!(panel.box_widget("compact").observe_children().n_items(), 4);
    let quick = panel.box_widget("compact").observe_children();
    for (index, expected) in ["quick-receive", "quick-send", "quick-devices", "quick-hide"]
        .iter()
        .enumerate()
    {
        assert_eq!(
            quick
                .item(index as u32)
                .unwrap()
                .downcast::<gtk::Widget>()
                .unwrap(),
            panel.button(expected).upcast::<gtk::Widget>()
        );
    }

    for action in ["Receive", "Send", "Hide", "Devices"] {
        assert!(compact_text.contains(action));
    }
    assert!(!compact_text.contains(&crate::i18n::pin_code("4821")));
    expand_details(&panel.box_widget("incoming"));
    let reviewed = labels(&panel.box_widget("incoming"))
        .iter()
        .map(|label| label.text().to_string())
        .collect::<Vec<_>>()
        .join(" ");
    assert!(reviewed.contains(&crate::i18n::pin_code("4821")));
    assert!(reviewed.contains("Example.pdf"));
    assert!(!compact_text.contains("Example.pdf"));
    // The actual card row has direct consent, no Receive indirection or lower body.
    fn consent_controls(widget: &gtk::Widget, found: &mut Vec<String>) {
        assert!(!widget.is::<gtk::Expander>());
        assert_ne!(widget.widget_name(), "row-action");
        if let Some(button) = widget.downcast_ref::<gtk::Button>() {
            found.push(button.widget_name().to_string());
            assert!(!button.is_sensitive(), "synthetic preview cannot accept");
        }
        if widget.widget_name() == "row-summary" {
            assert_eq!(
                widget
                    .clone()
                    .downcast::<gtk::Label>()
                    .unwrap()
                    .text()
                    .to_string(),
                format!("PDF · {}", crate::i18n::pin_code("4821"))
            );
            assert!(widget.tooltip_text().unwrap().contains("Example.pdf"));
        }
        let mut child = widget.first_child();
        while let Some(current) = child {
            consent_controls(&current, found);
            child = current.next_sibling();
        }
    }
    let mut direct = Vec::new();
    consent_controls(panel.box_widget("nearby").upcast_ref(), &mut direct);
    assert_eq!(direct, ["decline", "accept"]);
    let transfer_captions = labels(&panel.box_widget("transfers"))
        .iter()
        .map(|label| label.text().to_string())
        .collect::<Vec<_>>();
    assert!(transfer_captions.contains(&crate::i18n::transfer_recipient("Example phone")));
    assert!(transfer_captions.contains(&crate::i18n::transfer_recipient("Example laptop")));
    assert!(transfer_captions.contains(&crate::i18n::transfer_progress(
        2048,
        12345,
        crate::i18n::ProgressKind::Sent,
    )));
    assert!(transfer_captions.contains(&crate::i18n::transfer_phase("Queued").to_string()));

    panel.set_drawer(Drawer::Open);
    let open_width = panel.window.measure(gtk::Orientation::Horizontal, -1);
    panel.set_drawer(Drawer::Closed);
    let closed_width = panel.window.measure(gtk::Orientation::Horizontal, -1);
    assert_eq!(
        open_width.1.max(420),
        closed_width.1.max(420),
        "one card width: open={open_width:?}, closed={closed_width:?}"
    );
    for drawer in [Drawer::Open, Drawer::Closed] {
        panel.set_drawer(drawer);
        assert_eq!(
            panel.window.measure(gtk::Orientation::Horizontal, -1).0,
            surface::WIDTH
        );
        paint_card(
            &panel,
            if drawer == Drawer::Open {
                "fixed-expanded"
            } else {
                "fixed-collapsed"
            },
            true,
        );
        assert_eq!(panel.window.width(), surface::WIDTH);
        assert!(
            panel
                .box_widget("root")
                .measure(gtk::Orientation::Horizontal, -1)
                .0
                <= surface::WIDTH
        );
    }

    panel.actions.activate_action("settings", None);
    let settings = panel.settings.borrow().as_ref().unwrap().clone();
    assert!(settings.is_visible());
    panel.actions.activate_action("about", None);
    let about = panel.about.borrow().as_ref().unwrap().clone();
    assert!(about.is_visible());
    assert_eq!(about.program_name().as_deref(), Some("rQuickShare"));
    assert_eq!(about.version().as_deref(), Some(env!("CARGO_PKG_VERSION")));
    assert_eq!(
        about.comments().as_deref(),
        Some("rQuickShare Native is the native Linux edition of this independent fork. Based on Martichou/rquickshare. Maintained by morozhenko apps.")
    );
    assert_eq!(
        about.logo_icon_name().as_deref(),
        Some(crate::tray::ICON_NAME)
    );
    assert_eq!(about.website().as_deref(), Some("https://morozhenko.com"));
    assert!(!about.is_resizable());
    about.close();
    assert!(!about.is_visible());
    panel.actions.activate_action("about", None);
    assert_eq!(*panel.about.borrow().as_ref().unwrap(), about);
    assert!(about.is_visible());
    assert_eq!(panel.drawer(), Drawer::Closed);
    settings::verify_editor_smoke(panel.settings_owner.borrow().as_ref().unwrap());
    assert!(labels(&panel.box_widget("incoming"))
        .iter()
        .any(|label| label.text() == crate::i18n::pin_code("4821")));
    assert_eq!(
        panel
            .actions
            .lookup_action("devices")
            .unwrap()
            .state()
            .unwrap()
            .get::<bool>(),
        Some(true)
    );
    panel.set_drawer(Drawer::Closed);
    panel.set_drawer(Drawer::Open);
    assert!(panel.pages.get_visible()); // Full content stays configured while Cancel hides its parent.
    assert!(panel.builder.object::<gtk::Widget>("tabs").is_none());
    assert_eq!(panel.pages.visible_child_name().as_deref(), Some("nearby"));
    panel.pages.set_visible_child_name("incoming");
    panel.set_drawer(Drawer::Closed);
    settings.close();
    assert!(!settings.is_visible());
    panel.actions.activate_action("settings", None);
    assert_eq!(*panel.settings.borrow().as_ref().unwrap(), settings);
    assert!(settings.is_visible());
    tray::verify_tray_projection(&panel);
    assert!(panel.builder.object::<gtk::Widget>("recipients").is_none());
    assert!(panel.builder.object::<gtk::Widget>("send-many").is_none());
    // Arrange: a connected presentation with no prepared source, without a mutation handle.
    panel.model.borrow_mut().state.view =
        Some(Arc::new(rqs_lib::service::client::state::StateView {
            sources: vec![],
            epoch: 123,
            revision: 1,
            lifecycle: Lifecycle::Running,
            synchronized: true,
            resources: Default::default(),
            endpoints: vec![rqs_lib::service::client::state::Endpoint {
                id: "device".into(),
                name: "Example device".into(),
            }],
            selections: vec![],
            transfers: vec![],
            warnings: vec![],
            notices: vec![],
        }));
    panel.model.borrow_mut().preview = false;
    panel.set_drawer(Drawer::Closed);
    panel.show_sending();
    assert_eq!(panel.drawer(), Drawer::Closed);
    assert!(
        panel.sending.is_visible(),
        "content choice must precede recipient Send"
    );
    panel.sending.popdown();
    panel.render();
    fn assert_no_choices(widget: &gtk::Widget) {
        assert!(!widget.is::<gtk::CheckButton>());
        let mut child = widget.first_child();
        while let Some(next) = child {
            child = next.next_sibling();
            assert_no_choices(&next);
        }
    }
    assert_no_choices(panel.box_widget("nearby").upcast_ref());
    outbound::verify(&panel);
    panel.show_transfers();
    assert!(panel.transfers.borrow().is_some());
    assert_eq!(panel.pages.visible_child_name().as_deref(), Some("nearby"));
    assert!(!panel.model.borrow().closing);

    image::tests::verify_image_smoke(&panel, &application);
    compose::tests::verify_composer_smoke(&panel, &application);
    text::tests::verify_review_smoke(&panel, &application);
    panel.safety_update(crate::desktop_guard::SafetyState {
        locked: Some(false),
        inhibited: Some(false),
        closed: false,
    });
    assert_eq!(
        panel.pages.visible_child_name().as_deref(),
        Some("nearby") // The mockup keeps requests in the single device list.
    );
    panel.window.present();
    assert!(panel.window.is_visible());
    assert!(panel.activate_at((1, 1)));
    assert!(!panel.window.is_visible());
    assert!(panel.activate_at((1, 1)));
    assert!(panel.window.is_visible());
    panel.window.close();
    assert!(!panel.window.is_visible());
    assert!(!panel.model.borrow().closing);
    panel.submit(|| panic!("preview must not admit actions"));
    panel.window.present();
    assert!(panel.window.is_visible());
    panel.safety_update(crate::desktop_guard::SafetyState {
        locked: Some(true),
        inhibited: Some(false),
        closed: false,
    });
    assert!(!panel.window.is_visible() && !settings.is_visible());
    panel.safety_update(crate::desktop_guard::SafetyState {
        locked: Some(false),
        inhibited: Some(false),
        closed: false,
    });
    assert!(!panel.window.is_visible());
    panel.window.present();
    panel.safety_update(crate::desktop_guard::SafetyState::default());
    assert!(!panel.window.is_visible());
    // Explicit unavailable backend is distinct from preview and has visible recovery.
    {
        let mut model = panel.model.borrow_mut();
        model.preview = false;
        model.preview_page = None;
        model.state.disconnected = true;
    }
    panel.render();
    assert_eq!(
        panel.builder.object::<gtk::Label>("status").unwrap().text(),
        "Transfer service unavailable. Check installation, then reconnect."
    );
    assert!(panel.reconnect_button.get_visible());
    assert!(panel.box_widget("recovery").get_visible());
    assert!(!panel.button("recover").get_visible() && !panel.button("abort").get_visible());
    assert!(!panel.button("files").is_sensitive());
    assert!(panel
        .actions
        .lookup_action("reconnect")
        .unwrap()
        .is_enabled());
    panel.actions.activate_action("reconnect", None);
    assert!(panel.reconnect_requested());
    panel.close();
    assert!(panel.model.borrow().closing);
    assert!(panel.settings.borrow().is_none());
    assert!(panel.about.borrow().is_none());
    assert!(!about.is_visible());
    assert!(panel.settings_owner.borrow().is_none());
    assert!(panel.context.parent().is_none());
    assert!(!settings.is_visible());
    panel.close(); // Explicit Quit and the application cleanup may both close the panel.
}

// Optional actual-widget frames; no connected writer, timer-based state or daemon transfer fixture.
pub(super) fn capture_card(panel: &Rc<Panel>, name: &str) {
    paint_card(panel, name, false);
}

fn paint_card(panel: &Rc<Panel>, name: &str, force: bool) {
    use gtk::gdk::prelude::PaintableExt;
    let output = std::env::var_os("RQS_CARD_PREVIEW_OUTPUT");
    if output.is_none() && !force {
        return;
    }
    let destination = output.map(|output| {
        let output = std::path::PathBuf::from(output);
        assert!(output.is_absolute());
        std::fs::create_dir_all(&output).unwrap();
        output.join(format!("{name}.png"))
    });
    panel.window.present();
    panel.window.queue_draw();
    let finished = Rc::new(Cell::new(false));
    let captured = finished.clone();
    let loop_ = glib::MainLoop::new(None, false);
    let completed = loop_.clone();
    let clock = panel.window.frame_clock().unwrap();
    let window = panel.window.clone();
    // Snapshot only after GTK has painted the new layout, not the previous frame cached at tick time.
    let paint = clock.connect_after_paint(move |_| {
        if window.width() <= 0 || window.height() != window.default_height() {
            return;
        }
        let paintable = gtk::WidgetPaintable::new(Some(&window));
        let snapshot = gtk::Snapshot::new();
        paintable.snapshot(&snapshot, window.width().into(), window.height().into());
        let Some(node) = snapshot.to_node() else {
            return;
        };
        if let Some(destination) = destination.as_ref() {
            window
                .renderer()
                .unwrap()
                .render_texture(&node, None)
                .save_to_png(destination)
                .unwrap();
        }
        captured.set(true);
        completed.quit();
    });
    let expired = Rc::new(Cell::new(false));
    let timeout_fired = expired.clone();
    let deadline = loop_.clone();
    let timeout = glib::timeout_add_local_once(std::time::Duration::from_secs(10), move || {
        timeout_fired.set(true);
        deadline.quit();
    });
    loop_.run();
    if !expired.get() {
        timeout.remove();
    }
    clock.disconnect(paint);
    assert!(
        finished.get(),
        "actual card frame did not become renderable"
    );
}
