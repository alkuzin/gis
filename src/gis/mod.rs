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

//! GIS main module.

use gtk::{prelude::*, Application, ApplicationWindow};

pub mod config;

/// Application main window wrapper struct.
pub struct MainWindow {
    /// Application main window.
    window: ApplicationWindow,
}

impl MainWindow {
    /// Construct new main window.
    ///
    /// # Parameters
    /// - `app` - given application lifecycle manager.
    ///
    /// # Returns
    /// - New `MainWindow` object.
    pub fn new(app: &Application) -> Self {
        let window = ApplicationWindow::new(app);
        window.set_title(config::APP_TITLE);
        window.set_default_size(config::APP_WIDTH, config::APP_HEIGHT);

        Self { window }
    }

    /// Display all widgets associated with main window.
    pub fn show(&self) {
        // Show all widgets.
        self.window.show_all();
    }
}