use std::{
    ffi::{c_char, c_void},
    fmt::{Debug, Formatter},
    mem,
    os::macos::raw::dev_t,
    sync::Arc,
    time::Duration,
};

use core_foundation::{
    array::CFArray,
    base::{CFAllocatorCopyDescriptionCallBack, CFAllocatorReleaseCallBack, CFAllocatorRetainCallBack, TCFType},
    string::CFString,
};

use dispatch::queue::Queue;

use crate::fs_events::{
    context::{FileSystemEventStreamContext, RawFSEventStreamContext},
    ffi::{FSEventStreamRef, stream_create, stream_flush_async, stream_flush_sync, stream_get_device_being_watched, stream_get_latest_event_id, stream_release, stream_retain, stream_set_dispatch_queue, stream_set_exclusion_paths, stream_show, stream_start, stream_stop},
    r#enum::{FSEventStreamPointInTime, FSEventStreamCreateFlags, FSEventStreamEventFlags, FSEventStreamEventId},
};

pub mod ffi;
pub mod context;
pub mod r#enum;

pub trait EventStreamCallback = Fn(Vec<*mut c_void>, isize, Vec<String>, Vec<FSEventStreamEventFlags>, Vec<FSEventStreamEventId>) -> ();

extern "C" fn event_stream_callback<F>(_: *const FSEventStreamRef, info: *mut c_void, num_events: isize, event_paths: *mut *mut c_char, event_flags: *const FSEventStreamEventFlags, event_ids: *const FSEventStreamEventId)
    where F: EventStreamCallback
{
    if num_events <= 0 {
        return;
    }

    let event_paths_slice = unsafe { std::slice::from_raw_parts(event_paths, num_events as usize) };

    let event_flags_slice = unsafe { std::slice::from_raw_parts(event_flags, num_events as usize) };

    let event_ids_slice = unsafe { std::slice::from_raw_parts(event_ids, num_events as usize) };

    let event_paths: Vec<String> = event_paths_slice
        .iter()
        .map(|path| unsafe { std::ffi::CStr::from_ptr(*path).to_string_lossy().into_owned() })
        .collect();

    let event_flags: Vec<FSEventStreamEventFlags> = event_flags_slice
        .iter()
        .map(|flag| *flag)
        .collect();

    let event_ids: Vec<FSEventStreamEventId> = event_ids_slice
        .iter()
        .map(|id| *id)
        .collect();

    if event_paths.len() != event_flags.len() || event_paths.len() != event_ids.len() {
        panic!("event_paths, event_flags, and event_ids must be the same length");
    }

    let info_container = unsafe { &*(info as *mut Option<Vec<*mut c_void>>) };

    if let None = info_container {
        panic!("info must contain at least one element");
    }

    if let Some(info) = info_container {
        if info.len() < 1 {
            panic!("info must contain at least one element");
        }

        let callback_fn = unsafe { &*(info[0] as *mut F) };
        let additional_info = info[1..].to_vec();

        callback_fn(additional_info, num_events, event_paths, event_flags, event_ids);
    }
}

pub struct FileSystemEventStream<'a> {
    stream_ref: FSEventStreamRef,
    is_started: bool,
    queue: Option<&'a Queue>,
}

impl<'a> FileSystemEventStream<'a> {
    pub fn new<F>(
        paths_to_watch: &Vec<&str>,
        since_when: FSEventStreamPointInTime,
        latency: f64,
        flags: FSEventStreamCreateFlags,
        callback: F,
        context: &mut FileSystemEventStreamContext
    ) -> Self
        where F: 'static + Send + EventStreamCallback
    {
        let cf_strings: Vec<CFString> = paths_to_watch
            .iter()
            .map(|path| CFString::new(path))
            .collect();

        let cf_array: CFArray<CFString> = CFArray::from_CFTypes(&cf_strings);

        let callback_ptr = Box::into_raw(Box::new(callback)) as *mut c_void;
        let mut info_container: Vec<*mut c_void> = vec![callback_ptr];

        if let Some(info) = context.info.as_mut() {
            info_container.append(info);
        }

        context.update_info_container(Some(info_container));

        let mut raw_context: RawFSEventStreamContext = context.into();

        let since_when: u64 = match since_when {
            FSEventStreamPointInTime::SinceNow => 0,
            FSEventStreamPointInTime::Since(eventId) => eventId,
            FSEventStreamPointInTime::SinceStartOfTime => 0
        };

        let stream_ref = unsafe {
            stream_create(
                std::ptr::null_mut(),
                event_stream_callback::<F>,
                &mut raw_context,
                cf_array.as_concrete_TypeRef(),
                since_when,
                latency,
                flags,
            )
        };
        Self { stream_ref, is_started: false, queue: None }
    }

    pub fn set_dispatch_queue(&mut self, queue: &'a Queue) {
        if self.is_started {
            panic!("Cannot set dispatch queue on a started stream");
        }

        unsafe {
            stream_set_dispatch_queue(self.stream_ref, queue.ptr);
        }

        self.queue = Some(queue);
    }

    pub fn exclude_paths(&self, paths_to_exclude: Vec<&str>) {
        if self.is_started {
            panic!("Cannot exclude path on a started stream");
        }

        let cf_strings: Vec<CFString> = paths_to_exclude
            .iter()
            .map(|path| CFString::new(path))
            .collect();

        let cf_array: CFArray<CFString> = CFArray::from_CFTypes(&cf_strings);

        unsafe {
            stream_set_exclusion_paths(self.stream_ref, cf_array.as_concrete_TypeRef());
        }
    }

    pub fn start(&mut self) {
        if self.queue.is_none() {
            panic!("Cannot start a stream without a dispatch queue");
        }

        unsafe {
            stream_start(self.stream_ref);
        }

        self.is_started = true;
    }

    pub fn stop(&mut self) {
        unsafe {
            stream_stop(self.stream_ref);
        }
        self.is_started = false;
    }

    pub fn flush(&self) {
        if !self.is_started {
            panic!("Cannot flush a stopped stream");
        }

        unsafe {
            stream_flush_sync(self.stream_ref);
        }
    }

    pub fn flush_async(&self) {
        if !self.is_started {
            panic!("Cannot flush a stopped stream");
        }

        unsafe {
            stream_flush_async(self.stream_ref);
        }
    }

    pub fn get_latest_event_id(&self) -> FSEventStreamEventId {
        unsafe {
            stream_get_latest_event_id(self.stream_ref)
        }
    }

    pub fn get_device_id(&self) -> dev_t {
        unsafe {
            stream_get_device_being_watched(self.stream_ref)
        }
    }
}

impl<'a> Debug for FileSystemEventStream<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        unsafe {
            stream_show(self.stream_ref);
        }
        write!(f, "FileSystemEventStream {{ stream_ref: {:?} }}", self.stream_ref)
    }
}

impl<'a> Clone for FileSystemEventStream<'a> {
    fn clone(&self) -> Self {
        unsafe {
            stream_retain(self.stream_ref);
        }

        FileSystemEventStream {
            stream_ref: self.stream_ref,
            is_started: self.is_started,
            queue: self.queue.clone(),
        }
    }
}

impl<'a> Drop for FileSystemEventStream<'a> {
    fn drop(&mut self) {
        unsafe {
            stream_release(self.stream_ref);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::ffi::c_char;
    use core_foundation::runloop::CFRunLoop;
    use dispatch::queue::attr::QueueAttr;
    use crate::fs_events::ffi::FSEventStreamCallback;
    use crate::fs_events::r#enum::FSEventStreamEventFlags;
    use crate::fs_events::r#enum::FSEventStreamPointInTime::SinceNow;
    use super::*;

    #[test]
    pub fn test_create_stream() {
        let dispatch_queue = Queue::create("test_queue", QueueAttr::Serial);
        extern fn fs_event_stream_callback(
            stream_ref: *const FSEventStreamRef,
            client_callback_info: *mut c_void,
            num_events: isize,
            event_paths: *mut *mut c_char,
            event_flags: *const FSEventStreamEventFlags,
            event_ids: *const ffi::FSEventStreamEventId,
        ) {
            let event_path = unsafe {
                String::from_utf8_lossy(std::ffi::CStr::from_ptr(*event_paths).to_bytes()).to_string()
            };

            println!("event_path: {:?}", event_path);
        }

        let mut context = FileSystemEventStreamContext::init(None);
        let paths = vec![""];
        let mut stream = FileSystemEventStream::new(
            &paths,
            SinceNow,
            0.0,
            ffi::FSEventStreamCreateFlags::NONE,
            fs_event_stream_callback,
            &mut context,
        );
        stream.set_dispatch_queue(&dispatch_queue);
        stream.start();
        CFRunLoop::run_current();
    }
}