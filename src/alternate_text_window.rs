use eframe::{egui, NativeOptions};
use egui::{pos2, vec2};

///
/// Creates the tooltip Window using eframe::NativeOptions
/// 
/// ```
/// NativeOptions {
///            viewport: egui::ViewportBuilder::default()
///               .with_inner_size(vec2(200.0, 500.0))
///                .with_always_on_top()
///                .with_decorations(false) // Sets whether the window should have a border, a title bar, etc. 
///                .with_transparent(true),
///            ..Default::default()
///       },
/// ```
/// 
pub fn run_tooltip_window(text: String) -> eframe::Result<()> {
    eframe::run_native(
        "Tooltip Demo",
        NativeOptions {
            viewport: egui::ViewportBuilder::default()
                .with_inner_size(vec2(200.0, 500.0))
                .with_always_on_top()
                .with_decorations(false)
                .with_transparent(true),
            ..Default::default()
        },
        Box::new(move |_cc| Ok(Box::new(MyApp { text, moved: false }))),
    )
}

struct MyApp {
    text: String,
    moved: bool, // <- track whether the window is positioned already
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // Close the window on ESC
        if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
            println!("Window closed");
            std::process::exit(0);
        }else{
        // Move the window only once 
        if !self.moved {
            if let Some(mouse_pos) = ctx.input(|i| i.pointer.hover_pos()) {
                let target = mouse_pos + vec2(10.0, 10.0);
                ctx.send_viewport_cmd(egui::ViewportCommand::OuterPosition(target));
                self.moved = true;
            }
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.label(egui::RichText::new(&self.text).strong());
            });
        });

        ctx.request_repaint();
    }
}
}
