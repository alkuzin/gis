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
        todo!()
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
