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

//! Application inter-thread communication module.

use std::sync::{
    mpsc::{self, Receiver, Sender}, Arc, Mutex, MutexGuard, LazyLock
};

/// Enumeration of GIS actions.
pub enum GISAction {
    /// Signal to update map image.
    MapImageUpdated
}

/// GIS global communication channel type.
pub type GISChannel = (
    Arc<Mutex<Sender<GISAction>>>,
    Arc<Mutex<Receiver<GISAction>>>
);

/// Global communication channel instance.
static GLOBAL_CHANNEL: LazyLock<Mutex<GISChannel>> = LazyLock::new(|| {
    let (sender, receiver) = mpsc::channel();
    Mutex::new((Arc::new(Mutex::new(sender)), Arc::new(Mutex::new(receiver))))
});

/// Get global channel instance.
///
/// # Returns
/// - Mutex guard for global channel sender & receiver tuple.
pub fn get_global_channel() -> MutexGuard<'static, GISChannel> {
    GLOBAL_CHANNEL.lock().unwrap()
}

/// Get the global sender.
///
/// # Returns
/// - A locked reference to the Sender.
pub fn get_global_sender() -> Arc<Mutex<Sender<GISAction>>> {
    let channel = get_global_channel();
    channel.0.clone()
}

/// Get the global receiver.
///
/// # Returns
/// - A locked reference to the Receiver.
pub fn get_global_receiver() -> Arc<Mutex<Receiver<GISAction>>> {
    let channel = get_global_channel();
    channel.1.clone()
}
