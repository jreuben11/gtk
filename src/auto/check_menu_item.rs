// This file was generated by gir (7dd2bcd) from gir-files (11e0e6d)
// DO NOT EDIT

use Actionable;
use Bin;
use Buildable;
use Container;
use MenuItem;
use Widget;
use ffi;
use glib::object::Downcast;
use glib::translate::*;

glib_wrapper! {
    pub struct CheckMenuItem(Object<ffi::GtkCheckMenuItem>): MenuItem, Bin, Container, Widget, Buildable, Actionable;

    match fn {
        get_type => || ffi::gtk_check_menu_item_get_type(),
    }
}

impl CheckMenuItem {
    pub fn new() -> CheckMenuItem {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_check_menu_item_new()).downcast_unchecked()
        }
    }

    pub fn new_with_label(label: &str) -> CheckMenuItem {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_check_menu_item_new_with_label(label.to_glib_none().0)).downcast_unchecked()
        }
    }

    pub fn new_with_mnemonic(label: &str) -> CheckMenuItem {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_check_menu_item_new_with_mnemonic(label.to_glib_none().0)).downcast_unchecked()
        }
    }

    pub fn get_active(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_check_menu_item_get_active(self.to_glib_none().0))
        }
    }

    pub fn get_draw_as_radio(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_check_menu_item_get_draw_as_radio(self.to_glib_none().0))
        }
    }

    pub fn get_inconsistent(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_check_menu_item_get_inconsistent(self.to_glib_none().0))
        }
    }

    pub fn set_active(&self, is_active: bool) {
        unsafe {
            ffi::gtk_check_menu_item_set_active(self.to_glib_none().0, is_active.to_glib());
        }
    }

    pub fn set_draw_as_radio(&self, draw_as_radio: bool) {
        unsafe {
            ffi::gtk_check_menu_item_set_draw_as_radio(self.to_glib_none().0, draw_as_radio.to_glib());
        }
    }

    pub fn set_inconsistent(&self, setting: bool) {
        unsafe {
            ffi::gtk_check_menu_item_set_inconsistent(self.to_glib_none().0, setting.to_glib());
        }
    }

    pub fn toggled(&self) {
        unsafe {
            ffi::gtk_check_menu_item_toggled(self.to_glib_none().0);
        }
    }
}
