use crate::ffi::{dispatch_queue_attr_t, DISPATCH_QUEUE_CONCURRENT, DISPATCH_QUEUE_SERIAL};

pub enum QueueAttr {
    Serial,
    Concurrent,
}

impl QueueAttr {
    pub fn to_raw(&self) -> dispatch_queue_attr_t {
        match self {
            QueueAttr::Serial => DISPATCH_QUEUE_SERIAL,
            QueueAttr::Concurrent => DISPATCH_QUEUE_CONCURRENT,
        }
    }

    pub fn from_raw(raw: dispatch_queue_attr_t) -> Self {
        match raw {
            DISPATCH_QUEUE_SERIAL => QueueAttr::Serial,
            _ => panic!("Invalid raw value for QueueAttr"),
        }
    }
}