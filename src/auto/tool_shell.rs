// This file was generated by gir (6a48033) from gir-files (db49619)
// DO NOT EDIT

use Orientation;
use ReliefStyle;
use SizeGroup;
use ToolbarStyle;
use Widget;
use ffi;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use pango;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct ToolShell(Object<ffi::GtkToolShell>): Widget;

    match fn {
        get_type => || ffi::gtk_tool_shell_get_type(),
    }
}

pub trait ToolShellExt {
    fn get_ellipsize_mode(&self) -> pango::EllipsizeMode;

    fn get_icon_size(&self) -> i32;

    fn get_orientation(&self) -> Orientation;

    fn get_relief_style(&self) -> ReliefStyle;

    fn get_style(&self) -> ToolbarStyle;

    fn get_text_alignment(&self) -> f32;

    fn get_text_orientation(&self) -> Orientation;

    fn get_text_size_group(&self) -> Option<SizeGroup>;

    fn rebuild_menu(&self);
}

impl<O: IsA<ToolShell>> ToolShellExt for O {
    fn get_ellipsize_mode(&self) -> pango::EllipsizeMode {
        unsafe {
            from_glib(ffi::gtk_tool_shell_get_ellipsize_mode(self.to_glib_none().0))
        }
    }

    fn get_icon_size(&self) -> i32 {
        unsafe {
            ffi::gtk_tool_shell_get_icon_size(self.to_glib_none().0)
        }
    }

    fn get_orientation(&self) -> Orientation {
        unsafe {
            from_glib(ffi::gtk_tool_shell_get_orientation(self.to_glib_none().0))
        }
    }

    fn get_relief_style(&self) -> ReliefStyle {
        unsafe {
            from_glib(ffi::gtk_tool_shell_get_relief_style(self.to_glib_none().0))
        }
    }

    fn get_style(&self) -> ToolbarStyle {
        unsafe {
            from_glib(ffi::gtk_tool_shell_get_style(self.to_glib_none().0))
        }
    }

    fn get_text_alignment(&self) -> f32 {
        unsafe {
            ffi::gtk_tool_shell_get_text_alignment(self.to_glib_none().0)
        }
    }

    fn get_text_orientation(&self) -> Orientation {
        unsafe {
            from_glib(ffi::gtk_tool_shell_get_text_orientation(self.to_glib_none().0))
        }
    }

    fn get_text_size_group(&self) -> Option<SizeGroup> {
        unsafe {
            from_glib_none(ffi::gtk_tool_shell_get_text_size_group(self.to_glib_none().0))
        }
    }

    fn rebuild_menu(&self) {
        unsafe {
            ffi::gtk_tool_shell_rebuild_menu(self.to_glib_none().0);
        }
    }
}
