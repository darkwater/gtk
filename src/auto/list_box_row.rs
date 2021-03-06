// This file was generated by gir (d121f7e) from gir-files (71d73f0)
// DO NOT EDIT

use Bin;
use Container;
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
    pub struct ListBoxRow(Object<ffi::GtkListBoxRow>): Bin, Container, Widget;

    match fn {
        get_type => || ffi::gtk_list_box_row_get_type(),
    }
}

impl ListBoxRow {
    #[cfg(feature = "v3_10")]
    pub fn new() -> ListBoxRow {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_list_box_row_new()).downcast_unchecked()
        }
    }
}

pub trait ListBoxRowExt {
    #[cfg(feature = "v3_10")]
    fn changed(&self);

    #[cfg(feature = "v3_14")]
    fn get_activatable(&self) -> bool;

    #[cfg(feature = "v3_10")]
    fn get_header(&self) -> Option<Widget>;

    #[cfg(feature = "v3_10")]
    fn get_index(&self) -> i32;

    #[cfg(feature = "v3_14")]
    fn get_selectable(&self) -> bool;

    #[cfg(feature = "v3_14")]
    fn is_selected(&self) -> bool;

    #[cfg(feature = "v3_14")]
    fn set_activatable(&self, activatable: bool);

    #[cfg(feature = "v3_10")]
    fn set_header<'a, P: IsA<Widget> + 'a, Q: Into<Option<&'a P>>>(&self, header: Q);

    #[cfg(feature = "v3_14")]
    fn set_selectable(&self, selectable: bool);

    fn connect_activate<F: Fn(&Self) + 'static>(&self, f: F) -> u64;
}

impl<O: IsA<ListBoxRow> + IsA<glib::object::Object>> ListBoxRowExt for O {
    #[cfg(feature = "v3_10")]
    fn changed(&self) {
        unsafe {
            ffi::gtk_list_box_row_changed(self.to_glib_none().0);
        }
    }

    #[cfg(feature = "v3_14")]
    fn get_activatable(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_list_box_row_get_activatable(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_10")]
    fn get_header(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_list_box_row_get_header(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_10")]
    fn get_index(&self) -> i32 {
        unsafe {
            ffi::gtk_list_box_row_get_index(self.to_glib_none().0)
        }
    }

    #[cfg(feature = "v3_14")]
    fn get_selectable(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_list_box_row_get_selectable(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_14")]
    fn is_selected(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_list_box_row_is_selected(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_14")]
    fn set_activatable(&self, activatable: bool) {
        unsafe {
            ffi::gtk_list_box_row_set_activatable(self.to_glib_none().0, activatable.to_glib());
        }
    }

    #[cfg(feature = "v3_10")]
    fn set_header<'a, P: IsA<Widget> + 'a, Q: Into<Option<&'a P>>>(&self, header: Q) {
        let header = header.into();
        let header = header.to_glib_none();
        unsafe {
            ffi::gtk_list_box_row_set_header(self.to_glib_none().0, header.0);
        }
    }

    #[cfg(feature = "v3_14")]
    fn set_selectable(&self, selectable: bool) {
        unsafe {
            ffi::gtk_list_box_row_set_selectable(self.to_glib_none().0, selectable.to_glib());
        }
    }

    fn connect_activate<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "activate",
                transmute(activate_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn activate_trampoline<P>(this: *mut ffi::GtkListBoxRow, f: glib_ffi::gpointer)
where P: IsA<ListBoxRow> {
    callback_guard!();
    let f: &Box_<Fn(&P) + 'static> = transmute(f);
    f(&ListBoxRow::from_glib_none(this).downcast_unchecked())
}
