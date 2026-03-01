#![windows_subsystem = "windows"]

use eframe::egui;
use quick_img_convert::OutputFormat;

fn load_icon() -> egui::IconData {
    let png_bytes = include_bytes!("../../assets/Untitled.png");
    let img = image::load_from_memory(png_bytes).expect("Failed to load icon").into_rgba8();
    let (w, h) = img.dimensions();
    egui::IconData {
        rgba: img.into_raw(),
        width: w,
        height: h,
    }
}

fn main() -> eframe::Result {
    let icon = load_icon();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 160.0])
            .with_resizable(false)
            .with_icon(icon),
        ..Default::default()
    };

    eframe::run_native(
        "QuickImgConvert",
        options,
        Box::new(|_cc| Ok(Box::new(App::default()))),
    )
}

#[derive(Default)]
struct App {
    format_idx: usize,
    status: Option<String>,
}

const FORMATS: [&str; 2] = ["PNG (lossless)", "JPG (qualité 95%)"];

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("QuickImgConvert");
            ui.add_space(10.0);

            egui::ComboBox::from_label("Format de sortie")
                .selected_text(FORMATS[self.format_idx])
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.format_idx, 0, FORMATS[0]);
                    ui.selectable_value(&mut self.format_idx, 1, FORMATS[1]);
                });

            ui.add_space(10.0);

            if ui.button("Convertir").clicked() {
                let output_format = match self.format_idx {
                    1 => OutputFormat::Jpg(95),
                    _ => OutputFormat::Png,
                };
                let (converted, total) = quick_img_convert::convert_all(&output_format);
                self.status = Some(if total == 0 {
                    "Aucun fichier .jif, .webp ou .avif trouvé dans le répertoire courant.".to_string()
                } else {
                    format!(
                        "{}/{} fichier(s) converti(s) en .{}.",
                        converted,
                        total,
                        output_format.extension()
                    )
                });
            }

            if let Some(ref status) = self.status {
                ui.add_space(5.0);
                ui.label(status);
            }
        });
    }
}
