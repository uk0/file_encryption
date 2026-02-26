use eframe::egui;
use file_encryption::crypto;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([520.0, 480.0])
            .with_min_inner_size([420.0, 400.0])
            .with_title("File Encryption Tool"),
        ..Default::default()
    };
    eframe::run_native(
        "File Encryption Tool",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_visuals(egui::Visuals::dark());
            Ok(Box::new(App::default()))
        }),
    )
}

#[derive(PartialEq, Clone, Copy)]
enum Mode {
    Encrypt,
    Decrypt,
}

#[derive(PartialEq, Clone, Copy)]
enum Platform {
    MacOS,
    Windows,
    Linux,
    LinuxARM64,
}

impl Platform {
    fn id(&self) -> i8 {
        match self {
            Platform::MacOS => 1,
            Platform::Windows => 2,
            Platform::Linux => 3,
            Platform::LinuxARM64 => 4,
        }
    }

    fn label(&self) -> &'static str {
        match self {
            Platform::MacOS => "macOS (x86/ARM)",
            Platform::Windows => "Windows (x86_64)",
            Platform::Linux => "Linux (x86_64)",
            Platform::LinuxARM64 => "Linux (ARM64)",
        }
    }

    fn is_windows(&self) -> bool {
        matches!(self, Platform::Windows)
    }
}

struct App {
    mode: Mode,
    input_file: String,
    output_dir: String,
    platform: Platform,
    password: String,
    password_confirm: String,
    bin_dir: String,
    status: Arc<Mutex<Status>>,
}

struct Status {
    message: String,
    is_error: bool,
    is_busy: bool,
}

impl Default for App {
    fn default() -> Self {
        // Default bin_dir: look for bin/ next to the executable
        let exe_dir = std::env::current_exe()
            .ok()
            .and_then(|p| p.parent().map(|d| d.to_path_buf()))
            .unwrap_or_default();
        let bin_dir = exe_dir.join("bin");
        let bin_str = if bin_dir.exists() {
            bin_dir.to_string_lossy().to_string()
        } else {
            exe_dir.to_string_lossy().to_string()
        };

        Self {
            mode: Mode::Encrypt,
            input_file: String::new(),
            output_dir: String::new(),
            platform: Platform::MacOS,
            password: String::new(),
            password_confirm: String::new(),
            bin_dir: bin_str,
            status: Arc::new(Mutex::new(Status {
                message: "Ready".into(),
                is_error: false,
                is_busy: false,
            })),
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("File Encryption Tool");
            ui.add_space(8.0);

            // Mode selector
            ui.horizontal(|ui| {
                ui.label("Mode:");
                ui.selectable_value(&mut self.mode, Mode::Encrypt, "Encrypt");
                ui.selectable_value(&mut self.mode, Mode::Decrypt, "Decrypt");
            });

            ui.add_space(12.0);
            ui.separator();
            ui.add_space(8.0);

            match self.mode {
                Mode::Encrypt => self.draw_encrypt_ui(ui),
                Mode::Decrypt => self.draw_decrypt_ui(ui),
            }

            ui.add_space(12.0);
            ui.separator();
            ui.add_space(8.0);

            // Status bar
            let status = self.status.lock().unwrap();
            let color = if status.is_error {
                egui::Color32::from_rgb(255, 100, 100)
            } else if status.is_busy {
                egui::Color32::from_rgb(255, 200, 80)
            } else {
                egui::Color32::from_rgb(100, 255, 100)
            };
            ui.colored_label(color, &status.message);
        });

        // Repaint while busy
        if self.status.lock().unwrap().is_busy {
            ctx.request_repaint();
        }
    }
}

impl App {
    fn draw_encrypt_ui(&mut self, ui: &mut egui::Ui) {
        // Input file
        ui.horizontal(|ui| {
            ui.label("File:       ");
            ui.add(egui::TextEdit::singleline(&mut self.input_file).desired_width(300.0));
            if ui.button("Browse...").clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_file() {
                    self.input_file = path.to_string_lossy().to_string();
                }
            }
        });

        ui.add_space(4.0);

        // Output directory
        ui.horizontal(|ui| {
            ui.label("Output:   ");
            ui.add(egui::TextEdit::singleline(&mut self.output_dir).desired_width(300.0));
            if ui.button("Browse...").clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_folder() {
                    self.output_dir = path.to_string_lossy().to_string();
                }
            }
        });

        ui.add_space(4.0);

        // Bin directory (where platform stubs live)
        ui.horizontal(|ui| {
            ui.label("Bin Dir:   ");
            ui.add(egui::TextEdit::singleline(&mut self.bin_dir).desired_width(300.0));
            if ui.button("Browse...").clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_folder() {
                    self.bin_dir = path.to_string_lossy().to_string();
                }
            }
        });

        ui.add_space(4.0);

        // Platform
        ui.horizontal(|ui| {
            ui.label("Platform:");
            egui::ComboBox::from_id_salt("platform")
                .selected_text(self.platform.label())
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.platform, Platform::MacOS, "macOS (x86/ARM)");
                    ui.selectable_value(&mut self.platform, Platform::Windows, "Windows (x86_64)");
                    ui.selectable_value(&mut self.platform, Platform::Linux, "Linux (x86_64)");
                    ui.selectable_value(&mut self.platform, Platform::LinuxARM64, "Linux (ARM64)");
                });
        });

        ui.add_space(4.0);

        // Password
        ui.horizontal(|ui| {
            ui.label("Password:");
            ui.add(egui::TextEdit::singleline(&mut self.password)
                .password(true)
                .desired_width(250.0));
        });

        ui.horizontal(|ui| {
            ui.label("Confirm:  ");
            ui.add(egui::TextEdit::singleline(&mut self.password_confirm)
                .password(true)
                .desired_width(250.0));
        });

        ui.add_space(12.0);

        let is_busy = self.status.lock().unwrap().is_busy;
        let enabled = !is_busy
            && !self.input_file.is_empty()
            && !self.output_dir.is_empty()
            && !self.password.is_empty()
            && self.password == self.password_confirm;

        if ui.add_enabled(enabled, egui::Button::new("Encrypt").min_size(egui::vec2(100.0, 30.0))).clicked() {
            self.do_encrypt();
        }

        if !self.password.is_empty()
            && !self.password_confirm.is_empty()
            && self.password != self.password_confirm
        {
            ui.colored_label(egui::Color32::from_rgb(255, 100, 100), "Passwords do not match");
        }
    }

    fn draw_decrypt_ui(&mut self, ui: &mut egui::Ui) {
        // Encrypted file
        ui.horizontal(|ui| {
            ui.label("File:       ");
            ui.add(egui::TextEdit::singleline(&mut self.input_file).desired_width(300.0));
            if ui.button("Browse...").clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_file() {
                    self.input_file = path.to_string_lossy().to_string();
                }
            }
        });

        ui.add_space(4.0);

        // Output directory
        ui.horizontal(|ui| {
            ui.label("Output:   ");
            ui.add(egui::TextEdit::singleline(&mut self.output_dir).desired_width(300.0));
            if ui.button("Browse...").clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_folder() {
                    self.output_dir = path.to_string_lossy().to_string();
                }
            }
        });

        ui.add_space(4.0);

        // Password
        ui.horizontal(|ui| {
            ui.label("Password:");
            ui.add(egui::TextEdit::singleline(&mut self.password)
                .password(true)
                .desired_width(250.0));
        });

        ui.add_space(12.0);

        let is_busy = self.status.lock().unwrap().is_busy;
        let enabled = !is_busy
            && !self.input_file.is_empty()
            && !self.output_dir.is_empty()
            && !self.password.is_empty();

        if ui.add_enabled(enabled, egui::Button::new("Decrypt").min_size(egui::vec2(100.0, 30.0))).clicked() {
            self.do_decrypt();
        }
    }

    fn do_encrypt(&mut self) {
        let input = self.input_file.clone();
        let output = self.output_dir.clone();
        let password = self.password.clone();
        let platform = self.platform;
        let bin_dir = self.bin_dir.clone();
        let status = Arc::clone(&self.status);

        let binary_name = crypto::get_platform_binary(platform.id());
        let binary_path = PathBuf::from(&bin_dir).join(binary_name);
        let binary_path_str = binary_path.to_string_lossy().to_string();

        {
            let mut s = status.lock().unwrap();
            s.message = "Encrypting...".into();
            s.is_error = false;
            s.is_busy = true;
        }

        thread::spawn(move || {
            let result = crypto::encrypt_file(
                &input,
                &password,
                &output,
                &binary_path_str,
                platform.is_windows(),
            );
            let mut s = status.lock().unwrap();
            s.is_busy = false;
            match result {
                Ok(name) => {
                    s.message = format!("Encrypted: {}", name);
                    s.is_error = false;
                }
                Err(e) => {
                    s.message = format!("Error: {}", e);
                    s.is_error = true;
                }
            }
        });
    }

    fn do_decrypt(&mut self) {
        let input = self.input_file.clone();
        let output = self.output_dir.clone();
        let password = self.password.clone();
        let status = Arc::clone(&self.status);

        {
            let mut s = status.lock().unwrap();
            s.message = "Decrypting...".into();
            s.is_error = false;
            s.is_busy = true;
        }

        thread::spawn(move || {
            let result = crypto::decrypt_and_save(&input, &password, &output);
            let mut s = status.lock().unwrap();
            s.is_busy = false;
            match result {
                Ok(path) => {
                    s.message = format!("Decrypted: {}", path);
                    s.is_error = false;
                }
                Err(e) => {
                    s.message = format!("Error: {}", e);
                    s.is_error = true;
                }
            }
        });
    }
}
