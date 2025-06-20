pub mod api_selector;
pub mod chart;
pub mod console;
pub mod error_display;
pub mod fetch_button;
pub mod param_controls;

use crate::gui::{
    api_selector::ApiSelector, chart::Chart, console::Console, error_display::ErrorDisplay,
    fetch_button::FetchButton, param_controls::ParamControls,
};

use eframe::egui;

pub struct DataDandy {
    pub log: Vec<String>,
    pub show_console: bool,
    pub api_selector: ApiSelector,
    pub param_controls: ParamControls,
    pub fetch_button: FetchButton,
    pub error: Option<String>,
    pub chart: Chart,
}

impl DataDandy {
    pub fn new() -> Self {
        Self {
            log: vec!["Welcome to Data Dandy!".to_string()],
            show_console: false,
            api_selector: ApiSelector {
                selected: 0,
                options: vec![
                    "Crude Oil Price".to_string(),
                    "Gold Price".to_string(),
                    "GDP by Country".to_string(),
                ],
            },
            param_controls: ParamControls,
            fetch_button: FetchButton { loading: false },
            error: None,
            chart: Chart,
        }
    }
}

impl eframe::App for DataDandy {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Top menu bar for navigation and toggling console
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("☰ Menu").clicked() {
                    // Add menu logic here
                }
                ui.separator();
                if ui.button("🖥 Console").clicked() {
                    self.show_console = !self.show_console;
                }
            });
        });

        // Main content
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(10.0);
            ui.heading(
                egui::RichText::new("DataDandy Dashboard")
                    .size(28.0)
                    .strong(),
            );
            ui.add_space(20.0);

            egui::Grid::new("form_grid")
                .num_columns(4)
                .spacing([12.0, 12.0])
                .striped(false)
                .show(ui, |ui| {
                    // Row 1: API
                    ui.label(egui::RichText::new("API").strong().size(18.0));
                    ui.vertical(|ui| {
                        ui.label(egui::RichText::new("API endpoint").weak().italics());
                        self.api_selector.ui(ui);
                    });
                    ui.allocate_ui(egui::Vec2::new(200.0, 0.0), |ui| {}); // Spacer column
                    Console { log: &self.log }.ui(ui); // Console in col 4
                    ui.end_row();

                    // Row 2: Params
                    ui.label(egui::RichText::new("Params").strong().size(18.0));
                    self.param_controls.ui(ui);
                    ui.allocate_ui(egui::Vec2::new(200.0, 0.0), |ui| {});
                    ui.label(""); // Empty for console (already shown above)
                    ui.end_row();

                    // Row 3: Button
                    ui.label("");
                    if self.fetch_button.ui(ui) {
                        // Fetch logic
                    }
                    ui.allocate_ui(egui::Vec2::new(200.0, 0.0), |ui| {});
                    ui.label("");
                    ui.end_row();

                    // Row 4: Error
                    ui.label("");
                    ErrorDisplay {
                        error: self.error.as_deref(),
                    }
                    .ui(ui);
                    ui.allocate_ui(egui::Vec2::new(200.0, 0.0), |ui| {});
                    ui.label("");
                    ui.end_row();

                    // Row 5: Chart
                    ui.label("");
                    self.chart.ui(ui);
                    ui.allocate_ui(egui::Vec2::new(200.0, 0.0), |ui| {});
                    ui.label("");
                    ui.end_row();
                });
        });

        // Right console panel, only if toggled on
        if self.show_console {
            egui::SidePanel::right("console_panel")
                .resizable(false)
                .min_width(1000.0)
                .default_width(1000.0)
                .show(ctx, |ui| {
                    ui.heading("Console Log");
                    Console { log: &self.log }.ui(ui);
                });
        }
    }
}
