// This file was generated by gir (dc20488) from gir-files (11e0e6d)
// DO NOT EDIT

use Bin;
use Buildable;
use Container;
#[cfg(feature = "3.10")]
use PlacesOpenFlags;
use ScrolledWindow;
use Widget;
use ffi;
use glib::object::Downcast;
use glib::translate::*;

glib_wrapper! {
    pub struct PlacesSidebar(Object<ffi::GtkPlacesSidebar>): ScrolledWindow, Bin, Container, Widget, Buildable;

    match fn {
        get_type => || ffi::gtk_places_sidebar_get_type(),
    }
}

impl PlacesSidebar {
    #[cfg(feature = "3.10")]
    pub fn new() -> PlacesSidebar {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_places_sidebar_new()).downcast_unchecked()
        }
    }

    //#[cfg(feature = "3.10")]
    //pub fn add_shortcut<T: IsA</*Ignored*/glib::File>>(&self, location: &T) {
    //    unsafe { TODO: call ffi::gtk_places_sidebar_add_shortcut() }
    //}

    #[cfg(feature = "3.12")]
    pub fn get_local_only(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_places_sidebar_get_local_only(self.to_glib_none().0))
        }
    }

    //#[cfg(feature = "3.10")]
    //pub fn get_location(&self) -> /*Ignored*/Option<glib::File> {
    //    unsafe { TODO: call ffi::gtk_places_sidebar_get_location() }
    //}

    //#[cfg(feature = "3.10")]
    //pub fn get_nth_bookmark(&self, n: i32) -> /*Ignored*/Option<glib::File> {
    //    unsafe { TODO: call ffi::gtk_places_sidebar_get_nth_bookmark() }
    //}

    #[cfg(feature = "3.10")]
    pub fn get_open_flags(&self) -> PlacesOpenFlags {
        unsafe {
            ffi::gtk_places_sidebar_get_open_flags(self.to_glib_none().0)
        }
    }

    #[cfg(feature = "3.10")]
    pub fn get_show_connect_to_server(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_places_sidebar_get_show_connect_to_server(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "3.10")]
    pub fn get_show_desktop(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_places_sidebar_get_show_desktop(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "3.14")]
    pub fn get_show_enter_location(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_places_sidebar_get_show_enter_location(self.to_glib_none().0))
        }
    }

    //#[cfg(feature = "3.10")]
    //pub fn list_shortcuts(&self) -> /*Ignored*/Vec<glib::File> {
    //    unsafe { TODO: call ffi::gtk_places_sidebar_list_shortcuts() }
    //}

    //#[cfg(feature = "3.10")]
    //pub fn remove_shortcut<T: IsA</*Ignored*/glib::File>>(&self, location: &T) {
    //    unsafe { TODO: call ffi::gtk_places_sidebar_remove_shortcut() }
    //}

    #[cfg(feature = "3.12")]
    pub fn set_local_only(&self, local_only: bool) {
        unsafe {
            ffi::gtk_places_sidebar_set_local_only(self.to_glib_none().0, local_only.to_glib());
        }
    }

    //#[cfg(feature = "3.10")]
    //pub fn set_location<T: IsA</*Ignored*/glib::File>>(&self, location: Option<&T>) {
    //    unsafe { TODO: call ffi::gtk_places_sidebar_set_location() }
    //}

    #[cfg(feature = "3.10")]
    pub fn set_open_flags(&self, flags: PlacesOpenFlags) {
        unsafe {
            ffi::gtk_places_sidebar_set_open_flags(self.to_glib_none().0, flags);
        }
    }

    #[cfg(feature = "3.10")]
    pub fn set_show_connect_to_server(&self, show_connect_to_server: bool) {
        unsafe {
            ffi::gtk_places_sidebar_set_show_connect_to_server(self.to_glib_none().0, show_connect_to_server.to_glib());
        }
    }

    #[cfg(feature = "3.10")]
    pub fn set_show_desktop(&self, show_desktop: bool) {
        unsafe {
            ffi::gtk_places_sidebar_set_show_desktop(self.to_glib_none().0, show_desktop.to_glib());
        }
    }

    #[cfg(feature = "3.14")]
    pub fn set_show_enter_location(&self, show_enter_location: bool) {
        unsafe {
            ffi::gtk_places_sidebar_set_show_enter_location(self.to_glib_none().0, show_enter_location.to_glib());
        }
    }
}