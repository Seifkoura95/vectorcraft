//! Help → About VectorCraft.

use crate::VectorcraftApp;
use crate::theme;

/// The About window, while `UiState::about` is set.
pub(super) fn show(app: &mut VectorcraftApp, ctx: &egui::Context) {
    if !app.ui.about {
        return;
    }
    let mut open = true;
    egui::Window::new(crate::branded(tl!("About VectorCraft")))
        .collapsible(false)
        .resizable(false)
        .open(&mut open)
        .pivot(egui::Align2::CENTER_CENTER)
        .default_pos(ctx.content_rect().center())
        .frame(egui::Frame::window(&ctx.global_style()).inner_margin(egui::Margin::same(18)))
        .show(ctx, |ui| {
            // Tabs: About · Contributors · Models (the credits are compiled in, see `credits`).
            let tab_id = egui::Id::new("about_tab");
            let mut tab = ui.data_mut(|d| d.get_temp::<u8>(tab_id)).unwrap_or(0);
            ui.horizontal(|ui| {
                for (i, label) in [tl!("About"), tl!("Contributors"), tl!("Models")].into_iter().enumerate() {
                    let i = i as u8;
                    if ui.selectable_label(tab == i, label).clicked() {
                        tab = i;
                    }
                }
            });
            ui.data_mut(|d| d.insert_temp(tab_id, tab));
            ui.separator();
            match tab {
                1 => {
                    ui.set_width(660.0);
                    crate::credits::contributors_ui(ui);
                }
                2 => {
                    ui.set_width(660.0);
                    crate::credits::models_ui(ui);
                }
                _ => about_tab(app, ui),
            }
        });
    app.ui.about = open;
}

/// The About tab: mark, version, community links, licences and the GPU in use.
fn about_tab(app: &mut VectorcraftApp, ui: &mut egui::Ui) {
    ui.set_width(380.0);
    ui.horizontal(|ui| {
        let (r, _) = ui.allocate_exact_size(egui::vec2(44.0, 44.0), egui::Sense::hover());
        crate::brand::paint_mark(ui, r);
        ui.vertical(|ui| {
            ui.label(egui::RichText::new(crate::APP_NAME).font(theme::semibold(22.0)));
            if cfg!(feature = "simplehub") {
                ui.label(format!("Version {}", env!("CARGO_PKG_VERSION")));
            } else {
                ui.label(crate::i18n::fmt(
                    tl!("Version {version} — open-source vector illustration in pure Rust."),
                    &[("version", env!("CARGO_PKG_VERSION"))],
                ));
            }
        });
    });
    ui.add_space(12.0);
    if cfg!(feature = "simplehub") {
        // SimpleHub build. VectorCraft is MIT OR Apache-2.0; the licence asks for attribution,
        // and the ArtCraft community links are left out (the ArtCraft marks are not open source).
        ui.label("Vector illustration for SimpleHub. Paths, type, gradients and effects, with SVG, PDF and bitmap export.");
        ui.add_space(12.0);
        ui.label(
            egui::RichText::new(
                "Built on VectorCraft, an open source vector illustration app by the ArtCraft Team and the VectorCraft contributors, used under the MIT or Apache 2.0 licence.",
            )
            .size(11.0),
        );
        ui.label(
            egui::RichText::new(
                "Not affiliated with or endorsed by Adobe, Microsoft, Autodesk or Avid. Other product names are trademarks of their respective owners.", // brand-ok: the non-affiliation notice names the vendors on purpose
            )
            .size(11.0),
        );
        ui.hyperlink_to("github.com/storytold/vectorcraft", "https://github.com/storytold/vectorcraft");
    } else {
        crate::community::links(app, ui);
        ui.add_space(12.0);
        ui.label(
            egui::RichText::new(tl!(
                "Part of ArtCraft. MIT OR Apache-2.0. Fonts: Source Sans 3, Inter, JetBrains Mono (OFL). Icons: Lucide (ISC) + VectorCraft."
            ))
            .size(11.0),
        );
    }
    // The GPU the window renders with, for bug reports (Preferences › Performance picks it).
    if let Some(adapter) = &app.graphics_adapter {
        ui.add_space(6.0);
        ui.label(egui::RichText::new(crate::i18n::fmt(tl!("Graphics: {adapter}"), &[("adapter", adapter.as_str())])).size(11.0));
    }
}
