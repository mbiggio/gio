use crate::AsyncResult;
use crate::Cancellable;
use crate::Task;
use glib::object::IsA;
use glib::translate::*;
use libc::c_void;
use std::boxed::Box as Box_;
use std::ptr;

impl Task {
    pub fn new<P: IsA<Cancellable>, Q: FnOnce(&AsyncResult, Option<&glib::Object>) + 'static>(
        source_object: Option<&glib::Object>,
        cancellable: Option<&P>,
        callback: Q,
    ) -> Task {
        let callback_data = Box_::new(callback);
        unsafe extern "C" fn trampoline<
            Q: FnOnce(&AsyncResult, Option<&glib::Object>) + 'static,
        >(
            source_object: *mut gobject_sys::GObject,
            res: *mut gio_sys::GAsyncResult,
            user_data: glib_sys::gpointer,
        ) {
            let source_object = Option::<glib::Object>::from_glib_borrow(source_object);
            let res = AsyncResult::from_glib_borrow(res);
            let callback: Box_<Q> = Box::from_raw(user_data as *mut _);
            callback(&res, source_object.as_ref().as_ref());
        };
        let callback = trampoline::<Q>;
        unsafe {
            from_glib_full(gio_sys::g_task_new(
                source_object.to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(callback_data) as *mut _,
            ))
        }
    }

    pub fn return_error(&self, error: glib::Error) {
        unsafe {
            gio_sys::g_task_return_error(self.to_glib_none().0, error.to_glib_full() as *mut _);
        }
    }

    pub fn get_priority(&self) -> glib::source::Priority {
        unsafe { FromGlib::from_glib(gio_sys::g_task_get_priority(self.to_glib_none().0)) }
    }

    pub fn set_priority(&self, priority: glib::source::Priority) {
        unsafe {
            gio_sys::g_task_set_priority(self.to_glib_none().0, priority.to_glib());
        }
    }

    pub fn return_value(&self, result: &glib::Value) {
        unsafe extern "C" fn value_free(value: *mut c_void) {
            gobject_sys::g_value_unset(value as *mut gobject_sys::GValue);
            glib_sys::g_free(value);
        }
        unsafe {
            let value: *mut gobject_sys::GValue =
                <&glib::Value>::to_glib_full_from_slice(&[result]);
            gio_sys::g_task_return_pointer(
                self.to_glib_none().0,
                value as *mut c_void,
                Some(value_free),
            )
        }
    }

    pub fn propagate_value(&self) -> Result<glib::Value, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let value = gio_sys::g_task_propagate_pointer(self.to_glib_none().0, &mut error);
            if !error.is_null() {
                return Err(from_glib_full(error));
            }
            let value = from_glib_full(value as *mut gobject_sys::GValue);
            match value {
                Some(value) => Ok(value),
                None => Ok(glib::Value::from_type(glib::types::Type::Unit)),
            }
        }
    }
}
