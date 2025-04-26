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

//! Data and business logic of the application main module.

use std::{path::PathBuf, sync::{LazyLock, Mutex, MutexGuard}};

/// Project data struct.
#[derive(Debug, Default)]
pub struct ProjectContext {
    /// Path to project data file.
    pub project_file: PathBuf,
}

/// Project context instance.
static PROJECT_CONTEXT: LazyLock<Mutex<ProjectContext>> = LazyLock::new(|| {
    Mutex::new(ProjectContext::default())
});

/// Get project context instance.
///
/// # Returns
/// - Mutex guard for ProjectContext.
pub fn get_project_context() -> MutexGuard<'static, ProjectContext> {
    PROJECT_CONTEXT.lock().unwrap()
}