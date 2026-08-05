use eframe::{
    egui::{self, vec2, Color32, Label, RichText, Sense, Ui, Vec2},
    Result,
};
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::fs::{read_to_string, write};
use std::io::Result;
use time::Date;

// window parameters
const WINDOW_SIZE: Vec2 = vec2(825., 400.);

// left panel parameters
const LEFT_PANEL_SIZE: f32 = 100.;
const HEADER_SIZE: f32 = 25.;
const LEFT_PANEL_SPACE_BETWEEN_HEADER_LABELS: f32 = 5.;
const LEFT_PANEL_HABIT_TEXT_SIZE: f32 = 20.;

// centeral panel parameters
const YEAR: i32 = 2026;
const DAYS_OF_YEAR: u16 = 365;
const WEEK_DAYS: [&str; 7] = ["Thur", "Fri", "Sat", "Sun", "Mon", "Tus", "Wed"];
const MONTHS: [&str; 12] = [
    "Jan", "Fab", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov",
    "Dec",
];
const DAY_LABEL_SIZE: Vec2 = vec2(35., 0.);
const SPACE_BETWEEN_CELLS: Vec2 = vec2(2., -4.);
const CELL_SIZE: Vec2 = vec2(10.5, 10.5);
const CELL_RADIUS: f32 = 3.;
const UNMARKED_CELL_COLOR: Color32 = Color32::from_gray(40);
const MARKED_CELL_COLOR: Color32 = Color32::from_rgb(0, 109, 50);

struct HabitTracker {
    habits: Vec<Habit>,
    // neeced for building habit selecter widget
    selected_habit: &'static str,
}

#[derive(Debug, Clone)]
struct Habit {
    name: &'static str,
    //cells: Vec<Vec<Cell>>,
    cells: Vec<Cell>,
}

#[derive(Clone, PartialEq, Debug)]
struct Cell {
    date: Date,
    // if marked/clicked then green, else gray
    color: Color32,
}

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

pub fn load_tracker(file: &str) -> Result<()> {
    todo!();
    Ok(())
}

pub fn save_tracker() {}

impl Cell {
    fn new(day: u16) -> Self {
        // SAFETY: 2026 has 365, so it's nover gonna panic
        let date = Date::from_ordinal_date(YEAR, day).unwrap();
        Self {
            date,
            color: UNMARKED_CELL_COLOR,
        }
    }
}

impl HabitTracker {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let mut cells: Vec<Cell> = Vec::new();

        for day in 1..=DAYS_OF_YEAR {
            let cell = Cell::new(day);
            cells.push(cell);
        }

        let habit_1 = Habit {
            name: "reading",
            cells: cells.clone(),
        };
        let habit_2 = Habit {
            name: "sport",
            cells: cells.clone(),
        };
        let habit_3 = Habit {
            name: "writing",
            cells: cells.clone(),
        };
        let habits = vec![habit_1.clone(), habit_2, habit_3];

        Self {
            habits,
            selected_habit: habit_1.name,
        }
    }
}

impl HabitTracker {
    // NOTE let tweaking visuals for later

    // visuals and style
    fn _overide_left_panel_widgets_look(ui: &mut Ui) {
        let ui_visuals = ui.visuals_mut();
        // keyboard focus
        ui_visuals.widgets.active.weak_bg_fill = Color32::LIGHT_GREEN;
        // hovering with mouse
        ui_visuals.widgets.hovered.weak_bg_fill = Color32::LIGHT_GREEN;
        // when selected
        ui_visuals.selection.bg_fill = Color32::BLUE;
    }

    fn overide_cells_spacing(ui: &mut Ui) {
        ui.spacing_mut().item_spacing = SPACE_BETWEEN_CELLS;
    }

    // left panel
    fn display_left_panel_header(ui: &mut Ui) {
        let header = "Habits";
        let label = RichText::new(header).size(HEADER_SIZE).strong();

        ui.heading(label);
    }

    fn dispaly_left_panel_widgets(&mut self, ui: &mut Ui) {
        for habit in self.habits.iter() {
            let habit_label =
                RichText::new(habit.name).size(LEFT_PANEL_HABIT_TEXT_SIZE);
            let _response = ui.selectable_value(
                &mut self.selected_habit,
                habit.name,
                habit_label,
            );
        }
    }

    // central panel
    fn display_central_panel_cell(&mut self, ui: &mut Ui, curr_day_cell: usize) {
        let selected_habit = self
            .habits
            .iter_mut()
            .find(|habit| habit.name == self.selected_habit);

        if let Some(habit) = selected_habit {
            let (rect, response) = ui.allocate_exact_size(CELL_SIZE, Sense::click());
            ui.painter().rect_filled(
                rect,
                CELL_RADIUS,
                habit.cells[curr_day_cell].color,
            );

            if response.clicked() {
                if habit.cells[curr_day_cell].color == UNMARKED_CELL_COLOR {
                    habit.cells[curr_day_cell].color = MARKED_CELL_COLOR;
                    ui.painter().rect_filled(
                        rect,
                        CELL_RADIUS,
                        habit.cells[curr_day_cell].color,
                    );
                } else {
                    habit.cells[curr_day_cell].color = UNMARKED_CELL_COLOR;
                    ui.painter().rect_filled(
                        rect,
                        CELL_RADIUS,
                        habit.cells[curr_day_cell].color,
                    );
                }
            }

            // enable tooltip (movable tiny pop window when hovering on a cell)
            let msg = format!(
                "{} {}",
                habit.cells[curr_day_cell].date,
                habit.cells[curr_day_cell].date.weekday()
            );
            response.on_hover_text_at_pointer(msg);
        }
    }

    fn display_centeral_panel_header(&self, ui: &mut Ui) {
        let header = self.selected_habit;
        let label = RichText::new(header).size(HEADER_SIZE).strong();

        ui.heading(label);
    }

    fn display_week_day(ui: &mut Ui, day: &str) {
        let day_msg = RichText::new(day).size(10.15);
        ui.add_sized(DAY_LABEL_SIZE, Label::new(day_msg));
    }

    fn display_months_raw(ui: &mut Ui) {
        ui.add_space(43.);
        ui.label(MONTHS[0]);
        ui.add_space(30.);
        ui.label(MONTHS[1]);
        ui.add_space(20.);
        ui.label(MONTHS[2]);
        ui.add_space(24.);
        ui.label(MONTHS[3]);
        ui.add_space(22.);
        ui.label(MONTHS[4]);
        ui.add_space(25.);
        ui.label(MONTHS[5]);
        ui.add_space(25.);
        ui.label(MONTHS[6]);
        ui.add_space(25.);
        ui.label(MONTHS[7]);
        ui.add_space(25.);
        ui.label(MONTHS[8]);
        ui.add_space(25.);
        ui.label(MONTHS[9]);
        ui.add_space(25.);
        ui.label(MONTHS[10]);
        ui.add_space(25.);
        ui.label(MONTHS[11]);
    }
}

impl eframe::App for HabitTracker {
    // this act like while loop, will get exectued 60 times per second
    fn ui(&mut self, ui: &mut Ui, _frame: &mut eframe::Frame) {
        // Left panel
        egui::Panel::left("left_panel")
            .resizable(false)
            .default_size(LEFT_PANEL_SIZE)
            .show(ui, |ui| {
                HabitTracker::display_left_panel_header(ui);

                ui.add_space(LEFT_PANEL_SPACE_BETWEEN_HEADER_LABELS);

                ui.vertical_centered_justified(|ui| {
                    self.dispaly_left_panel_widgets(ui);
                })
            });

        // Centeral Panel
        egui::CentralPanel::default().show(ui, |ui| {
            ui.vertical(|ui| {
                self.display_centeral_panel_header(ui);

                ui.horizontal(|ui| {
                    HabitTracker::display_months_raw(ui);
                });

                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        for day in WEEK_DAYS {
                            HabitTracker::display_week_day(ui, day);
                        }
                    });

                    ui.vertical(|ui| {
                        ui.scope(|ui| {
                            HabitTracker::overide_cells_spacing(ui);
                            for (week_day_indx, week_day) in
                                WEEK_DAYS.iter().enumerate()
                            {
                                ui.horizontal(|ui| {
                                    for week in 0..52 {
                                        // each cell in each column is week * 7, add it with
                                        //week_day_indx cuz notice +1 added in each
                                        //raw starting from adding 0
                                        let cell_indx = week * 7 + week_day_indx;
                                        // NOTE i will guess your allocation logic goes here
                                        self.display_central_panel_cell(
                                            ui, cell_indx,
                                        );

                                        // adding last cell cuz 52 * 7 = 364 of total
                                        //cells were displayed
                                        if *week_day == "Thur" && week == 51 {
                                            const LAST_CELL_INDX: usize = 364;
                                            self.display_central_panel_cell(
                                                ui,
                                                LAST_CELL_INDX,
                                            );
                                        }
                                    }
                                });
                            }
                        });
                    });
                });
            });
        });
    }
}
