use eframe::{egui, epi};

#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))]
pub struct TemplateApp {
    label: String,

    #[cfg_attr(feature = "persistence", serde(skip))]
    value: f32,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            label: "Hello JLer!".to_owned(),
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
        let Self { label, value } = self;

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading(label.clone());
            ui.label(value.to_string());
            egui::warn_if_debug_build(ui);
        });
    }
}
