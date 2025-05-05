// GIS - Geographic information system.
// Copyright (C) 2025 Alexander (@alkuzin).
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

//! GIS entry point.

mod gis;

use gtk::{prelude::*, Application, glib::ExitCode};
use gis::{config, MainWindow};

fn main() -> ExitCode {
    // Set up an application.
    let flags = Default::default();
    let app   = Application::new(Some(config::APP_ID), flags);

    // Launch an application.
    app.connect_activate(move |app| {
        let mut main_window = MainWindow::new(app);
        main_window.show();
    });

    app.run()
}