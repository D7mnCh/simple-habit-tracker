/*
- i'll try to finish this project in a week as max
- i think it's time to introduce some hints from gbt
*/

use eframe::{
    egui::{self, vec2, Color32, Rect, RichText, Sense, Ui, Vec2},
    Result,
};

// window parameters
const WINDOW_SIZE: Vec2 = vec2(850., 400.);

// left panel parameters
const LEFT_PANEL_SIZE: f32 = 100.;
const LEFT_PENEL_HEADER_TEXT_SIZE: f32 = 25.;
const LEFT_PANEL_SPACE_BETWEEN_HEADER_LABELS: f32 = 5.;
const LEFT_PANEL_HABIT_TEXT_SIZE: f32 = 20.;

// centeral panel parameters
const YEAR_DAYS: u16 = 364; // TODO modify it later
const WEEK_DAYS_ROW: u16 = 7;
const YEAER_WEEKS_COLLUMN: u16 = 52;
// NOTE this is gonna make space for all the widgets (that's bad...)
const SPACE_BETWEEN_CELLS: Vec2 = vec2(2., -4.);
const CELL_SIZE: Vec2 = vec2(12., 12.);

fn main() -> Result {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("simple habit tracker")
            .with_resizable(false)
            .with_inner_size(WINDOW_SIZE),
        ..Default::default()
    };

    eframe::run_native(
        "simple habit tracker",
        native_options,
        Box::new(|cc| Ok(Box::new(HabitTracker::new(cc)))),
    )?;
    Ok(())
}

struct HabitTracker {
    //habits: Vec<Habit>,
    // NOTE maybe, but 2 source of info? (try)
    //selected_habit: Habit,
    habit: Habit,
}

#[derive(PartialEq)]
enum Habit {
    //Read(Vec<Cell>),
    //Write(Vec<Cell>),
    //Sport(Vec<Cell>),
    Read,
    Write,
    Sport,
}

struct Cell {
    rect: Rect,
    // if marked == gray ,else green
    color: Color32,
}

impl HabitTracker {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let mut cells: Vec<Cell> = Vec::new();
        for day in 0..YEAR_DAYS {
            let cell = Cell {
                rect: Rect::ZERO,
                color: Color32::from_gray(40),
            };
            cells.push(cell);
        }

        let habits = vec![Habit::Read, Habit::Write, Habit::Sport];

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
        ui.spacing_mut().item_spacing = SPACE_BETWEEN_CELLS;
    }

    // left panel
    fn display_left_panel_header(ui: &mut Ui) {
        let text = "Habits";
        let label = RichText::new(text)
            .size(LEFT_PENEL_HEADER_TEXT_SIZE)
            .strong();

        ui.heading(label);
    }

    fn dispaly_left_panel_widgets(&mut self, ui: &mut Ui) {
        // NOTE all habits gonna be blued, cuz habit == Habit::...(if equel then blue)
        //for habit in self.habits.iter_mut() {
        //    match habit {
        //        Habit::Write => {
        //            let write_msg =
        //                RichText::new("writing").size(LEFT_PANEL_HABIT_TEXT_SIZE);
        //            ui.selectable_value(habit, Habit::Write, write_msg);
        //        }
        //        Habit::Read => {
        //            let read_msg =
        //                RichText::new("reading").size(LEFT_PANEL_HABIT_TEXT_SIZE);
        //            ui.selectable_value(habit, Habit::Read, read_msg);
        //        }
        //        Habit::Sport => {
        //            let sport_msg =
        //                RichText::new("workout").size(LEFT_PANEL_HABIT_TEXT_SIZE);
        //            ui.selectable_value(habit, Habit::Sport, sport_msg);
        //        }
        //    };
        //}

        let write_msg = RichText::new("writing").size(LEFT_PANEL_HABIT_TEXT_SIZE);
        let read_msg = RichText::new("reading").size(LEFT_PANEL_HABIT_TEXT_SIZE);
        let sport_msg = RichText::new("workout").size(LEFT_PANEL_HABIT_TEXT_SIZE);
        let _response = ui.selectable_value(&mut self.habit, Habit::Read, read_msg);
        let _response =
            ui.selectable_value(&mut self.habit, Habit::Write, write_msg);
        let _response =
            ui.selectable_value(&mut self.habit, Habit::Sport, sport_msg);
    }

    // central panel
    // TODO Need to store data somewhere
    fn dispaly_central_panel_cell(&self, ui: &mut Ui) {
        // NOTE i can create a rect and then use allocate_rect for sense parameter,
        // when creating rect i don't need to use ui
        let (rect, response) = ui.allocate_exact_size(CELL_SIZE, Sense::click());
        // TODO use self.color
        ui.painter().rect_filled(rect, 4., Color32::from_gray(40));

        if response.clicked() {
            ui.painter()
                .rect_filled(rect, 4., Color32::from_rgb(38, 166, 65));
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

            // NOTE maybe i'll change the logic here to get 365
            for day in 0..WEEK_DAYS_ROW {
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        for week in 0..YEAER_WEEKS_COLLUMN {
                            self.dispaly_central_panel_cell(ui);
                        }
                    });
                });
            }
        });
    }
}
