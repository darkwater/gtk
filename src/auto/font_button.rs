// This file was generated by gir (d121f7e) from gir-files (71d73f0)
// DO NOT EDIT

use Actionable;
use Bin;
use Button;
use Container;
use FontChooser;
use Widget;
use ffi;
use glib;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use std::boxed::Box as Box_;
use std::mem::transmute;

glib_wrapper! {
    pub struct FontButton(Object<ffi::GtkFontButton>): Button, Bin, Container, Widget, Actionable, FontChooser;

    match fn {
        get_type => || ffi::gtk_font_button_get_type(),
    }
}

impl FontButton {
    pub fn new() -> FontButton {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_font_button_new()).downcast_unchecked()
        }
    }

    pub fn new_with_font(fontname: &str) -> FontButton {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_font_button_new_with_font(fontname.to_glib_none().0)).downcast_unchecked()
        }
    }
}

pub trait FontButtonExt {
    fn get_font_name(&self) -> Option<String>;

    fn get_show_size(&self) -> bool;

    fn get_show_style(&self) -> bool;

    fn get_title(&self) -> Option<String>;

    fn get_use_font(&self) -> bool;

    fn get_use_size(&self) -> bool;

    fn set_font_name(&self, fontname: &str) -> bool;

    fn set_show_size(&self, show_size: bool);

    fn set_show_style(&self, show_style: bool);

    fn set_title(&self, title: &str);

    fn set_use_font(&self, use_font: bool);

    fn set_use_size(&self, use_size: bool);

    fn connect_font_set<F: Fn(&Self) + 'static>(&self, f: F) -> u64;
}

impl<O: IsA<FontButton> + IsA<glib::object::Object>> FontButtonExt for O {
    fn get_font_name(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_font_button_get_font_name(self.to_glib_none().0))
        }
    }

    fn get_show_size(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_font_button_get_show_size(self.to_glib_none().0))
        }
    }

    fn get_show_style(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_font_button_get_show_style(self.to_glib_none().0))
        }
    }

    fn get_title(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_font_button_get_title(self.to_glib_none().0))
        }
    }

    fn get_use_font(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_font_button_get_use_font(self.to_glib_none().0))
        }
    }

    fn get_use_size(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_font_button_get_use_size(self.to_glib_none().0))
        }
    }

    fn set_font_name(&self, fontname: &str) -> bool {
        unsafe {
            from_glib(ffi::gtk_font_button_set_font_name(self.to_glib_none().0, fontname.to_glib_none().0))
        }
    }

    fn set_show_size(&self, show_size: bool) {
        unsafe {
            ffi::gtk_font_button_set_show_size(self.to_glib_none().0, show_size.to_glib());
        }
    }

    fn set_show_style(&self, show_style: bool) {
        unsafe {
            ffi::gtk_font_button_set_show_style(self.to_glib_none().0, show_style.to_glib());
        }
    }

    fn set_title(&self, title: &str) {
        unsafe {
            ffi::gtk_font_button_set_title(self.to_glib_none().0, title.to_glib_none().0);
        }
    }

    fn set_use_font(&self, use_font: bool) {
        unsafe {
            ffi::gtk_font_button_set_use_font(self.to_glib_none().0, use_font.to_glib());
        }
    }

    fn set_use_size(&self, use_size: bool) {
        unsafe {
            ffi::gtk_font_button_set_use_size(self.to_glib_none().0, use_size.to_glib());
        }
    }

    fn connect_font_set<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "font-set",
                transmute(font_set_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn font_set_trampoline<P>(this: *mut ffi::GtkFontButton, f: glib_ffi::gpointer)
where P: IsA<FontButton> {
    callback_guard!();
    let f: &Box_<Fn(&P) + 'static> = transmute(f);
    f(&FontButton::from_glib_none(this).downcast_unchecked())
}
