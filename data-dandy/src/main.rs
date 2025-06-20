mod api;
mod gui;
mod processing;
mod types;

use eframe::egui;

fn main() {
    println!("Launching Data Dandy dashboard...");
    api::te::test_api_connection(&mut vec![]); // <-- test API connection
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Data Dandy Dashboard",
        options,
        Box::new(|_cc| Ok(Box::new(gui::DataDandy::new()))),
    )
    .unwrap();
}
