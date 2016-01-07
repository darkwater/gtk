// This file was generated by gir (4cd15d1) from gir-files (11e0e6d)
// DO NOT EDIT

use Bin;
use Buildable;
use Container;
use Dialog;
use License;
use Widget;
use Window;
use ffi;
use gdk_pixbuf;
use glib::object::Downcast;
use glib::translate::*;

glib_wrapper! {
    pub struct AboutDialog(Object<ffi::GtkAboutDialog>): Widget, Container, Bin, Window, Dialog, Buildable;

    match fn {
        get_type => || ffi::gtk_about_dialog_get_type(),
    }
}

impl AboutDialog {
    pub fn new() -> AboutDialog {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_about_dialog_new()).downcast_unchecked()
        }
    }

    //#[cfg(gtk_3_4)]
    //pub fn add_credit_section(&self, section_name: &str, people: /*Unknown conversion*/Unknown rust type: "CArray TypeId { ns_id: 0, id: 28 }") {
    //    unsafe { TODO: call ffi::gtk_about_dialog_add_credit_section() }
    //}

    //pub fn get_artists(&self) -> /*Unknown conversion*/Unknown rust type: "CArray TypeId { ns_id: 0, id: 28 }" {
    //    unsafe { TODO: call ffi::gtk_about_dialog_get_artists() }
    //}

    //pub fn get_authors(&self) -> /*Unknown conversion*/Unknown rust type: "CArray TypeId { ns_id: 0, id: 28 }" {
    //    unsafe { TODO: call ffi::gtk_about_dialog_get_authors() }
    //}

    pub fn get_comments(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_about_dialog_get_comments(self.to_glib_none().0))
        }
    }

    pub fn get_copyright(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_about_dialog_get_copyright(self.to_glib_none().0))
        }
    }

    //pub fn get_documenters(&self) -> /*Unknown conversion*/Unknown rust type: "CArray TypeId { ns_id: 0, id: 28 }" {
    //    unsafe { TODO: call ffi::gtk_about_dialog_get_documenters() }
    //}

    pub fn get_license(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_about_dialog_get_license(self.to_glib_none().0))
        }
    }

    pub fn get_license_type(&self) -> License {
        unsafe {
            ffi::gtk_about_dialog_get_license_type(self.to_glib_none().0)
        }
    }

    pub fn get_logo(&self) -> Option<gdk_pixbuf::Pixbuf> {
        unsafe {
            from_glib_none(ffi::gtk_about_dialog_get_logo(self.to_glib_none().0))
        }
    }

    pub fn get_logo_icon_name(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_about_dialog_get_logo_icon_name(self.to_glib_none().0))
        }
    }

    pub fn get_program_name(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_about_dialog_get_program_name(self.to_glib_none().0))
        }
    }

    pub fn get_translator_credits(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_about_dialog_get_translator_credits(self.to_glib_none().0))
        }
    }

    pub fn get_version(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_about_dialog_get_version(self.to_glib_none().0))
        }
    }

    pub fn get_website(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_about_dialog_get_website(self.to_glib_none().0))
        }
    }

    pub fn get_website_label(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_about_dialog_get_website_label(self.to_glib_none().0))
        }
    }

    pub fn get_wrap_license(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_about_dialog_get_wrap_license(self.to_glib_none().0))
        }
    }

    //pub fn set_artists(&self, artists: /*Unknown conversion*/Unknown rust type: "CArray TypeId { ns_id: 0, id: 28 }") {
    //    unsafe { TODO: call ffi::gtk_about_dialog_set_artists() }
    //}

    //pub fn set_authors(&self, authors: /*Unknown conversion*/Unknown rust type: "CArray TypeId { ns_id: 0, id: 28 }") {
    //    unsafe { TODO: call ffi::gtk_about_dialog_set_authors() }
    //}

    pub fn set_comments(&self, comments: Option<&str>) {
        unsafe {
            ffi::gtk_about_dialog_set_comments(self.to_glib_none().0, comments.to_glib_none().0);
        }
    }

    pub fn set_copyright(&self, copyright: Option<&str>) {
        unsafe {
            ffi::gtk_about_dialog_set_copyright(self.to_glib_none().0, copyright.to_glib_none().0);
        }
    }

    //pub fn set_documenters(&self, documenters: /*Unknown conversion*/Unknown rust type: "CArray TypeId { ns_id: 0, id: 28 }") {
    //    unsafe { TODO: call ffi::gtk_about_dialog_set_documenters() }
    //}

    pub fn set_license(&self, license: Option<&str>) {
        unsafe {
            ffi::gtk_about_dialog_set_license(self.to_glib_none().0, license.to_glib_none().0);
        }
    }

    pub fn set_license_type(&self, license_type: License) {
        unsafe {
            ffi::gtk_about_dialog_set_license_type(self.to_glib_none().0, license_type);
        }
    }

    pub fn set_logo(&self, logo: Option<&gdk_pixbuf::Pixbuf>) {
        unsafe {
            ffi::gtk_about_dialog_set_logo(self.to_glib_none().0, logo.to_glib_none().0);
        }
    }

    pub fn set_logo_icon_name(&self, icon_name: Option<&str>) {
        unsafe {
            ffi::gtk_about_dialog_set_logo_icon_name(self.to_glib_none().0, icon_name.to_glib_none().0);
        }
    }

    pub fn set_program_name(&self, name: &str) {
        unsafe {
            ffi::gtk_about_dialog_set_program_name(self.to_glib_none().0, name.to_glib_none().0);
        }
    }

    pub fn set_translator_credits(&self, translator_credits: Option<&str>) {
        unsafe {
            ffi::gtk_about_dialog_set_translator_credits(self.to_glib_none().0, translator_credits.to_glib_none().0);
        }
    }

    pub fn set_version(&self, version: Option<&str>) {
        unsafe {
            ffi::gtk_about_dialog_set_version(self.to_glib_none().0, version.to_glib_none().0);
        }
    }

    pub fn set_website(&self, website: Option<&str>) {
        unsafe {
            ffi::gtk_about_dialog_set_website(self.to_glib_none().0, website.to_glib_none().0);
        }
    }

    pub fn set_website_label(&self, website_label: &str) {
        unsafe {
            ffi::gtk_about_dialog_set_website_label(self.to_glib_none().0, website_label.to_glib_none().0);
        }
    }

    pub fn set_wrap_license(&self, wrap_license: bool) {
        unsafe {
            ffi::gtk_about_dialog_set_wrap_license(self.to_glib_none().0, wrap_license.to_glib());
        }
    }

}
