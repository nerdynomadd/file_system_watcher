use bitflags::bitflags;

pub enum FSEventStreamPointInTime {
    SinceNow,
    Since(FSEventStreamEventId),
    SinceStartOfTime,
}

pub type FSEventStreamEventId = u64;

bitflags! {
    #[derive(Copy, Clone, Debug)]
    pub struct FSEventStreamCreateFlags: u32 {
        const NONE                   = 0x00000000;
        const USE_CF_TYPES           = 0x00000001;
        const NO_DEFER               = 0x00000002;
        const WATCH_ROOT             = 0x00000004;
        const IGNORE_SELF            = 0x00000008;
        const FILE_EVENTS            = 0x00000010;
        const MARK_SELF              = 0x00000020;
        const USE_EXTENDED_DATA      = 0x00000040;
        const FULL_HISTORY           = 0x00000080;
    }
}

bitflags! {
    #[derive(Copy, Clone, Debug)]
    pub struct FSEventStreamEventFlags: u32 {
        const NONE                   = 0x00000000;
        const MUST_SCAN_SUB_DIRS     = 0x00000001;
        const USER_DROPPED           = 0x00000002;
        const KERNEL_DROPPED         = 0x00000004;
        const EVENT_IDS_WRAPPED      = 0x00000008;
        const HISTORY_DONE           = 0x00000010;
        const ROOT_CHANGED           = 0x00000020;
        const MOUNT                  = 0x00000040;
        const UNMOUNT                = 0x00000080;
        const ITEM_CREATED           = 0x00000100;
        const ITEM_REMOVED           = 0x00000200;
        const ITEM_INODE_META_MOD    = 0x00000400;
        const ITEM_RENAMED           = 0x00000800;
        const ITEM_MODIFIED          = 0x00001000;
        const ITEM_FINDER_INFO_MOD   = 0x00002000;
        const ITEM_CHANGE_OWNER      = 0x00004000;
        const ITEM_XATTR_MOD         = 0x00008000;
        const ITEM_IS_FILE           = 0x00010000;
        const ITEM_IS_DIR            = 0x00020000;
        const ITEM_IS_SYMLINK        = 0x00040000;
        const OWN_EVENT              = 0x00080000;
        const ITEM_IS_HARDLINK       = 0x00100000;
        const ITEM_IS_LAST_HARDLINK  = 0x00200000;
        const ITEM_CLONED            = 0x00400000;
    }
}