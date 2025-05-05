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

//! "Map" menu handlers.

use gtk::{
    FileChooserAction, FileChooserDialog, FileFilter,
    prelude::*, glib::current_dir
};
use crate::gis::model::get_project_context;
use std::path::PathBuf;
use std::rc::Rc;
use crate::gis::controller::MapController;

/// "New" menu item handler.
pub fn new(map_controller: Rc<MapController>) {
    // Create a new file chooser dialog.
    let dialog = FileChooserDialog::new::<gtk::Window>(
        Some("Open File"), None, FileChooserAction::Save,
    );

    // Create a filter for image files.
    let image_filter = FileFilter::new();
    image_filter.set_name(Some("Image Files"));
    image_filter.add_mime_type("image/jpeg");
    image_filter.add_mime_type("image/png");
    image_filter.add_mime_type("image/gif");
    image_filter.add_mime_type("image/bmp");
    image_filter.add_mime_type("image/tiff");
    dialog.add_filter(image_filter);

    // Create a filter for all files.
    let all_files_filter = FileFilter::new();
    all_files_filter.set_name(Some("All Files"));
    all_files_filter.add_pattern("*.*");
    dialog.add_filter(all_files_filter);

    // Add buttons to the dialog.
    dialog.add_button("Select map", gtk::ResponseType::Ok);
    dialog.add_button("Cancel", gtk::ResponseType::Cancel);

    // Set the current directory for the dialog.
    dialog.set_current_folder(current_dir());
    dialog.show();

    // Handle the response from the dialog.
    dialog.connect_response(move |dialog, response| {
        if response == gtk::ResponseType::Ok {
            if let Some(path) = dialog.filename() {
                // Save project map file path to project context.
                let mut context = get_project_context();
                context.map_image_file = PathBuf::from(path);

                // Manually drop mutex guard in order to update map image.
                drop(context);
                map_controller.update();
            }
        }
        dialog.close();
    });
}
