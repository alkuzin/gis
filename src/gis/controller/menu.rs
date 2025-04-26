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

//! Application menu controller related declarations.

use crate::gis::{controller::ProjectController, view::MenuView};
use gtk::{prelude::*, Box as GtkBox};
use std::rc::Rc;

/// Responsible for handling user interactions with the menu UI.
pub struct MenuController {
    /// "Project" menu items handler manager.
    project_controller: Rc<ProjectController>,
    /// Menu UI manager.
    menu_view: MenuView,
}

impl MenuController {
    /// Construct new menu controller.
    ///
    /// # Returns
    /// - New `MenuController` object.
    pub fn new() -> Self {
        let project_controller = Default::default();
        let menu_view = MenuView::new();

        Self { project_controller, menu_view }
    }

    /// Initialize menu.
    pub fn init(&mut self) {
        self.menu_view.init();

        // Set menu items handlers.
        self.set_project_menu_handlers();
    }

    /// Get menu layout.
    ///
    /// # Returns
    /// - Menu items vertical layout.
    #[inline(always)]
    pub fn layout(&self) -> &GtkBox {
        self.menu_view.layout()
    }

    /// Set project menu items handlers.
    fn set_project_menu_handlers(&mut self) {
        let menu_items = &self.menu_view.items();

        // Set "New" menu item handler.
        let pc_clone = Rc::clone(&self.project_controller);
        menu_items[0].connect_activate(move |_| {
            pc_clone.create_new_project();
        });

        // Set "Open" menu item handler.
        let pc_clone = Rc::clone(&self.project_controller);
        menu_items[1].connect_activate(move |_| {
            pc_clone.open_project();
        });

        // Set "Save" menu item handler.
        let pc_clone = Rc::clone(&self.project_controller);
        menu_items[2].connect_activate(move |_| {
            pc_clone.save_project();
        });

        // Set "Exit" menu item handler.
        let pc_clone = Rc::clone(&self.project_controller);
        menu_items[3].connect_activate(move |_| {
            pc_clone.exit_project();
        });
    }
}