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

//! Map view related declarations.

use gtk::Box as GtkBox;

/// Responsible for managing the map UI.
pub struct MapView {
    /// Layout that holds the map.
    layout: GtkBox,
}

impl MapView {
    /// Construct new map view.
    ///
    /// # Returns
    /// - New `MapView` object.
    pub fn new() -> Self {
        let layout = GtkBox::new(gtk::Orientation::Vertical, 0);
        Self { layout }
    }

    /// Initialize menu.
    pub fn init(&mut self) {
        // TODO: get map image path from project context.
        // let image = gtk::Image::from_file(path_from_project_context);
        // self.layout.add(&image);
    }

    /// Get menu layout.
    ///
    /// # Returns
    /// - Menu items vertical layout.
    #[inline(always)]
    pub fn layout(&self) -> &GtkBox {
        &self.layout
    }
}
