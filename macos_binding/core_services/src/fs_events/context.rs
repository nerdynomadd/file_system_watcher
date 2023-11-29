use std::os::raw::c_void;
use core_foundation::base::{CFAllocatorCopyDescriptionCallBack, CFAllocatorReleaseCallBack, CFAllocatorRetainCallBack};


pub struct FileSystemEventStreamContext {
    pub version: u32,
    pub info: Option<Vec<*mut c_void>>,
    pub retain: Option<CFAllocatorRetainCallBack>,
    pub release: Option<CFAllocatorReleaseCallBack>,
    pub copy_description: Option<CFAllocatorCopyDescriptionCallBack>,
}

impl FileSystemEventStreamContext {
    pub fn init(info: Option<Vec<*mut c_void>>) -> Self {
        Self {
            version: 0,
            info,
            retain: None,
            release: None,
            copy_description: None,
        }
    }

    pub fn init_with_callbacks(
        version: u32,
        info: Option<Vec<*mut c_void>>,
        retain: CFAllocatorRetainCallBack,
        release: CFAllocatorReleaseCallBack,
        copy_description: CFAllocatorCopyDescriptionCallBack,
    ) -> Self {
        Self {
            version,
            info,
            retain: Some(retain),
            release: Some(release),
            copy_description: Some(copy_description),
        }
    }

    pub(crate) fn update_info_container(&mut self, info: Option<Vec<*mut c_void>>) {
        self.info = info;
    }
}

impl<'a> Into<RawFSEventStreamContext> for &'a mut FileSystemEventStreamContext {
    fn into(self) -> RawFSEventStreamContext {
        RawFSEventStreamContext::new(
            self.version.clone(),
            self.info.clone().map(|info| Box::into_raw(Box::new(info.clone())) as *mut c_void).unwrap_or(std::ptr::null_mut()),
            self.retain.clone(),
            self.release.clone(),
            self.copy_description.clone()
        )
    }
}

#[repr(C)]
pub struct RawFSEventStreamContext
{
    pub version: u32,
    pub info: *mut c_void,
    pub retain: Option<CFAllocatorRetainCallBack>,
    pub release: Option<CFAllocatorReleaseCallBack>,
    pub copy_description: Option<CFAllocatorCopyDescriptionCallBack>,
}

impl RawFSEventStreamContext {
    pub fn new(version: u32,
    info: *mut c_void,
    retain: Option<CFAllocatorRetainCallBack>,
    release: Option<CFAllocatorReleaseCallBack>,
    copy_description: Option<CFAllocatorCopyDescriptionCallBack>) -> Self
    {
        Self {
            version,
            info,
            retain,
            release,
            copy_description,
        }
    }
}