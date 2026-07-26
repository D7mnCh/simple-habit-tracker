/*
- i'll try to finish this project in a week as max
*/

use eframe::{
    egui::{self, Color32, Response, RichText, Ui},
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

#[derive(Default)]
struct HabitTracker {
    habits: Vec<Habit>,
    // NOTE i'll have two sources from one information
    // habit_selected: Habit
    responses: Vec<Response>,
}

struct Habit {
    name: RichText,
    selected: bool,
}

impl Habit {
    fn new(name: RichText, selected: bool) -> Self {
        Self { name, selected }
    }
}

impl HabitTracker {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let habit_1 = Habit::new(
            RichText::new("reading").size(LEFT_PANEL_HABIT_TEXT_SIZE),
            false,
        );
        let habit_2 = Habit::new(
            RichText::new("workout").size(LEFT_PANEL_HABIT_TEXT_SIZE),
            false,
        );
        let habit_3 = Habit::new(
            RichText::new("writing").size(LEFT_PANEL_HABIT_TEXT_SIZE),
            false,
        );
        let habits = vec![habit_1, habit_2, habit_3];

        let responses = Vec::new();

        Self { habits, responses }
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
}

impl eframe::App for HabitTracker {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        //HabitTracker::_change_ui_visuals(ui);

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
                    // constructing widgets
                    for habit in self.habits.iter_mut() {
                        let response =
                            ui.toggle_value(&mut habit.selected, habit.name.clone());
                        self.responses.push(response);
                    }

                    // TODO if selected on, turns off highlighting from the others (if any)
                    // widgets functionnlity
                    for response in self.responses.iter() {
                        //
                    }
                })
            });

        // Centeral Panl
        egui::CentralPanel::default().show(ui, |ui| {
            ui.heading("Hello World!");
        });
    }
}
