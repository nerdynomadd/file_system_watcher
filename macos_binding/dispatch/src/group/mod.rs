#![allow(dead_code)]

use std::time::Duration;
use crate::ffi::{dispatch_group_async_f, dispatch_group_create, dispatch_group_enter, dispatch_group_leave, dispatch_group_notify_f, dispatch_group_t, dispatch_group_wait, dispatch_release, dispatch_resume, dispatch_retain, dispatch_suspend, DISPATCH_TIME_FOREVER};
use crate::queue::Queue;
use crate::r#fn::{get_context_and_async_fn, get_time_after_delay};

pub struct Group {
    ptr: dispatch_group_t,
}

impl Group {
    pub fn enter(&self) -> GroupEnterGuard {
        GroupEnterGuard::new(self)
    }

    pub fn create() -> Self {
        let ptr = unsafe { dispatch_group_create() };
        Group { ptr }
    }

    pub fn exec_async<F>(&self, queue: Queue, work: F)
        where F: 'static + Send + FnOnce() {
        let (context, work) = get_context_and_async_fn(work);
        unsafe {
            dispatch_group_async_f(self.ptr, queue.ptr, context, work);
        }
    }

    pub fn notify<F>(&self, queue: Queue, work: F)
        where F: 'static + Send + FnOnce() {
        let (context, work) = get_context_and_async_fn(work);
        unsafe {
            dispatch_group_notify_f(self.ptr, queue.ptr, context, work);
        }
    }

    pub fn wait(&self) {
        unsafe {
            dispatch_group_wait(self.ptr, DISPATCH_TIME_FOREVER);
        }
    }

    pub fn wait_timeout(&self, timeout: Duration) {
        let when = get_time_after_delay(timeout);
        unsafe {
            dispatch_group_wait(self.ptr, when);
        }
    }

    pub fn suspend(&self) -> GroupExecutionGuard {
        GroupExecutionGuard::new(self)
    }
}

pub struct GroupEnterGuard {
    group: Group,
}

impl GroupEnterGuard {
    pub fn new(group: &Group) -> Self {
        unsafe {
            dispatch_group_enter(group.ptr)
        }
        GroupEnterGuard { group: group.clone() }
    }
}

impl Clone for GroupEnterGuard {
    fn clone(&self) -> Self {
        Self::new(&self.group)
    }
}

impl Drop for GroupEnterGuard {
    fn drop(&mut self) {
        unsafe {
            dispatch_group_leave(self.group.ptr);
        }
    }
}

unsafe impl Send for Group {}
unsafe impl Sync for Group {}

impl Drop for Group {
    fn drop(&mut self) {
        unsafe {
            dispatch_release(self.ptr);
        }
    }
}

impl Clone for Group {
    fn clone(&self) -> Self {
        unsafe {
            dispatch_retain(self.ptr);
        }
        Group { ptr: self.ptr }
    }
}

pub struct GroupExecutionGuard {
    group: Group
}

impl GroupExecutionGuard {
    pub fn new(group: &Group) -> Self {
        unsafe {
            dispatch_suspend(group.ptr);
        }
        GroupExecutionGuard { group: group.clone() }
    }
}

impl Clone for GroupExecutionGuard {
    fn clone(&self) -> Self {
        GroupExecutionGuard::new(&self.group)
    }
}

impl Drop for GroupExecutionGuard {
    fn drop(&mut self) {
        unsafe {
            dispatch_resume(self.group.ptr);
        }
    }
}