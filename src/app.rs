
/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    functions_state: FunctionsState
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            functions_state: Default::default()
        }
    }
}

impl TemplateApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            let mut app: TemplateApp = eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
            app.functions_state = Default::default();
            return app;
        }

        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui: &mut egui::Ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.set_min_width(200.0);
                    self.functions_state.draw_all_execute(ui);
                });
                ui.separator();
                ui.vertical(|ui| {
                    self.functions_state.draw_all_edit(ui);
                });
            });
            
            ui.horizontal(|ui| {
                if ui.button("Execute").clicked() {
                    self.functions_state.execute();
                }

                if ui.button("Clear").clicked() {
                    self.functions_state.commands_execute.clear();
                }
            });

            for result in &mut self.functions_state.command_results {
                ui.label(result.to_owned());
            }
        });
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
struct FunctionsState {
    #[serde(skip)]
    pub commands_edit: [UserFunctions;2],
    pub commands_execute: Vec<UserFunctions>,
    pub command_results: Vec<String>
}

impl FunctionsState {
    fn draw_all_edit(&mut self, ui: &mut egui::Ui) {
        for command in &mut self.commands_edit {
            let user_function = command.draw_block(ui);

            if let Some(value) = user_function {
                self.commands_execute.push(value);
            }
        }
    }

    fn draw_all_execute(&self, ui: &mut egui::Ui) {
        for command in &self.commands_execute {
            ui.label(command.to_string());
        }
    }

    fn execute(&mut self) {
        self.command_results.clear();

        for command in &self.commands_execute {
            match command {
                UserFunctions::Print(value) => {
                    self.command_results.push(value.to_owned());
                },
                UserFunctions::Test(_) => {
                    self.command_results.push("This is just a text function! :>".to_owned());
                }
            }
        }
    }
}

impl Default for FunctionsState {
    fn default() -> Self {
        Self { 
            commands_edit: [
                UserFunctions::Print("".to_string()),
                UserFunctions::Test("".to_string())
            ],
            commands_execute: Vec::new(),
            command_results: Vec::new(),
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub enum UserFunctions {
    Print(String),
    Test(String),
}

impl UserFunctions {
    pub fn draw_block(&mut self, ui: &mut egui::Ui) -> Option<UserFunctions> {
        match self {
            UserFunctions::Print(value) => {
                let mut clicked: bool = false;

                ui.horizontal(|ui: &mut egui::Ui| {
                    ui.label("Print");
                    ui.text_edit_singleline(value);

                    clicked = ui.button("+").clicked();
                });

                if clicked { Some(UserFunctions::Print(value.to_owned())) } else { None }
            },
            UserFunctions::Test(value) => {
                let mut clicked: bool = false;

                ui.horizontal(|ui: &mut egui::Ui| {
                    ui.label("Test");
                    ui.text_edit_singleline(value);
                    
                    clicked = ui.button("+").clicked();
                });

                if clicked { Some(UserFunctions::Test(value.to_owned())) } else { None }
            }
        }
    }
}

impl std::fmt::Display for UserFunctions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserFunctions::Print(value) => write!(f, "Print: {}", value),
            UserFunctions::Test(value) => write!(f, "Test: {}", value)
        }
    }
}

impl Default for UserFunctions {
    fn default() -> Self {
        UserFunctions::Print("".to_string())
    }
}