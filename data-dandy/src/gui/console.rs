pub struct Console<'a> {
    pub log: &'a [String],
}

impl<'a> Console<'a> {
    pub fn ui(&self, ui: &mut egui::Ui) {
        let bg = egui::Color32::BLACK;
        let fg = egui::Color32::from_rgb(0, 255, 0);
        let frame = egui::Frame::new()
            .fill(bg)
            .inner_margin(egui::Margin::same(4));
        frame.show(ui, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                for line in self.log {
                    ui.colored_label(fg, line);
                }
            });
        });
    }
}
