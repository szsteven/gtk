// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Adjustment;
use CellRenderer;
use CellRendererText;
use ffi;
use glib;
use glib::StaticType;
use glib::Value;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib_wrapper! {
    pub struct CellRendererSpin(Object<ffi::GtkCellRendererSpin, ffi::GtkCellRendererSpinClass, CellRendererSpinClass>) @extends CellRendererText, CellRenderer;

    match fn {
        get_type => || ffi::gtk_cell_renderer_spin_get_type(),
    }
}

impl CellRendererSpin {
    pub fn new() -> CellRendererSpin {
        assert_initialized_main_thread!();
        unsafe {
            CellRenderer::from_glib_none(ffi::gtk_cell_renderer_spin_new()).unsafe_cast()
        }
    }
}

impl Default for CellRendererSpin {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_CELL_RENDERER_SPIN: Option<&CellRendererSpin> = None;

pub trait CellRendererSpinExt: 'static {
    fn get_property_adjustment(&self) -> Option<Adjustment>;

    fn set_property_adjustment<P: IsA<Adjustment> + glib::value::SetValueOptional>(&self, adjustment: Option<&P>);

    fn get_property_climb_rate(&self) -> f64;

    fn set_property_climb_rate(&self, climb_rate: f64);

    fn get_property_digits(&self) -> u32;

    fn set_property_digits(&self, digits: u32);

    fn connect_property_adjustment_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_climb_rate_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_digits_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<CellRendererSpin>> CellRendererSpinExt for O {
    fn get_property_adjustment(&self) -> Option<Adjustment> {
        unsafe {
            let mut value = Value::from_type(<Adjustment as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"adjustment\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn set_property_adjustment<P: IsA<Adjustment> + glib::value::SetValueOptional>(&self, adjustment: Option<&P>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"adjustment\0".as_ptr() as *const _, Value::from(adjustment).to_glib_none().0);
        }
    }

    fn get_property_climb_rate(&self) -> f64 {
        unsafe {
            let mut value = Value::from_type(<f64 as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"climb-rate\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_climb_rate(&self, climb_rate: f64) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"climb-rate\0".as_ptr() as *const _, Value::from(&climb_rate).to_glib_none().0);
        }
    }

    fn get_property_digits(&self) -> u32 {
        unsafe {
            let mut value = Value::from_type(<u32 as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"digits\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_digits(&self, digits: u32) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"digits\0".as_ptr() as *const _, Value::from(&digits).to_glib_none().0);
        }
    }

    fn connect_property_adjustment_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::adjustment\0".as_ptr() as *const _,
                Some(transmute(notify_adjustment_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_climb_rate_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::climb-rate\0".as_ptr() as *const _,
                Some(transmute(notify_climb_rate_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_digits_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::digits\0".as_ptr() as *const _,
                Some(transmute(notify_digits_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

unsafe extern "C" fn notify_adjustment_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkCellRendererSpin, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<CellRendererSpin> {
    let f: &F = transmute(f);
    f(&CellRendererSpin::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_climb_rate_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkCellRendererSpin, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<CellRendererSpin> {
    let f: &F = transmute(f);
    f(&CellRendererSpin::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_digits_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkCellRendererSpin, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<CellRendererSpin> {
    let f: &F = transmute(f);
    f(&CellRendererSpin::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for CellRendererSpin {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CellRendererSpin")
    }
}
