use eframe::{egui, NativeOptions};
use egui::{pos2, vec2};

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
        Box::new(move |_cc| Ok(Box::new(MyApp { text }))),
    )
}

struct MyApp {
    text: String,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if let Some(mouse_pos) = ctx.input(|i| i.pointer.hover_pos()) {
            ctx.send_viewport_cmd(egui::ViewportCommand::OuterPosition(
                mouse_pos + vec2(10.0, 10.0),
            ));
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.label(egui::RichText::new(&self.text).strong());
            });
        });

        ctx.request_repaint();
    }
}
