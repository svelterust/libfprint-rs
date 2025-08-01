use glib::translate::{ToGlibPtr, ToGlibPtrMut};
use std::ptr::NonNull;

use crate::FpDevice;

/// This struct allows you to discover fingerprint scanning hardware. This is the starting point when integrating libfprint-rs into your software.
pub struct FpContext {
    inner: NonNull<libfprint_sys::FpContext>,
}

impl FpContext {
    fn from_ptr(ptr: *mut libfprint_sys::FpContext) -> Option<Self> {
        NonNull::new(ptr).map(|inner| FpContext { inner })
    }
    
    fn as_ptr(&self) -> *mut libfprint_sys::FpContext {
        self.inner.as_ptr()
    }
}

impl Drop for FpContext {
    fn drop(&mut self) {
        // Note: libfprint contexts are typically not freed explicitly
        // as they're managed by the library lifecycle
    }
}

impl<'a> ToGlibPtr<'a, *mut libfprint_sys::FpContext> for FpContext {
    type Storage = &'a Self;

    fn to_glib_none(&'a self) -> glib::translate::Stash<'a, *mut libfprint_sys::FpContext, Self> {
        glib::translate::Stash(self.as_ptr(), self)
    }
}

impl<'a> ToGlibPtrMut<'a, *mut libfprint_sys::FpContext> for FpContext {
    type Storage = &'a mut Self;

    fn to_glib_none_mut(&'a mut self) -> glib::translate::StashMut<'a, *mut libfprint_sys::FpContext, Self> {
        glib::translate::StashMut(self.as_ptr(), self)
    }
}
impl FpContext {
    #[cfg(not(doctest))]
    /// Create a new `FpContext`
    /// # Examples:
    /// ```rust
    /// use libfprint_rs::FpContext;
    ///
    /// let context = FpContext::new();
    /// ```
    pub fn new() -> Option<Self> {
        unsafe { 
            let ptr = libfprint_sys::fp_context_new();
            Self::from_ptr(ptr)
        }
    }
    
    #[cfg(not(doctest))]
    /// Get the list of devices connected to the system
    /// # Examples:
    /// ```rust
    /// use libfprint_rs::FpContext;
    ///
    /// let context = FpContext::new().unwrap();
    /// let devices = context.devices();
    /// ```
    pub fn devices(&self) -> Vec<FpDevice> {
        unsafe {
            let devs_ptr = libfprint_sys::fp_context_get_devices(self.as_ptr());
            if devs_ptr.is_null() {
                return Vec::new();
            }
            
            // Cast to glib GPtrArray and extract devices
            let ptr_array = devs_ptr as *mut glib::ffi::GPtrArray;
            let len = (*ptr_array).len as usize;
            let mut devices = Vec::new();
            
            for i in 0..len {
                let device_ptr = *(*ptr_array).pdata.add(i) as *mut libfprint_sys::FpDevice;
                if let Some(device) = FpDevice::from_ptr(device_ptr) {
                    devices.push(device);
                }
            }
            
            devices
        }
    }

    /// Enumerate all the devices connected to the system
    ///
    /// This function will enumerate all the devices connected to the system and add them to the context.
    pub fn enumerate(&self) {
        unsafe {
            libfprint_sys::fp_context_enumerate(self.as_ptr());
        }
    }
}
