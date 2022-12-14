use winfsp_sys::WCHAR;

pub use winfsp_sys::FSP_FSCTL_DEFAULT_ALIGNMENT;
pub use winfsp_sys::FSP_FSCTL_DEVICECONTROL_SIZEMAX;
pub use winfsp_sys::FSP_FSCTL_TRANSACT_BATCH_BUFFER_SIZEMIN;
pub use winfsp_sys::FSP_FSCTL_TRANSACT_BUFFER_SIZEMIN;
pub use winfsp_sys::FSP_FSCTL_TRANSACT_REQ_SIZEMAX;
pub use winfsp_sys::FSP_FSCTL_TRANSACT_RSP_SIZEMAX;

#[repr(u32)]
#[derive(Copy, Clone)]
pub enum FspCleanupFlags {
    FspCleanupDelete = 0x01,
    FspCleanupSetAllocationSize = 0x02,
    FspCleanupSetArchiveBit = 0x10,
    FspCleanupSetLastAccessTime = 0x20,
    FspCleanupSetLastWriteTime = 0x40,
    FspCleanupSetChangeTime = 0x80,
}

impl FspCleanupFlags {
    /// Check if the provided bitfield has a flag.
    pub fn is_flagged(&self, flag: u32) -> bool {
        (*self as u32) & flag != 0
    }
}
pub const FSP_FSCTL_TRANSACT_PATH_SIZEMAX: u32 = (1024 * std::mem::size_of::<WCHAR>()) as u32;
