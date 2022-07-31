extern crate libc;

use libc::c_char;

use std::os::raw::c_int;
use std::ffi::CStr;
use std::str;

use wacom_sys;

pub struct Error {
    code: wacom_sys::WacomErrorCode,
    msg: String,
}

impl Error {
    pub fn print_error(&self) {
        eprintln!("Error: {:?}\n{:?}", self.code, self.msg);
    }
}

pub struct Device {
    pub device: wacom_sys::WacomDevice,
    pub width: i32,
    pub height: i32,
}

impl Device {
    pub fn new( vendor_id: c_int, product_id: c_int) -> Result<Device, Error> {
        let database = unsafe {
            wacom_sys::libwacom_database_new() as *mut wacom_sys::WacomDeviceDatabase
        };

        let err_ptr = unsafe { wacom_sys::libwacom_error_new() };
        if err_ptr.is_null() {
            unsafe { wacom_sys::libwacom_database_destroy(database); }
            return Err(Error{
                code: wacom_sys::WacomErrorCode::WERROR_BAD_ALLOC,
                msg: String::from("Memory allocation rror creating error pointer.")
            });
        }

        let wacom_device_ptr = unsafe {
            wacom_sys::libwacom_new_from_usbid(
                database,
                vendor_id,
                product_id,
                err_ptr
            )
        };

        let mut err = unsafe { *err_ptr };

        if wacom_device_ptr.is_null() {
            let code: wacom_sys::WacomErrorCode = unsafe {
                wacom_sys::libwacom_error_get_code(&mut err)
            };

            unsafe { wacom_sys::libwacom_database_destroy(database); }

            let c_buf: *const c_char = unsafe { wacom_sys::libwacom_error_get_message(&mut err) };
            let c_str: &CStr = unsafe { CStr::from_ptr(c_buf) };
            let str_slice: &str = c_str.to_str().unwrap();
            let msg: String = str_slice.to_owned();

            let ptr = Box::new(err_ptr);
            let ptr2 = Box::into_raw(ptr);
            unsafe { wacom_sys::libwacom_error_free(ptr2); }
            return Err(Error { code, msg });
        }

        let width = unsafe { wacom_sys::libwacom_get_width(wacom_device_ptr) };
        let height = unsafe { wacom_sys::libwacom_get_height(wacom_device_ptr) };
        Ok(
            Device{
                device: unsafe{ *wacom_device_ptr },
                width,
                height,
            }
        )
    }
}
