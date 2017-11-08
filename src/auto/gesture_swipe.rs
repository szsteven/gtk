// This file was generated by gir (0409d73) from gir-files (469db10)
// DO NOT EDIT

use EventController;
use Gesture;
use GestureSingle;
#[cfg(any(feature = "v3_14", feature = "dox"))]
use Widget;
use ffi;
use glib;
use glib::object::Downcast;
use glib::object::IsA;
#[cfg(any(feature = "v3_14", feature = "dox"))]
use glib::signal::SignalHandlerId;
#[cfg(any(feature = "v3_14", feature = "dox"))]
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
#[cfg(any(feature = "v3_14", feature = "dox"))]
use libc;
#[cfg(any(feature = "v3_14", feature = "dox"))]
use std::boxed::Box as Box_;
use std::mem;
#[cfg(any(feature = "v3_14", feature = "dox"))]
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct GestureSwipe(Object<ffi::GtkGestureSwipe, ffi::GtkGestureSwipeClass>): GestureSingle, Gesture, EventController;

    match fn {
        get_type => || ffi::gtk_gesture_swipe_get_type(),
    }
}

impl GestureSwipe {
    #[cfg(any(feature = "v3_14", feature = "dox"))]
    pub fn new<P: IsA<Widget>>(widget: &P) -> GestureSwipe {
        skip_assert_initialized!();
        unsafe {
            Gesture::from_glib_full(ffi::gtk_gesture_swipe_new(widget.to_glib_none().0)).downcast_unchecked()
        }
    }
}

pub trait GestureSwipeExt {
    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn get_velocity(&self) -> Option<(f64, f64)>;

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn connect_swipe<F: Fn(&Self, f64, f64) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<GestureSwipe> + IsA<glib::object::Object>> GestureSwipeExt for O {
    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn get_velocity(&self) -> Option<(f64, f64)> {
        unsafe {
            let mut velocity_x = mem::uninitialized();
            let mut velocity_y = mem::uninitialized();
            let ret = from_glib(ffi::gtk_gesture_swipe_get_velocity(self.to_glib_none().0, &mut velocity_x, &mut velocity_y));
            if ret { Some((velocity_x, velocity_y)) } else { None }
        }
    }

    #[cfg(any(feature = "v3_14", feature = "dox"))]
    fn connect_swipe<F: Fn(&Self, f64, f64) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, f64, f64) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "swipe",
                transmute(swipe_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

#[cfg(any(feature = "v3_14", feature = "dox"))]
unsafe extern "C" fn swipe_trampoline<P>(this: *mut ffi::GtkGestureSwipe, velocity_x: libc::c_double, velocity_y: libc::c_double, f: glib_ffi::gpointer)
where P: IsA<GestureSwipe> {
    callback_guard!();
    let f: &&(Fn(&P, f64, f64) + 'static) = transmute(f);
    f(&GestureSwipe::from_glib_borrow(this).downcast_unchecked(), velocity_x, velocity_y)
}
