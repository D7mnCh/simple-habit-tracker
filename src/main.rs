/*
- i'll try to finish this project in a week as max
- i think it's time to introduce some hints from gbt
*/

use eframe::{
    egui::{self, vec2, Color32, RichText, Sense, Ui, Vec2},
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
const UNMARKED_CELL_COLOR: Color32 = Color32::from_gray(40);
const MARKED_CELL_COLOR: Color32 = Color32::from_rgb(38, 166, 65);

#[derive(Debug, Clone)]

// TODO
struct Habit {
    name: &'static str,
    cells: Vec<Cell>,
}

//enum Habit {
//    Read(Vec<Cell>),
//    Write(Vec<Cell>),
//    Sport(Vec<Cell>),
//}

struct HabitTracker {
    habits: Vec<Habit>,
    // neeced for building habit selecter widget
    selected_habit: &'static str,
}

// NOTE thinking of not letting user manipulate cells to mark them only with a config file
#[derive(Clone, PartialEq, Debug)]
struct Cell {
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

// NOTE didn't understand the recurion thing here
impl PartialEq for Habit {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self.name, other.name),
            ("sport", "sport") | ("write", "write") | ("read", "read")
        )
    }
}

impl Habit {
    //fn get_cells_mut(&mut self) -> &mut Vec<Cell> {
    //    match self {
    //        Self::Read(cells) => cells,
    //        Self::Write(cells) => cells,
    //        Self::Sport(cells) => cells,
    //    }
    //}
    //
    //fn which(&self) -> Habit {
    //    match self {
    //        Self::Read(_) => Self::Read(Vec::new()),
    //        Self::Write(_) => Self::Write(Vec::new()),
    //        Self::Sport(_) => Self::Sport(Vec::new()),
    //    }
    //}
}

impl HabitTracker {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let mut cells: Vec<Cell> = Vec::new();
        for _day in 0..YEAR_DAYS {
            let cell = Cell {
                //rect: Rect::ZERO,
                color: UNMARKED_CELL_COLOR,
            };
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
    fn dispaly_central_panel_cells(&mut self, ui: &mut Ui) {
        let habit_cells = self
            .habits
            .iter_mut()
            .find(|habit| habit.name == self.selected_habit);

        if let Some(habit) = habit_cells {
            let cells = &mut habit.cells;
            for cell in cells {
                let (rect, response) =
                    ui.allocate_exact_size(CELL_SIZE, Sense::click());
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
