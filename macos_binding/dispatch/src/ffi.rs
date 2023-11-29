#![allow(missing_docs)]
#![allow(non_camel_case_types)]

use std::ffi::{c_char, c_long, c_ulong, c_void};

#[repr(C)]
pub struct dispatch_object_s { _private: [u8; 0] }

pub type dispatch_function_t = extern fn(*mut c_void);
pub type dispatch_semaphore_t = *mut dispatch_object_s;
pub type dispatch_group_t = *mut dispatch_object_s;
pub type dispatch_object_t = *mut dispatch_object_s;
pub type dispatch_once_t = c_long;
pub type dispatch_queue_t = *mut dispatch_object_s;
pub type dispatch_time_t = u64;
pub type dispatch_queue_attr_t = *const dispatch_object_s;

extern {
    static _dispatch_main_q: dispatch_object_s;
    static _dispatch_queue_attr_concurrent: dispatch_object_s;

    pub fn dispatch_get_global_queue(identifier: c_long, flags: c_ulong) -> dispatch_queue_t;
    pub fn dispatch_queue_create(label: *const c_char, attr: dispatch_queue_attr_t) -> dispatch_queue_t;
    pub fn dispatch_queue_get_label(queue: dispatch_queue_t) -> *const c_char;
    pub fn dispatch_set_target_queue(object: dispatch_object_t, queue: dispatch_queue_t);
    pub fn dispatch_main();

    pub fn dispatch_async_f(queue: dispatch_queue_t, context: *mut c_void, work: dispatch_function_t);
    pub fn dispatch_async_and_wait_f(queue: dispatch_queue_t, context: *mut c_void, work: dispatch_function_t);
    pub fn dispatch_sync_f(queue: dispatch_queue_t, context: *mut c_void, work: dispatch_function_t);
    pub fn dispatch_after_f(when: dispatch_time_t, queue: dispatch_queue_t, context: *mut c_void, work: dispatch_function_t);
    pub fn dispatch_apply_f(iterations: usize, queue: dispatch_queue_t, context: *mut c_void, work: extern fn(*mut c_void, usize));
    pub fn dispatch_once_f(predicate: *mut dispatch_once_t, context: *mut c_void, function: dispatch_function_t);

    pub fn dispatch_group_async_f(group: dispatch_group_t, queue: dispatch_queue_t, context: *mut c_void, work: dispatch_function_t);
    pub fn dispatch_group_create() -> dispatch_group_t;
    pub fn dispatch_group_enter(group: dispatch_group_t);
    pub fn dispatch_group_leave(group: dispatch_group_t);
    pub fn dispatch_group_notify_f(group: dispatch_group_t, queue: dispatch_queue_t, context: *mut c_void, work: dispatch_function_t);
    pub fn dispatch_group_wait(group: dispatch_group_t, timeout: dispatch_time_t) -> c_long;

    pub fn dispatch_get_context(object: dispatch_object_t) -> *mut c_void;
    pub fn dispatch_release(object: dispatch_object_t);
    pub fn dispatch_resume(object: dispatch_object_t);
    pub fn dispatch_retain(object: dispatch_object_t);
    pub fn dispatch_set_context(object: dispatch_object_t, context: *mut c_void);
    pub fn dispatch_set_finalizer_f(object: dispatch_object_t, finalizer: dispatch_function_t);
    pub fn dispatch_suspend(object: dispatch_object_t);

    pub fn dispatch_semaphore_create(value: c_long) -> dispatch_semaphore_t;
    pub fn dispatch_semaphore_signal(dsema: dispatch_semaphore_t) -> c_long;
    pub fn dispatch_semaphore_wait(dsema: dispatch_semaphore_t, timeout: dispatch_time_t) -> c_long;

    pub fn dispatch_barrier_async_f(queue: dispatch_queue_t, context: *mut c_void, work: dispatch_function_t);
    pub fn dispatch_barrier_sync_f(queue: dispatch_queue_t, context: *mut c_void, work: dispatch_function_t);

    pub fn dispatch_time(when: dispatch_time_t, delta: i64) -> dispatch_time_t;
}

pub fn dispatch_get_main_queue() -> dispatch_queue_t {
    unsafe { &_dispatch_main_q as *const _ as dispatch_queue_t }
}

pub const DISPATCH_QUEUE_SERIAL: dispatch_queue_attr_t = 0 as dispatch_queue_attr_t;
pub static DISPATCH_QUEUE_CONCURRENT: &'static dispatch_object_s = unsafe { &_dispatch_queue_attr_concurrent };

pub const DISPATCH_QUEUE_PRIORITY_HIGH: c_long       = 2;
pub const DISPATCH_QUEUE_PRIORITY_DEFAULT: c_long    = 0;
pub const DISPATCH_QUEUE_PRIORITY_LOW: c_long        = -2;
pub const DISPATCH_QUEUE_PRIORITY_BACKGROUND: c_long = -1 << 15;

pub const DISPATCH_TIME_NOW: dispatch_time_t     = 0;
pub const DISPATCH_TIME_FOREVER: dispatch_time_t = !0;
