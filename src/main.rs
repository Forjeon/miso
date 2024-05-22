mod custom_button;

use custom_button::CustomButton;
use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, Button, PopoverMenuBar};
use gtk::gio::{ActionEntry, Menu, MenuItem, MenuModel};


const APP_ID: &str = "com.forjeon.miso";


fn main() -> glib::ExitCode {
	// Create the MISO GTK-RS application
	let app = Application::builder().application_id(APP_ID).build();
	app.connect_activate(build_ui);

	// Keyboard accelerators (shortcut hotkeys)
	// TODO: HERE
	//app.set_accels_for_action("ACTIONNAME", &["ACTION KEYBOARD SHORTCUT"]);
	/*
Ctrl+SPACE → start/pause/resume
Ctrl+Q → quit
Ctrl+J → Jump to selected phase in timer
Ctrl+N → create new phase
Ctrl+D → duplicate selected phase
Del → delete selected phase
	*/

	// Start MISO
	app.run()
}


fn build_ui(app: &Application) {
	// Create the Hello World button
	let button = CustomButton::new();
	button.set_margin_top(12);
	button.set_margin_bottom(12);
	button.set_margin_start(12);
	button.set_margin_end(12);
	/*
	let button = Button::builder()
		.label("Press Me!")
		.margin_top(12)
		.margin_bottom(12)
		.margin_start(12)
		.margin_end(12)
		.build();
	button.connect_clicked(|button| {
		button.set_label("Hellow World!");
	});
	*/

	// Build the menubar
		// File menu
	let file_menu = Menu::new();
	file_menu.insert_item(0, &MenuItem::new(Some("Save Modes..."), Some("file.save")));
	file_menu.insert_item(5, &MenuItem::new(Some("Load Modes..."), Some("file.load")));
		// Mode menu
	let mode_menu = Menu::new();
	mode_menu.insert_item(0, &MenuItem::new(Some("New Mode"), Some("mode.new")));
	mode_menu.insert_item(5, &MenuItem::new(Some("Delete Mode"), Some("mode.del")));
		// Menu model
	let menu = Menu::new();
	menu.insert_submenu(0, Some("File"), &file_menu);
	menu.insert_submenu(1, Some("Mode"), &mode_menu);
	let menu_model: MenuModel = menu.into();
		// Menubar
	let menubar = PopoverMenuBar::from_model(Some(&menu_model));
		// Window box to hold the menubar
	let top_window_box = gtk::Box::builder()
		.orientation(gtk::Orientation::Vertical)
		.build();
	top_window_box.append(&menubar);
		// Show button
	top_window_box.append(&button);
	
	// Top application window
	let top_window = ApplicationWindow::builder()
		.application(app)
		.title("MISO")
		.child(&top_window_box)
		.show_menubar(true)
		.build();

	// Menubar actions
	// TODO: HERE

	// Present the GUI
	top_window.present();
}
/*
File > Save Modes
File > Load Modes (happens automatically on startup)
Mode > New Mode
Mode > Delete Mode

Ctrl+SPACE → start/pause/resume
Ctrl+Q → quit
Ctrl+J → Jump to selected phase in timer
Ctrl+N → create new phase
Ctrl+D → duplicate selected phase
Del → delete selected phase

+------------------------------------------------------------------------------+
|     MISO                                                      ——    []     X |
+------+------+----------------------------------------------------------------+
| File | Mode |                                                                |
+----+-+------+----------------------------------------------------------------+
| \/ |   School 30 - Work 15 - Play 10                                         |
+----+--+--------+--------+-+--------------------------------------------------+
|  New  |  Jump  | Delete |O|                                                  |
+-------+-----+--+---+----+ |                    Work Phase                    |
| School      |  30  | .. | |                                                  |
+-------------+------+----+ |                      09:21                       |
| Work        |  15  | .. | |                                                  |
+-------------+------+----+ |                    +--------+                    |
| Play        |  10  | .. | |                    | Resume |                    |
+-------------+------+----+ |                    +--------+                    |
|                         | |                                                  |
|                         | |                    +--------+                    |
|                         | |                    |  Quit  |                    |
|                         | |                    +--------+                    |
|                         | |                                                  |
+-------------------------+-+--------------------------------------------------+
*/
