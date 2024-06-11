// This file is part of Albums.
//
// Copyright (c) 2024 Max Rodriguez
// All rights reserved.
//
// Albums is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Albums is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Albums.  If not, see <http://www.gnu.org/licenses/>.
//
// SPDX-License-Identifier: GPL-3.0-or-later

mod imp;

use crate::window::AlbumsApplicationWindow;
use adw::prelude::*;
use adw::subclass::prelude::*;
use gettextrs::gettext;
use glib::{clone, g_debug, g_error};
use gtk::{gdk, gio, glib};

/// Enum that represents the types of content that
/// can be displayed by the `AlbumsViewer` object.
#[derive(Debug)]
pub enum ViewerContentType {
    Renderable,
    Image,
    Video,
    Invalid,
}

glib::wrapper! {
    pub struct AlbumsViewer(ObjectSubclass<imp::AlbumsViewer>)
        @extends gtk::Widget, adw::Bin;
}

#[gtk::template_callbacks]
impl AlbumsViewer {
    pub fn new() -> Self {
        glib::Object::new()
    }

    fn window(&self) -> AlbumsApplicationWindow {
        self.root()
            .expect("Must be in a GtkApplicationWindow.")
            .downcast()
            .expect("Failed to downcast to AlbumsApplicationWindow.")
    }

    /// Sets the content type setting for the viewer page.
    /// The `ViewerContentType` enum given directly correlates
    /// to a stack page that has the proper widget for the content.
    pub fn set_content_type(&self, content_type: &ViewerContentType) {
        match content_type {
            ViewerContentType::Renderable => self.imp().viewer_stack.set_visible_child_name("image"),
            ViewerContentType::Image => self.imp().viewer_stack.set_visible_child_name("image"),
            ViewerContentType::Video => self.imp().viewer_stack.set_visible_child_name("video"),
            _ => g_debug!("Viewer", "Received invalid ViewerContentType enum!"),
        }
    }

    pub fn set_content_file(&self, file: &gio::File) {
        match self.imp().viewer_stack.visible_child_name().unwrap().as_str() {
            "render" => self.imp().viewer_picture.set_file(Some(file)),
            "image" => {
                glib::spawn_future_local(clone!(@weak self as viewer, @strong file => async move {
                    let image: glycin::Image = glycin::Loader::new(file).load().await.expect("FIXME");
                    let texture: gdk::Texture = image.next_frame().await.expect("FIXME").texture;

                    viewer.imp().viewer_picture.set_paintable(Some(&texture));
                }));
            }
            "video" => self.imp().viewer_video.set_file(Some(file)),
            _ => g_error!("Viewer", "Found unexpected visible child name in viewer stack."),
        }
    }

    /// Returns a new `AdwNavigationPage` object that
    /// has its child set to the `&self` GObject.
    pub fn wrap_in_navigation_page(&self) -> adw::NavigationPage {
        let new_navigation_page: adw::NavigationPage = adw::NavigationPage::builder()
            .title(gettext("Loading Content"))
            .child(self)
            .build();
        new_navigation_page
    }

    #[template_callback]
    fn details_toggle(&self, _: &gtk::ToggleButton) {
        self.imp()
            .split_view
            .set_show_sidebar(!self.imp().split_view.shows_sidebar());
    }

    #[template_callback]
    fn fullscreen_toggle(&self, button: &gtk::ToggleButton) {
        let fullscreen: bool = self.window().is_fullscreened();
        self.window().set_fullscreened(!fullscreen);

        if !fullscreen {
            button.set_tooltip_text(Some(&gettext("Exit Fullscreen")));
        } else {
            button.set_tooltip_text(Some(&gettext("View Fullscreen")));
        }
    }
}

impl Default for AlbumsViewer {
    fn default() -> Self {
        Self::new()
    }
}
