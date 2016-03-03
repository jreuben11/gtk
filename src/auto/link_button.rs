// This file was generated by gir (7dd2bcd) from gir-files (11e0e6d)
// DO NOT EDIT

use Actionable;
use Bin;
use Buildable;
use Button;
use Container;
use Widget;
use ffi;
use glib::object::Downcast;
use glib::translate::*;

glib_wrapper! {
    pub struct LinkButton(Object<ffi::GtkLinkButton>): Button, Bin, Container, Widget, Buildable, Actionable;

    match fn {
        get_type => || ffi::gtk_link_button_get_type(),
    }
}

impl LinkButton {
    pub fn new(uri: &str) -> LinkButton {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_link_button_new(uri.to_glib_none().0)).downcast_unchecked()
        }
    }

    pub fn new_with_label(uri: &str, label: Option<&str>) -> LinkButton {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_link_button_new_with_label(uri.to_glib_none().0, label.to_glib_none().0)).downcast_unchecked()
        }
    }

    pub fn get_uri(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_link_button_get_uri(self.to_glib_none().0))
        }
    }

    pub fn get_visited(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_link_button_get_visited(self.to_glib_none().0))
        }
    }

    pub fn set_uri(&self, uri: &str) {
        unsafe {
            ffi::gtk_link_button_set_uri(self.to_glib_none().0, uri.to_glib_none().0);
        }
    }

    pub fn set_visited(&self, visited: bool) {
        unsafe {
            ffi::gtk_link_button_set_visited(self.to_glib_none().0, visited.to_glib());
        }
    }
}
