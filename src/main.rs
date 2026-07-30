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
const YEAR_DAYS: u16 = 365;
// NOTE this is gonna make space for all the widgets (that's bad...)
const SPACE_BETWEEN_CELLS: Vec2 = vec2(2., -4.);
const CELL_SIZE: Vec2 = vec2(12., 12.);
// NOTE should i make an enum for cell color ?
const UNMARKED_CELL_COLOR: Color32 = Color32::from_gray(40);
const MARKED_CELL_COLOR: Color32 = Color32::from_rgb(38, 166, 65);

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
    habits: Vec<Habit>,
    // neeced for building habit selecter widget
    selected_habit: Habit,
}

#[derive(Debug, Clone)]
enum Habit {
    Read(Vec<Cell>),
    Write(Vec<Cell>),
    Sport(Vec<Cell>),
}

// NOTE didn't understand the recurion thing here
impl PartialEq for Habit {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self.which(), other.clone()),
            (Self::Read(_), Self::Read(_))
                | (Self::Write(_), Self::Write(_))
                | (Self::Sport(_), Self::Sport(_))
        )
    }
}

impl Habit {
    fn get_cells_mut(&mut self) -> &mut Vec<Cell> {
        match self {
            Self::Read(cells) => cells,
            Self::Write(cells) => cells,
            Self::Sport(cells) => cells,
        }
    }

    fn which(&self) -> Habit {
        match self {
            Self::Read(_) => Self::Read(Vec::new()),
            Self::Write(_) => Self::Write(Vec::new()),
            Self::Sport(_) => Self::Sport(Vec::new()),
        }
    }
}

// NOTE thinking of not letting user manipulate cells to mark them only with a config file
#[derive(Clone, PartialEq, Debug)]
struct Cell {
    rect: Rect,
    // if marked/clicked then green, else gray
    color: Color32,
}

impl HabitTracker {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let mut cells: Vec<Cell> = Vec::new();
        for _day in 0..YEAR_DAYS {
            let cell = Cell {
                rect: Rect::ZERO,
                color: UNMARKED_CELL_COLOR,
            };
            cells.push(cell);
        }

        //let habits = vec![Habit::Read, Habit::Write, Habit::Sport];
        let habits = vec![
            Habit::Read(cells.clone()),
            Habit::Write(cells.clone()),
            Habit::Sport(cells.clone()),
        ];

        // Vec::new() for selected_habit cuz we are only using it for habit selection,
        //we don't need actually the data
        let cells: Vec<Cell> = Vec::new();

        Self {
            habits,
            selected_habit: Habit::Read(cells),
        }
    }
}

impl HabitTracker {
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

    fn change_cells_spacing(ui: &mut Ui) {
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
        for habit in self.habits.iter() {
            match habit {
                Habit::Write(_) => {
                    let write_msg =
                        RichText::new("writing").size(LEFT_PANEL_HABIT_TEXT_SIZE);
                    let _response = ui.selectable_value(
                        &mut self.selected_habit,
                        Habit::Write(Vec::new()),
                        write_msg,
                    );
                }
                Habit::Read(_) => {
                    let read_msg =
                        RichText::new("reading").size(LEFT_PANEL_HABIT_TEXT_SIZE);
                    let _response = ui.selectable_value(
                        &mut self.selected_habit,
                        Habit::Read(Vec::new()),
                        read_msg,
                    );
                }
                Habit::Sport(_) => {
                    let sport_msg =
                        RichText::new("sport").size(LEFT_PANEL_HABIT_TEXT_SIZE);
                    let _response = ui.selectable_value(
                        &mut self.selected_habit,
                        Habit::Sport(Vec::new()),
                        sport_msg,
                    );
                }
            };
        }
    }

    // central panel
    // NOTE display from name should not take &mut self?, maybe this is allowed cuz
    //i am using an immediate mode gui library
    fn dispaly_central_panel_cells(&mut self, ui: &mut Ui) {
        match &self.selected_habit {
            Habit::Write(_) => {
                for habit in self.habits.iter_mut() {
                    if *habit == self.selected_habit {
                        let cells = habit.get_cells_mut();
                        for cell in cells {
                            let (rect, response) =
                                ui.allocate_exact_size(CELL_SIZE, Sense::click());
                            cell.rect = rect;
                            ui.painter().rect_filled(rect, 4., cell.color);

                            if response.clicked() {
                                if cell.color == UNMARKED_CELL_COLOR {
                                    cell.color = MARKED_CELL_COLOR;
                                    ui.painter().rect_filled(rect, 4., cell.color);
                                } else {
                                    cell.color = UNMARKED_CELL_COLOR;
                                    ui.painter().rect_filled(rect, 4., cell.color);
                                }
                            }
                        }
                        break;
                    }
                }
            }
            Habit::Sport(_) => {
                for habit in self.habits.iter_mut() {
                    if *habit == self.selected_habit {
                        let cells = habit.get_cells_mut();
                        for cell in cells {
                            let (rect, response) =
                                ui.allocate_exact_size(CELL_SIZE, Sense::click());
                            cell.rect = rect;
                            ui.painter().rect_filled(rect, 4., cell.color);

                            if response.clicked() {
                                if cell.color == UNMARKED_CELL_COLOR {
                                    cell.color = MARKED_CELL_COLOR;
                                    ui.painter().rect_filled(rect, 4., cell.color);
                                } else {
                                    cell.color = UNMARKED_CELL_COLOR;
                                    ui.painter().rect_filled(rect, 4., cell.color);
                                }
                            }
                        }
                        break;
                    }
                }
            }
            Habit::Read(_) => {
                for habit in self.habits.iter_mut() {
                    if *habit == self.selected_habit {
                        let cells = habit.get_cells_mut();
                        for cell in cells {
                            let (rect, response) =
                                ui.allocate_exact_size(CELL_SIZE, Sense::click());
                            cell.rect = rect;
                            ui.painter().rect_filled(rect, 4., cell.color);

                            if response.clicked() {
                                if cell.color == UNMARKED_CELL_COLOR {
                                    cell.color = MARKED_CELL_COLOR;
                                    ui.painter().rect_filled(rect, 4., cell.color);
                                } else {
                                    cell.color = UNMARKED_CELL_COLOR;
                                    ui.painter().rect_filled(rect, 4., cell.color);
                                }
                            }
                        }
                        break;
                    }
                }
            }
        }
    }
}

impl eframe::App for HabitTracker {
    // this act like while loop, will get exectued 60 times per second
    // NOTE this method should only be used for display ?
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
            ui.heading("TODO");

            ui.scope(|ui| {
                ui.horizontal_wrapped(|ui| {
                    HabitTracker::change_cells_spacing(ui);
                    self.dispaly_central_panel_cells(ui);
                });
            });
        });
    }
}
