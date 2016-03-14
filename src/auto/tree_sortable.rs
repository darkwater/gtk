// This file was generated by gir (dc20488) from gir-files (11e0e6d)
// DO NOT EDIT

use SortType;
use TreeModel;
use ffi;
use glib::object::IsA;
use glib::translate::*;
use std::mem;

glib_wrapper! {
    pub struct TreeSortable(Object<ffi::GtkTreeSortable>): TreeModel;

    match fn {
        get_type => || ffi::gtk_tree_sortable_get_type(),
    }
}

pub trait TreeSortableExt {
    fn get_sort_column_id(&self) -> Option<(i32, SortType)>;

    fn has_default_sort_func(&self) -> bool;

    //fn set_default_sort_func(&self, sort_func: /*Unknown conversion*//*Unimplemented*/TreeIterCompareFunc, user_data: /*Unimplemented*/Option<Fundamental: Pointer>, destroy: /*Unknown conversion*//*Unimplemented*/DestroyNotify);

    fn set_sort_column_id(&self, sort_column_id: i32, order: SortType);

    //fn set_sort_func(&self, sort_column_id: i32, sort_func: /*Unknown conversion*//*Unimplemented*/TreeIterCompareFunc, user_data: /*Unimplemented*/Option<Fundamental: Pointer>, destroy: /*Unknown conversion*//*Unimplemented*/DestroyNotify);

    fn sort_column_changed(&self);
}

impl<O: IsA<TreeSortable>> TreeSortableExt for O {
    fn get_sort_column_id(&self) -> Option<(i32, SortType)> {
        unsafe {
            let mut sort_column_id = mem::uninitialized();
            let mut order = mem::uninitialized();
            let ret = from_glib(ffi::gtk_tree_sortable_get_sort_column_id(self.to_glib_none().0, &mut sort_column_id, &mut order));
            if ret { Some((sort_column_id, order)) } else { None }
        }
    }

    fn has_default_sort_func(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_tree_sortable_has_default_sort_func(self.to_glib_none().0))
        }
    }

    //fn set_default_sort_func(&self, sort_func: /*Unknown conversion*//*Unimplemented*/TreeIterCompareFunc, user_data: /*Unimplemented*/Option<Fundamental: Pointer>, destroy: /*Unknown conversion*//*Unimplemented*/DestroyNotify) {
    //    unsafe { TODO: call ffi::gtk_tree_sortable_set_default_sort_func() }
    //}

    fn set_sort_column_id(&self, sort_column_id: i32, order: SortType) {
        unsafe {
            ffi::gtk_tree_sortable_set_sort_column_id(self.to_glib_none().0, sort_column_id, order);
        }
    }

    //fn set_sort_func(&self, sort_column_id: i32, sort_func: /*Unknown conversion*//*Unimplemented*/TreeIterCompareFunc, user_data: /*Unimplemented*/Option<Fundamental: Pointer>, destroy: /*Unknown conversion*//*Unimplemented*/DestroyNotify) {
    //    unsafe { TODO: call ffi::gtk_tree_sortable_set_sort_func() }
    //}

    fn sort_column_changed(&self) {
        unsafe {
            ffi::gtk_tree_sortable_sort_column_changed(self.to_glib_none().0);
        }
    }
}