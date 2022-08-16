#![feature(io_error_more)]

mod fs;
mod fsp;
mod service;

use clap::Parser;
use std::path::PathBuf;
use windows::w;
use windows::Win32::Foundation::{ERROR_DELAY_LOAD_FAILED, STATUS_NOT_IMPLEMENTED, STATUS_SUCCESS};
use windows::Win32::System::LibraryLoader::LoadLibraryW;
use winfsp_sys::*;


unsafe extern "C" fn _svc_start(service: *mut FSP_SERVICE, argc: u32, argv: *mut *mut u16) -> i32 {
    let args = Args::parse();

    match service::svc_start(fsp::FspService::from_raw_unchecked(service), args) {
        Err(e) => STATUS_NOT_IMPLEMENTED.0,
        Ok(_) => STATUS_SUCCESS.0
    }
}

unsafe extern "C" fn _svc_stop(service: *mut FSP_SERVICE) -> i32 {
    service::svc_stop(fsp::FspService::from_raw_unchecked(service)).0
}

/// MainArgs
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// -1: enable all debug logs
    #[clap(short = 'd', default_value = "0")]
    flags: u32,

    /// file path
    #[clap(short = 'D', long)]
    logfile: Option<PathBuf>,

    #[clap(short = 'u', long)]
    volume_prefix: Option<String>,

    #[clap(short = 'p', long)]
    directory: PathBuf,

    #[clap(short = 'm', long)]
    mountpoint: PathBuf,
}

fn main() {
    unsafe {
        // todo: make this fallible
        if LoadLibraryW(w!("winfsp-x64.dll")).is_err() {
            std::process::exit(ERROR_DELAY_LOAD_FAILED.0 as i32)
        }
        FspServiceRunEx(
            w!("snowflake-fsp").as_ptr().cast_mut(),
            Some(_svc_start),
            Some(_svc_stop),
            None,
            std::ptr::null_mut(),
        );
    }
}
