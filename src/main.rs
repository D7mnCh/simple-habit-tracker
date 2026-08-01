use eframe::{
    egui::{self, vec2, Color32, Label, RichText, Sense, Ui, Vec2},
    Result,
};

// window parameters
const WINDOW_SIZE: Vec2 = vec2(800., 400.);

// left panel parameters
const LEFT_PANEL_SIZE: f32 = 100.;
const LEFT_PENEL_HEADER_TEXT_SIZE: f32 = 25.;
const LEFT_PANEL_SPACE_BETWEEN_HEADER_LABELS: f32 = 5.;
const LEFT_PANEL_HABIT_TEXT_SIZE: f32 = 20.;

// centeral panel parameters
const DAYS: [&str; 7] = ["Sun", "Mon", "Tus", "Wed", "Thur", "Fri", "Sat"];
const WEEKS: u16 = 52;
const DAY_LABEL_SIZE: Vec2 = vec2(35., 15.);
const SPACE_BETWEEN_CELLS: Vec2 = vec2(2., -4.);
const CELL_SIZE: Vec2 = vec2(10., 10.);
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
    cells: Vec<Vec<Cell>>,
}

// NOTE thinking of not letting user manipulate cells to mark them only with a config file
#[derive(Clone, PartialEq, Debug)]
struct Cell {
    // if marked/clicked then green, else gray
    color: Color32,
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

impl HabitTracker {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // NOTE i've get 364 days
        let mut cells: Vec<Vec<Cell>> = Vec::new();

        for _day in DAYS {
            let mut raw_cells: Vec<Cell> = Vec::new();
            for _week in 0..WEEKS {
                let cell = Cell {
                    color: UNMARKED_CELL_COLOR,
                };
                raw_cells.push(cell);
            }
            cells.push(raw_cells);
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
    fn dispaly_central_panel_raw_cells(&mut self, ui: &mut Ui, raw_cells: usize) {
        let habit_cells = self
            .habits
            .iter_mut()
            .find(|habit| habit.name == self.selected_habit);

        if let Some(habit) = habit_cells {
            for cell in habit.cells[raw_cells].iter_mut() {
                let (rect, response) =
                    ui.allocate_exact_size(CELL_SIZE, Sense::click());
                ui.painter().rect_filled(rect, CELL_RADIUS, cell.color);

                if response.clicked() {
                    if cell.color == UNMARKED_CELL_COLOR {
                        cell.color = MARKED_CELL_COLOR;
                        ui.painter().rect_filled(rect, CELL_RADIUS, cell.color);
                    } else {
                        cell.color = UNMARKED_CELL_COLOR;
                        ui.painter().rect_filled(rect, CELL_RADIUS, cell.color);
                    }
                }
            }
        }
    }

    fn _display_days_columns(_ui: &mut Ui) {}
    fn _display_months_raw(_ui: &mut Ui) {}
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
            ui.heading("TODO");
            ui.horizontal(|ui| {
                // custom
                ui.add_space(40.);
                ui.label("Jan");
                ui.add_space(30.);
                ui.label("Feb");
                ui.add_space(30.);
                ui.label("Mar");
                ui.add_space(30.);
                ui.label("Apr");
                ui.add_space(30.);
                ui.label("May");
                ui.add_space(30.);
                ui.label("Jun");
                ui.add_space(30.);
                ui.label("Jul");
                ui.add_space(30.);
                ui.label("Aug");
                ui.add_space(30.);
                ui.label("Sep");
                ui.add_space(30.);
                ui.label("Nov");
                ui.add_space(30.);
                ui.label("Dec");
            });

            // NOTE i want each mounth have it's own cells
            ui.scope(|ui| {
                HabitTracker::overide_cells_spacing(ui);
                ui.vertical(|ui| {
                    for (raw_cells, day) in DAYS.iter().enumerate() {
                        ui.horizontal(|ui| {
                            ui.add_sized(DAY_LABEL_SIZE, Label::new(*day));
                            self.dispaly_central_panel_raw_cells(ui, raw_cells);
                        });
                    }
                });
            });
        });
    }
}
