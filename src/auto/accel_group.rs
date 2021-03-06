// This file was generated by gir (d121f7e) from gir-files (71d73f0)
// DO NOT EDIT

use ffi;
use gdk;
use gdk_ffi;
use glib;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use libc;
use std::boxed::Box as Box_;
use std::mem::transmute;

glib_wrapper! {
    pub struct AccelGroup(Object<ffi::GtkAccelGroup>);

    match fn {
        get_type => || ffi::gtk_accel_group_get_type(),
    }
}

impl AccelGroup {
    pub fn new() -> AccelGroup {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gtk_accel_group_new())
        }
    }

    //pub fn from_accel_closure(closure: /*Ignored*/&glib::Closure) -> Option<AccelGroup> {
    //    unsafe { TODO: call ffi::gtk_accel_group_from_accel_closure() }
    //}
}

pub trait AccelGroupExt {
    //fn activate<P: IsA<glib::Object>>(&self, accel_quark: /*Ignored*/glib::Quark, acceleratable: &P, accel_key: u32, accel_mods: gdk::ModifierType) -> bool;

    //fn connect(&self, accel_key: u32, accel_mods: gdk::ModifierType, accel_flags: AccelFlags, closure: /*Ignored*/&glib::Closure);

    //fn connect_by_path(&self, accel_path: &str, closure: /*Ignored*/&glib::Closure);

    //fn disconnect<'a, P: Into<Option<&'a /*Ignored*/glib::Closure>>>(&self, closure: P) -> bool;

    fn disconnect_key(&self, accel_key: u32, accel_mods: gdk::ModifierType) -> bool;

    //fn find<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, find_func: /*Unknown conversion*//*Unimplemented*/AccelGroupFindFunc, data: P) -> /*Ignored*/Option<AccelKey>;

    fn get_is_locked(&self) -> bool;

    fn get_modifier_mask(&self) -> gdk::ModifierType;

    fn lock(&self);

    fn unlock(&self);

    fn connect_accel_activate<F: Fn(&Self, &glib::Object, u32, gdk::ModifierType) -> bool + 'static>(&self, f: F) -> u64;

    //fn connect_accel_changed<Unsupported or ignored types>(&self, f: F) -> u64;
}

impl<O: IsA<AccelGroup> + IsA<glib::object::Object>> AccelGroupExt for O {
    //fn activate<P: IsA<glib::Object>>(&self, accel_quark: /*Ignored*/glib::Quark, acceleratable: &P, accel_key: u32, accel_mods: gdk::ModifierType) -> bool {
    //    unsafe { TODO: call ffi::gtk_accel_group_activate() }
    //}

    //fn connect(&self, accel_key: u32, accel_mods: gdk::ModifierType, accel_flags: AccelFlags, closure: /*Ignored*/&glib::Closure) {
    //    unsafe { TODO: call ffi::gtk_accel_group_connect() }
    //}

    //fn connect_by_path(&self, accel_path: &str, closure: /*Ignored*/&glib::Closure) {
    //    unsafe { TODO: call ffi::gtk_accel_group_connect_by_path() }
    //}

    //fn disconnect<'a, P: Into<Option<&'a /*Ignored*/glib::Closure>>>(&self, closure: P) -> bool {
    //    unsafe { TODO: call ffi::gtk_accel_group_disconnect() }
    //}

    fn disconnect_key(&self, accel_key: u32, accel_mods: gdk::ModifierType) -> bool {
        unsafe {
            from_glib(ffi::gtk_accel_group_disconnect_key(self.to_glib_none().0, accel_key, accel_mods.to_glib()))
        }
    }

    //fn find<P: Into<Option</*Unimplemented*/Fundamental: Pointer>>>(&self, find_func: /*Unknown conversion*//*Unimplemented*/AccelGroupFindFunc, data: P) -> /*Ignored*/Option<AccelKey> {
    //    unsafe { TODO: call ffi::gtk_accel_group_find() }
    //}

    fn get_is_locked(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_accel_group_get_is_locked(self.to_glib_none().0))
        }
    }

    fn get_modifier_mask(&self) -> gdk::ModifierType {
        unsafe {
            from_glib(ffi::gtk_accel_group_get_modifier_mask(self.to_glib_none().0))
        }
    }

    fn lock(&self) {
        unsafe {
            ffi::gtk_accel_group_lock(self.to_glib_none().0);
        }
    }

    fn unlock(&self) {
        unsafe {
            ffi::gtk_accel_group_unlock(self.to_glib_none().0);
        }
    }

    fn connect_accel_activate<F: Fn(&Self, &glib::Object, u32, gdk::ModifierType) -> bool + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &glib::Object, u32, gdk::ModifierType) -> bool + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "accel-activate",
                transmute(accel_activate_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    //fn connect_accel_changed<Unsupported or ignored types>(&self, f: F) -> u64 {
    //    Ignored accel_closure: GObject.Closure
    //}
}

unsafe extern "C" fn accel_activate_trampoline<P>(this: *mut ffi::GtkAccelGroup, acceleratable: *mut gobject_ffi::GObject, keyval: libc::c_uint, modifier: gdk_ffi::GdkModifierType, f: glib_ffi::gpointer) -> glib_ffi::gboolean
where P: IsA<AccelGroup> {
    callback_guard!();
    let f: &Box_<Fn(&P, &glib::Object, u32, gdk::ModifierType) -> bool + 'static> = transmute(f);
    f(&AccelGroup::from_glib_none(this).downcast_unchecked(), &from_glib_none(acceleratable), keyval, from_glib(modifier)).to_glib()
}
