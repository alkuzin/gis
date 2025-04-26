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

pub mod config;
mod view;
mod controller;

use gtk::{prelude::*, Application, ApplicationWindow};
use controller::MenuController;

/// Application main window wrapper struct.
pub struct MainWindow {
    /// Application main window.
    window: ApplicationWindow,
    /// Application menu controller.
    menu_controller: MenuController,
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
        let menu_controller = MenuController::new();

        Self { window, menu_controller }
    }

    /// Display all widgets associated with main window.
    pub fn show(&mut self) {
        // Initialize main window.
        self.window.set_title(config::APP_TITLE);
        self.window.set_default_size(config::APP_WIDTH, config::APP_HEIGHT);
        self.menu_controller.init();
        self.window.add(self.menu_controller.layout());

        // Show all widgets.
        self.window.show_all();
    }
}