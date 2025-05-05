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

//! Map controller module.

use crate::gis::view::MapView;
use gtk::Box as GtkBox;

/// Responsible for handling user interactions with map UI.
#[derive(Clone)]
pub struct MapController {
    /// Map UI manager.
    map_view: MapView,
}

impl MapController {
    /// Construct new panel controller.
    ///
    /// # Returns
    /// - New `PanelController` object.
    pub fn new() -> Self {
        let map_view = MapView::new();
        Self { map_view }
    }

    /// Initialize map controller.
    pub fn init(&mut self) {
        self.map_view.init();
    }

    /// Update map.
    pub fn update(&self) {
        self.map_view.update_image();
    }

    /// Get map layout.
    ///
    /// # Returns
    /// - Map horizontal layout.
    #[inline(always)]
    pub fn layout(&self) -> &GtkBox {
        self.map_view.layout()
    }
}
