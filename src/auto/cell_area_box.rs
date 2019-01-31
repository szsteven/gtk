// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Buildable;
use CellArea;
use CellLayout;
use CellRenderer;
use Orientable;
use ffi;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_ffi;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib_wrapper! {
    pub struct CellAreaBox(Object<ffi::GtkCellAreaBox, ffi::GtkCellAreaBoxClass, CellAreaBoxClass>) @extends CellArea, @implements Buildable, CellLayout, Orientable;

    match fn {
        get_type => || ffi::gtk_cell_area_box_get_type(),
    }
}

impl CellAreaBox {
    pub fn new() -> CellAreaBox {
        assert_initialized_main_thread!();
        unsafe {
            CellArea::from_glib_none(ffi::gtk_cell_area_box_new()).unsafe_cast()
        }
    }
}

impl Default for CellAreaBox {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_CELL_AREA_BOX: Option<&CellAreaBox> = None;

pub trait CellAreaBoxExt: 'static {
    fn get_spacing(&self) -> i32;

    fn pack_end<P: IsA<CellRenderer>>(&self, renderer: &P, expand: bool, align: bool, fixed: bool);

    fn pack_start<P: IsA<CellRenderer>>(&self, renderer: &P, expand: bool, align: bool, fixed: bool);

    fn set_spacing(&self, spacing: i32);

    fn connect_property_spacing_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<CellAreaBox>> CellAreaBoxExt for O {
    fn get_spacing(&self) -> i32 {
        unsafe {
            ffi::gtk_cell_area_box_get_spacing(self.as_ref().to_glib_none().0)
        }
    }

    fn pack_end<P: IsA<CellRenderer>>(&self, renderer: &P, expand: bool, align: bool, fixed: bool) {
        unsafe {
            ffi::gtk_cell_area_box_pack_end(self.as_ref().to_glib_none().0, renderer.as_ref().to_glib_none().0, expand.to_glib(), align.to_glib(), fixed.to_glib());
        }
    }

    fn pack_start<P: IsA<CellRenderer>>(&self, renderer: &P, expand: bool, align: bool, fixed: bool) {
        unsafe {
            ffi::gtk_cell_area_box_pack_start(self.as_ref().to_glib_none().0, renderer.as_ref().to_glib_none().0, expand.to_glib(), align.to_glib(), fixed.to_glib());
        }
    }

    fn set_spacing(&self, spacing: i32) {
        unsafe {
            ffi::gtk_cell_area_box_set_spacing(self.as_ref().to_glib_none().0, spacing);
        }
    }

    fn connect_property_spacing_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::spacing\0".as_ptr() as *const _,
                Some(transmute(notify_spacing_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

unsafe extern "C" fn notify_spacing_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkCellAreaBox, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<CellAreaBox> {
    let f: &F = transmute(f);
    f(&CellAreaBox::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for CellAreaBox {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CellAreaBox")
    }
}
