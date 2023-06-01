/* utils/uname.rs - implementation of uname
 * Copyright (C) 2023 Elizabeth Myers. All rights reserved.
 * SPDX-License-Identifier: GPL-2.0-only
 */

use getargs::{Opt, Options};

use crate::err::Result;

mod args {
    #[derive(Debug, PartialEq, Eq)]
    pub enum SysName {
        ShowSysName,
        NoSysName,
    }

    #[derive(Debug, PartialEq, Eq)]
    pub enum NodeName {
        ShowNodeName,
        NoNodeName,
    }

    #[derive(Debug, PartialEq, Eq)]
    pub enum Release {
        ShowRelease,
        NoRelease,
    }

    #[derive(Debug, PartialEq, Eq)]
    pub enum Version {
        ShowVersion,
        NoVersion,
    }

    #[derive(Debug, PartialEq, Eq)]
    pub enum Machine {
        ShowMachine,
        NoMachine,
    }

    #[derive(Debug, PartialEq, Eq)]
    pub struct UnameFlags {
        pub sysname: SysName,
        pub nodename: NodeName,
        pub release: Release,
        pub version: Version,
        pub machine: Machine,
    }

    impl UnameFlags {
        pub const fn new() -> Self {
            Self {
                sysname: SysName::NoSysName,
                nodename: NodeName::NoNodeName,
                release: Release::NoRelease,
                version: Version::NoVersion,
                machine: Machine::NoMachine,
            }
        }
    }
}

#[cfg(unix)]
fn uname(uargs: &args::UnameFlags) -> String {
    use libc::{uname, utsname};
    use std::ffi::CStr;
    use std::mem::MaybeUninit;

    let mut uts = MaybeUninit::<utsname>::uninit();

    if unsafe { uname(uts.as_mut_ptr()) } != 0 {
        return "Unknown".to_string();
    }

    let uts = unsafe { uts.assume_init() };

    let mut ret = String::new();

    if uargs.sysname == args::SysName::ShowSysName {
        let sysname = unsafe { CStr::from_ptr(uts.sysname.as_ptr()) }
            .to_str()
            .expect("Could not get sysname");
        ret.push_str(format!("{sysname} ").as_str());
    }

    if uargs.nodename == args::NodeName::ShowNodeName {
        let nodename = unsafe { CStr::from_ptr(uts.nodename.as_ptr()) }
            .to_str()
            .expect("Could not get nodename");
        ret.push_str(format!("{nodename} ").as_str());
    }

    if uargs.release == args::Release::ShowRelease {
        let release = unsafe { CStr::from_ptr(uts.release.as_ptr()) }
            .to_str()
            .expect("Could not get release");
        ret.push_str(format!("{release} ").as_str());
    }

    if uargs.version == args::Version::ShowVersion {
        let version = unsafe { CStr::from_ptr(uts.version.as_ptr()) }
            .to_str()
            .expect("Could not get version");
        ret.push_str(format!("{version} ").as_str());
    }

    if uargs.machine == args::Machine::ShowMachine {
        let machine = unsafe { CStr::from_ptr(uts.machine.as_ptr()) }
            .to_str()
            .expect("Could not get machine");
        ret.push_str(format!("{machine} ").as_str());
    }

    ret.trim_end().to_string()
}

#[cfg(windows)]
fn uname(uargs: &args::UnameFlags) -> String {
    use crate::platform::windows::hostinfo::{architecture, hostname, release, version};

    let mut ret = String::new();

    if uargs.sysname == args::SysName::ShowSysName {
        ret.push_str("Windows ");
    }

    if uargs.nodename == args::NodeName::ShowNodeName {
        let nodename = hostname();
        ret.push_str(format!("{nodename} ").as_str());
    }

    if uargs.release == args::Release::ShowRelease {
        let release = release();
        ret.push_str(format!("{release} ").as_str());
    }

    if uargs.version == args::Version::ShowVersion {
        let version = version();
        ret.push_str(format!("{version} ").as_str());
    }

    if uargs.machine == args::Machine::ShowMachine {
        let machine = architecture();
        ret.push_str(format!("{machine} ").as_str());
    }

    ret.trim_end().to_string()
}

#[allow(clippy::unnecessary_wraps)]
pub fn util(args: &[String]) -> Result {
    let mut uargs = args::UnameFlags::new();

    let mut opts = Options::new(args.iter().skip(1).map(String::as_str));
    while let Some(opt) = opts.next_opt().expect("argument parsing error") {
        match opt {
            Opt::Short('h') | Opt::Long("help") => {
                eprintln!("Usage: {} [-amnrsv]", args[0]);
                return Ok(());
            }
            Opt::Short('a') => {
                uargs.machine = args::Machine::ShowMachine;
                uargs.nodename = args::NodeName::ShowNodeName;
                uargs.release = args::Release::ShowRelease;
                uargs.sysname = args::SysName::ShowSysName;
                uargs.version = args::Version::ShowVersion;
            }
            Opt::Short('m') => uargs.machine = args::Machine::ShowMachine,
            Opt::Short('n') => uargs.nodename = args::NodeName::ShowNodeName,
            Opt::Short('r') => uargs.release = args::Release::ShowRelease,
            Opt::Short('s') => uargs.sysname = args::SysName::ShowSysName,
            Opt::Short('v') => uargs.version = args::Version::ShowVersion,
            _ => {}
        }
    }

    if uargs.machine == args::Machine::NoMachine
        && uargs.nodename == args::NodeName::NoNodeName
        && uargs.release == args::Release::NoRelease
        && uargs.sysname == args::SysName::NoSysName
        && uargs.version == args::Version::NoVersion
    {
        uargs.sysname = args::SysName::ShowSysName;
    }

    println!("{}", uname(&uargs));

    Ok(())
}
