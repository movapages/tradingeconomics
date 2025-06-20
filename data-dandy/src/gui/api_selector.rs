pub struct ApiSelector {
    pub selected: usize,
    pub options: Vec<String>,
}

impl ApiSelector {
    pub fn ui(&mut self, ui: &mut egui::Ui) {
        egui::ComboBox::from_label("")
            .selected_text(&self.options[self.selected])
            .show_ui(ui, |ui| {
                for (i, name) in self.options.iter().enumerate() {
                    ui.selectable_value(&mut self.selected, i, name);
                }
            });
    }
}
