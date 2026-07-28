/*
- i'll try to finish this project in a week as max
*/

use eframe::{
    egui::{self, vec2, Checkbox, Color32, Label, RichText, Ui, Vec2},
    Result,
};

// window parameters
const WIDTH: f32 = 900.;
const HEIGHT: f32 = 400.;
const INNER_SIZE: Vec2 = vec2(WIDTH, HEIGHT);

// left panel parameters
const LEFT_PANEL_SIZE: f32 = 150.;
const LEFT_PENEL_HEADER_TEXT_SIZE: f32 = 30.;
const LEFT_PANEL_SPACE_BETWEEN_HEADER_LABELS: f32 = 5.;
const LEFT_PANEL_HABIT_TEXT_SIZE: f32 = 20.;

// centeral panel parameters
const SPACE_BETWEEN_CHECKBOXES_HORIZONTAL: f32 = 1.;
const SPACE_BETWEEN_CHECKBOXES_VERTICAL: f32 = 1.;
const SPACE_BETWEEN_CHECKBOXES: Vec2 = vec2(
    SPACE_BETWEEN_CHECKBOXES_HORIZONTAL,
    SPACE_BETWEEN_CHECKBOXES_VERTICAL,
);
const YEAR_DAYS: u16 = 365;
const DAYS_ROW: u16 = 7;
const MOUNTHS_COLLUMN: u16 = 12;

fn main() -> Result {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("simple habit tracker")
            .with_resizable(false)
            .with_inner_size(INNER_SIZE),
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
    // NOTE i think when modifying visuals or style, you are modifying the whole
    // widgets

    // NOTE let tweaking visuals for later
    fn _change_ui_visuals(ui: &mut Ui) {
        let ui_visuals = ui.visuals_mut();
        // keyboard focus
        ui_visuals.widgets.active.weak_bg_fill = Color32::LIGHT_GREEN;
        // hovering with mouse
        ui_visuals.widgets.hovered.weak_bg_fill = Color32::LIGHT_GREEN;
        // when selected
        ui_visuals.selection.bg_fill = Color32::BLUE;
    }

    fn change_ui_style(ui: &mut Ui) {
        // reduce checkboxes spaces between each other
        ui.spacing_mut().item_spacing = SPACE_BETWEEN_CHECKBOXES;
    }

    // left panel
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

    // central panel
    fn dispaly_central_panel_cells(&self, ui: &mut Ui) {
        for _ in 0..MOUNTHS_COLLUMN {
            let mut checked = false;
            // NOTE checked is based on a database for that habit
            let cell = Checkbox::without_text(&mut checked);
            let _response = ui.add(cell);
        }
    }
}

impl eframe::App for HabitTracker {
    // this act like while loop, will get exectued 60 times per second
    // NOTE this method should only be used for display ?
    fn ui(&mut self, ui: &mut Ui, _frame: &mut eframe::Frame) {
        //HabitTracker::_change_ui_visuals(ui);
        HabitTracker::change_ui_style(ui);

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
            ui.heading("TODO");

            for day in 0..DAYS_ROW {
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        match day {
                            1 => {
                                let response = ui.add(Label::new("Sunday  "));
                                dbg!(&response.intrinsic_size());
                            }
                            3 => {
                                let response = ui.add(Label::new("Tuesday "));
                                dbg!(&response.intrinsic_size());
                            }
                            5 => {
                                let response = ui.add(Label::new("Thursday"));
                                dbg!(&response.intrinsic_size());
                            }
                            _ => ui.add_space(52.7),
                        }
                        self.dispaly_central_panel_cells(ui);
                    });
                });
            }
        });
    }
}
