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
pub mod model;
mod view;
mod controller;

use gtk::{prelude::*, glib::ControlFlow, Application, ApplicationWindow};
use controller::PanelController;
use model::GISAction;

/// Application main window wrapper struct.
pub struct MainWindow {
    /// Application main pane; controller.
    panel_controller: PanelController,
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
    pub fn new(app: &Application) -> Self
    {
        let main_window = Self {
            panel_controller: PanelController::new(),
            window:           ApplicationWindow::new(app),
        };

        let receiver       = model::get_global_receiver();
        let receiver_clone = receiver.clone();

        // Spawn a thread to listen for messages.
        std::thread::spawn(move || {
            let receiver_guard = receiver_clone.lock().unwrap();

            for action in receiver_guard.iter() {
                match action {
                    // Update map image.
                    GISAction::MapImageUpdated => {
                        // Use GTK's main thread for UI updates.
                        gtk::glib::idle_add(move || {
                            // TODO: Update UI here.
                            println!("Map image updated!");
                            ControlFlow::Break
                        });
                    }
                }
            }
        });

        main_window
    }

    /// Display all widgets associated with main window.
    pub fn show(&mut self) {
        // Initialize main window.
        self.window.set_title(config::APP_TITLE);
        self.window.set_default_size(config::APP_WIDTH, config::APP_HEIGHT);
        self.panel_controller.init();
        self.window.add(self.panel_controller.layout());

        // Show all widgets.
        self.window.show_all();
    }
}