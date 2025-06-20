pub struct FetchButton {
    pub loading: bool,
}

impl FetchButton {
    pub fn ui(&mut self, ui: &mut egui::Ui) -> bool {
        if self.loading {
            ui.add(egui::Spinner::default());
            false
        } else {
            ui.button("Fetch Data").clicked()
        }
    }
}
