use std::{time::Duration};

use fltk::{prelude::*, *}; // Library for GUI
use rodio::{OutputStream, Sink, source::SineWave, Source}; // Library for audio playback
mod ui; // reference to ui.rs


fn main() {
	let app = app::App::default();
	let mut ui = ui::UserInterface::make_window(); // create a window

	//function that plays when the button is pressed
	ui.but.set_callback(move |_| {

	});
	app.run().unwrap();
}