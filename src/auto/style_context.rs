// This file was generated by gir (29b5d65) from gir-files (71d73f0)
// DO NOT EDIT

use CssSection;
use JunctionSides;
use RegionFlags;
use StateFlags;
use StateType;
use StyleProvider;
use TextDirection;
use ffi;
use gdk;
use glib::Value;
use glib::object::IsA;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;

glib_wrapper! {
    pub struct StyleContext(Object<ffi::GtkStyleContext>);

    match fn {
        get_type => || ffi::gtk_style_context_get_type(),
    }
}

impl StyleContext {
    pub fn new() -> StyleContext {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_style_context_new())
        }
    }

    pub fn add_class(&self, class_name: &str) {
        unsafe {
            ffi::gtk_style_context_add_class(self.to_glib_none().0, class_name.to_glib_none().0);
        }
    }

    pub fn add_provider<T: IsA<StyleProvider>>(&self, provider: &T, priority: u32) {
        unsafe {
            ffi::gtk_style_context_add_provider(self.to_glib_none().0, provider.to_glib_none().0, priority);
        }
    }

    pub fn add_region(&self, region_name: &str, flags: RegionFlags) {
        unsafe {
            ffi::gtk_style_context_add_region(self.to_glib_none().0, region_name.to_glib_none().0, flags.to_glib());
        }
    }

    //pub fn cancel_animations(&self, region_id: /*Unimplemented*/Option<Fundamental: Pointer>) {
    //    unsafe { TODO: call ffi::gtk_style_context_cancel_animations() }
    //}

    //pub fn get(&self, state: StateFlags, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call ffi::gtk_style_context_get() }
    //}

    //pub fn get_background_color(&self, state: StateFlags, color: /*Ignored*/gdk::RGBA) {
    //    unsafe { TODO: call ffi::gtk_style_context_get_background_color() }
    //}

    //pub fn get_border(&self, state: StateFlags, border: /*Ignored*/Border) {
    //    unsafe { TODO: call ffi::gtk_style_context_get_border() }
    //}

    //pub fn get_border_color(&self, state: StateFlags, color: /*Ignored*/gdk::RGBA) {
    //    unsafe { TODO: call ffi::gtk_style_context_get_border_color() }
    //}

    //pub fn get_color(&self, state: StateFlags, color: /*Ignored*/gdk::RGBA) {
    //    unsafe { TODO: call ffi::gtk_style_context_get_color() }
    //}

    pub fn get_direction(&self) -> TextDirection {
        unsafe {
            from_glib(ffi::gtk_style_context_get_direction(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_8")]
    pub fn get_frame_clock(&self) -> Option<gdk::FrameClock> {
        unsafe {
            from_glib_none(ffi::gtk_style_context_get_frame_clock(self.to_glib_none().0))
        }
    }

    pub fn get_junction_sides(&self) -> JunctionSides {
        unsafe {
            from_glib(ffi::gtk_style_context_get_junction_sides(self.to_glib_none().0))
        }
    }

    //pub fn get_margin(&self, state: StateFlags, margin: /*Ignored*/Border) {
    //    unsafe { TODO: call ffi::gtk_style_context_get_margin() }
    //}

    //pub fn get_padding(&self, state: StateFlags, padding: /*Ignored*/Border) {
    //    unsafe { TODO: call ffi::gtk_style_context_get_padding() }
    //}

    pub fn get_parent(&self) -> Option<StyleContext> {
        unsafe {
            from_glib_none(ffi::gtk_style_context_get_parent(self.to_glib_none().0))
        }
    }

    //pub fn get_path(&self) -> /*Ignored*/Option<WidgetPath> {
    //    unsafe { TODO: call ffi::gtk_style_context_get_path() }
    //}

    #[cfg(feature = "v3_10")]
    pub fn get_scale(&self) -> i32 {
        unsafe {
            ffi::gtk_style_context_get_scale(self.to_glib_none().0)
        }
    }

    pub fn get_screen(&self) -> Option<gdk::Screen> {
        unsafe {
            from_glib_none(ffi::gtk_style_context_get_screen(self.to_glib_none().0))
        }
    }

    pub fn get_section(&self, property: &str) -> Option<CssSection> {
        unsafe {
            from_glib_none(ffi::gtk_style_context_get_section(self.to_glib_none().0, property.to_glib_none().0))
        }
    }

    pub fn get_state(&self) -> StateFlags {
        unsafe {
            from_glib(ffi::gtk_style_context_get_state(self.to_glib_none().0))
        }
    }

    //pub fn get_style(&self, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) {
    //    unsafe { TODO: call ffi::gtk_style_context_get_style() }
    //}

    //pub fn get_style_valist(&self, args: /*Unknown conversion*//*Unimplemented*/Unsupported) {
    //    unsafe { TODO: call ffi::gtk_style_context_get_style_valist() }
    //}

    //pub fn get_valist(&self, state: StateFlags, args: /*Unknown conversion*//*Unimplemented*/Unsupported) {
    //    unsafe { TODO: call ffi::gtk_style_context_get_valist() }
    //}

    pub fn has_class(&self, class_name: &str) -> bool {
        unsafe {
            from_glib(ffi::gtk_style_context_has_class(self.to_glib_none().0, class_name.to_glib_none().0))
        }
    }

    pub fn has_region(&self, region_name: &str) -> Option<RegionFlags> {
        unsafe {
            let mut flags_return = mem::uninitialized();
            let ret = from_glib(ffi::gtk_style_context_has_region(self.to_glib_none().0, region_name.to_glib_none().0, &mut flags_return));
            if ret { Some(from_glib(flags_return)) } else { None }
        }
    }

    pub fn invalidate(&self) {
        unsafe {
            ffi::gtk_style_context_invalidate(self.to_glib_none().0);
        }
    }

    pub fn list_classes(&self) -> Vec<String> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(ffi::gtk_style_context_list_classes(self.to_glib_none().0))
        }
    }

    pub fn list_regions(&self) -> Vec<String> {
        unsafe {
            FromGlibPtrContainer::from_glib_container(ffi::gtk_style_context_list_regions(self.to_glib_none().0))
        }
    }

    //pub fn lookup_color(&self, color_name: &str, color: /*Ignored*/gdk::RGBA) -> bool {
    //    unsafe { TODO: call ffi::gtk_style_context_lookup_color() }
    //}

    //pub fn lookup_icon_set(&self, stock_id: &str) -> /*Ignored*/Option<IconSet> {
    //    unsafe { TODO: call ffi::gtk_style_context_lookup_icon_set() }
    //}

    //pub fn notify_state_change(&self, window: &gdk::Window, region_id: /*Unimplemented*/Option<Fundamental: Pointer>, state: StateType, state_value: bool) {
    //    unsafe { TODO: call ffi::gtk_style_context_notify_state_change() }
    //}

    pub fn pop_animatable_region(&self) {
        unsafe {
            ffi::gtk_style_context_pop_animatable_region(self.to_glib_none().0);
        }
    }

    //pub fn push_animatable_region(&self, region_id: /*Unimplemented*/Option<Fundamental: Pointer>) {
    //    unsafe { TODO: call ffi::gtk_style_context_push_animatable_region() }
    //}

    pub fn remove_class(&self, class_name: &str) {
        unsafe {
            ffi::gtk_style_context_remove_class(self.to_glib_none().0, class_name.to_glib_none().0);
        }
    }

    pub fn remove_provider<T: IsA<StyleProvider>>(&self, provider: &T) {
        unsafe {
            ffi::gtk_style_context_remove_provider(self.to_glib_none().0, provider.to_glib_none().0);
        }
    }

    pub fn remove_region(&self, region_name: &str) {
        unsafe {
            ffi::gtk_style_context_remove_region(self.to_glib_none().0, region_name.to_glib_none().0);
        }
    }

    pub fn restore(&self) {
        unsafe {
            ffi::gtk_style_context_restore(self.to_glib_none().0);
        }
    }

    pub fn save(&self) {
        unsafe {
            ffi::gtk_style_context_save(self.to_glib_none().0);
        }
    }

    pub fn scroll_animations(&self, window: &gdk::Window, dx: i32, dy: i32) {
        unsafe {
            ffi::gtk_style_context_scroll_animations(self.to_glib_none().0, window.to_glib_none().0, dx, dy);
        }
    }

    pub fn set_background(&self, window: &gdk::Window) {
        unsafe {
            ffi::gtk_style_context_set_background(self.to_glib_none().0, window.to_glib_none().0);
        }
    }

    pub fn set_direction(&self, direction: TextDirection) {
        unsafe {
            ffi::gtk_style_context_set_direction(self.to_glib_none().0, direction.to_glib());
        }
    }

    #[cfg(feature = "v3_8")]
    pub fn set_frame_clock(&self, frame_clock: &gdk::FrameClock) {
        unsafe {
            ffi::gtk_style_context_set_frame_clock(self.to_glib_none().0, frame_clock.to_glib_none().0);
        }
    }

    pub fn set_junction_sides(&self, sides: JunctionSides) {
        unsafe {
            ffi::gtk_style_context_set_junction_sides(self.to_glib_none().0, sides.to_glib());
        }
    }

    pub fn set_parent(&self, parent: Option<&StyleContext>) {
        unsafe {
            ffi::gtk_style_context_set_parent(self.to_glib_none().0, parent.to_glib_none().0);
        }
    }

    //pub fn set_path(&self, path: /*Ignored*/&mut WidgetPath) {
    //    unsafe { TODO: call ffi::gtk_style_context_set_path() }
    //}

    #[cfg(feature = "v3_10")]
    pub fn set_scale(&self, scale: i32) {
        unsafe {
            ffi::gtk_style_context_set_scale(self.to_glib_none().0, scale);
        }
    }

    pub fn set_screen(&self, screen: &gdk::Screen) {
        unsafe {
            ffi::gtk_style_context_set_screen(self.to_glib_none().0, screen.to_glib_none().0);
        }
    }

    pub fn set_state(&self, flags: StateFlags) {
        unsafe {
            ffi::gtk_style_context_set_state(self.to_glib_none().0, flags.to_glib());
        }
    }

    pub fn state_is_running(&self, state: StateType) -> Option<f64> {
        unsafe {
            let mut progress = mem::uninitialized();
            let ret = from_glib(ffi::gtk_style_context_state_is_running(self.to_glib_none().0, state.to_glib(), &mut progress));
            if ret { Some(progress) } else { None }
        }
    }

    //#[cfg(feature = "v3_20")]
    //pub fn to_string(&self, flags: /*Ignored*/StyleContextPrintFlags) -> String {
    //    unsafe { TODO: call ffi::gtk_style_context_to_string() }
    //}

    #[cfg(feature = "v3_8")]
    pub fn get_property_paint_clock(&self) -> Option<gdk::FrameClock> {
        let mut value = Value::from(None::<&gdk::FrameClock>);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "paint-clock".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get()
    }

    #[cfg(feature = "v3_8")]
    pub fn set_property_paint_clock(&self, paint_clock: Option<&gdk::FrameClock>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "paint-clock".to_glib_none().0, Value::from(paint_clock).to_glib_none().0);
        }
    }

    pub fn add_provider_for_screen<T: IsA<StyleProvider>>(screen: &gdk::Screen, provider: &T, priority: u32) {
        skip_assert_initialized!();
        unsafe {
            ffi::gtk_style_context_add_provider_for_screen(screen.to_glib_none().0, provider.to_glib_none().0, priority);
        }
    }

    pub fn remove_provider_for_screen<T: IsA<StyleProvider>>(screen: &gdk::Screen, provider: &T) {
        skip_assert_initialized!();
        unsafe {
            ffi::gtk_style_context_remove_provider_for_screen(screen.to_glib_none().0, provider.to_glib_none().0);
        }
    }

    pub fn reset_widgets(screen: &gdk::Screen) {
        assert_initialized_main_thread!();
        unsafe {
            ffi::gtk_style_context_reset_widgets(screen.to_glib_none().0);
        }
    }

    pub fn connect_changed<F: Fn(&StyleContext) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&StyleContext) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "changed",
                transmute(changed_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn changed_trampoline(this: *mut ffi::GtkStyleContext, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&StyleContext) + 'static> = transmute(f);
    f(&from_glib_none(this))
}
