// This file was generated by gir (29b5d65) from gir-files (71d73f0)
// DO NOT EDIT

use Container;
use DirectionType;
use MenuDirectionType;
use MenuItem;
use Object;
use Widget;
use ffi;
#[cfg(feature = "v3_6")]
use gio;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use libc;
use signal::Inhibit;
use std::boxed::Box as Box_;
use std::mem::transmute;

glib_wrapper! {
    pub struct MenuShell(Object<ffi::GtkMenuShell>): Container, Widget;

    match fn {
        get_type => || ffi::gtk_menu_shell_get_type(),
    }
}

pub trait MenuShellExt {
    fn activate_item<T: IsA<Widget>>(&self, menu_item: &T, force_deactivate: bool);

    fn append<T: IsA<MenuItem>>(&self, child: &T);

    #[cfg(feature = "v3_6")]
    fn bind_model<'a, T: IsA<gio::MenuModel>, U: Into<Option<&'a str>>>(&self, model: Option<&T>, action_namespace: U, with_separators: bool);

    fn cancel(&self);

    fn deactivate(&self);

    fn deselect(&self);

    fn get_parent_shell(&self) -> Option<Widget>;

    fn get_selected_item(&self) -> Option<Widget>;

    fn get_take_focus(&self) -> bool;

    fn insert<T: IsA<Widget>>(&self, child: &T, position: i32);

    fn prepend<T: IsA<Widget>>(&self, child: &T);

    fn select_first(&self, search_sensitive: bool);

    fn select_item<T: IsA<Widget>>(&self, menu_item: &T);

    fn set_take_focus(&self, take_focus: bool);

    fn connect_activate_current<F: Fn(&Self, bool) + 'static>(&self, f: F) -> u64;

    fn connect_cancel<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_cycle_focus<F: Fn(&Self, DirectionType) + 'static>(&self, f: F) -> u64;

    fn connect_deactivate<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_insert<F: Fn(&Self, &Widget, i32) + 'static>(&self, f: F) -> u64;

    fn connect_move_current<F: Fn(&Self, MenuDirectionType) + 'static>(&self, f: F) -> u64;

    fn connect_move_selected<F: Fn(&Self, i32) -> Inhibit + 'static>(&self, f: F) -> u64;

    fn connect_selection_done<F: Fn(&Self) + 'static>(&self, f: F) -> u64;
}

impl<O: IsA<MenuShell> + IsA<Object>> MenuShellExt for O {
    fn activate_item<T: IsA<Widget>>(&self, menu_item: &T, force_deactivate: bool) {
        unsafe {
            ffi::gtk_menu_shell_activate_item(self.to_glib_none().0, menu_item.to_glib_none().0, force_deactivate.to_glib());
        }
    }

    fn append<T: IsA<MenuItem>>(&self, child: &T) {
        unsafe {
            ffi::gtk_menu_shell_append(self.to_glib_none().0, child.to_glib_none().0);
        }
    }

    #[cfg(feature = "v3_6")]
    fn bind_model<'a, T: IsA<gio::MenuModel>, U: Into<Option<&'a str>>>(&self, model: Option<&T>, action_namespace: U, with_separators: bool) {
        unsafe {
            ffi::gtk_menu_shell_bind_model(self.to_glib_none().0, model.to_glib_none().0, action_namespace.into().to_glib_none().0, with_separators.to_glib());
        }
    }

    fn cancel(&self) {
        unsafe {
            ffi::gtk_menu_shell_cancel(self.to_glib_none().0);
        }
    }

    fn deactivate(&self) {
        unsafe {
            ffi::gtk_menu_shell_deactivate(self.to_glib_none().0);
        }
    }

    fn deselect(&self) {
        unsafe {
            ffi::gtk_menu_shell_deselect(self.to_glib_none().0);
        }
    }

    fn get_parent_shell(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_menu_shell_get_parent_shell(self.to_glib_none().0))
        }
    }

    fn get_selected_item(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_menu_shell_get_selected_item(self.to_glib_none().0))
        }
    }

    fn get_take_focus(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_menu_shell_get_take_focus(self.to_glib_none().0))
        }
    }

    fn insert<T: IsA<Widget>>(&self, child: &T, position: i32) {
        unsafe {
            ffi::gtk_menu_shell_insert(self.to_glib_none().0, child.to_glib_none().0, position);
        }
    }

    fn prepend<T: IsA<Widget>>(&self, child: &T) {
        unsafe {
            ffi::gtk_menu_shell_prepend(self.to_glib_none().0, child.to_glib_none().0);
        }
    }

    fn select_first(&self, search_sensitive: bool) {
        unsafe {
            ffi::gtk_menu_shell_select_first(self.to_glib_none().0, search_sensitive.to_glib());
        }
    }

    fn select_item<T: IsA<Widget>>(&self, menu_item: &T) {
        unsafe {
            ffi::gtk_menu_shell_select_item(self.to_glib_none().0, menu_item.to_glib_none().0);
        }
    }

    fn set_take_focus(&self, take_focus: bool) {
        unsafe {
            ffi::gtk_menu_shell_set_take_focus(self.to_glib_none().0, take_focus.to_glib());
        }
    }

    fn connect_activate_current<F: Fn(&Self, bool) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self, bool) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "activate-current",
                transmute(activate_current_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_cancel<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "cancel",
                transmute(cancel_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_cycle_focus<F: Fn(&Self, DirectionType) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self, DirectionType) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "cycle-focus",
                transmute(cycle_focus_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_deactivate<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "deactivate",
                transmute(deactivate_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_insert<F: Fn(&Self, &Widget, i32) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &Widget, i32) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "insert",
                transmute(insert_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_move_current<F: Fn(&Self, MenuDirectionType) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self, MenuDirectionType) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "move-current",
                transmute(move_current_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_move_selected<F: Fn(&Self, i32) -> Inhibit + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self, i32) -> Inhibit + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "move-selected",
                transmute(move_selected_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_selection_done<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "selection-done",
                transmute(selection_done_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn activate_current_trampoline<T>(this: *mut ffi::GtkMenuShell, force_hide: glib_ffi::gboolean, f: glib_ffi::gpointer)
where T: IsA<MenuShell> {
    callback_guard!();
    let f: &Box_<Fn(&T, bool) + 'static> = transmute(f);
    f(&MenuShell::from_glib_none(this).downcast_unchecked(), from_glib(force_hide))
}

unsafe extern "C" fn cancel_trampoline<T>(this: *mut ffi::GtkMenuShell, f: glib_ffi::gpointer)
where T: IsA<MenuShell> {
    callback_guard!();
    let f: &Box_<Fn(&T) + 'static> = transmute(f);
    f(&MenuShell::from_glib_none(this).downcast_unchecked())
}

unsafe extern "C" fn cycle_focus_trampoline<T>(this: *mut ffi::GtkMenuShell, direction: ffi::GtkDirectionType, f: glib_ffi::gpointer)
where T: IsA<MenuShell> {
    callback_guard!();
    let f: &Box_<Fn(&T, DirectionType) + 'static> = transmute(f);
    f(&MenuShell::from_glib_none(this).downcast_unchecked(), from_glib(direction))
}

unsafe extern "C" fn deactivate_trampoline<T>(this: *mut ffi::GtkMenuShell, f: glib_ffi::gpointer)
where T: IsA<MenuShell> {
    callback_guard!();
    let f: &Box_<Fn(&T) + 'static> = transmute(f);
    f(&MenuShell::from_glib_none(this).downcast_unchecked())
}

unsafe extern "C" fn insert_trampoline<T>(this: *mut ffi::GtkMenuShell, child: *mut ffi::GtkWidget, position: libc::c_int, f: glib_ffi::gpointer)
where T: IsA<MenuShell> {
    callback_guard!();
    let f: &Box_<Fn(&T, &Widget, i32) + 'static> = transmute(f);
    f(&MenuShell::from_glib_none(this).downcast_unchecked(), &from_glib_none(child), position)
}

unsafe extern "C" fn move_current_trampoline<T>(this: *mut ffi::GtkMenuShell, direction: ffi::GtkMenuDirectionType, f: glib_ffi::gpointer)
where T: IsA<MenuShell> {
    callback_guard!();
    let f: &Box_<Fn(&T, MenuDirectionType) + 'static> = transmute(f);
    f(&MenuShell::from_glib_none(this).downcast_unchecked(), from_glib(direction))
}

unsafe extern "C" fn move_selected_trampoline<T>(this: *mut ffi::GtkMenuShell, distance: libc::c_int, f: glib_ffi::gpointer) -> glib_ffi::gboolean
where T: IsA<MenuShell> {
    callback_guard!();
    let f: &Box_<Fn(&T, i32) -> Inhibit + 'static> = transmute(f);
    f(&MenuShell::from_glib_none(this).downcast_unchecked(), distance).to_glib()
}

unsafe extern "C" fn selection_done_trampoline<T>(this: *mut ffi::GtkMenuShell, f: glib_ffi::gpointer)
where T: IsA<MenuShell> {
    callback_guard!();
    let f: &Box_<Fn(&T) + 'static> = transmute(f);
    f(&MenuShell::from_glib_none(this).downcast_unchecked())
}
