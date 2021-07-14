use eframe::{egui, epi};

#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))]
pub struct TemplateApp {
    label: String,
    msg_send: String,

    #[cfg_attr(feature = "persistence", serde(skip))]
    value: f32,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            label: "Hello JLer!".to_owned(),
            msg_send: "".to_owned(),
            value: 1.0,
        }
    }
}

impl epi::App for TemplateApp {
    fn name(&self) -> &str {
        "pick"
    }

    #[cfg(feature = "persistence")]
    fn load(&mut self, storage: &dyn epi::Storage) {
        *self = epi::get_value(storage, epi::APP_KEY).unwrap_or_default()
    }

    #[cfg(feature = "persistence")]
    fn save(&mut self, storage: &mut dyn epi::Storage) {
        epi::set_value(storage, epi::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::CtxRef, _frame: &mut epi::Frame<'_>) {
        let Self {
            label,
            msg_send,
            value,
        } = self;

        egui::SidePanel::right("right_panel")
            .max_width(1024.0)
            .resizable(true)
            .show(ctx, |ui| {
                ui.with_layout(egui::Layout::default().with_cross_justify(true), |ui| {
                    ui.heading("Side Panel");
                    ui.label(value.to_string());
                });
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(label.as_str());
            ui.label(value.to_string());
        });

        egui::Window::new("Window1")
            .id(egui::Id::new("Window1"))
            .resizable(false)
            .collapsible(false)
            .title_bar(false)
            .scroll(false)
            .auto_sized()
            .enabled(true)
            .anchor(egui::Align2::LEFT_BOTTOM, egui::Vec2::ZERO)
            .show(ctx, |ui| {
                ui.with_layout(egui::Layout::default().with_cross_justify(true), |ui| {
                    ui.add(egui::TextEdit::multiline(msg_send).desired_width(6000.0));
                });
            });
    }
}
