/*
- i'll try to finish this project in a week as max
*/

use eframe::{
    egui::{self, Color32, RichText, Ui},
    Result,
};

const LEFT_PANEL_SIZE: f32 = 150.;
const LEFT_PENEL_HEADER_TEXT_SIZE: f32 = 30.;
const LEFT_PANEL_SPACE_BETWEEN_HEADER_LABELS: f32 = 5.;
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

struct HabitTracker {
    habit: Habit,
}

#[derive(PartialEq)]
enum Habit {
    Read,
    Write,
    Sport,
}

impl HabitTracker {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self { habit: Habit::Read }
    }
}

impl HabitTracker {
    fn _change_ui_visuals(ui: &mut Ui) {
        let ui_visuals = ui.visuals_mut();
        // keyboard focus
        ui_visuals.widgets.active.weak_bg_fill = Color32::LIGHT_GREEN;
        // hovering with mouse
        ui_visuals.widgets.hovered.weak_bg_fill = Color32::LIGHT_GREEN;
        // when selected
        ui_visuals.selection.bg_fill = Color32::BLUE;
    }

    // NOTE i didn't use self but it looks kinda wierd if i remove it
    //and access this member function via path
    fn display_left_panel_header(ui: &mut Ui) {
        let text = "Habits";
        let label = RichText::new(text).size(LEFT_PENEL_HEADER_TEXT_SIZE);

        ui.heading(label);
    }

    fn dispaly_left_panel_widgets(&mut self, ui: &mut Ui) {
        let read_msg = RichText::new("reading").size(LEFT_PANEL_HABIT_TEXT_SIZE);
        let write_msg = RichText::new("workout").size(LEFT_PANEL_HABIT_TEXT_SIZE);
        let sport_msg = RichText::new("writing").size(LEFT_PANEL_HABIT_TEXT_SIZE);

        let _response = ui.selectable_value(&mut self.habit, Habit::Read, read_msg);
        let _response =
            ui.selectable_value(&mut self.habit, Habit::Write, write_msg);
        let _response =
            ui.selectable_value(&mut self.habit, Habit::Sport, sport_msg);
    }
}

impl eframe::App for HabitTracker {
    // this act like while loop, will get exectued 60 times per second
    // NOTE this method should only used for display ?
    fn ui(&mut self, ui: &mut Ui, _frame: &mut eframe::Frame) {
        //HabitTracker::_change_ui_visuals(ui);

        // Left panel
        egui::Panel::left("my_left_panel")
            .resizable(false)
            .default_size(LEFT_PANEL_SIZE)
            .show(ui, |ui| {
                HabitTracker::display_left_panel_header(ui);

                ui.add_space(LEFT_PANEL_SPACE_BETWEEN_HEADER_LABELS);

                ui.vertical_centered_justified(|ui| {
                    self.dispaly_left_panel_widgets(ui);
                })
            });

        // Centeral Panl
        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading("Hello World!");
        });
    }
}
