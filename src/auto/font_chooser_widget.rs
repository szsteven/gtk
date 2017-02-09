// This file was generated by gir (29b5d65) from gir-files (71d73f0)
// DO NOT EDIT

use Box;
use Container;
use FontChooser;
use Orientable;
use Widget;
use ffi;
use glib::object::Downcast;
use glib::translate::*;

glib_wrapper! {
    pub struct FontChooserWidget(Object<ffi::GtkFontChooserWidget>): Box, Container, Widget, Orientable, FontChooser;

    match fn {
        get_type => || ffi::gtk_font_chooser_widget_get_type(),
    }
}

impl FontChooserWidget {
    pub fn new() -> FontChooserWidget {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_font_chooser_widget_new()).downcast_unchecked()
        }
    }
}
