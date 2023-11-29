use std::ffi::c_char;
use std::os::macos::raw::dev_t;
use std::os::raw::c_void;
use core_foundation::array::CFArrayRef;
use core_foundation::base::{Boolean, CFAllocatorRef};
use core_foundation::date::CFTimeInterval;
use crate::fs_events::context::{RawFSEventStreamContext};
use crate::fs_events::r#enum::{FSEventStreamCreateFlags, FSEventStreamEventFlags, FSEventStreamEventId};

pub type FSEventStreamRef = *mut c_void;

pub type ConstFSEventStreamRef = *const c_void;

pub type FSEventStreamCallback = extern "C" fn(
    stream_ref: *const FSEventStreamRef,
    client_callback_info: *mut c_void,
    num_events: isize,
    event_paths: *mut *mut c_char,
    event_flags: *const FSEventStreamEventFlags,
    event_ids: *const FSEventStreamEventId,
);

extern "C" {
    #[link_name = "FSEventStreamCreate"]
    pub fn stream_create(
        allocator: CFAllocatorRef,
        callback: FSEventStreamCallback,
        context: *mut RawFSEventStreamContext,
        pathsToWatch: CFArrayRef,
        sinceWhen: FSEventStreamEventId,
        latency: CFTimeInterval,
        flags: FSEventStreamCreateFlags,
    ) -> FSEventStreamRef;

    #[link_name = "FSEventStreamStart"]
    pub fn stream_start(streamRef: FSEventStreamRef);

    #[link_name = "FSEventStreamStop"]
    pub fn stream_stop(streamRef: FSEventStreamRef);

    #[link_name = "FSEventStreamFlushSync"]
    pub fn stream_flush_sync(streamRef: FSEventStreamRef);

    #[link_name = "FSEventStreamFlushAsync"]
    pub fn stream_flush_async(streamRef: FSEventStreamRef) -> FSEventStreamEventId;

    #[link_name = "FSEventStreamShow"]
    pub fn stream_show(streamRef: ConstFSEventStreamRef);

    #[link_name = "FSEventStreamRelease"]
    pub fn stream_release(streamRef: FSEventStreamRef);

    #[link_name = "FSEventStreamRetain"]
    pub fn stream_retain(streamRef: FSEventStreamRef);

    #[link_name = "FSEventStreamInvalidate"]
    pub fn stream_invalidate(streamRef: FSEventStreamRef);

    #[link_name = "FSEventStreamSetDispatchQueue"]
    pub fn stream_set_dispatch_queue(streamRef: FSEventStreamRef, q: dispatch::ffi::dispatch_queue_t);

    #[link_name = "FSEventStreamSetExclusionPaths"]
    pub fn stream_set_exclusion_paths(streamRef: FSEventStreamRef, pathsToExclude: CFArrayRef) -> Boolean;

    #[link_name = "FSEventStreamGetLatestEventId"]
    pub fn stream_get_latest_event_id(streamRef: ConstFSEventStreamRef) -> FSEventStreamEventId;

    #[link_name = "FSEventStreamGetDeviceBeingWatched"]
    pub fn stream_get_device_being_watched(streamRef: ConstFSEventStreamRef) -> dev_t;
}