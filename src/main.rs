#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() {
	// Log to stdout (if you run with `RUST_LOG=debug`).
	tracing_subscriber::fmt::init();

	use egui::Vec2;
	let options = eframe::NativeOptions {
		always_on_top: false,
		maximized: false,
		decorated: true,
		drag_and_drop_support: true,
		icon_data: None,
		initial_window_pos: None,
		initial_window_size: Option::from(Vec2::new(500.0, 200.0)),
		min_window_size: Option::from(Vec2::new(500.0, 150.0)),
		max_window_size: None,
		resizable: true,
		transparent: false,
		vsync: true,
		multisampling: 0,
		depth_buffer: 0,
		stencil_buffer: 0,
	};
	
	eframe::run_native(
		"YouTube Downloader",
		options,
		Box::new(|cc| Box::new(youtube_downloader::GUI::new(cc))),
	);
}
