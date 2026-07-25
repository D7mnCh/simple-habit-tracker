/*
- i'll try to finish this project in a week as max
*/

use eframe::{
    egui::{self, Color32, RichText, Stroke},
    Result,
};

const LEFT_PANEL_SIZE: f32 = 150.;
const LEFT_PENEL_HEADER_TEXT_SIZE: f32 = 30.;
const LEFT_PANEL_SPACE_BETWEEN_HEADER_LABELS: f32 = 10.;
const LEFT_PANEL_HABIT_TEXT_SIZE: f32 = 20.;

fn main() -> Result {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("Simple habit tracker")
            .with_resizable(false),
        ..Default::default()
    };

    eframe::run_native(
        "My egui App",
        native_options,
        Box::new(|cc| Ok(Box::new(HabitTracker::new(cc)))),
    )?;
    Ok(())
}

// NOTE there's only one selectable widget
#[derive(Default)]
struct HabitTracker {
    selected: bool,
}

// NOTE i don't want for now to impl dynamic adding/deleting habits
enum _Habit {}

impl HabitTracker {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_global_style.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self::default()
    }
}

impl eframe::App for HabitTracker {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // Left panel

        egui::Panel::left("my_left_panel")
            .resizable(false)
            .default_size(LEFT_PANEL_SIZE)
            .show(ui, |ui| {
                let text = "Habits";
                let label = RichText::new(text).size(LEFT_PENEL_HEADER_TEXT_SIZE);
                ui.heading(label);

                ui.add_space(LEFT_PANEL_SPACE_BETWEEN_HEADER_LABELS);

                ui.vertical_centered_justified(|ui| {
                    let ui_visuals = ui.visuals_mut();
                    // keyboard focus
                    ui_visuals.widgets.active.weak_bg_fill = Color32::LIGHT_GREEN;
                    // hovering with mouse
                    ui_visuals.widgets.hovered.weak_bg_fill = Color32::LIGHT_GREEN;
                    //ui_visuals.selection.bg_fill = Color32::BLUE;

                    let i = 1;
                    let text = RichText::new(format!("{i} -> Habit-{i}"))
                        .size(LEFT_PANEL_HABIT_TEXT_SIZE);
                    let response = ui.toggle_value(&mut self.selected, text);

                    let i = 2;
                    let text = RichText::new(format!("{i} -> Habit-{i}"))
                        .size(LEFT_PANEL_HABIT_TEXT_SIZE);
                    let response = ui.toggle_value(&mut self.selected, text);

                    let i = 3;
                    let text = RichText::new(format!("{i} -> Habit-{i}"))
                        .size(LEFT_PANEL_HABIT_TEXT_SIZE);
                    let response = ui.toggle_value(&mut self.selected, text);
                })
            });

        // Centeral Panl
        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading("Hello World!");
        });
    }
}
