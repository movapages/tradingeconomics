pub struct ErrorDisplay<'a> {
    pub error: Option<&'a str>,
}

impl<'a> ErrorDisplay<'a> {
    pub fn ui(&self, ui: &mut egui::Ui) {
        if let Some(msg) = self.error {
            ui.colored_label(egui::Color32::RED, msg);
        }
    }
}
