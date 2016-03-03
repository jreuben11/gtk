// This file was generated by gir (7dd2bcd) from gir-files (11e0e6d)
// DO NOT EDIT

#[cfg(feature = "3.12")]
use Adjustment;
use Buildable;
use Container;
#[cfg(feature = "3.12")]
use FlowBoxChild;
use Orientable;
#[cfg(feature = "3.12")]
use SelectionMode;
use Widget;
use ffi;
use glib::object::Downcast;
use glib::object::IsA;
use glib::translate::*;

glib_wrapper! {
    pub struct FlowBox(Object<ffi::GtkFlowBox>): Container, Widget, Buildable, Orientable;

    match fn {
        get_type => || ffi::gtk_flow_box_get_type(),
    }
}

impl FlowBox {
    #[cfg(feature = "3.12")]
    pub fn new() -> FlowBox {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_flow_box_new()).downcast_unchecked()
        }
    }

    #[cfg(feature = "3.12")]
    pub fn get_activate_on_single_click(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_flow_box_get_activate_on_single_click(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "3.12")]
    pub fn get_child_at_index(&self, idx: i32) -> Option<FlowBoxChild> {
        unsafe {
            from_glib_none(ffi::gtk_flow_box_get_child_at_index(self.to_glib_none().0, idx))
        }
    }

    #[cfg(feature = "3.12")]
    pub fn get_column_spacing(&self) -> u32 {
        unsafe {
            ffi::gtk_flow_box_get_column_spacing(self.to_glib_none().0)
        }
    }

    #[cfg(feature = "3.12")]
    pub fn get_homogeneous(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_flow_box_get_homogeneous(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "3.12")]
    pub fn get_max_children_per_line(&self) -> u32 {
        unsafe {
            ffi::gtk_flow_box_get_max_children_per_line(self.to_glib_none().0)
        }
    }

    #[cfg(feature = "3.12")]
    pub fn get_min_children_per_line(&self) -> u32 {
        unsafe {
            ffi::gtk_flow_box_get_min_children_per_line(self.to_glib_none().0)
        }
    }

    #[cfg(feature = "3.12")]
    pub fn get_row_spacing(&self) -> u32 {
        unsafe {
            ffi::gtk_flow_box_get_row_spacing(self.to_glib_none().0)
        }
    }

    #[cfg(feature = "3.12")]
    pub fn get_selected_children(&self) -> Vec<FlowBoxChild> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(ffi::gtk_flow_box_get_selected_children(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "3.12")]
    pub fn get_selection_mode(&self) -> SelectionMode {
        unsafe {
            ffi::gtk_flow_box_get_selection_mode(self.to_glib_none().0)
        }
    }

    #[cfg(feature = "3.12")]
    pub fn insert<T: IsA<Widget>>(&self, widget: &T, position: i32) {
        unsafe {
            ffi::gtk_flow_box_insert(self.to_glib_none().0, widget.to_glib_none().0, position);
        }
    }

    #[cfg(feature = "3.12")]
    pub fn invalidate_filter(&self) {
        unsafe {
            ffi::gtk_flow_box_invalidate_filter(self.to_glib_none().0);
        }
    }

    #[cfg(feature = "3.12")]
    pub fn invalidate_sort(&self) {
        unsafe {
            ffi::gtk_flow_box_invalidate_sort(self.to_glib_none().0);
        }
    }

    #[cfg(feature = "3.12")]
    pub fn select_all(&self) {
        unsafe {
            ffi::gtk_flow_box_select_all(self.to_glib_none().0);
        }
    }

    #[cfg(feature = "3.12")]
    pub fn select_child(&self, child: &FlowBoxChild) {
        unsafe {
            ffi::gtk_flow_box_select_child(self.to_glib_none().0, child.to_glib_none().0);
        }
    }

    //#[cfg(feature = "3.12")]
    //pub fn selected_foreach(&self, func: /*Unknown conversion*//*Unimplemented*/FlowBoxForeachFunc, data: /*Unimplemented*/Fundamental: Pointer) {
    //    unsafe { TODO: call ffi::gtk_flow_box_selected_foreach() }
    //}

    #[cfg(feature = "3.12")]
    pub fn set_activate_on_single_click(&self, single: bool) {
        unsafe {
            ffi::gtk_flow_box_set_activate_on_single_click(self.to_glib_none().0, single.to_glib());
        }
    }

    #[cfg(feature = "3.12")]
    pub fn set_column_spacing(&self, spacing: u32) {
        unsafe {
            ffi::gtk_flow_box_set_column_spacing(self.to_glib_none().0, spacing);
        }
    }

    //#[cfg(feature = "3.12")]
    //pub fn set_filter_func(&self, filter_func: /*Unknown conversion*//*Unimplemented*/FlowBoxFilterFunc, user_data: /*Unimplemented*/Fundamental: Pointer, destroy: /*Unknown conversion*//*Unimplemented*/DestroyNotify) {
    //    unsafe { TODO: call ffi::gtk_flow_box_set_filter_func() }
    //}

    #[cfg(feature = "3.12")]
    pub fn set_hadjustment(&self, adjustment: &Adjustment) {
        unsafe {
            ffi::gtk_flow_box_set_hadjustment(self.to_glib_none().0, adjustment.to_glib_none().0);
        }
    }

    #[cfg(feature = "3.12")]
    pub fn set_homogeneous(&self, homogeneous: bool) {
        unsafe {
            ffi::gtk_flow_box_set_homogeneous(self.to_glib_none().0, homogeneous.to_glib());
        }
    }

    #[cfg(feature = "3.12")]
    pub fn set_max_children_per_line(&self, n_children: u32) {
        unsafe {
            ffi::gtk_flow_box_set_max_children_per_line(self.to_glib_none().0, n_children);
        }
    }

    #[cfg(feature = "3.12")]
    pub fn set_min_children_per_line(&self, n_children: u32) {
        unsafe {
            ffi::gtk_flow_box_set_min_children_per_line(self.to_glib_none().0, n_children);
        }
    }

    #[cfg(feature = "3.12")]
    pub fn set_row_spacing(&self, spacing: u32) {
        unsafe {
            ffi::gtk_flow_box_set_row_spacing(self.to_glib_none().0, spacing);
        }
    }

    #[cfg(feature = "3.12")]
    pub fn set_selection_mode(&self, mode: SelectionMode) {
        unsafe {
            ffi::gtk_flow_box_set_selection_mode(self.to_glib_none().0, mode);
        }
    }

    //#[cfg(feature = "3.12")]
    //pub fn set_sort_func(&self, sort_func: /*Unknown conversion*//*Unimplemented*/FlowBoxSortFunc, user_data: /*Unimplemented*/Fundamental: Pointer, destroy: /*Unknown conversion*//*Unimplemented*/DestroyNotify) {
    //    unsafe { TODO: call ffi::gtk_flow_box_set_sort_func() }
    //}

    #[cfg(feature = "3.12")]
    pub fn set_vadjustment(&self, adjustment: &Adjustment) {
        unsafe {
            ffi::gtk_flow_box_set_vadjustment(self.to_glib_none().0, adjustment.to_glib_none().0);
        }
    }

    #[cfg(feature = "3.12")]
    pub fn unselect_all(&self) {
        unsafe {
            ffi::gtk_flow_box_unselect_all(self.to_glib_none().0);
        }
    }

    #[cfg(feature = "3.12")]
    pub fn unselect_child(&self, child: &FlowBoxChild) {
        unsafe {
            ffi::gtk_flow_box_unselect_child(self.to_glib_none().0, child.to_glib_none().0);
        }
    }
}
