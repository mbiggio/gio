// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use ffi;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct SrvTarget(Boxed<ffi::GSrvTarget>);

    match fn {
        copy => |ptr| ffi::g_srv_target_copy(mut_override(ptr)),
        free => |ptr| ffi::g_srv_target_free(ptr),
        get_type => || ffi::g_srv_target_get_type(),
    }
}

impl SrvTarget {
    pub fn new(hostname: &str, port: u16, priority: u16, weight: u16) -> SrvTarget {
        unsafe {
            from_glib_full(ffi::g_srv_target_new(hostname.to_glib_none().0, port, priority, weight))
        }
    }

    pub fn get_hostname(&mut self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::g_srv_target_get_hostname(self.to_glib_none_mut().0))
        }
    }

    pub fn get_port(&mut self) -> u16 {
        unsafe {
            ffi::g_srv_target_get_port(self.to_glib_none_mut().0)
        }
    }

    pub fn get_priority(&mut self) -> u16 {
        unsafe {
            ffi::g_srv_target_get_priority(self.to_glib_none_mut().0)
        }
    }

    pub fn get_weight(&mut self) -> u16 {
        unsafe {
            ffi::g_srv_target_get_weight(self.to_glib_none_mut().0)
        }
    }

    //pub fn list_sort(targets: /*Unimplemented*/&[&Fundamental: Pointer]) -> /*Unimplemented*/Vec<Fundamental: Pointer> {
    //    unsafe { TODO: call ffi::g_srv_target_list_sort() }
    //}
}
