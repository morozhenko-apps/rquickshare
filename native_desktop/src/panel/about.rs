//! Compact native GTK About layout, matching the single-page RepoTicker hierarchy.
//! All links and attribution are trusted literals; no peer or transfer input is used.
use super::*;

const WEBSITE: &str = "https://morozhenko.com";
const PRODUCT: &str = "rQuickShare Native";
const FORK: &str = "Martichou/rquickshare";

pub(super) struct About {
    pub(super) window: gtk::Window,
    pub(super) icon: gtk::Image,
    pub(super) name: gtk::Label,
    pub(super) version: gtk::Label,
    pub(super) author: gtk::Label,
    pub(super) upstream: gtk::Label,
    pub(super) website: gtk::LinkButton,
    pub(super) license: gtk::Label,
}

impl About {
    pub(super) fn new(parent: &gtk::Window) -> Self {
        let window = gtk::Window::builder()
            .transient_for(parent)
            .application(&parent.application().unwrap())
            .default_width(410)
            .default_height(302)
            .resizable(false)
            .build();
        window.set_icon_name(Some(crate::tray::ICON_NAME));
        super::chrome::close_only_titlebar(&window);
        window.set_hide_on_close(true);

        let body = gtk::Box::new(gtk::Orientation::Vertical, 7);
        body.set_halign(gtk::Align::Center);
        body.set_valign(gtk::Align::Center);
        body.set_hexpand(true);
        body.set_vexpand(true);
        body.set_margin_top(14);
        body.set_margin_bottom(14);
        body.set_margin_start(16);
        body.set_margin_end(16);

        let icon = gtk::Image::from_icon_name(crate::tray::ICON_NAME);
        icon.set_pixel_size(72);
        icon.set_size_request(72, 72);
        body.append(&icon);

        let name = gtk::Label::new(Some(PRODUCT));
        name.add_css_class("rqs-about-name");
        body.append(&name);

        let version = detail_label();
        body.append(&version);
        let author = detail_label();
        body.append(&author);

        let website = gtk::LinkButton::with_label(WEBSITE, "morozhenko.com ↗");
        website.add_css_class("rqs-about-website");
        website.set_tooltip_text(Some(WEBSITE));
        body.append(&website);

        let upstream = detail_label();
        body.append(&upstream);
        let license = detail_label();
        body.append(&license);

        window.set_child(Some(&body));
        let about = Self {
            window,
            icon,
            name,
            version,
            author,
            upstream,
            website,
            license,
        };
        about.retranslate();
        about
    }

    pub(super) fn retranslate(&self) {
        self.window
            .set_title(Some(crate::i18n::about_label("About rQuickShare Native")));
        self.icon.set_pixel_size(72);
        self.name.set_text(PRODUCT);
        self.website.set_tooltip_text(Some(WEBSITE));
        self.version.set_text(&format!(
            "{} {}",
            crate::i18n::about_label("Version"),
            env!("CARGO_PKG_VERSION")
        ));
        let author_name = if crate::i18n::active() == "ru" {
            "Станислав Мосейко"
        } else {
            "Stanislav Moseyko"
        };
        self.author.set_text(&format!(
            "{} {} (morozhenko)",
            crate::i18n::about_label("Author:"),
            author_name
        ));
        self.upstream.set_text(&format!(
            "{} {FORK}",
            crate::i18n::about_label("Based on the fork")
        ));
        self.license
            .set_text(crate::i18n::about_label("Open source · GNU GPLv3"));
    }
}

fn detail_label() -> gtk::Label {
    let text = gtk::Label::new(None);
    text.set_wrap(true);
    text.set_wrap_mode(gtk::pango::WrapMode::WordChar);
    text.set_justify(gtk::Justification::Center);
    text.set_xalign(0.5);
    text.set_max_width_chars(48);
    text.set_selectable(false);
    text.add_css_class("rqs-about-muted");
    text
}
