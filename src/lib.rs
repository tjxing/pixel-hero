mod init;
mod conf;
mod emulator;
mod i18n;
mod rom;
mod log;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
