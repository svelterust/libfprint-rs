mod callback;
mod device;
mod device_sync;
mod enums;
mod user_data;

pub use device_sync::{FpEnrollProgress, FpMatchCb};
use glib::translate::{ToGlibPtr, ToGlibPtrMut, FromGlibPtrBorrow, FromGlibPtrNone};
use std::ptr::NonNull;

#[cfg(not(doctest))]
/// Fingerpint device routines. You can interact with fingerprint devices using this struct.
///
/// # Examples:
/// ```rust
/// use libfprint_rs::FpContext;
///
/// let context = FpContext::new().unwrap();
/// let devices = context.devices();
/// let device = devices.get(0).unwrap();
///
/// device.open_sync(None).unwrap();
/// let name = device.name().unwrap();
/// println!("Device name: {}", name);
/// ```
pub struct FpDevice {
    inner: NonNull<libfprint_sys::FpDevice>,
}

impl FpDevice {
    pub(crate) fn from_ptr(ptr: *mut libfprint_sys::FpDevice) -> Option<Self> {
        NonNull::new(ptr).map(|inner| FpDevice { inner })
    }
    
    pub(crate) fn as_ptr(&self) -> *mut libfprint_sys::FpDevice {
        self.inner.as_ptr()
    }
}

impl<'a> ToGlibPtr<'a, *mut libfprint_sys::FpDevice> for FpDevice {
    type Storage = &'a Self;

    fn to_glib_none(&'a self) -> glib::translate::Stash<'a, *mut libfprint_sys::FpDevice, Self> {
        glib::translate::Stash(self.as_ptr(), self)
    }
}

impl<'a> ToGlibPtrMut<'a, *mut libfprint_sys::FpDevice> for FpDevice {
    type Storage = &'a mut Self;

    fn to_glib_none_mut(&'a mut self) -> glib::translate::StashMut<'a, *mut libfprint_sys::FpDevice, Self> {
        glib::translate::StashMut(self.as_ptr(), self)
    }
}

impl FromGlibPtrBorrow<*mut libfprint_sys::FpDevice> for FpDevice {
    unsafe fn from_glib_borrow(ptr: *mut libfprint_sys::FpDevice) -> glib::translate::Borrowed<Self> {
        assert!(!ptr.is_null());
        glib::translate::Borrowed::new(FpDevice { inner: NonNull::new_unchecked(ptr) })
    }
}

impl FromGlibPtrNone<*mut libfprint_sys::FpDevice> for FpDevice {
    unsafe fn from_glib_none(ptr: *mut libfprint_sys::FpDevice) -> Self {
        assert!(!ptr.is_null());
        FpDevice { inner: NonNull::new_unchecked(ptr) }
    }
}

pub(crate) struct UserData<F, T> {
    function: F,
    data: Option<T>,
}

impl<F, T> Drop for UserData<F, T> {
    fn drop(&mut self) {
        if self.data.is_some() {
            drop(self.data.take())
        }
    }
}

macro_rules! fn_pointer {
    ($function:ident, $struct:ident) => {{
        let ptr: *mut std::ffi::c_void = match $function {
            Some(cb) => {
                let data = crate::device::UserData {
                    function: cb,
                    data: $struct,
                };
                let boxed = std::sync::Arc::new(data);
                std::sync::Arc::into_raw(boxed) as *mut std::ffi::c_void
            }
            None => std::ptr::null_mut(),
        };
        ptr
    }};
}

use fn_pointer;
