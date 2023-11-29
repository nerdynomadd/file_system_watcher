#![allow(dead_code)]

use std::ffi::{c_long, CStr, CString};
use std::time::Duration;
use std::str;
use crate::ffi::{dispatch_after_f, dispatch_apply_f, dispatch_async_and_wait_f, dispatch_async_f, dispatch_get_global_queue, dispatch_get_main_queue, dispatch_queue_create, dispatch_queue_get_label, dispatch_queue_t, dispatch_release, dispatch_resume, dispatch_retain, dispatch_suspend, dispatch_sync_f};
use crate::queue::attr::QueueAttr;
use crate::queue::priority::QueuePriority;
use crate::r#fn::{get_context_and_apply_fn, get_context_and_async_fn, get_context_and_sync_function, get_time_after_delay};

pub mod priority;
pub mod attr;

pub struct Queue {
    pub ptr: dispatch_queue_t,
}

impl Queue {
    pub fn main() -> Self {
        let queue = dispatch_get_main_queue();
        unsafe {
            dispatch_retain(queue);
        }
        Queue { ptr: queue }
    }

    pub fn global(priority: QueuePriority) -> Self {
        unsafe {
            let queue = dispatch_get_global_queue(priority.to_raw() as c_long, 0);
            dispatch_retain(queue);
            Queue { ptr: queue }
        }
    }

    pub fn create(label: &str, attr: QueueAttr) -> Self {
        let label = CString::new(label).unwrap();
        let queue = unsafe {
            dispatch_queue_create(label.as_ptr(), attr.to_raw())
        };
        Queue { ptr: queue }
    }

    pub fn label(&self) -> String {
        let label_ptr = unsafe { dispatch_queue_get_label(self.ptr) };
        if label_ptr.is_null() {
            return String::new();
        }

        let c_str = unsafe { CStr::from_ptr(label_ptr) };
        c_str.to_string_lossy().into_owned()
    }

    pub fn dispatch_sync<T, F>(&self, work: F) -> T
        where F: Send + FnOnce() -> T, T: Send
    {
        let mut result = None;
        {
            let result_ref = &mut result;
            let work = move || {
                *result_ref = Some(work())
            };

            let mut work = Some(work);
            let (context, work) = get_context_and_sync_function(&mut work);
            unsafe {
                dispatch_sync_f(self.ptr, context, work);
            }
        }
        result.unwrap()
    }

    pub fn dispatch_async_and_wait<T, F>(&self, work: F) -> T
        where F: 'static + Send + FnOnce() -> T, T: Send
    {
        let mut result = None;
        {
            let result_ref = &mut result;
            let work = move || {
                *result_ref = Some(work())
            };
            let (context, work) = get_context_and_async_fn(work);
            unsafe {
                dispatch_async_and_wait_f(self.ptr, context, work);
            }
        }
        result.unwrap()
    }

    pub fn dispatch_async<F>(&self, work: F) where F: 'static + Send + FnOnce() {
        let (context, work) = get_context_and_async_fn(work);
        unsafe {
            dispatch_async_f(self.ptr, context, work);
        }
    }

    pub fn dispatch_async_after<F>(&self, work: F, delay: Duration) where F: 'static + Send + FnOnce() {
        let when = get_time_after_delay(delay);
        let (context, work) = get_context_and_async_fn(work);
        unsafe {
            dispatch_after_f(when, self.ptr, context, work);
        }
    }

    pub fn dispatch_apply<F>(&self, iterations: usize, work: F) where F: 'static + Send + Fn(usize) {
        let (context, work) = get_context_and_apply_fn(&work);
        unsafe {
            dispatch_apply_f(iterations, self.ptr, context, work);
        }
    }

    pub fn suspend(&self) -> QueueExecutionGuard {
        QueueExecutionGuard::new(self)
    }
}

unsafe impl Sync for Queue {}
unsafe impl Send for Queue {}

impl Clone for Queue {
    fn clone(&self) -> Self {
        unsafe {
            dispatch_retain(self.ptr);
        }
        Queue { ptr: self.ptr }
    }
}

impl Drop for Queue {
    fn drop(&mut self) {
        unsafe {
            dispatch_release(self.ptr)
        }
    }
}

pub struct QueueExecutionGuard {
    queue: Queue,
}

impl QueueExecutionGuard {
    pub fn new(queue: &Queue) -> Self {
        unsafe {
            dispatch_suspend(queue.ptr);
        }
        QueueExecutionGuard { queue: queue.clone() }
    }
}

impl Clone for QueueExecutionGuard {
    fn clone(&self) -> Self {
        QueueExecutionGuard::new(&self.queue)
    }
}

impl Drop for QueueExecutionGuard {
    fn drop(&mut self) {
        unsafe {
            dispatch_resume(self.queue.ptr);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc;
    use std::time::Duration;

    // Test the creation of the main queue
    #[test]
    fn test_main_queue() {
        let main_queue = Queue::main();
        assert_eq!(main_queue.ptr.is_null(), false);
        assert_eq!(main_queue.label(), "com.apple.main-thread");
    }

    // Test the creation of a global queue
    #[test]
    fn test_global_queue() {
        let global_queue = Queue::global(QueuePriority::High);
        assert_eq!(global_queue.ptr.is_null(), false);
        assert_eq!(global_queue.label(), "com.apple.root.user-initiated-qos");
    }

    // Test the creation of a custom queue
    #[test]
    fn test_create_queue() {
        let custom_queue = Queue::create("com.example.myqueue", QueueAttr::Serial);
        assert_eq!(custom_queue.ptr.is_null(), false);
        assert_eq!(custom_queue.label(), "com.example.myqueue");
    }

    // Test getting the label of a queue
    #[test]
    fn test_queue_label() {
        let queue = Queue::create("com.example.myqueue", QueueAttr::Serial);
        assert_eq!(queue.label(), "com.example.myqueue");
    }

    // Test synchronous dispatch
    #[test]
    fn test_dispatch_sync() {
        let queue = Queue::create("com.example.myqueue", QueueAttr::Serial);
        let result = queue.dispatch_sync(|| 42);
        assert_eq!(result, 42);
    }

    #[test]
    fn test_dispatch_on_main_queue() {
        let queue = Queue::create("com.example.myqueue", QueueAttr::Serial);
        let main_queue = Queue::main();

        queue.dispatch_sync(|| {
            main_queue.dispatch_async_and_wait(|| {
                println!("Hello from main queue");
            });
        })
    }

    // Test asynchronous dispatch and wait
    #[test]
    fn test_dispatch_async_and_wait() {
        let queue = Queue::create("com.example.myqueue", QueueAttr::Serial);
        let (tx, rx) = mpsc::channel();
        queue.dispatch_async_and_wait(move || {
            tx.send(42).unwrap();
        });
        assert_eq!(rx.recv().unwrap(), 42);
    }

    // Test asynchronous dispatch
    #[test]
    fn test_dispatch_async() {
        let queue = Queue::create("com.example.myqueue", QueueAttr::Serial);
        let (tx, rx) = mpsc::channel();
        queue.dispatch_async(move || {
            tx.send(12).unwrap();
        });
        assert_eq!(rx.recv_timeout(Duration::from_secs(5)).unwrap(), 12);
    }

    // Test dispatch after a delay
    #[test]
    fn test_dispatch_async_after() {
        let queue = Queue::create("com.example.myqueue", QueueAttr::Serial);
        let (tx, rx) = mpsc::channel();
        let start_time = std::time::Instant::now();
        queue.dispatch_async_after(move || {
            tx.send(12).unwrap();
        }, Duration::from_secs(2));
        assert_eq!(rx.recv().unwrap(), 12);
        assert!(start_time.elapsed() >= Duration::from_secs(2));
    }

    // Test dispatch apply
    #[test]
    fn test_dispatch_apply() {
        let queue = Queue::create("com.example.myqueue", QueueAttr::Serial);
        let (tx, rx) = mpsc::channel();
        let iterations = 10;
        queue.dispatch_apply(iterations, move |i| {
            tx.send(i).unwrap();
        });
        for i in 0..iterations {
            assert_eq!(rx.recv().unwrap(), i);
        }
    }

    // Test cloning a queue
    #[test]
    fn test_clone_queue() {
        let queue = Queue::main();
        let cloned_queue = queue.clone();
        assert_eq!(queue.ptr, cloned_queue.ptr);
        assert_eq!(queue.label(), cloned_queue.label());
    }

    // Test dropping a queue
    #[test]
    fn test_drop_queue() {
        {
            let queue = Queue::main();
            assert_eq!(queue.ptr.is_null(), false);
        } // Queue goes out of scope and should be dropped here
        // Test to ensure the queue was properly released
    }
}