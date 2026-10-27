//! Keyed flat rows keep direct controls and focus across progress/readiness updates.
use super::*;
use std::collections::{BTreeMap, BTreeSet};

struct Entry {
    root: gtk::Box,
    transfer: Option<TransferView>,
    endpoint: Option<(String, bool)>,
}
#[derive(Default)]
pub(super) struct Rows {
    entries: BTreeMap<(String, String), Entry>,
}
impl Rows {
    pub(super) fn clear(&mut self) {
        self.entries.clear();
    }
}
// Return the smallest valid scroll movement that fully reveals a keyed row.
// An oversized row is aligned to its start; any already visible row is unchanged.
fn reveal_vertical_interval(
    start: f64,
    end: f64,
    offset: f64,
    page_size: f64,
    lower: f64,
    upper: f64,
) -> Option<f64> {
    if page_size <= 0.0 || end <= start {
        return None;
    }
    let max_offset = (upper - page_size).max(lower);
    let target = if start < offset {
        start
    } else if end > offset + page_size {
        if end - start > page_size {
            start
        } else {
            end - page_size
        }
    } else {
        return None;
    };
    let value = target.clamp(lower, max_offset);
    (value != offset).then_some(value)
}

fn named(root: &gtk::Widget, name: &str) -> Option<gtk::Widget> {
    if root.widget_name() == name {
        return Some(root.clone());
    }
    let mut next = root.first_child();
    while let Some(child) = next {
        if let Some(found) = named(&child, name) {
            return Some(found);
        }
        next = child.next_sibling();
    }
    None
}
#[cfg(test)]
fn section_title(text: &str) -> gtk::Box {
    let row = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    let title = label(text);
    title.add_css_class("dim-label");
    row.append(&title);
    row
}
fn bound_current_endpoint<'a>(
    direction: Direction,
    associated: Option<&'a str>,
    discovered: &BTreeSet<&str>,
) -> Option<&'a str> {
    if direction != Direction::Outbound {
        return None;
    }
    associated.filter(|id| discovered.contains(id))
}
fn card_transfer_visible(
    direction: Direction,
    phase: Phase,
    associated: Option<&str>,
    discovered: &BTreeSet<&str>,
) -> bool {
    !phase.is_terminal() && bound_current_endpoint(direction, associated, discovered).is_none()
}
fn payload_label(details: &TransferDetails) -> String {
    use rqs_lib::sessions::details::PayloadKind;
    match details.status {
        DetailsStatus::Unavailable => {
            return crate::i18n::label("Waiting for details").into();
        }
        DetailsStatus::Unsupported => {
            return crate::i18n::label("Unsupported content").into();
        }
        DetailsStatus::Ready => {}
    }
    match details.kind {
        PayloadKind::Files if details.files.len() == 1 => details.files[0]
            .name
            .rsplit_once('.')
            .map(|(_, ext)| ext)
            .filter(|ext| {
                !ext.is_empty() && ext.len() <= 12 && ext.bytes().all(|b| b.is_ascii_alphanumeric())
            })
            .map(|ext| ext.to_ascii_uppercase())
            .unwrap_or_else(|| crate::i18n::label("File").into()),
        PayloadKind::Files => crate::i18n::file_count(details.files.len()),
        PayloadKind::Text => crate::i18n::label("Text").into(),
        PayloadKind::Url => crate::i18n::label("Link").into(),
    }
}
/// Missing metadata is an in-progress handshake, not an anonymous file.
fn peer_title(name: &str, status: DetailsStatus) -> &str {
    if name.is_empty() && status == DetailsStatus::Unavailable {
        crate::i18n::label("Connecting…")
    } else {
        name
    }
}
fn transferring(phase: Phase) -> bool {
    matches!(phase, Phase::Sending | Phase::Receiving)
}
fn progress_fraction(done: u64, total: u64) -> f64 {
    if total == 0 {
        0.0
    } else {
        done.min(total) as f64 / total as f64
    }
}
fn same_description(a: &TransferView, b: &TransferView) -> bool {
    a.transfer == b.transfer
        && a.completed_files == b.completed_files
        && a.recipient == b.recipient
        && a.associated_endpoint == b.associated_endpoint
        && a.direction == b.direction
        && a.phase == b.phase
        && a.consent_revision == b.consent_revision
        && a.content_revision == b.content_revision
        && a.reveal_revision == b.reveal_revision
        && a.details.kind == b.details.kind
        && a.details.status == b.details.status
        && a.details.sender == b.details.sender
        && a.details.device == b.details.device
        && a.details.pin == b.details.pin
        && a.details.total_bytes == b.details.total_bytes
        && a.details.files == b.details.files
}
fn arrange(container: &gtk::Box, desired: &[gtk::Box]) {
    let mut next = container.first_child();
    while let Some(child) = next {
        next = child.next_sibling();
        if !desired
            .iter()
            .any(|row| row.upcast_ref::<gtk::Widget>() == &child)
        {
            container.remove(&child);
        }
    }
    let mut previous: Option<gtk::Widget> = None;
    for row in desired {
        if row.parent().as_ref() != Some(container.upcast_ref()) {
            if let Some(parent) = row.parent().and_downcast::<gtk::Box>() {
                parent.remove(row);
            }
            container.append(row);
        }
        if row.prev_sibling() != previous {
            container.reorder_child_after(row, previous.as_ref());
        }
        previous = Some(row.clone().upcast());
    }
}
#[derive(Clone, Copy)]
enum Badge {
    Available,
    Unavailable,
    Transfer(Phase),
}
fn badge_style(badge: Badge) -> (&'static str, &'static str) {
    match badge {
        Badge::Available => ("●", "success"),
        Badge::Unavailable | Badge::Transfer(Phase::Cancelled | Phase::Rejected) => {
            ("−", "dim-label")
        }
        Badge::Transfer(Phase::Failed | Phase::Disconnected) => ("!", "error"),
        Badge::Transfer(Phase::Finished) => ("✓", "success"),
        Badge::Transfer(Phase::AwaitingConsent) => ("?", "warning"),
        Badge::Transfer(Phase::Queued | Phase::CancelRequested) => ("…", "warning"),
        Badge::Transfer(Phase::Sending) => ("↑", "accent"),
        Badge::Transfer(Phase::Receiving) => ("↓", "accent"),
        Badge::Transfer(Phase::Active) => ("●", "accent"),
    }
}
fn device_icon(
    kind: rqs_lib::service::client::state::DeviceKind,
    status: &str,
    badge: Badge,
) -> gtk::Overlay {
    use rqs_lib::service::client::state::DeviceKind;
    let overlay = gtk::Overlay::new();
    let image = gtk::Image::from_icon_name(match kind {
        DeviceKind::Phone => "phone-symbolic",
        DeviceKind::Tablet => "tablet-symbolic",
        _ => "computer-symbolic",
    });
    image.set_pixel_size(28);
    overlay.set_widget_name("row-icon");
    overlay.set_child(Some(&image));
    let (symbol, class) = badge_style(badge);
    let marker = gtk::Label::new(Some(symbol));
    marker.add_css_class("device-status");
    marker.set_widget_name("device-status");
    marker.add_css_class(class);
    marker.set_halign(gtk::Align::End);
    marker.set_valign(gtk::Align::End);
    overlay.add_overlay(&marker);
    overlay.set_tooltip_text(Some(status));
    overlay.update_property(&[gtk::accessible::Property::Label(status)]);
    overlay
}
pub(super) fn transfer_row(
    sender: &str,
    kind: rqs_lib::service::client::state::DeviceKind,
    phase: Phase,
    details: &gtk::Box,
) -> gtk::Box {
    let incoming = phase == Phase::AwaitingConsent;
    let root = gtk::Box::new(gtk::Orientation::Vertical, 0);
    let heading = gtk::Box::new(gtk::Orientation::Horizontal, 8);
    heading.set_widget_name("transfer-heading");
    heading.append(&device_icon(
        kind,
        phase_text(phase),
        Badge::Transfer(phase),
    ));
    let title = if sender.is_empty() {
        label(if incoming {
            crate::i18n::label("Incoming request")
        } else {
            crate::i18n::label("Transfer")
        })
    } else {
        label(&crate::i18n::isolate_untrusted(sender))
    };
    title.set_widget_name("row-peer");
    title.set_selectable(false);
    title.set_hexpand(true);
    title.set_wrap(false);
    title.set_ellipsize(gtk::pango::EllipsizeMode::End);
    title.set_max_width_chars(20);
    title.set_width_request(0);
    heading.append(&title);
    let summary = label("");
    summary.set_widget_name("row-summary");
    summary.set_selectable(false);
    summary.set_wrap(false);
    heading.append(&summary);
    let progress = gtk::ProgressBar::new();
    progress.set_widget_name("row-progress");
    progress.set_width_request(72);
    progress.set_valign(gtk::Align::Center);
    progress.set_visible(matches!(phase, Phase::Sending | Phase::Receiving));
    heading.append(&progress);
    let status = label(phase_text(phase));
    status.set_selectable(false);
    status.set_widget_name("row-status");
    status.set_visible(phase.is_terminal());
    heading.append(&status);
    root.append(&heading);
    append_inline_controls(&heading, details);
    root
}
fn append_inline_controls(heading: &gtk::Box, details: &gtk::Box) {
    for name in [
        "decline",
        "accept",
        "review-text",
        "cancel",
        "cancel-reveal",
    ] {
        if let Some(action) = named(details.upcast_ref(), name) {
            let parent = action.parent().and_downcast::<gtk::Box>().unwrap();
            parent.remove(&action);
            heading.append(&action);
        }
    }
}
impl Panel {
    fn append_result_actions(
        self: &Rc<Self>,
        root: &gtk::Box,
        transfer: &TransferView,
        ready: bool,
    ) {
        let heading = root.first_child().and_downcast::<gtk::Box>().unwrap();
        if transfer.direction == Direction::Inbound {
            if let Some(revision) = transfer.reveal_revision {
                if transfer.phase == Phase::Finished {
                    let reveal = gtk::Button::with_label(crate::i18n::label("Show in folder"));
                    reveal.set_widget_name("result-reveal");
                    reveal.set_sensitive(ready);
                    let weak = Rc::downgrade(self);
                    let target = transfer.transfer.clone();
                    reveal.connect_clicked(move |_| {
                        if let Some(panel) = weak.upgrade() {
                            panel.reveal_result(target.clone(), revision);
                        }
                    });
                    heading.append(&reveal);
                } else if !transfer.completed_files.is_empty() {
                    let saved = gtk::Box::new(gtk::Orientation::Vertical, 8);
                    for (index, file) in transfer.completed_files.iter().enumerate() {
                        let name = label(&crate::i18n::isolate_untrusted(&file.name));
                        name.set_max_width_chars(40);
                        name.set_ellipsize(gtk::pango::EllipsizeMode::Middle);
                        saved.append(&name);
                        if let Some(action) = self.file_action(transfer, true) {
                            action(&saved, index as u32);
                        }
                    }
                    let popover = gtk::Popover::new();
                    popover.set_child(Some(&saved));
                    let menu = gtk::MenuButton::builder()
                        .label(crate::i18n::label("Saved files"))
                        .popover(&popover)
                        .build();
                    heading.append(&menu);
                }
            }
        }
    }
    fn align_history_row(&self, root: &gtk::Box, transfer: &TransferView) {
        let heading = root.first_child().and_downcast::<gtk::Box>().unwrap();
        let peer = gtk::Box::new(gtk::Orientation::Horizontal, 8);
        let content = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        let actions = gtk::Box::new(gtk::Orientation::Horizontal, 8);
        let status = named(heading.upcast_ref(), "row-status").unwrap();
        status.set_visible(true);
        while let Some(child) = heading.first_child() {
            heading.remove(&child);
            match child.widget_name().as_str() {
                "row-icon" => {
                    child.set_width_request(32);
                    peer.append(&child);
                }
                "row-peer" => peer.append(&child),
                "row-summary" | "row-progress" => content.append(&child),
                "row-status" => (),
                _ => actions.append(&child),
            }
        }
        let direction = label(if transfer.direction == Direction::Inbound {
            crate::i18n::label("Receive")
        } else {
            crate::i18n::label("Send")
        });
        direction.set_wrap(false);
        let date = transfers::timestamp(None);
        date.set_widget_name("history-date");
        let row = self.history_columns.row([
            peer.upcast_ref(),
            direction.upcast_ref(),
            content.upcast_ref(),
            &status,
            date.upcast_ref(),
            actions.upcast_ref(),
        ]);
        root.remove(&heading);
        root.append(&row);
    }
    pub(super) fn focus_transfer(&self, id: &str) {
        let Some(row) = self
            .rows
            .borrow()
            .entries
            .get(&("nearby".to_owned(), id.to_owned()))
            .map(|entry| entry.root.clone())
        else {
            return;
        };
        // Focus the keyed request, not the untrusted advertised device name.
        // A focus change alone does not guarantee that an offscreen request
        // becomes visible after GTK's new row allocation.
        if let Some(accept) = named(row.upcast_ref(), "accept") {
            accept.grab_focus();
        } else {
            row.set_focusable(true);
            row.grab_focus();
        }
        let Some(scroll) = self.builder.object::<gtk::ScrolledWindow>("nearby-scroll") else {
            return;
        };
        let content = self.box_widget("nearby");
        // A one-shot frame callback observes the settled row geometry. There
        // is no timer, retry or recurrent frame work. If the row is removed or
        // the drawer is hidden before the frame, it cannot scroll stale UI.
        scroll.add_tick_callback(move |scroll, _| {
            if scroll.is_mapped() {
                if let Some(bounds) = row.compute_bounds(&content) {
                    let adjustment = scroll.vadjustment();
                    if let Some(value) = reveal_vertical_interval(
                        bounds.y() as f64,
                        (bounds.y() + bounds.height()) as f64,
                        adjustment.value(),
                        adjustment.page_size(),
                        adjustment.lower(),
                        adjustment.upper(),
                    ) {
                        adjustment.set_value(value);
                    }
                }
            }
            glib::ControlFlow::Break
        });
    }
    fn cached_transfer(
        self: &Rc<Self>,
        rows: &mut Rows,
        page: &str,
        transfer: &TransferView,
        ready: bool,
        incoming: bool,
    ) -> gtk::Box {
        let key = (page.to_owned(), transfer.transfer.id().to_owned());
        let reusable = rows.entries.get(&key).is_some_and(|entry| {
            entry
                .transfer
                .as_ref()
                .is_some_and(|old| same_description(old, transfer))
        });
        if !reusable {
            let details = if incoming {
                self.incoming_card(transfer, ready, true)
            } else {
                self.transfer_card(transfer, ready, true)
            };
            let root = transfer_row(
                peer_title(
                    transfer
                        .recipient
                        .as_deref()
                        .unwrap_or(&transfer.details.sender),
                    transfer.details.status,
                ),
                transfer.details.device,
                transfer.phase,
                &details,
            );
            if page == "nearby" && incoming {
                root.add_css_class("incoming-request");
            }
            if let Some(summary) = named(root.upcast_ref(), "row-summary") {
                let mut description = if incoming {
                    String::from(crate::i18n::label(
                        "Check this code on the sending device before accepting.",
                    ))
                } else {
                    String::new()
                };
                for file in transfer.details.files.iter() {
                    description.push_str(&format!(
                        "\n{} · {}",
                        crate::i18n::isolate_untrusted(&file.name),
                        crate::i18n::byte_count(file.size)
                    ));
                }
                summary.set_tooltip_text(Some(&description));
                summary.update_property(&[gtk::accessible::Property::Description(&description)]);
            }
            if page == "transfers" {
                self.append_result_actions(&root, transfer, ready);
                self.align_history_row(&root, transfer);
            } else {
                for name in ["review-text", "cancel-reveal"] {
                    if let Some(action) = named(root.upcast_ref(), name) {
                        action
                            .parent()
                            .and_downcast::<gtk::Box>()
                            .unwrap()
                            .remove(&action);
                    }
                }
            }
            rows.entries.insert(
                key.clone(),
                Entry {
                    root,
                    transfer: Some(transfer.clone()),
                    endpoint: None,
                },
            );
        }
        let root = rows.entries[&key].root.clone();
        if page == "transfers" {
            if let Some(date) =
                named(root.upcast_ref(), "history-date").and_downcast::<gtk::Label>()
            {
                let updated = self
                    .model
                    .borrow()
                    .state
                    .journal
                    .as_ref()
                    .and_then(|journal| {
                        journal
                            .records
                            .iter()
                            .find(|r| r.id == transfer.transfer.id())
                            .map(|r| r.updated)
                    });
                date.set_text(&transfers::timestamp(updated).text());
            }
        }

        for (name, enabled) in [
            ("result-reveal", ready && transfer.reveal_revision.is_some()),
            ("review-text", ready && transfer.content_revision.is_some()),
            (
                "accept",
                ready && transfer.pending_control.is_none() && transfer.can_accept(),
            ),
            (
                "decline",
                ready && transfer.pending_control.is_none() && transfer.consent_revision.is_some(),
            ),
            (
                "cancel",
                ready
                    && transfer.pending_control.is_none()
                    && transfer.phase != Phase::CancelRequested,
            ),
        ] {
            if let Some(button) = named(root.upcast_ref(), name) {
                button.set_sensitive(enabled);
            }
        }
        for index in 0..transfer.completed_files.len() {
            if let Some(button) = named(root.upcast_ref(), &format!("reveal-{index}")) {
                button.set_sensitive(ready && transfer.reveal_revision.is_some());
            }
        }
        if let Some(cancel) = named(root.upcast_ref(), "cancel-reveal") {
            cancel.set_visible(self.revealing(&transfer.transfer));
        }
        if let Some(summary) = named(root.upcast_ref(), "row-summary").and_downcast::<gtk::Label>()
        {
            let text = if transfer.phase.is_terminal() {
                format!(
                    "{} · {}",
                    payload_label(&transfer.details),
                    crate::i18n::byte_count(transfer.details.total_bytes)
                )
            } else if !transfer.details.pin.is_empty() {
                let code = crate::i18n::pin_code(&transfer.details.pin);
                if code.is_empty() {
                    payload_label(&transfer.details)
                } else {
                    format!("{} · {}", payload_label(&transfer.details), code)
                }
            } else {
                payload_label(&transfer.details)
            };
            summary.set_text(&text);
            summary.set_visible(!transferring(transfer.phase));
        }
        if let Some(progress) =
            named(root.upcast_ref(), "row-progress").and_downcast::<gtk::ProgressBar>()
        {
            progress.set_visible(transferring(transfer.phase));
            progress.set_fraction(progress_fraction(
                transfer.details.transferred_bytes,
                transfer.details.total_bytes,
            ));
            progress.set_tooltip_text(Some(phase_text(transfer.phase)));
        }
        root
    }
    pub(super) fn render_rows(
        self: &Rc<Self>,
        view: &rqs_lib::service::client::state::StateView,
        selection: Option<rqs_lib::service::client::Selection>,
        ready: bool,
        _current: Option<&TransferView>,
    ) {
        let mut rows = self.rows.borrow_mut();
        let mut used = BTreeSet::new();
        let mut pages: BTreeMap<&str, Vec<gtk::Box>> = ["nearby", "incoming", "transfers"]
            .into_iter()
            .map(|page| (page, vec![]))
            .collect();
        let endpoints = view
            .endpoints
            .iter()
            .map(|peer| (peer.id.clone(), (peer.name.clone(), true)))
            .collect::<BTreeMap<_, _>>();
        for (id, projection) in endpoints {
            let key = ("nearby".to_owned(), id.clone());
            if rows
                .entries
                .get(&key)
                .is_none_or(|entry| entry.endpoint.as_ref() != Some(&projection))
            {
                let root = gtk::Box::new(gtk::Orientation::Vertical, 6);
                let controls = gtk::Box::new(gtk::Orientation::Horizontal, 8);
                controls.add_css_class("device-row");
                let state = if projection.1 {
                    Badge::Available
                } else {
                    Badge::Unavailable
                };
                controls.append(&device_icon(
                    rqs_lib::service::client::state::DeviceKind::Unknown,
                    if projection.1 {
                        crate::i18n::label("Nearby advertised device")
                    } else {
                        crate::i18n::label("Unavailable")
                    },
                    state,
                ));
                let title = label(&crate::i18n::isolate_untrusted(&projection.0));
                title.set_selectable(false);
                title.set_selectable(false);
                title.set_hexpand(true);
                title.set_wrap(false);
                title.set_ellipsize(gtk::pango::EllipsizeMode::End);
                title.set_max_width_chars(20);
                controls.append(&title);
                let progress = gtk::ProgressBar::new();
                progress.set_widget_name("device-progress");
                progress.set_width_request(72);
                progress.set_valign(gtk::Align::Center);
                controls.append(&progress);
                let strip = gtk::Revealer::new();
                strip.set_widget_name("send-strip");
                strip.set_transition_type(gtk::RevealerTransitionType::SlideLeft);
                strip.set_transition_duration(160);
                let inputs = gtk::Box::new(gtk::Orientation::Horizontal, 3);
                for (kind, icon, title) in [
                    (outbound::Kind::Files, "text-x-generic-symbolic", crate::i18n::label("File")),
                    (outbound::Kind::Folder, "folder-symbolic", crate::i18n::label("Folder")),
                    (outbound::Kind::Text, "insert-text-symbolic", crate::i18n::label("Text")),
                    (
                        outbound::Kind::Clipboard,
                        "edit-paste-symbolic",
                        crate::i18n::label("Clipboard"),
                    ),
                ] {
                    let button = gtk::Button::from_icon_name(icon);
                    button.set_tooltip_text(Some(title));
                    button.update_property(&[gtk::accessible::Property::Label(title)]);
                    let weak = Rc::downgrade(self);
                    let target = outbound::Target {
                        epoch: view.epoch,
                        endpoint: id.clone(),
                    };
                    button.connect_clicked(move |_| {
                        if let Some(panel) = weak.upgrade() {
                            panel.input_for(kind, target.clone());
                        }
                    });
                    inputs.append(&button);
                }
                strip.set_child(Some(&inputs));
                controls.append(&strip);
                let send = gtk::Button::with_label(crate::i18n::label("Send"));
                send.set_widget_name("send-device");
                let weak = Rc::downgrade(self);
                let endpoint_for_send = id.clone();
                send.connect_clicked(move |_| {
                    if let Some(panel) = weak.upgrade() {
                        panel.send_to(&endpoint_for_send);
                    }
                });
                controls.append(&send);
                root.append(&controls);
                let code = label("");
                code.set_widget_name("device-code");
                root.append(&code);
                rows.entries.insert(
                    key.clone(),
                    Entry {
                        root,
                        transfer: None,
                        endpoint: Some(projection.clone()),
                    },
                );
            }
            let root = rows.entries[&key].root.clone();
            if let Some(strip) =
                named(root.upcast_ref(), "send-strip").and_downcast::<gtk::Revealer>()
            {
                strip.set_reveal_child(
                    selection.is_none()
                        && self.model.borrow().strip_target.as_deref() == Some(id.as_str()),
                );
                strip.set_sensitive(ready);
            }
            let active = view.transfers.iter().find(|transfer| {
                transfer.direction == Direction::Outbound
                    && !transfer.phase.is_terminal()
                    && transfer.associated_endpoint.as_deref() == Some(id.as_str())
            });
            if let Some(marker) =
                named(root.upcast_ref(), "device-status").and_downcast::<gtk::Label>()
            {
                let (symbol, class) = badge_style(
                    active.map_or(Badge::Available, |transfer| Badge::Transfer(transfer.phase)),
                );
                marker.set_text(symbol);
                for old in ["success", "warning", "error", "accent", "dim-label"] {
                    marker.remove_css_class(old);
                }
                marker.add_css_class(class);
            }
            if let Some(progress) =
                named(root.upcast_ref(), "device-progress").and_downcast::<gtk::ProgressBar>()
            {
                progress.set_visible(active.is_some());
                if let Some(transfer) = active {
                    progress.set_fraction(progress_fraction(
                        transfer.details.transferred_bytes,
                        transfer.details.total_bytes,
                    ));
                    progress.set_tooltip_text(Some(phase_text(transfer.phase)));
                }
            }
            if let Some(code) = named(root.upcast_ref(), "device-code").and_downcast::<gtk::Label>()
            {
                let pin = active
                    .map(|transfer| transfer.details.pin.as_str())
                    .unwrap_or_default();
                let displayed = crate::i18n::pin_code(pin);
                code.set_visible(!displayed.is_empty());
                code.set_text(&displayed);
            }
            if let Some(send) = named(root.upcast_ref(), "send-device") {
                send.set_sensitive(ready && projection.1 && active.is_none());
            }
            pages.get_mut("nearby").unwrap().push(root);
            used.insert(key);
        }
        let discovered: BTreeSet<&str> =
            view.endpoints.iter().map(|peer| peer.id.as_str()).collect();
        for transfer in &view.transfers {
            for page in ["nearby", "incoming", "transfers"] {
                let incoming = transfer.direction == Direction::Inbound
                    && transfer.phase == Phase::AwaitingConsent;
                if page == "incoming" && !incoming {
                    continue;
                }
                if page == "nearby"
                    && !self.results.borrow().visible(transfer.transfer.id())
                    && !card_transfer_visible(
                        transfer.direction,
                        transfer.phase,
                        transfer.associated_endpoint.as_deref(),
                        &discovered,
                    )
                {
                    continue;
                }
                let root = self.cached_transfer(&mut rows, page, transfer, ready, incoming);
                if page == "nearby" {
                    if let Some(status) = named(root.upcast_ref(), "row-status") {
                        status.set_visible(transfer.phase.is_terminal());
                    }
                }
                pages.get_mut(page).unwrap().push(root);
                used.insert((page.to_owned(), transfer.transfer.id().to_owned()));
            }
        }
        for (page, title) in [
            (
                "nearby",
                "No nearby devices. Make the receiving device visible.",
            ),
            ("incoming", "No incoming requests"),
            ("transfers", "No transfers"),
        ] {
            if pages[page].is_empty() {
                let key = (
                    page.to_owned(),
                    format!("empty:{}:{title}", selection.is_some()),
                );
                let entry = rows.entries.entry(key.clone()).or_insert_with(|| {
                    let root = card();
                    if !title.is_empty() {
                        let text = label(title);
                        text.set_selectable(false);
                        root.append(&text);
                    }
                    Entry {
                        root,
                        transfer: None,
                        endpoint: None,
                    }
                });
                pages.get_mut(page).unwrap().push(entry.root.clone());
                used.insert(key);
            }
        }
        for (page, desired) in pages {
            arrange(&self.box_widget(page), &desired);
        }
        rows.entries.retain(|key, _| used.contains(key));
    }
}

#[cfg(test)]
mod metadata_label_tests {
    use super::*;
    use rqs_lib::sessions::details::{FileDetail, PayloadKind};

    #[test]
    fn given_missing_or_unsupported_metadata_when_card_is_rendered_then_no_fake_zero_files() {
        let mut details = TransferDetails::default();
        assert_eq!(details.status, DetailsStatus::Unavailable);
        assert_eq!(
            payload_label(&details),
            crate::i18n::label("Waiting for details")
        );
        assert_eq!(
            peer_title("", details.status),
            crate::i18n::label("Connecting…")
        );
        assert_eq!(peer_title("Sender", details.status), "Sender");

        details.status = DetailsStatus::Unsupported;
        assert_eq!(
            payload_label(&details),
            crate::i18n::label("Unsupported content")
        );
        assert_eq!(peer_title("", details.status), "");
        assert_eq!(peer_title("Sender", details.status), "Sender");
    }

    #[test]
    fn given_metadata_becomes_ready_when_rendering_then_actual_payload_kind_and_count_appear() {
        let mut details = TransferDetails {
            status: DetailsStatus::Ready,
            ..TransferDetails::default()
        };
        details.files = vec![FileDetail {
            name: "photo.jpg".into(),
            size: 8,
        }]
        .into();
        assert_eq!(payload_label(&details), "JPG");
        details.files = vec![
            FileDetail {
                name: "a.txt".into(),
                size: 1,
            },
            FileDetail {
                name: "b.txt".into(),
                size: 2,
            },
        ]
        .into();
        assert_eq!(payload_label(&details), crate::i18n::file_count(2));
        details.kind = PayloadKind::Text;
        details.files = std::sync::Arc::default();
        assert_eq!(payload_label(&details), crate::i18n::label("Text"));
        details.kind = PayloadKind::Url;
        assert_eq!(payload_label(&details), crate::i18n::label("Link"));
        assert_eq!(peer_title("", details.status), "");
    }
}

#[cfg(test)]
pub(super) fn verify_keyed_layout() {
    // Arrange
    let container = gtk::Box::new(gtk::Orientation::Vertical, 0);
    let first = card();
    let expanded = gtk::Expander::new(Some("Files"));
    expanded.set_child(Some(&label("Untrusted filename")));
    expanded.set_expanded(true);
    first.append(&expanded);
    let second = card();
    // Act / Assert: identical projection does not detach or replace either row.
    arrange(&container, &[first.clone(), second.clone()]);
    let child = expanded.child().unwrap();
    arrange(&container, &[first.clone(), second.clone()]);
    assert_eq!(container.first_child(), Some(first.clone().upcast()));
    assert_eq!(expanded.child(), Some(child.clone()));
    assert!(expanded.is_expanded());
    arrange(&container, &[second.clone(), first.clone()]);
    assert_eq!(container.first_child(), Some(second.clone().upcast()));
    assert_eq!(container.last_child(), Some(first.clone().upcast()));
    assert!(expanded.is_expanded());
    assert_eq!(expanded.child(), Some(child));
    arrange(&container, std::slice::from_ref(&first));
    assert!(second.parent().is_none());
    assert_eq!(container.first_child(), Some(first.clone().upcast()));

    // Reparenting is required when a completed operation moves between the
    // unlinked activity drawer and an authoritative same-endpoint group.
    let activity = gtk::Box::new(gtk::Orientation::Vertical, 0);
    arrange(&activity, std::slice::from_ref(&first));
    assert!(container.first_child().is_none());
    assert_eq!(activity.first_child(), Some(first.clone().upcast()));
    arrange(&container, std::slice::from_ref(&first));
    assert!(activity.first_child().is_none());
    assert_eq!(container.first_child(), Some(first.upcast()));

    // Arrange: history content actions exist below the old disclosure.
    let details = card();
    let clicks = Rc::new(Cell::new(0));
    for name in [
        "decline",
        "accept",
        "review-text",
        "cancel",
        "cancel-reveal",
    ] {
        let action = gtk::Button::with_label(name);
        action.set_widget_name(name);
        let clicks = clicks.clone();
        action.connect_clicked(move |_| clicks.set(clicks.get() + 1));
        details.append(&action);
    }
    details.append(&label("Duplicate detail metadata"));
    let history = transfer_row(
        "<Mercury>",
        rqs_lib::service::client::state::DeviceKind::Phone,
        Phase::Finished,
        &details,
    );
    // Act: reuse the original controls, removing the disclosure and detail body.
    let heading = history.first_child().and_downcast::<gtk::Box>().unwrap();
    // Assert: controls are directly clickable and no hidden duplicate body remains.
    assert!(named(history.upcast_ref(), "transfer-details").is_none());
    assert_eq!(history.observe_children().n_items(), 1);
    assert_eq!(history.first_child(), Some(heading.clone().upcast()));
    assert!(details.parent().is_none());
    for name in [
        "decline",
        "accept",
        "review-text",
        "cancel",
        "cancel-reveal",
    ] {
        let action = named(history.upcast_ref(), name)
            .and_downcast::<gtk::Button>()
            .unwrap();
        assert_eq!(action.parent(), Some(heading.clone().upcast()));
        action.emit_clicked();
    }
    assert_eq!(clicks.get(), 5);
    assert_eq!(details.observe_children().n_items(), 1);

    let live = ["ep-1", "ep-2"].into_iter().collect::<BTreeSet<_>>();
    assert_eq!(
        bound_current_endpoint(Direction::Outbound, Some("ep-1"), &live),
        Some("ep-1")
    );
    assert_eq!(
        bound_current_endpoint(Direction::Outbound, Some("ep-3"), &live),
        None
    );
    assert_eq!(
        bound_current_endpoint(Direction::Outbound, Some("Mercury"), &live),
        None
    );
    assert_eq!(
        bound_current_endpoint(Direction::Inbound, Some("ep-1"), &live),
        None
    );
    assert_eq!(
        bound_current_endpoint(Direction::Outbound, None, &live),
        None
    );
    let untrusted_header = section_title("<Mercury> is not a verified identity");
    assert_eq!(
        untrusted_header
            .first_child()
            .unwrap()
            .downcast::<gtk::Label>()
            .unwrap()
            .text(),
        "<Mercury> is not a verified identity"
    );
}

#[cfg(test)]
mod badge_tests {
    use super::*;
    #[test]
    fn given_each_transfer_and_discovery_state_when_badged_then_shape_and_color_distinguish_outcomes(
    ) {
        for (phase, expected) in [
            (Phase::Queued, ("…", "warning")),
            (Phase::Active, ("●", "accent")),
            (Phase::AwaitingConsent, ("?", "warning")),
            (Phase::Receiving, ("↓", "accent")),
            (Phase::Sending, ("↑", "accent")),
            (Phase::CancelRequested, ("…", "warning")),
            (Phase::Finished, ("✓", "success")),
            (Phase::Rejected, ("−", "dim-label")),
            (Phase::Cancelled, ("−", "dim-label")),
            (Phase::Failed, ("!", "error")),
            (Phase::Disconnected, ("!", "error")),
        ] {
            assert_eq!(badge_style(Badge::Transfer(phase)), expected);
        }
        assert_eq!(badge_style(Badge::Available), ("●", "success"));
        assert_eq!(badge_style(Badge::Unavailable), ("−", "dim-label"));
    }
}

#[cfg(test)]
mod focus_scroll_tests {
    use super::reveal_vertical_interval;

    #[test]
    fn given_offscreen_or_visible_consent_row_when_requested_then_scroll_moves_minimally() {
        for (start, end, offset, page, expected) in [
            (0.0, 40.0, 400.0, 200.0, Some(0.0)),
            (620.0, 680.0, 400.0, 200.0, Some(480.0)),
            (450.0, 490.0, 400.0, 200.0, None),
            (350.0, 420.0, 400.0, 200.0, Some(350.0)),
            (600.0, 900.0, 100.0, 200.0, Some(600.0)),
            (780.0, 830.0, 500.0, 200.0, Some(630.0)),
            (120.0, 140.0, 0.0, 0.0, None),
            (120.0, 120.0, 0.0, 200.0, None),
        ] {
            assert_eq!(
                reveal_vertical_interval(start, end, offset, page, 0.0, 1000.0),
                expected,
                "row={start}..{end}, viewport={offset}..{}",
                offset + page
            );
        }
        // The scroll target cannot leave the adjustment's legal range.
        assert_eq!(
            reveal_vertical_interval(980.0, 990.0, 200.0, 300.0, 0.0, 1000.0),
            Some(690.0)
        );
    }
}

#[cfg(test)]
mod device_contract_tests {
    use super::*;
    #[test]
    fn given_transfer_direction_phase_and_association_when_projected_then_card_contains_only_live_peer_rows(
    ) {
        // Arrange / Act / Assert
        let peers = ["current"].into_iter().collect();
        for phase in [
            Phase::Finished,
            Phase::Rejected,
            Phase::Cancelled,
            Phase::Failed,
            Phase::Disconnected,
        ] {
            for direction in [Direction::Inbound, Direction::Outbound] {
                assert!(!card_transfer_visible(direction, phase, None, &peers));
            }
        }
        for phase in [
            Phase::Queued,
            Phase::Active,
            Phase::AwaitingConsent,
            Phase::Sending,
            Phase::Receiving,
            Phase::CancelRequested,
        ] {
            assert!(!card_transfer_visible(
                Direction::Outbound,
                phase,
                Some("current"),
                &peers
            ));
            assert!(card_transfer_visible(
                Direction::Outbound,
                phase,
                Some("vanished"),
                &peers
            ));
            assert!(card_transfer_visible(
                Direction::Outbound,
                phase,
                None,
                &peers
            ));
            assert!(card_transfer_visible(
                Direction::Inbound,
                phase,
                Some("current"),
                &peers
            ));
        }
    }

    #[test]
    fn given_all_peer_direction_phase_memberships_when_projected_then_only_authoritative_live_rows_are_hidden(
    ) {
        // Arrange: distinct current and stale endpoint identities.
        let phases = [
            Phase::Queued,
            Phase::Active,
            Phase::AwaitingConsent,
            Phase::Receiving,
            Phase::Sending,
            Phase::CancelRequested,
            Phase::Finished,
            Phase::Rejected,
            Phase::Cancelled,
            Phase::Failed,
            Phase::Disconnected,
        ];
        let memberships: [BTreeSet<&str>; 3] = [
            BTreeSet::new(),
            ["current"].into_iter().collect(),
            ["stale"].into_iter().collect(),
        ];
        for discovered in &memberships {
            for association in [None, Some("current"), Some("stale")] {
                for direction in [Direction::Inbound, Direction::Outbound] {
                    for phase in phases {
                        // Act: use the independently defined ownership contract.
                        let expected_bound = if direction == Direction::Outbound {
                            association.filter(|id| discovered.contains(id))
                        } else {
                            None
                        };
                        let expected_visible = !phase.is_terminal() && expected_bound.is_none();

                        // Assert: exact identity and complete visibility decision.
                        assert_eq!(
                            bound_current_endpoint(direction, association, discovered),
                            expected_bound
                        );
                        assert_eq!(
                            card_transfer_visible(direction, phase, association, discovered),
                            expected_visible,
                            "{direction:?} {phase:?} {association:?} {discovered:?}"
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn given_all_progress_extremes_when_projected_then_fraction_is_exact_finite_and_monotonic() {
        // Arrange / Act / Assert: independently compute the saturated ratio.
        let boundaries = [0, 1, 2, 3, 254, 255, 65_535, u64::MAX];
        for total in boundaries {
            let mut previous = 0.0;
            for done in boundaries {
                let expected = if total == 0 {
                    0.0
                } else {
                    (done.min(total) as f64) / (total as f64)
                };
                let actual = progress_fraction(done, total);
                assert_eq!(actual, expected, "done={done} total={total}");
                assert!(actual.is_finite());
                assert!((0.0..=1.0).contains(&actual));
                assert!(actual >= previous, "progress decreased for total={total}");
                previous = actual;
            }
        }
    }

    #[test]
    fn given_byte_progress_when_unknown_or_overrun_then_bar_stays_finite_and_bounded() {
        for (done, total, expected) in [
            (0, 0, 0.0),
            (1, 0, 0.0),
            (0, 1, 0.0),
            (1, 2, 0.5),
            (2, 2, 1.0),
            (3, 2, 1.0),
            (u64::MAX, u64::MAX, 1.0),
        ] {
            assert_eq!(progress_fraction(done, total), expected);
        }
    }
}

#[cfg(test)]
mod untrusted_payload_label_tests {
    use super::*;
    use rqs_lib::service::client::state::FileDetail;
    use rqs_lib::sessions::details::PayloadKind;

    fn summary(kind: PayloadKind, names: &[&str]) -> String {
        // These label-sanitization fixtures exercise metadata that has
        // arrived. Unavailable metadata has a separate privacy-safe label.
        let details = TransferDetails {
            kind,
            status: DetailsStatus::Ready,
            files: names
                .iter()
                .map(|name| FileDetail {
                    name: (*name).to_owned(),
                    size: 1,
                })
                .collect::<Vec<_>>()
                .into(),
            ..Default::default()
        };
        payload_label(&details)
    }

    #[test]
    fn given_untrusted_single_file_extensions_when_projected_then_only_bounded_ascii_label_is_displayed(
    ) {
        // Arrange: all the raw names originate at the untrusted remote peer.
        for (filename, expected) in [
            ("receipt.txt", "TXT"),
            ("receipt.TXT", "TXT"),
            ("archive.tar.gz", "GZ"),
            ("invoice.x1", "X1"),
            ("a.0", "0"),
            ("a.123456789012", "123456789012"),
            ("a.1234567890123", "File"),
            ("a.", "File"),
            ("filename", "File"),
            (".hidden", "HIDDEN"),
            ("nested/path/<b>private</b>.txt", "TXT"),
            ("<img src=x>.svg", "SVG"),
            ("nested/path/<script>.bad!", "File"),
            ("untrusted.é", "File"),
            ("untrusted.中文", "File"),
            ("untrusted.💣", "File"),
            ("name._", "File"),
            ("name.dash-1", "File"),
            ("name.space ext", "File"),
            ("name.new\nline", "File"),
        ] {
            let actual = summary(PayloadKind::Files, &[filename]);
            if expected == "File" {
                assert_eq!(actual, crate::i18n::label("File"), "filename={filename:?}");
            } else {
                assert_eq!(actual, expected, "filename={filename:?}");
                assert!(
                    actual.is_ascii(),
                    "remote extension must not escape as Unicode"
                );
                assert!(actual.bytes().all(|byte| byte.is_ascii_alphanumeric()));
                assert!(!actual.contains('/') && !actual.contains('<') && !actual.contains('>'));
            }
        }
    }

    #[test]
    fn given_file_manifest_cardinality_and_payload_kind_when_projected_then_summary_never_echoes_remote_paths(
    ) {
        // Arrange: a multi-file payload or non-file resource never exposes
        // untrusted file metadata as the summary.
        for (kind, names, expected) in [
            (PayloadKind::Files, vec![], "0 files"),
            (PayloadKind::Files, vec!["private.bin"], "BIN"),
            (
                PayloadKind::Files,
                vec!["<secret>.txt", "private/location.html"],
                "2 files",
            ),
            (
                PayloadKind::Files,
                vec!["x.bin", "y.bin", "z.bin"],
                "3 files",
            ),
            (PayloadKind::Text, vec![], "Text"),
            (PayloadKind::Text, vec!["<secret>.txt"], "Text"),
            (PayloadKind::Url, vec![], "Link"),
            (PayloadKind::Url, vec!["private/location.html"], "Link"),
        ] {
            let actual = summary(kind, &names);
            let localized_expected = match expected {
                "2 files" => crate::i18n::file_count(2),
                "3 files" => crate::i18n::file_count(3),
                "Text" => crate::i18n::label("Text").to_owned(),
                "Link" => crate::i18n::label("Link").to_owned(),
                _ => expected.to_owned(),
            };
            assert_eq!(
                actual, localized_expected,
                "kind={kind:?} names={names:?}"
            );
            assert!(!actual.contains('<') && !actual.contains('/') && !actual.contains('>'));
        }
    }
}
