use eframe::{egui, epi};

#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))]
pub struct MainApp {
    pub app_size: [f32; 2],

    label: String,
    msg_send: String,
    #[cfg_attr(feature = "persistence", serde(skip))]
    value: f32,
}

impl Default for MainApp {
    fn default() -> Self {
        Self {
            app_size: [800.0, 450.0],
            label: "Hello JLer!".to_owned(),
            msg_send: "".to_owned(),
            value: 1.0,
        }
    }
}

impl epi::App for MainApp {
    fn name(&self) -> &str {
        "GG弹幕"
    }

    fn setup(
        &mut self,
        ctx: &egui::CtxRef,
        _frame: &mut epi::Frame<'_>,
        _storage: Option<&dyn epi::Storage>,
    ) {
        ctx.set_visuals(egui::Visuals::dark());
        let mut style: egui::Style = (*ctx.style()).clone();
        // style.spacing.window_padding = [7.0, 8.0].into();
        style.visuals.window_shadow = egui::epaint::Shadow::small_light();
        style.visuals.window_corner_radius = 11.2;
        ctx.set_style(style);
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
            app_size,
            label,
            msg_send,
            value,
        } = self;

        let input = ctx.input();

        // 按键事件
        if !input.modifiers.ctrl && input.key_released(egui::Key::Enter) {
            *msg_send = String::default();
        }

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
            ui.heading(format!("{}{}", label.as_str(), msg_send.as_str()));
            ui.end_row();
            ui.label(value.to_string());
        });

        egui::Window::new("Window1")
            .id(egui::Id::new("Window1"))
            .resizable(false)
            .collapsible(false)
            .title_bar(false)
            .scroll(false)
            .fixed_size(egui::Vec2::new(570.0, 80.0))
            .enabled(true)
            .anchor(egui::Align2::LEFT_BOTTOM, egui::Vec2::new(10.0, -10.0))
            .show(ctx, |ui| {
                ui.with_layout(egui::Layout::default().with_cross_justify(true), |ui| {
                    ui.add(egui::TextEdit::multiline(msg_send).frame(false));
                });
            });

        let pos = egui::Pos2::new(input.time as f32 * 200.0, 100.0);
        if pos.x < app_size[0] {
            egui::Window::new("dm")
                .id(egui::Id::new("dm"))
                .resizable(false)
                .collapsible(false)
                .title_bar(false)
                .scroll(false)
                .fixed_pos(pos)
                .frame(egui::Frame::none())
                .enabled(true)
                .show(ctx, |ui| {
                    ui.with_layout(egui::Layout::default().with_cross_justify(true), |ui| {
                        ui.label("label");
                    });
                });
        }

        ctx.request_repaint();
    }
}
