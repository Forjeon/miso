use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow};

const APP_ID: &str = "com.forjeon.miso";

fn main() -> glib::ExitCode {
	let app = Application::builder().application_id(APP_ID).build();
	app.connect_activate(build_ui);
	app.run()
}

fn build_ui(app: &Application) {
	let top_window = ApplicationWindow::builder()
		.application(app)
		.title("MISO")
		.build();

	// Build the menubar
	let menu = Menu::new();
	menu.insert_submenu(0, Some("File"), &file_menu);
	menu.insert_submenu(1, Some("Edit"), &edit_menu);
	let menu_model: MenuModel = menu.into();
	// Build the File menu

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
