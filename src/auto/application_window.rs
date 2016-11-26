// This file was generated by gir (5e8c56e) from gir-files (71d73f0)
// DO NOT EDIT

use Application;
use Bin;
use Container;
use Widget;
use Window;
use ffi;
use gio;
use gio_ffi;
use glib::object::Downcast;
use glib::translate::*;

glib_wrapper! {
    pub struct ApplicationWindow(Object<ffi::GtkApplicationWindow>): [
        Window,
        Bin,
        Container,
        Widget,
        gio::ActionGroup => gio_ffi::GActionGroup,
        gio::ActionMap => gio_ffi::GActionMap,
    ];

    match fn {
        get_type => || ffi::gtk_application_window_get_type(),
    }
}

impl ApplicationWindow {
    pub fn new(application: &Application) -> ApplicationWindow {
        skip_assert_initialized!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_application_window_new(application.to_glib_none().0)).downcast_unchecked()
        }
    }

    //#[cfg(feature = "v3_20")]
    //pub fn get_help_overlay(&self) -> /*Ignored*/Option<ShortcutsWindow> {
    //    unsafe { TODO: call ffi::gtk_application_window_get_help_overlay() }
    //}

    #[cfg(feature = "v3_6")]
    pub fn get_id(&self) -> u32 {
        unsafe {
            ffi::gtk_application_window_get_id(self.to_glib_none().0)
        }
    }

    pub fn get_show_menubar(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_application_window_get_show_menubar(self.to_glib_none().0))
        }
    }

    //#[cfg(feature = "v3_20")]
    //pub fn set_help_overlay(&self, help_overlay: /*Ignored*/Option<&ShortcutsWindow>) {
    //    unsafe { TODO: call ffi::gtk_application_window_set_help_overlay() }
    //}

    pub fn set_show_menubar(&self, show_menubar: bool) {
        unsafe {
            ffi::gtk_application_window_set_show_menubar(self.to_glib_none().0, show_menubar.to_glib());
        }
    }
}
