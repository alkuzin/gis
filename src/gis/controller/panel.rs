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

//! Main panel controller module.

use crate::gis::controller::{MapController, MenuController};
use gtk::{prelude::*, Box as GtkBox};

/// Responsible for TODO:
pub struct PanelController {
    /// Application menu controller.
    menu_controller: MenuController,
    /// Application map controller.
    map_controller: MapController,
    /// Layout that holds the control & menu layouts.
    main_layout: GtkBox,
    /// Layout that holds the control panel & map viewer.
    control_layout: GtkBox,
    // TODO: implement dashboard_controller & dashboard_view.
}

impl PanelController {
    /// Construct new panel controller.
    ///
    /// # Returns
    /// - New `PanelController` object.
    pub fn new() -> Self {
        let menu_controller = MenuController::new();
        let map_controller  = MapController::new();
        let main_layout     = GtkBox::new(gtk::Orientation::Vertical, 0);
        let control_layout  = GtkBox::new(gtk::Orientation::Horizontal, 0);

        Self { menu_controller, map_controller, main_layout, control_layout }
    }

    /// Initialize main panel.
    pub fn init(&mut self) {
        // Set controllers.
        self.menu_controller.init();
        self.map_controller.init();

        // Set control layout.
        self.control_layout.add(self.map_controller.layout());

        // Set main layout.
        self.main_layout.add(self.menu_controller.layout());
        self.main_layout.add(&self.control_layout);
    }

    /// Get panel layout.
    ///
    /// # Returns
    /// - Main panel horizontal layout.
    #[inline(always)]
    pub fn layout(&self) -> &GtkBox {
        &self.main_layout
    }
}
