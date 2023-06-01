/* platform/windows/hostinfo.rs - Windows host info routines for reutils
 * Copyright (C) 2023 Elizabeth Myers. All rights reserved.
 * SPDX-License-Identifier: GPL-2.0-only
 */

pub mod version;

use std::ffi::{c_void, OsString};
use std::os::windows::ffi::OsStringExt;
use std::ptr;

use windows::core::{HSTRING, PWSTR};
use windows::Win32::Storage::FileSystem::*;
use windows::Win32::System::Diagnostics::Debug;
use windows::Win32::System::SystemInformation::*;

use crate::platform::windows::hostinfo::version::*;

pub fn hostname() -> String {
    let mut buflen: u32 = 0;

    unsafe {
        // Get buffer size
        GetComputerNameExW(
            ComputerNamePhysicalDnsFullyQualified,
            PWSTR(ptr::null_mut()),
            &mut buflen,
        )
    };

    if buflen == 0 {
        // Something has gone terribly wrong!
        panic!("GetComputerNameExW did not provide buffer size");
    }

    let mut buffer = vec![0_u16; buflen as usize];
    if unsafe {
        GetComputerNameExW(
            ComputerNamePhysicalDnsFullyQualified,
            PWSTR(buffer.as_mut_ptr()),
            &mut buflen,
        )
    } == false
    {
        panic!("GetComputerNameExW could not read hostname");
    }

    if (buflen as usize) != (buffer.len() - 1) {
        panic!("GetComputerNameExW changed the buffer size unexpectedly");
    }

    let end = buffer.iter().position(|&b| b == 0).unwrap_or(buffer.len());
    OsString::from_wide(&buffer[0..end]).into_string().unwrap()
}

/* Windows doesn't provide a good API for getting the version on modern Windows
 * This is the best we can do.
 */
pub fn release() -> String {
    let filename = HSTRING::from("kernel32.dll");
    let mut dummy = 0u32;

    let cbinfo = unsafe { GetFileVersionInfoSizeExW(FILE_VER_GET_NEUTRAL, &filename, &mut dummy) };
    if cbinfo == 0 {
        panic!("GetFileVersionInfoSizeExW failed");
    }

    let mut buffer = vec![0u8; cbinfo as usize];

    if unsafe {
        GetFileVersionInfoExW(
            FILE_VER_GET_NEUTRAL,
            &filename,
            dummy,
            buffer.len() as u32,
            buffer.as_mut_ptr() as *mut c_void,
        )
    } == false
    {
        panic!("GetFileVersionInfoExW failed");
    }

    let mut size = 0u32;
    let mut p: *mut c_void = std::ptr::null_mut();

    if unsafe {
        VerQueryValueW(
            buffer.as_mut_ptr() as *mut c_void,
            &HSTRING::from("\\"),
            &mut p,
            &mut size,
        )
    } == false
    {
        panic!("VerQueryValueW failed");
    }

    if p.is_null() {
        panic!("VerQueryValueW returned null pointer");
    }

    let pfixed = unsafe { *(p as *const VS_FIXEDFILEINFO) };
    format!(
        "{}.{}.{}.{}",
        (pfixed.dwFileVersionMS & 0xFFFF0000u32) >> 16,
        pfixed.dwFileVersionMS & 0x0000FFFFu32,
        (pfixed.dwFileVersionLS & 0xFFFF0000u32) >> 16,
        pfixed.dwFileVersionLS & 0x0000FFFFu32
    )
}

/* Basically the inverse of the Microsoft documentation.
 * We start from the latest version and work our way down.
 */
pub fn version() -> String {
    let release_ver = if is_windows_11_or_greater() {
        "11"
    } else if is_windows_10_or_greater() {
        "10"
    } else if is_windows_threshold_or_greater() {
        "Threshold"
    } else if is_windows_8_point_1_or_greater() {
        "8.1"
    } else if is_windows_8_or_greater() {
        "8"
    } else if is_windows_7_sp1_or_greater() {
        "7 SP1"
    } else if is_windows_7_or_greater() {
        "7"
    } else if is_windows_vista_sp2_or_greater() {
        "Vista SP2"
    } else if is_windows_vista_sp1_or_greater() {
        "Vista SP1"
    } else if is_windows_vista_or_greater() {
        "Vista"
    } else if is_windows_xp_sp3_or_greater() {
        "XP SP3"
    } else if is_windows_xp_sp2_or_greater() {
        "XP SP2"
    } else if is_windows_xp_sp1_or_greater() {
        "XP SP1"
    } else if is_windows_xp_or_greater() {
        "XP"
    } else {
        "<unknown>"
    };

    let edition = if is_windows_server() {
        "Server"
    } else {
        "Client"
    };

    format!("Windows {release_ver} {edition}")
}

pub fn architecture() -> String {
    let mut system_info: SYSTEM_INFO = Default::default();

    unsafe {
        GetNativeSystemInfo(&mut system_info);
    }

    let processor_arch = unsafe { system_info.Anonymous.Anonymous.wProcessorArchitecture };
    match processor_arch {
        Debug::PROCESSOR_ARCHITECTURE_AMD64 => "amd64".to_string(),
        Debug::PROCESSOR_ARCHITECTURE_ARM => "arm".to_string(),
        //Debug::PROCESSOR_ARCHITECTURE_ARM64 => "arm64".to_string(),
        Debug::PROCESSOR_ARCHITECTURE_IA64 => "ia64".to_string(),
        Debug::PROCESSOR_ARCHITECTURE_INTEL => "x86".to_string(),
        _ => "unknown".to_string(),
    }
}
