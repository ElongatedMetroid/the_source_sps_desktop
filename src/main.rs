use the_source_sps_desktop::app::App;

#[tokio::main]
async fn main() {
    let native_options = eframe::NativeOptions::default();

    eframe::run_native(
        "The Source Desktop",
        native_options,
        Box::new(|cc| Box::new(App::new(cc).unwrap())),
    )
    .unwrap();
}
