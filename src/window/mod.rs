// This file is part of Memories.
//
// Copyright (c) 2024 Max Rodriguez
// All rights reserved.
//
// Memories is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Memories is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Memories.  If not, see <http://www.gnu.org/licenses/>.
//
// SPDX-License-Identifier: GPL-3.0-or-later

mod imp;
pub mod theme_selector;

use crate::application::MrsApplication;
use crate::preferences::MrsPreferencesDialog;
use adw::prelude::*;
use adw::subclass::prelude::*;
use gtk::{gio, glib};

glib::wrapper! {
    pub struct MrsApplicationWindow(ObjectSubclass<imp::MrsApplicationWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow, adw::ApplicationWindow,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Root;
}

#[gtk::template_callbacks]
impl MrsApplicationWindow {
    pub fn new(application: &MrsApplication) -> Self {
        glib::Object::builder()
            .property("application", application)
            .build()
    }

    pub fn app(&self) -> Option<MrsApplication> {
        self.application().and_downcast()
    }

    fn setup_gactions(&self) {
        let preferences_action = gio::ActionEntry::builder("preferences")
            .activate(move |win: &Self, _, _| {
                MrsPreferencesDialog::default().present(win);
            })
            .build();

        let shortcuts_window_action = gio::ActionEntry::builder("show-help-overlay")
            .activate(move |win: &Self, _, _| {
                // GActions are setup after constructor, which guarantees that
                // the help overlay is setup for the window, so we can unwrap().
                win.help_overlay().unwrap().present();
            })
            .build();

        self.add_action_entries([preferences_action, shortcuts_window_action]);
    }

    #[template_callback]
    fn master_stack_child_visible(&self) {
        let media_grid_imp = self.imp().library_view.imp().media_grid.imp();

        if let Some(child_name) = self.imp().master_stack.visible_child_name() {
            if child_name == self.imp().library_page.name().unwrap() {
                // If the photo grid has no model, load the photo library now.
                if media_grid_imp.photo_grid_view.model().is_none() {
                    self.imp().library_view.load_library();
                }
            }
        }
    }
}
