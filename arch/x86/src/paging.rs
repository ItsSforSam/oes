//! Handling of pages and MMU
use x86_64::structures::paging::page_table::PageTableEntry;
#[repr(align(4096))] // Page alined
pub struct PageTable {
    entries: [PageTableEntry; 512],
}
