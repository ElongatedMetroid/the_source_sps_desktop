use std::process::{Child, Command};

use futures::executor::block_on;

use crate::{config::Config, grades_client::GradesClient};

pub struct App {
    web_driver_child: Child,
    grades: Option<GradesClient>,
    /// Should be None unless a config was not provided
    config: Option<Config>,
}

impl App {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Result<Self, Box<dyn std::error::Error>> {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.

        let mut app = Self {
            web_driver_child: Command::new("geckodriver").spawn().unwrap(),
            grades: None,
            config: None,
        };

        // if config does not exist, create one
        match Config::load() {
            Ok(config) => {
                app.grades = Some(block_on(GradesClient::start_client(config)).unwrap());
            }
            // TODO: Handle config error (instead of just assuming the file does not exist)
            Err(_) => {
                app.config = Some(Config::default());
            },
        };

        Ok(app)
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if self.grades.is_none() {
                ui.text_edit_singleline(&mut self.config.as_mut().unwrap().username);
                ui.text_edit_singleline(&mut self.config.as_mut().unwrap().password);
                if ui.button("Log In").clicked() {
                    Config::write(self.config.as_ref().unwrap()).unwrap();

                    self.grades =
                        Some(block_on(GradesClient::start_client(self.config.take().unwrap())).unwrap());
                }
            } else {
                ui.label("Grades");
                //ui.label(text)
            }
        });
    }
    fn on_close_event(&mut self) -> bool {
        self.web_driver_child.kill().unwrap();

        true
    }
}
