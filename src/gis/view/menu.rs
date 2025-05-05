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

//! Application menu view related declarations.

use gtk::{prelude::*, Box as GtkBox, Menu, MenuBar, MenuItem};
use std::rc::Rc;

/// Responsible for managing the menu system UI.
pub struct MenuView {
    /// Layout that holds the menu bar.
    layout: GtkBox,
    /// Bar that holds menu buttons.
    menu_bar: MenuBar,
    /// List of menu items.
    menu_items: Vec<Rc<MenuItem>>,
}

impl MenuView {
    /// Construct new menu view.
    ///
    /// # Returns
    /// - New `MenuView` object.
    pub fn new() -> Self {
        let layout     = GtkBox::new(gtk::Orientation::Vertical, 0);
        let menu_bar   = MenuBar::new();
        let menu_items = Vec::default();

        Self { layout, menu_bar, menu_items }
    }

    /// Initialize menu.
    pub fn init(&mut self) {
        self.set_project_menu();
        self.set_map_menu();
        self.layout.add(&self.menu_bar);
    }

    /// Get menu layout.
    ///
    /// # Returns
    /// - Menu items vertical layout.
    #[inline(always)]
    pub fn layout(&self) -> &GtkBox {
        &self.layout
    }

    /// Get menu items.
    ///
    /// # Returns
    /// - List of menu items.
    #[inline(always)]
    pub fn items(&self) -> &Vec<Rc<MenuItem>> {
        &self.menu_items
    }

    /// Set "Project" menu.
    fn set_project_menu(&mut self) {
        // Create "Project" menu.
        let project_menu_item = MenuItem::with_label("Project");
        let project_menu      = Menu::new();

        // Set project menu items.
        let new_item  = MenuItem::with_label("New");
        let open_item = MenuItem::with_label("Open");
        let save_item = MenuItem::with_label("Save");
        let exit_item = MenuItem::with_label("Exit");

        project_menu.append(&new_item);
        project_menu.append(&open_item);
        project_menu.append(&save_item);
        project_menu.append(&exit_item);

        self.menu_items.push(Rc::new(new_item));
        self.menu_items.push(Rc::new(open_item));
        self.menu_items.push(Rc::new(save_item));
        self.menu_items.push(Rc::new(exit_item));

        // Add project menu to menu bar.
        project_menu_item.set_submenu(Some(&project_menu));
        self.menu_bar.add(&project_menu_item);
    }

    /// Set "Map" menu.
    fn set_map_menu(&mut self) {
        // Create "Project" menu.
        let map_menu_item = MenuItem::with_label("Map");
        let map_menu      = Menu::new();

        // Set project menu items.
        let new_item = MenuItem::with_label("New");
        map_menu.append(&new_item);
        self.menu_items.push(Rc::new(new_item));

        // Add project menu to menu bar.
        map_menu_item.set_submenu(Some(&map_menu));
        self.menu_bar.add(&map_menu_item);
    }
}
