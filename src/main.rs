use eframe::{
    egui::{
        self, vec2, Color32, EventFilter, Id, Key, Label, Rgba, RichText, Sense, Ui,
        Vec2, Window,
    },
    Result,
};
use serde::{Deserialize, Serialize};
use serde_json::Result as ResultSerdeJson;
use std::io::{self, Result as ResultIo};
use std::{
    fs::{self, File},
    path::Path,
};
use time::Date;

// window parameters
const WINDOW_SIZE: Vec2 = vec2(825., 400.);

// left panel parameters
const LEFT_PANEL_SIZE: f32 = 100.;
const HEADER_SIZE: f32 = 25.;
const LEFT_PANEL_SPACE_BETWEEN_HEADER_LABELS: f32 = 5.;
const LEFT_PANEL_HABIT_TEXT_SIZE: f32 = 12.;

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
// NOTE colors are not working
const MARKED_CELL_COLOR: Rgba = Rgba::from_rgb(0.001, 0.102, 0.023);
const UNMARKED_CELL_COLOR: Rgba = Rgba::from_gray(0.040);
const HALF_MARKED_CELL_COLOR: Rgba = Rgba::from_rgb(0.201, 0.098, 0.002);

// I/O
const TRACKER_FILE: &str = "save.json";

#[derive(Debug, Deserialize, Serialize)]
struct HabitTracker {
    habits: Vec<Habit>,
    float_window: FloatWindow,
    // neeced for building habit selecter widget
    // used String instead of &'static str for serde derive issues
    selected_habit: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct Habit {
    // used String instead of &'static str cuz of serde derive issue thing
    name: String,
    cells: Vec<Cell>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct FloatWindow {
    name: String,
    state: Option<FloatWindowState>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
enum FloatWindowState {
    AddHabit(String),
    DeleteHabit(String),
}

#[derive(Deserialize, Serialize, Clone, PartialEq, Debug)]
struct Cell {
    // TODO make date [Option<Date>;DAY_OF_YEAR], Optinon needed for construction
    // i could make Option Vec<Cell>, but Option is just needed for Date cuz it doesn't
    //have Default trait
    date: Date,
    color: Rgba,
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

impl FloatWindow {
    fn reset_add_habit_name(&mut self) {
        match &mut self.state {
            Some(FloatWindowState::AddHabit(name)) => *name = String::new(),
            _ => unreachable!("caller should only call this method when FloatWindowState is AddHabit "),
        }
    }
}

impl Habit {
    fn new(name: String, cells: Vec<Cell>) -> Habit {
        Self { name, cells }
    }
}

impl Cell {
    fn new(day: u16) -> Self {
        // SAFETY: 2026 has 365, so it's nover gonna panic
        let date = Date::from_ordinal_date(YEAR, day).unwrap();
        Self {
            date,
            color: UNMARKED_CELL_COLOR,
        }
    }

    fn gen_cells_with_date() -> Vec<Cell> {
        // TODO you should construct empty array/vec, cuz i'll get habits data from
        //file, this program only add to that file,if false then i'll modify that file
        //every time i lunch the app wapping out the saving
        let mut cells: Vec<Cell> = Vec::new();
        for day in 1..=DAYS_OF_YEAR {
            let cell = Cell::new(day);
            cells.push(cell);
        }

        cells
    }
}

impl HabitTracker {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let _ = Cell::gen_cells_with_date();

        // TODO i wanna remove shadowing here, it's wierd
        let habit_tracker = Self {
            float_window: FloatWindow {
                name: String::new(),
                state: None,
            },
            habits: Vec::new(),
            selected_habit: String::new(),
        };

        HabitTracker::check_file(&habit_tracker);
        HabitTracker::load_file()
    }

    fn check_file(habit_tracker: &HabitTracker) {
        if !Path::new(TRACKER_FILE).exists() {
            let _ = File::create(TRACKER_FILE).unwrap();
        }

        let file = fs::metadata(TRACKER_FILE).unwrap();
        if file.len() == 0 {
            HabitTracker::save_file(habit_tracker);
        }
    }

    // Desirialize
    fn load_file() -> HabitTracker {
        let file = fs::read_to_string(TRACKER_FILE).unwrap();
        let habit_tracker: HabitTracker =
            serde_json::from_str(file.as_str()).unwrap();

        habit_tracker
    }

    // Serialize
    fn save_file(&self) {
        // doing to_string_pretty() slow the app
        //let json = serde_json::to_string_pretty(self).unwrap();
        let json = serde_json::to_string(self).unwrap();
        let _ = fs::write(TRACKER_FILE, json);
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
                RichText::new(habit.name.clone()).size(LEFT_PANEL_HABIT_TEXT_SIZE);
            let response = ui.selectable_value(
                &mut self.selected_habit,
                habit.name.clone(),
                habit_label,
            );

            // NOTE sometimes work, i think cuz the app is slow
            HabitTracker::disable_navigation_keys(ui, response.id);
        }
    }

    fn display_buttton_add_habit(&mut self, ui: &mut Ui) {
        if ui.button("add").clicked() {
            self.float_window.state =
                Some(FloatWindowState::AddHabit(String::new()));
            self.float_window.name = String::from("add habit")
        }
    }

    fn display_buttton_delete_habit(&mut self, ui: &mut Ui) {
        if ui.button("delete").clicked() {
            self.float_window.state =
                Some(FloatWindowState::DeleteHabit(String::new()));
            self.float_window.name = String::from("delete habit")
        }
    }

    fn display_float_window_content(&mut self, ui: &mut Ui) {
        match &mut self.float_window.state {
            Some(FloatWindowState::AddHabit(name)) => {
                let response = ui.text_edit_singleline(name);
                HabitTracker::disable_navigation_keys(ui, response.id);

                if ui.input(|i| i.key_pressed(Key::Enter)) && !name.is_empty() {
                    let cells = Cell::gen_cells_with_date();
                    let habit = Habit::new(name.clone(), cells);
                    self.habits.push(habit);

                    HabitTracker::save_file(self);

                    self.float_window.reset_add_habit_name();
                }
            }

            Some(FloatWindowState::DeleteHabit(selected_habit_delete)) => {
                ui.vertical_centered(|ui| {
                    for habit in self.habits.iter() {
                        let habit_label = RichText::new(habit.name.clone())
                            .size(LEFT_PANEL_HABIT_TEXT_SIZE);
                        let _response = ui.selectable_value(
                            selected_habit_delete,
                            habit.name.clone(),
                            habit_label,
                        );
                    }

                    if ui.input(|i| i.key_pressed(Key::Enter)) && 
                        let Some(selected_habit_indx) = self
                            .habits
                            .iter_mut()
                            .position(|habit| habit.name == *selected_habit_delete)
                        {
                            let _ = self.habits.remove(selected_habit_indx);
                        }
                });
            }

            None => {}
        }
    }

    // central panel
    fn display_central_panel_cell(&mut self, ui: &mut Ui, curr_day_cell: usize) {
        // display only selected habit, ignore the others (for pefromance)
        // search the selected habit for habit's cells
        let curr_displayed_habit = self
            .habits
            .iter_mut()
            .find(|habit| habit.name == self.selected_habit);

        // i can't make self.save_file inside response block cuz of safety
        let mut should_save = false;
        if let Some(habit) = curr_displayed_habit {
            let (rect, response) = ui.allocate_exact_size(CELL_SIZE, Sense::click());
            ui.painter().rect_filled(
                rect,
                CELL_RADIUS,
                habit.cells[curr_day_cell].color,
            );

            // remove keyboard interaction (only mouse)
            if response.clicked() && !response.has_focus() {
                habit.cells[curr_day_cell].color =
                    match habit.cells[curr_day_cell].color {
                        UNMARKED_CELL_COLOR => HALF_MARKED_CELL_COLOR,
                        MARKED_CELL_COLOR => UNMARKED_CELL_COLOR,
                        HALF_MARKED_CELL_COLOR => MARKED_CELL_COLOR,
                        e => unreachable!("{:?}", e),
                    };

                ui.painter().rect_filled(
                    rect,
                    CELL_RADIUS,
                    habit.cells[curr_day_cell].color,
                );
                should_save = true;
            }

            // enable tooltip (movable tiny pop window when hovering on a cell)
            let msg = format!(
                "{} {}",
                habit.cells[curr_day_cell].date,
                habit.cells[curr_day_cell].date.weekday()
            );
            response.on_hover_text_at_pointer(msg);
        }

        if should_save {
            self.save_file();
        }
    }

    fn display_centeral_panel_header(&self, ui: &mut Ui) {
        let header = self.selected_habit.clone();
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

    // NOTE didn't work (well) maybe cuz of performance
    fn disable_navigation_keys(ui: &mut Ui, id: Id) {
        ui.memory_mut(|mem| {
            let event_filter = EventFilter {
                escape: true,
                tab: true,
                ..Default::default()
            };
            mem.set_focus_lock_filter(id, event_filter);
        });
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
                ui.vertical(|ui| {
                    HabitTracker::display_left_panel_header(ui);

                    ui.add_space(LEFT_PANEL_SPACE_BETWEEN_HEADER_LABELS);

                    ui.vertical_centered_justified(|ui| {
                        self.dispaly_left_panel_widgets(ui);
                    });
                    ui.with_layout(
                        egui::Layout::left_to_right(egui::Align::BOTTOM),
                        |ui| {
                            self.display_buttton_add_habit(ui);
                            self.display_buttton_delete_habit(ui);
                        },
                    );
                });
            });

        // float window
        // create open instance to satisfy borrow checker (closure unique access to self)
        let mut is_open = self.float_window.state.is_some();

        egui::Window::new(self.float_window.name.clone())
            .open(&mut is_open)
            .show(ui.ctx(), |ui| {
                self.display_float_window_content(ui);
            });

        if !is_open {
            self.float_window.state = None
        }

        // Centeral Panel
        egui::CentralPanel::default().show(ui, |ui| {
            // make empty centeral Panel if a selected habit deleted
            if self
                .habits
                .iter()
                .find(|habit| habit.name == self.selected_habit)
                .is_none()
            {
                self.selected_habit = String::new();
                return;
            }

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
