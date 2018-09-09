// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Error;
use ffi;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct Credentials(Object<ffi::GCredentials, ffi::GCredentialsClass>);

    match fn {
        get_type => || ffi::g_credentials_get_type(),
    }
}

impl Credentials {
    pub fn new() -> Credentials {
        unsafe {
            from_glib_full(ffi::g_credentials_new())
        }
    }
}

impl Default for Credentials {
    fn default() -> Self {
        Self::new()
    }
}

pub trait CredentialsExt {
    //fn get_native(&self, native_type: CredentialsType) -> /*Unimplemented*/Option<Fundamental: Pointer>;

    #[cfg(any(unix, feature = "dox"))]
    #[cfg(any(feature = "v2_36", feature = "dox"))]
    fn get_unix_pid(&self) -> Result<i32, Error>;

    #[cfg(any(unix, feature = "dox"))]
    fn get_unix_user(&self) -> Result<(), Error>;

    fn is_same_user(&self, other_credentials: &Credentials) -> Result<(), Error>;

    //fn set_native(&self, native_type: CredentialsType, native: /*Unimplemented*/Fundamental: Pointer);

    #[cfg(any(unix, feature = "dox"))]
    fn set_unix_user(&self, uid: u32) -> Result<(), Error>;

    fn to_string(&self) -> String;
}

impl<O: IsA<Credentials>> CredentialsExt for O {
    //fn get_native(&self, native_type: CredentialsType) -> /*Unimplemented*/Option<Fundamental: Pointer> {
    //    unsafe { TODO: call ffi::g_credentials_get_native() }
    //}

    #[cfg(any(unix, feature = "dox"))]
    #[cfg(any(feature = "v2_36", feature = "dox"))]
    fn get_unix_pid(&self) -> Result<i32, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_credentials_get_unix_pid(self.to_glib_none().0, &mut error);
            if error.is_null() { Ok(ret) } else { Err(from_glib_full(error)) }
        }
    }

    #[cfg(any(unix, feature = "dox"))]
    fn get_unix_user(&self) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::g_credentials_get_unix_user(self.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn is_same_user(&self, other_credentials: &Credentials) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::g_credentials_is_same_user(self.to_glib_none().0, other_credentials.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    //fn set_native(&self, native_type: CredentialsType, native: /*Unimplemented*/Fundamental: Pointer) {
    //    unsafe { TODO: call ffi::g_credentials_set_native() }
    //}

    #[cfg(any(unix, feature = "dox"))]
    fn set_unix_user(&self, uid: u32) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::g_credentials_set_unix_user(self.to_glib_none().0, uid, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn to_string(&self) -> String {
        unsafe {
            from_glib_full(ffi::g_credentials_to_string(self.to_glib_none().0))
        }
    }
}