// This file was generated by gir (13f739b) from gir-files (469db10)
// DO NOT EDIT

use IOStream;
#[cfg(any(feature = "v2_44", feature = "dox"))]
use InputStream;
#[cfg(any(feature = "v2_44", feature = "dox"))]
use OutputStream;
use ffi;
use glib;
use glib::object::Downcast;
use glib::object::IsA;
#[cfg(any(feature = "v2_44", feature = "dox"))]
use glib::signal::SignalHandlerId;
#[cfg(any(feature = "v2_44", feature = "dox"))]
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
#[cfg(any(feature = "v2_44", feature = "dox"))]
use std::boxed::Box as Box_;
use std::mem;
#[cfg(any(feature = "v2_44", feature = "dox"))]
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct SimpleIOStream(Object<ffi::GSimpleIOStream>): IOStream;

    match fn {
        get_type => || ffi::g_simple_io_stream_get_type(),
    }
}

impl SimpleIOStream {
    #[cfg(any(feature = "v2_44", feature = "dox"))]
    pub fn new<P: IsA<InputStream>, Q: IsA<OutputStream>>(input_stream: &P, output_stream: &Q) -> SimpleIOStream {
        unsafe {
            IOStream::from_glib_full(ffi::g_simple_io_stream_new(input_stream.to_glib_none().0, output_stream.to_glib_none().0)).downcast_unchecked()
        }
    }
}

pub trait SimpleIOStreamExt {
    #[cfg(any(feature = "v2_44", feature = "dox"))]
    fn connect_property_input_stream_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v2_44", feature = "dox"))]
    fn connect_property_output_stream_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<SimpleIOStream> + IsA<glib::object::Object>> SimpleIOStreamExt for O {
    #[cfg(any(feature = "v2_44", feature = "dox"))]
    fn connect_property_input_stream_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::input-stream",
                transmute(notify_input_stream_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v2_44", feature = "dox"))]
    fn connect_property_output_stream_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::output-stream",
                transmute(notify_output_stream_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

#[cfg(any(feature = "v2_44", feature = "dox"))]
unsafe extern "C" fn notify_input_stream_trampoline<P>(this: *mut ffi::GSimpleIOStream, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<SimpleIOStream> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SimpleIOStream::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v2_44", feature = "dox"))]
unsafe extern "C" fn notify_output_stream_trampoline<P>(this: *mut ffi::GSimpleIOStream, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<SimpleIOStream> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&SimpleIOStream::from_glib_borrow(this).downcast_unchecked())
}
