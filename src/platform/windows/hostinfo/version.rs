/* platform/windows/hostinfo/version.rs - Windows host info version routines for reutils
 * Copyright (C) 2023 Elizabeth Myers. All rights reserved.
 * SPDX-License-Identifier: GPL-2.0-only
 */

/* Sadly these API's are not exported in the official Windows crate. Winsafe has them, but I don't
 * really want to bring that in just for this.
 *
 * All this was adapted from MingW's versionhelpers.h header.
 *
 * I added a buildno parameter to is_windows_version_or_greater as a workaround for Windows 11
 * detection.
 *
 * IsWindows10OrGreater was slightly modified by me. I also added IsWindows11OrGreater.
 *
 * -- Elizafox
 */

use std::mem::size_of;

use windows::Win32::System::SystemInformation::{OSVERSIONINFOEXW, VER_MAJORVERSION, VER_MINORVERSION, VER_PRODUCT_TYPE, VER_SERVICEPACKMAJOR, VerSetConditionMask, VerifyVersionInfoW, _WIN32_WINNT_VISTA, _WIN32_WINNT_WIN10, _WIN32_WINNT_WIN7, _WIN32_WINNT_WIN8, _WIN32_WINNT_WINBLUE, _WIN32_WINNT_WINTHRESHOLD, _WIN32_WINNT_WINXP};
use windows::Win32::System::SystemServices::{VER_EQUAL, VER_GREATER_EQUAL, VER_NT_WORKSTATION};

fn is_windows_version_or_greater(major: u32, minor: u32, servpack: u16, buildno: u32) -> bool {
    let mut vi: OSVERSIONINFOEXW = Default::default();
    vi.dwOSVersionInfoSize = size_of::<OSVERSIONINFOEXW>() as u32;
    vi.dwMajorVersion = major;
    vi.dwMinorVersion = minor;
    vi.wServicePackMajor = servpack;
    vi.dwBuildNumber = buildno;

    unsafe {
        VerifyVersionInfoW(
            &mut vi,
            VER_MAJORVERSION | VER_MINORVERSION | VER_SERVICEPACKMAJOR,
            VerSetConditionMask(
                VerSetConditionMask(
                    VerSetConditionMask(0, VER_MAJORVERSION, VER_GREATER_EQUAL as u8),
                    VER_MINORVERSION,
                    VER_GREATER_EQUAL as u8,
                ),
                VER_SERVICEPACKMAJOR,
                VER_GREATER_EQUAL as u8,
            ),
        )
    }
    .into()
}

pub fn is_windows_xp_or_greater() -> bool {
    is_windows_version_or_greater(
        (_WIN32_WINNT_WINXP >> 8) & 0xFF,
        _WIN32_WINNT_WINXP & 0xFF,
        0,
        0,
    )
}

pub fn is_windows_xp_sp1_or_greater() -> bool {
    is_windows_version_or_greater(
        (_WIN32_WINNT_WINXP >> 8) & 0xFF,
        _WIN32_WINNT_WINXP & 0xFF,
        1,
        0,
    )
}

pub fn is_windows_xp_sp2_or_greater() -> bool {
    is_windows_version_or_greater(
        (_WIN32_WINNT_WINXP >> 8) & 0xFF,
        _WIN32_WINNT_WINXP & 0xFF,
        2,
        0,
    )
}

pub fn is_windows_xp_sp3_or_greater() -> bool {
    is_windows_version_or_greater(
        (_WIN32_WINNT_WINXP >> 8) & 0xFF,
        _WIN32_WINNT_WINXP & 0xFF,
        3,
        0,
    )
}

pub fn is_windows_vista_or_greater() -> bool {
    is_windows_version_or_greater(
        (_WIN32_WINNT_VISTA >> 8) & 0xFF,
        _WIN32_WINNT_VISTA & 0xFF,
        0,
        0,
    )
}

pub fn is_windows_vista_sp1_or_greater() -> bool {
    is_windows_version_or_greater(
        (_WIN32_WINNT_VISTA >> 8) & 0xFF,
        _WIN32_WINNT_VISTA & 0xFF,
        1,
        0,
    )
}

pub fn is_windows_vista_sp2_or_greater() -> bool {
    is_windows_version_or_greater(
        (_WIN32_WINNT_VISTA >> 8) & 0xFF,
        _WIN32_WINNT_VISTA & 0xFF,
        2,
        0,
    )
}

pub fn is_windows_7_or_greater() -> bool {
    is_windows_version_or_greater(
        (_WIN32_WINNT_WIN7 >> 8) & 0xFF,
        _WIN32_WINNT_WIN7 & 0xFF,
        0,
        0,
    )
}

pub fn is_windows_7_sp1_or_greater() -> bool {
    is_windows_version_or_greater(
        (_WIN32_WINNT_WIN7 >> 8) & 0xFF,
        _WIN32_WINNT_WIN7 & 0xFF,
        1,
        0,
    )
}

pub fn is_windows_8_or_greater() -> bool {
    is_windows_version_or_greater(
        (_WIN32_WINNT_WIN8 >> 8) & 0xFF,
        _WIN32_WINNT_WIN8 & 0xFF,
        0,
        0,
    )
}

pub fn is_windows_8_point_1_or_greater() -> bool {
    is_windows_version_or_greater(
        (_WIN32_WINNT_WINBLUE >> 8) & 0xFF,
        _WIN32_WINNT_WINBLUE & 0xFF,
        0,
        0,
    )
}

pub fn is_windows_threshold_or_greater() -> bool {
    is_windows_version_or_greater(
        (_WIN32_WINNT_WINTHRESHOLD >> 8) & 0xFF,
        _WIN32_WINNT_WINTHRESHOLD & 0xFF,
        0,
        0,
    )
}

pub fn is_windows_10_or_greater() -> bool {
    is_windows_version_or_greater(
        (_WIN32_WINNT_WIN10 >> 8) & 0xFF,
        _WIN32_WINNT_WIN10 & 0xFF,
        0,
        0,
    )
}

pub fn is_windows_11_or_greater() -> bool {
    is_windows_version_or_greater(
        (_WIN32_WINNT_WIN10 >> 8) & 0xFF,
        _WIN32_WINNT_WIN10 & 0xFF,
        0,
        22000,
    )
}

pub fn is_windows_server() -> bool {
    let mut vi: OSVERSIONINFOEXW = Default::default();
    vi.dwOSVersionInfoSize = size_of::<OSVERSIONINFOEXW>() as u32;
    vi.wProductType = VER_NT_WORKSTATION as u8;
    unsafe {
        !VerifyVersionInfoW(
            &mut vi,
            VER_PRODUCT_TYPE,
            VerSetConditionMask(0, VER_PRODUCT_TYPE, VER_EQUAL as u8),
        )
    }
    .into()
}
