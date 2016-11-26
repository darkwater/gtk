// This file was generated by gir (5e8c56e) from gir-files (71d73f0)
// DO NOT EDIT

use Bin;
use Container;
use Widget;
use ffi;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use std::boxed::Box as Box_;
use std::mem::transmute;

glib_wrapper! {
    pub struct Expander(Object<ffi::GtkExpander>): Bin, Container, Widget;

    match fn {
        get_type => || ffi::gtk_expander_get_type(),
    }
}

impl Expander {
    pub fn new(label: Option<&str>) -> Expander {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_expander_new(label.to_glib_none().0)).downcast_unchecked()
        }
    }

    pub fn new_with_mnemonic(label: &str) -> Expander {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_expander_new_with_mnemonic(label.to_glib_none().0)).downcast_unchecked()
        }
    }

    pub fn get_expanded(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_expander_get_expanded(self.to_glib_none().0))
        }
    }

    pub fn get_label(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_expander_get_label(self.to_glib_none().0))
        }
    }

    pub fn get_label_fill(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_expander_get_label_fill(self.to_glib_none().0))
        }
    }

    pub fn get_label_widget(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_expander_get_label_widget(self.to_glib_none().0))
        }
    }

    pub fn get_resize_toplevel(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_expander_get_resize_toplevel(self.to_glib_none().0))
        }
    }

    pub fn get_spacing(&self) -> i32 {
        unsafe {
            ffi::gtk_expander_get_spacing(self.to_glib_none().0)
        }
    }

    pub fn get_use_markup(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_expander_get_use_markup(self.to_glib_none().0))
        }
    }

    pub fn get_use_underline(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_expander_get_use_underline(self.to_glib_none().0))
        }
    }

    pub fn set_expanded(&self, expanded: bool) {
        unsafe {
            ffi::gtk_expander_set_expanded(self.to_glib_none().0, expanded.to_glib());
        }
    }

    pub fn set_label(&self, label: Option<&str>) {
        unsafe {
            ffi::gtk_expander_set_label(self.to_glib_none().0, label.to_glib_none().0);
        }
    }

    pub fn set_label_fill(&self, label_fill: bool) {
        unsafe {
            ffi::gtk_expander_set_label_fill(self.to_glib_none().0, label_fill.to_glib());
        }
    }

    pub fn set_label_widget<T: IsA<Widget>>(&self, label_widget: Option<&T>) {
        unsafe {
            ffi::gtk_expander_set_label_widget(self.to_glib_none().0, label_widget.to_glib_none().0);
        }
    }

    pub fn set_resize_toplevel(&self, resize_toplevel: bool) {
        unsafe {
            ffi::gtk_expander_set_resize_toplevel(self.to_glib_none().0, resize_toplevel.to_glib());
        }
    }

    pub fn set_spacing(&self, spacing: i32) {
        unsafe {
            ffi::gtk_expander_set_spacing(self.to_glib_none().0, spacing);
        }
    }

    pub fn set_use_markup(&self, use_markup: bool) {
        unsafe {
            ffi::gtk_expander_set_use_markup(self.to_glib_none().0, use_markup.to_glib());
        }
    }

    pub fn set_use_underline(&self, use_underline: bool) {
        unsafe {
            ffi::gtk_expander_set_use_underline(self.to_glib_none().0, use_underline.to_glib());
        }
    }

    pub fn connect_activate<F: Fn(&Expander) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Expander) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "activate",
                transmute(activate_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn activate_trampoline(this: *mut ffi::GtkExpander, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&Expander) + 'static> = transmute(f);
    f(&from_glib_none(this))
}
