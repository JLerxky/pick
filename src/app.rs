use eframe::{
    egui::{self, *},
    epi,
};

#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))]
pub struct MainApp {
    pub app_size: [f32; 2],

    msg_send: String,

    bullet_queue: Vec<Bullet>,
    lines: Vec<Line>,
    stroke: Stroke,
}

pub struct Line {
    pub pos: Vec<Pos2>,
    pub stroke: Stroke,
}

impl MainApp {
    pub fn ui_control(&mut self, ui: &mut egui::Ui) -> egui::Response {
        ui.horizontal(|ui| {
            egui::stroke_ui(ui, &mut self.stroke, "笔尖");
            ui.separator();
            if ui.button("清空").clicked() {
                self.lines.clear();
            }
        })
        .response
    }

    pub fn ui_content(&mut self, ui: &mut Ui) -> egui::Response {
        let (mut response, painter) =
            ui.allocate_painter(ui.available_size_before_wrap_finite(), Sense::drag());

        let to_screen = emath::RectTransform::from_to(
            Rect::from_min_size(Pos2::ZERO, response.rect.square_proportions()),
            response.rect,
        );
        let from_screen = to_screen.inverse();

        if self.lines.is_empty() {
            self.lines.push(Line {
                pos: vec![],
                stroke: self.stroke,
            });
        }

        let current_line = self.lines.last_mut().unwrap();

        if let Some(pointer_pos) = response.interact_pointer_pos() {
            let canvas_pos = from_screen * pointer_pos;
            if current_line.pos.last() != Some(&canvas_pos) {
                current_line.pos.push(canvas_pos);
                response.mark_changed();
            }
        } else if !current_line.pos.is_empty() {
            self.lines.push(Line {
                pos: vec![],
                stroke: self.stroke,
            });
            response.mark_changed();
        }

        let mut shapes = vec![];
        for line in &self.lines {
            if line.pos.len() >= 2 {
                let points: Vec<Pos2> = line.pos.iter().map(|p| to_screen * *p).collect();
                shapes.push(egui::Shape::line(points, line.stroke));
            }
        }
        painter.extend(shapes);

        response
    }
}

impl Default for MainApp {
    fn default() -> Self {
        Self {
            app_size: [800.0, 450.0],
            msg_send: "".to_owned(),
            bullet_queue: Vec::new(),
            lines: Default::default(),
            stroke: Stroke::new(1.0, Color32::LIGHT_BLUE),
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
        let mut fonts = egui::FontDefinitions::default();
        fonts.font_data.insert(
            "youzai".to_owned(),
            std::borrow::Cow::Borrowed(include_bytes!("../docs/youzai.ttf")),
        );
        fonts
            .fonts_for_family
            .get_mut(&egui::FontFamily::Proportional)
            .unwrap()
            .insert(0, "youzai".to_owned());
        fonts
            .family_and_size
            .get_mut(&egui::TextStyle::Heading)
            .unwrap()
            .1 = 28.0;
        ctx.set_fonts(fonts);
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
        egui::CentralPanel::default().show(ctx, |ui| {
            self.ui_control(ui);
            Frame::dark_canvas(ui.style())
                .fill(egui::Color32::WHITE)
                .show(ui, |ui| {
                    self.ui_content(ui);
                });
        });

        let Self {
            app_size,
            msg_send,
            bullet_queue,
            ..
        } = self;

        let input = ctx.input();

        // 按键事件
        if !input.modifiers.ctrl && input.key_released(egui::Key::Enter) {
            bullet_queue.push(Bullet {
                id: rand::Rng::gen_range(&mut rand::thread_rng(), 0..usize::MAX),
                speed: rand::Rng::gen_range(&mut rand::thread_rng(), 100..200) as f32,
                time: input.time,
                msg: msg_send.to_string(),
                height: rand::Rng::gen_range(&mut rand::thread_rng(), 0..app_size[1] as i32 - 100)
                    as f32,
                color: [
                    rand::Rng::gen_range(&mut rand::thread_rng(), 0..255),
                    rand::Rng::gen_range(&mut rand::thread_rng(), 0..255),
                    rand::Rng::gen_range(&mut rand::thread_rng(), 0..255),
                ],
            });

            *msg_send = String::default();
        }

        // egui::SidePanel::right("right_panel")
        //     .max_width(1024.0)
        //     .resizable(true)
        //     .show(ctx, |ui| {
        //         ui.with_layout(egui::Layout::default().with_cross_justify(true), |ui| {
        //             ui.heading("");
        //         });
        //     });

        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.with_layout(egui::Layout::default().with_cross_justify(true), |ui| {
                ui.add(egui::TextEdit::multiline(msg_send).frame(false));
            });
        });

        show_bullet_screen(bullet_queue, input, app_size, ctx);

        ctx.request_repaint();
    }
}

fn show_bullet_screen(
    bullet_queue: &mut Vec<Bullet>,
    input: &egui::InputState,
    app_size: &mut [f32; 2],
    ctx: &egui::CtxRef,
) {
    *bullet_queue = bullet_queue
        .iter()
        .filter_map(|bullet| {
            let pos = egui::Pos2::new(
                (input.time - bullet.time) as f32 * bullet.speed - 400.0,
                bullet.height,
            );
            if pos.x < app_size[0] {
                egui::Window::new(format!("dm{}", bullet.id))
                    .id(egui::Id::new(format!("dm{}", bullet.id)))
                    .resizable(false)
                    .collapsible(false)
                    .title_bar(false)
                    .scroll(false)
                    .fixed_pos(pos)
                    .frame(egui::Frame::none())
                    .enabled(true)
                    .show(ctx, |ui| {
                        ui.with_layout(egui::Layout::default().with_cross_justify(true), |ui| {
                            ui.add(egui::Label::new(&bullet.msg).strong().heading().text_color(
                                egui::Color32::from_rgb(
                                    bullet.color[0],
                                    bullet.color[1],
                                    bullet.color[2],
                                ),
                            ));
                        });
                    });
                return Some(bullet.clone());
            } else {
                return None;
            }
        })
        .collect();
}

#[derive(Clone)]
pub struct Bullet {
    id: usize,
    speed: f32,
    time: f64,
    msg: String,
    height: f32,
    color: [u8; 3],
}
