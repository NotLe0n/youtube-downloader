use egui::TextEdit;
use rfd::*;
use cmd_lib::*;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct GUI {
	// Example stuff:
	dl_path: String,
	dl_ext: String,

	// this how you opt-out of serialization of a member
	#[serde(skip)]
	dl_url: String,
	#[serde(skip)]
	dl_filename: String,
}

impl Default for GUI {
	fn default() -> Self {
		Self {
			// Example stuff:
			dl_path: "~/Videos/".to_owned(),
			dl_url: "".to_owned(),
			dl_filename: "".to_owned(),
			dl_ext: ".mp4".to_owned(),
		}
	}
}

impl GUI {
	/// Called once before the first frame.
	pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
		// This is also where you can customized the look at feel of egui using
		// `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

		// Load previous app state (if any).
		// Note that you must enable the `persistence` feature for this to work.
		if let Some(storage) = cc.storage {
			return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
		}

		Default::default()
	}
}

impl eframe::App for GUI {
	/// Called by the frame work to save state before shutdown.
	fn save(&mut self, storage: &mut dyn eframe::Storage) {
		eframe::set_value(storage, eframe::APP_KEY, self);
	}

	/// Called each time the UI needs repainting, which may be many times per second.
	fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
		let Self { dl_path, dl_url, dl_filename, dl_ext } = self;

		egui::CentralPanel::default().show(ctx, |ui| {
			ui.heading("YouTube downloader");
			egui::warn_if_debug_build(ui);

			// input box to enter the link
			ui.horizontal(|ui| {
				ui.label("YouTube link:");
				ui.add_sized([390.0, 20.0], TextEdit::singleline(dl_url));
			});
			
			// input box with button to select the directory
			ui.horizontal(|ui| {
				ui.label("Output path:");
				ui.text_edit_singleline(dl_path);
				if ui.button("Select Directory").clicked() {
					// open File Dialog
					let folder = FileDialog::new()
						.set_directory("/")
						.pick_folder();
					
					if folder.is_some() {
						let folder_path: String = folder.unwrap().display().to_string();
						*dl_path = folder_path; // set dl_path to the selected folder path
					}
				}
			});

			// input box to enter the file name
			ui.horizontal(|ui| {
				ui.label("Output file name:");
				ui.add_sized([180.0, 20.0], TextEdit::singleline(dl_filename));
			});

			// Combobox to select the file extension
			ui.horizontal(|ui| {
				ext_input(ui, dl_ext);
			});

			ui.horizontal(|ui| {
				// Download button, which executes youtube-dl
				if ui.button("Download").clicked() {
					// add slash at the end of the path if it doesn't exist
					if !dl_path.ends_with('/') {
						dl_path.push('/');
					}
	
					let final_dl_path = format!("{}{}{}", dl_path, dl_filename, dl_ext);
					let url = dl_url.clone(); // cloning so rust doesn't scream at me

					// execute youtube-dl in new thread as to not block the ui
					use std::thread;
					
					thread::spawn(move || {
						let output = spawn_with_output!(youtube-dl -o $final_dl_path $url).expect("command failed")
							.wait_with_output()
							.unwrap();
					});
					//run_cmd!(youtube-dl -o $final_dl_path $url).expect("command failed");
				}
				
				ui.add_sized([150.0, 10.0], egui::widgets::ProgressBar::new(0.5));
			});
			ui.collapsing("Output", |ui| {
				ui.label("output here");
			});
		});
	}
}

fn ext_input(ui: &mut egui::Ui, dl_ext: &mut String) {
    ui.label("Output file extension:");
	egui::ComboBox::from_label("")
		.selected_text(format!("{}", dl_ext))
		.width(80.0)
		.show_ui(ui, |ui| {
			for ext in [".mp4", ".m4a", ".webm", ".flv", ".mp3", ".wav", ".aac", ".3gp"] {
				// Add all items
				ui.selectable_value(dl_ext, ext.to_owned(), ext);
			}			
		}
	);
}