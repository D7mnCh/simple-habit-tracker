use eframe::egui::{
    self, Color32, EventFilter, Id, Key, Label, Rgba, RichText, Sense, TextEdit, Ui,
    Vec2, Vec2b, vec2,
};
use egui::Align2;
use serde::{Deserialize, Serialize};
use std::io;
use std::{
    error, fmt,
    fs::{self, File},
    path::Path,
};
use time::Date;

// main window settings
const WINDOW_SIZE: Vec2 = vec2(825., 400.);

// left panel settings
const LEFT_PANEL_RESIZABLE: bool = false;
const LEFT_PANEL_SIZE: f32 = 100.;
const HEADER_SIZE: f32 = 25.;
const LEFT_PANEL_SPACE_BETWEEN_HEADER_LABELS: f32 = 5.;
const LEFT_PANEL_HABIT_TEXT_SIZE: f32 = 12.;
const MAX_HABITS: usize = 6;

// time
const YEAR: i32 = 2026;
const DAYS_OF_YEAR: u16 = 365;
const WEEK_DAYS: [&str; 7] = ["Thur", "Fri", "Sat", "Sun", "Mon", "Tus", "Wed"];
const MONTHS: [&str; 12] = [
    "Jan", "Fab", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov",
    "Dec",
];

// centeral panel settings
const DAY_LABEL_SIZE: Vec2 = vec2(35., 0.);
const SPACE_BETWEEN_CELLS: Vec2 = vec2(2., -4.);
const CELL_SIZE: Vec2 = vec2(10.5, 10.5);
const CELL_RADIUS: f32 = 3.;
const MARKED_CELL_COLOR: Rgba = Rgba::from_rgb(0.001, 0.102, 0.023);
const UNMARKED_CELL_COLOR: Rgba = Rgba::from_gray(0.040);
const HALF_MARKED_CELL_COLOR: Rgba = Rgba::from_rgb(0.201, 0.098, 0.002);
const NOTES_LABEL_SIZE: f32 = 20.;
const SPACE_BTW_MONTHS: f32 = 25.;

// floating window settings
const RESIZABLE_FLOATING_WINDOW: Vec2b = Vec2b::FALSE;
const COLLAPSIBLE_FLOATTING_WINDOW: bool = false;
const WIDTH_FLOAT_WINDOW: f32 = 200.;
const POS_FLOAT_WINDOW: Align2 = Align2::CENTER_CENTER; // pos -> anchor
const MAX_ADD_CHARS_TEXT: usize = 24;

// I/O
const TRACKER_FILE: &str = "save.json";

#[derive(Default, Debug, Deserialize, Serialize)]
struct HabitTracker {
    habits: Vec<Habit>,
    float_window: FloatWindow,
    // neeced for building habit selecter widget
    // used String instead of &'static str for serde derive issues
    // id represent a selected habit (via indxing) instead of a
    //string is much cleaner and will reduce boilerplate
    // NOTE maybe i can remove Option by getting a prev element and do
    //early return if no habits?
    selected_habit: Option<usize>,
}

#[derive(Default, Deserialize, Serialize, Debug, Clone)]
struct Habit {
    // used String instead of &'static str cuz of serde derive issue thing
    name: String,
    cells: Vec<Cell>,
    notes: String,
}

#[derive(Default, Clone, Debug, Deserialize, Serialize)]
struct FloatWindow {
    state: Option<FloatWindowState>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
enum FloatWindowState {
    AddHabit { name: String, hint: String },
    DeleteHabit { selected_habit_indx: Option<usize> },
}

#[derive(Deserialize, Serialize, Clone, Debug)]
struct Cell {
    date: Date,
    color: Rgba,
}

#[derive(Debug)]
enum CustomError {
    Eframe(eframe::Error),
    Io(io::Error),
    SerdeJson(serde_json::Error),
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
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
        Box::new(|cc| Ok(Box::new(HabitTracker::new(cc)?))),
    )?;
    Ok(())
}

// my custmError need to impl StdError (error::Error) in fn main
impl error::Error for CustomError {}

impl From<eframe::Error> for CustomError {
    fn from(e: eframe::Error) -> Self {
        Self::Eframe(e)
    }
}

impl From<io::Error> for CustomError {
    fn from(e: io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<serde_json::Error> for CustomError {
    fn from(e: serde_json::Error) -> Self {
        Self::SerdeJson(e)
    }
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Eframe(e) => e.fmt(f),
            Self::Io(e) => e.fmt(f),
            Self::SerdeJson(e) => e.fmt(f),
        }
    }
}

impl FloatWindow {
    fn reset_add_habit_hint(&mut self) {
        if let Some(FloatWindowState::AddHabit { hint, .. }) = &mut self.state {
            hint.clear();
        }
    }

    fn reset_add_habit_name(&mut self) {
        if let Some(FloatWindowState::AddHabit { name, .. }) = &mut self.state {
            name.clear();
        }
    }
}

impl Habit {
    fn new(name: String, cells: Vec<Cell>) -> Habit {
        Self {
            name,
            cells,
            notes: String::new(),
        }
    }
}

impl Cell {
    fn new(day: u16) -> Self {
        let date =
            Date::from_ordinal_date(YEAR, day).expect("day must be valid for year");
        Self {
            date,
            color: UNMARKED_CELL_COLOR,
        }
    }

    fn gen_cells_with_date() -> Vec<Cell> {
        // .map(Cell::new), Cell::new is a function item with a signiture of
        //"fn(u16)-> Cell" it will coerce into Fnmut(u16) -> Cell(that's why is correct)
        (1..=DAYS_OF_YEAR).map(Cell::new).collect()
    }

    // if a block mutate a struct(e.g Cell), should create a method for it
    fn toggle_color(&mut self) {
        self.color = match self.color {
            UNMARKED_CELL_COLOR => HALF_MARKED_CELL_COLOR,
            MARKED_CELL_COLOR => UNMARKED_CELL_COLOR,
            HALF_MARKED_CELL_COLOR => MARKED_CELL_COLOR,
            e => unreachable!("{:?}", e),
        };
    }
}

impl HabitTracker {
    fn new(_cc: &eframe::CreationContext<'_>) -> Result<Self, CustomError> {
        HabitTracker::load_file()
    }

    // Desirialize
    fn load_file() -> Result<HabitTracker, CustomError> {
        if !Path::new(TRACKER_FILE).exists() {
            return Ok(HabitTracker::default());
        }

        let file = fs::read_to_string(TRACKER_FILE)?;
        // file can go empty if user whant too -.-
        if file.is_empty() {
            return Ok(HabitTracker::default());
        }

        // NOTE handle when user manipulate TRACKER_FILE ?

        Ok(serde_json::from_str(&file)?)
    }

    // Serialize
    fn save_file(&self) -> Result<(), CustomError> {
        let json = serde_json::to_string_pretty(self)?;
        fs::write(TRACKER_FILE, json)?;

        Ok(())
    }
}

impl HabitTracker {
    // NOTE let tweaking visuals for later
    // visuals and style
    fn _override_left_panel_widgets_look(ui: &mut Ui) {
        let ui_visuals = ui.visuals_mut();
        // keyboard focus
        ui_visuals.widgets.active.weak_bg_fill = Color32::LIGHT_GREEN;
        // hovering with mouse
        ui_visuals.widgets.hovered.weak_bg_fill = Color32::LIGHT_GREEN;
        // when selected
        ui_visuals.selection.bg_fill = Color32::BLUE;
    }

    fn override_cells_spacing(ui: &mut Ui) {
        ui.spacing_mut().item_spacing = SPACE_BETWEEN_CELLS;
    }

    // left panel
    fn display_left_panel_header(ui: &mut Ui) {
        let header = "Habits";
        let label = RichText::new(header).size(HEADER_SIZE).strong();

        ui.heading(label);
    }

    fn display_left_panel_habits(&mut self, ui: &mut Ui) {
        for (indx, habit) in self.habits.iter().enumerate() {
            let habit_label =
                RichText::new(&habit.name).size(LEFT_PANEL_HABIT_TEXT_SIZE);

            let response = ui.selectable_value(
                &mut self.selected_habit,
                Some(indx),
                habit_label,
            );

            // TODO calling this func at the end user can press tab and it will hover the first habit
            //i want to call this func at the top, i can't because i need response
            HabitTracker::disable_navigation_keys(ui, response.id);
        }
    }

    fn display_buttton_add_habit(&mut self, ui: &mut Ui) {
        if ui.button("add").clicked() {
            self.float_window.state = Some(FloatWindowState::AddHabit {
                name: String::new(),
                hint: String::new(),
            });
        }
    }

    fn display_buttton_delete_habit(&mut self, ui: &mut Ui) {
        if ui.button("delete").clicked() {
            self.float_window.state = Some(FloatWindowState::DeleteHabit {
                selected_habit_indx: None,
            });
        }
    }

    fn display_float_window_content(&mut self, ui: &mut Ui) {
        match &mut self.float_window.state {
            Some(FloatWindowState::AddHabit { name, hint }) => {
                let response = ui.add(
                    TextEdit::singleline(name)
                        .hint_text(hint.clone())
                        .desired_width(f32::INFINITY)
                        .char_limit(MAX_ADD_CHARS_TEXT)
                        .horizontal_align(egui::Align::Center),
                );
                HabitTracker::disable_navigation_keys(ui, response.id);

                if ui.input(|i| i.key_pressed(Key::Enter)) && !name.is_empty() {
                    // i can't reduce code here cuz hint mutation needs a condition
                    if self.habits.len() >= MAX_HABITS {
                        *hint = format!("{MAX_HABITS} habits max");
                        self.float_window.reset_add_habit_name();
                        return;
                    }

                    if self.habits.iter().any(|habit| habit.name == *name) {
                        *hint = "used habit name".to_owned();
                        self.float_window.reset_add_habit_name();
                        return;
                    }

                    let cells = Cell::gen_cells_with_date();
                    let habit = Habit::new(name.clone(), cells);
                    self.habits.push(habit);

                    self.float_window.reset_add_habit_name();
                    self.float_window.reset_add_habit_hint();
                }
            }

            // NOTE what about converting name field into usize?
            Some(FloatWindowState::DeleteHabit {
                selected_habit_indx,
            }) => {
                ui.vertical_centered(|ui| {
                    for (indx, habit) in self.habits.iter().enumerate() {
                        let habit_label = RichText::new(&habit.name)
                            .size(LEFT_PANEL_HABIT_TEXT_SIZE);
                        let _response = ui.selectable_value(
                            selected_habit_indx,
                            Some(indx),
                            habit_label,
                        );
                    }

                    if let Some(selected_habit_delete_indx) = selected_habit_indx {
                        if ui.input(|i| i.key_pressed(Key::Enter)) {
                            let _ = self.habits.remove(*selected_habit_delete_indx);
                        }
                    }
                });
            }

            None => {}
        }
    }

    // central panel
    fn display_central_panel_cell(
        &mut self,
        ui: &mut Ui,
        curr_day_cell_indx: usize,
    ) {
        // display only selected habit, ignore the others (for pefromance)
        let (rect, response) = ui.allocate_exact_size(CELL_SIZE, Sense::click());
        if let Some(selected_habit) = self.selected_habit {
            let cell = &mut self.habits[selected_habit].cells[curr_day_cell_indx];
            // has_focus() for removing keyboard interaction (only mouse)
            if response.clicked() && !response.has_focus() {
                cell.toggle_color();
            }

            // i was painting then mutating then repainting again, just mutate then
            //paint
            ui.painter().rect_filled(rect, CELL_RADIUS, cell.color);

            // enable tooltip (movable tiny pop window when hovering on a cell)
            let msg = format!("{} {}", cell.date, cell.date.weekday());
            response.on_hover_text_at_pointer(msg);
        }
    }

    fn display_centeral_panel_header(&self, ui: &mut Ui) {
        if let Some(selected_habit) = self.selected_habit {
            let header = self.habits[selected_habit].name.clone();
            let label = RichText::new(header).size(HEADER_SIZE).strong();

            ui.heading(label);
        }
    }

    fn display_habit_notes_text_edit(&mut self, ui: &mut Ui) {
        if let Some(selected_habit) = self.selected_habit {
            ui.add(
                TextEdit::multiline(&mut self.habits[selected_habit].notes)
                    .desired_rows(1)
                    .lock_focus(true),
            );
        }
    }

    fn display_week_day(ui: &mut Ui, day: &str) {
        let day_msg = RichText::new(day).size(10.15);
        ui.add_sized(DAY_LABEL_SIZE, Label::new(day_msg));
    }

    fn display_months_raw(ui: &mut Ui) {
        ui.add_space(45.);
        ui.label(MONTHS[0]);

        for indx in 1..=11 {
            ui.add_space(SPACE_BTW_MONTHS);
            ui.label(MONTHS[indx]);
        }
    }

    // NOTE it will work just once
    fn disable_navigation_keys(ui: &mut Ui, id: Id) {
        ui.memory_mut(|mem| {
            let event_filter = EventFilter {
                escape: true,
                tab: true,
                horizontal_arrows: true,
                vertical_arrows: true,
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
            .resizable(LEFT_PANEL_RESIZABLE)
            .default_size(LEFT_PANEL_SIZE)
            .show(ui, |ui| {
                ui.vertical(|ui| {
                    ui.vertical_centered_justified(|ui| {
                        HabitTracker::display_left_panel_header(ui);

                        ui.add_space(LEFT_PANEL_SPACE_BETWEEN_HEADER_LABELS);

                        self.display_left_panel_habits(ui);
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
        let float_window_name = match &self.float_window.state {
            Some(FloatWindowState::AddHabit { .. }) => "add habit".to_owned(),
            Some(FloatWindowState::DeleteHabit { .. }) => "delete habit".to_owned(),
            None => String::new(),
        };

        egui::Window::new(float_window_name)
            .open(&mut is_open)
            .resizable(RESIZABLE_FLOATING_WINDOW)
            .collapsible(COLLAPSIBLE_FLOATTING_WINDOW)
            .max_width(WIDTH_FLOAT_WINDOW)
            .anchor(POS_FLOAT_WINDOW, [0., 0.])
            .show(ui.ctx(), |ui| {
                self.display_float_window_content(ui);
            });

        if !is_open {
            self.float_window.state = None
        }

        // Centeral Panel
        egui::CentralPanel::default().show(ui, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                if self.selected_habit.is_none() {
                    return;
                }

                // make empty centeral Panel if a selected habit deleted
                // SEFETY: if self.selected_habit is checked before(early return),
                //so always will be Some(value)
                if self
                    .habits
                    .get(self.selected_habit.expect(
                        "self.selected_habit is Some, None varient was handled before",
                    ))
                    .is_none()
                {
                    self.selected_habit = None;
                    return;
                }

                ui.vertical(|ui| {
                    ui.vertical_centered(|ui| {
                        self.display_centeral_panel_header(ui);
                    });

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
                                HabitTracker::override_cells_spacing(ui);
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

                                            // adding last cell, cuz 52 * 7 = 364 of cells that get
                                            //displayed
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

                    ui.add_space(30.);

                    ui.label(RichText::new("Notes").size(NOTES_LABEL_SIZE));

                    self.display_habit_notes_text_edit(ui);
                });
            });
        });
    }

    // NOTE if app crashes, changes will get loss
    // TODO create a thread that runs every like 2mins to save_file
    fn on_exit(&mut self) {
        self.save_file().unwrap_or_else(|e| eprintln!("{e}"));
    }
}
