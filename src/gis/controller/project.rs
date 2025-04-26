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

//! Project menu controller module.

use gtk::{prelude::*, FileChooserAction, FileChooserDialog, glib::current_dir};
use crate::gis::{get_project_context, config};
use std::path::PathBuf;

/// Responsible for handling "Project" menu items actions.
#[derive(Default)]
pub struct ProjectController;

impl ProjectController {
    /// Construct new project controller.
    ///
    /// # Returns
    /// - New `ProjectController` object.
    pub fn new() -> Self {
        Self::default()
    }

    /// "New" menu item handler.
    pub fn create_new_project(&self) {
        // Create a new file chooser dialog.
        let dialog = FileChooserDialog::new::<gtk::Window>(
            Some("Open File"), None, FileChooserAction::Save,
        );

        // Add buttons to the dialog.
        dialog.add_button("Create", gtk::ResponseType::Ok.into());
        dialog.add_button("Cancel", gtk::ResponseType::Cancel.into());

        // Set the current directory for the dialog.
        dialog.set_current_folder(&current_dir());
        dialog.show();

        // Handle the response from the dialog.
        dialog.connect_response(move |dialog, response| {
            if response == gtk::ResponseType::Ok {
                if let Some(file) = dialog.filename() {
                    // Add project file extension in the end.
                    let path = format!(
                        "{}{}", file.to_str().unwrap(), config::APP_FILE_EXT
                    );

                    // Save project data file to project context.
                    let mut context = get_project_context();
                    context.project_file = PathBuf::from(path);
                }
            }
            dialog.close();
        });
    }

    /// "Open" menu item handler.
    pub fn open_project(&self) {
        todo!()
    }

    /// "Save" menu item handler.
    pub fn save_project(&self) {
        todo!()
    }

    /// "Exit" menu item handler.
    pub fn exit_project(&self) {
        std::process::exit(0);
    }
}
