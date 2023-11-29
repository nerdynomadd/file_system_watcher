use crate::ffi::{DISPATCH_QUEUE_PRIORITY_BACKGROUND, DISPATCH_QUEUE_PRIORITY_DEFAULT, DISPATCH_QUEUE_PRIORITY_HIGH, DISPATCH_QUEUE_PRIORITY_LOW};

pub enum QueuePriority {
    High,
    Default,
    Low,
    Background,
}

impl QueuePriority {
    pub fn to_raw(&self) -> i64 {
        (match self {
            QueuePriority::High => DISPATCH_QUEUE_PRIORITY_HIGH,
            QueuePriority::Default => DISPATCH_QUEUE_PRIORITY_DEFAULT,
            QueuePriority::Low => DISPATCH_QUEUE_PRIORITY_LOW,
            QueuePriority::Background => DISPATCH_QUEUE_PRIORITY_BACKGROUND,
        }) as i64
    }

    pub fn from_raw(raw: i64) -> Self {
        match raw {
            DISPATCH_QUEUE_PRIORITY_HIGH => QueuePriority::High,
            DISPATCH_QUEUE_PRIORITY_DEFAULT => QueuePriority::Default,
            DISPATCH_QUEUE_PRIORITY_LOW => QueuePriority::Low,
            DISPATCH_QUEUE_PRIORITY_BACKGROUND => QueuePriority::Background,
            _ => panic!("Invalid raw value for QueuePriority"),
        }
    }
}