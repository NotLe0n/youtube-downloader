use egui::{TextEdit, ScrollArea};
use rfd::FileDialog;
use std::io::Read;
use std::process::{Command, Stdio};
use std::sync::mpsc;

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
	#[serde(skip)]
	channel: (mpsc::Sender<String>, mpsc::Receiver<String>),//(mpsc::Sender<ChildStdout>, mpsc::Receiver<ChildStdout>),
	#[serde(skip)]
	dl_output: String,
}

impl Default for GUI {
	fn default() -> Self {
		Self {
			dl_path: String::from(dirs::video_dir().unwrap().display().to_string()),
			dl_url: String::new(),
			dl_filename: String::new(),
			dl_ext: ".mp4".to_owned(),
			channel: mpsc::channel(),
			dl_output: String::new(),
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
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		let Self { dl_path, dl_url, dl_filename, dl_ext, channel, dl_output } = self;

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
						.set_directory(std::path::MAIN_SEPARATOR.to_string())
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
					download_button_clicked(dl_path, dl_filename, dl_ext, dl_url, channel);
				}
			});

			ui.collapsing("Output", |ui| {
				ScrollArea::vertical().stick_to_bottom().show(ui, |ui|{
					let (_, rx) = channel;
					let output = rx.try_recv();
					if !output.is_err() {
						*dl_output = output.unwrap();
					}

					ui.label(format!("{}", dl_output));
				});
			}).fully_open();
		});
	}
}

fn download_button_clicked(dl_path: &mut String, dl_filename: &mut String, dl_ext: &mut String, dl_url: &mut String, channel: &mut (mpsc::Sender<String>, mpsc::Receiver<String>)) {
    // add slash at the end of the path if it doesn't exist
    if !dl_path.ends_with(std::path::MAIN_SEPARATOR) {
		dl_path.push(std::path::MAIN_SEPARATOR);
	}
    let dl_filename_with_ext = format!("{}{}", dl_filename, dl_ext);
    let path = dl_path.clone();
    let url = dl_url.clone(); // cloning so rust doesn't scream at me
	
    // execute youtube-dl in new thread as to not block the ui
	use std::thread;
    let tx = channel.0.clone();
    thread::spawn(move || {
		let cmd = Command::new("./lib/youtube-dl")
			.current_dir(path)
			.args([
				format!("-o {}", dl_filename_with_ext),
				url
			])
			.stderr(Stdio::piped())
			.stdout(Stdio::piped())
			.spawn();

		if cmd.is_err() {
			tx.send(format!("Command Failed: {}", cmd.err().unwrap())).unwrap();
			return;
		}

		let stdout = cmd.unwrap().stdout.take().unwrap();
	
		let bytes = stdout.bytes();
		let mut str = String::new();

		for b in bytes {
			let chr = b.unwrap() as char;

			// newline every time a new progress update gets sent
			if chr == '[' && !str.is_empty() {
				tx.send(str.clone()).unwrap();
				str += "\n";
			}

			// add all chars together, except control characters like backspace
			if !chr.is_control() {
				str += chr.to_string().as_str();
			}
		}
	});
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