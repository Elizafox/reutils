/* utils/df.rs - implementation of df
 * Copyright (C) 2023 Elizabeth Myers. All rights reserved.
 * SPDX-License-Identifier: GPL-2.0-only
 */

use getargs::{Opt, Options};

use crate::err::{Error, Result};
use crate::platform::fsent::{get_filesystem_stats, get_mounted_filesystems, get_path_mountpoint};

fn display_table(table: Vec<[String; 6]>) {
    let mut col_lengths: [usize; 5] = [0; 5];

    // Find the maximum length to pad
    for row in &table {
        for i in 0..5 {
            if row[i].len() > col_lengths[i] {
                col_lengths[i] = row[i].len();
            }
        }
    }

    // XXX - HACK ALERT!!!
    for row in table {
        println!(
            "{}{}{}{} {}{} {}{} {}{} {}",
            row[0],
            " ".repeat(col_lengths[0] - row[0].len()),
            " ".repeat(col_lengths[1] - row[1].len()),
            row[1],
            " ".repeat(col_lengths[2] - row[2].len()),
            row[2],
            " ".repeat(col_lengths[3] - row[3].len()),
            row[3],
            " ".repeat(col_lengths[4] - row[4].len()),
            row[4],
            row[5]
        );
    }
}

pub fn util(args: &[String]) -> Result {
    let mut use_1024_kb_units = false;
    let mut use_portable_format = false;

    let mut opts = Options::new(args.iter().skip(1).map(String::as_str));
    while let Some(opt) = opts.next_opt().expect("argument parsing error") {
        match opt {
            Opt::Short('k') => use_1024_kb_units = true,
            Opt::Short('P') => use_portable_format = true,
            Opt::Short('h') | Opt::Long("help") => {
                eprintln!("Usage: {} [-k] [-p] [-t]", args[0]);
                return Ok(());
            }
            _ => {}
        }
    }

    let block_size = if use_1024_kb_units {
        1024u64 // POSIX sez so
    } else if use_portable_format {
        512u64 // Also
    } else {
        4096u64 // Modern default
    };

    let mut args = Vec::<String>::new();
    for arg in opts.positionals() {
        let arg = get_path_mountpoint(arg)
            .map_err(|e| Error::new(1, format!("Could not get path: {e}")))?;
        args.push(arg);
    }
    let filesystems = if args.is_empty() {
        get_mounted_filesystems()
            .map_err(|e| Error::new(1, format!("Could not get mounted filesystems: {e}")))?
            .into_iter()
            .map(|fs| (fs.mount_point, fs.mount_from))
            .collect::<Vec<(String, String)>>()
    } else {
        get_mounted_filesystems()
            .map_err(|e| Error::new(1, format!("Could not get mounted filesystems: {e}")))?
            .into_iter()
            .filter(|fs| args.contains(&fs.mount_point))
            .map(|fs| (fs.mount_point, fs.mount_from))
            .collect::<Vec<(String, String)>>()
    };

    let mut table: Vec<[String; 6]> = vec![[
        "Filesystem".to_string(),
        format!("{block_size}-blocks"),
        "Used".to_string(),
        "Available".to_string(),
        "Capacity".to_string(),
        "Mounted on".to_string(),
    ]];
    for (mount_point, mount_from) in filesystems {
        let stats;
        match get_filesystem_stats(&mount_point) {
            Ok(s) => stats = s,
            Err(e) => {
                eprintln!("Could not get filesystem info for {mount_point}: {e}");
                continue;
            }
        }

        let (blocks_total, blocks_free) = if block_size == stats.block_size {
            (stats.blocks_total, stats.blocks_free)
        } else {
            (
                (stats.blocks_total * stats.block_size) / block_size,
                (stats.blocks_free * stats.block_size) / block_size,
            )
        };

        // Use the original values to avoid rounding error
        #[allow(clippy::cast_possible_truncation)]
        let capacity = if stats.blocks_total > 0 && stats.blocks_free > 0 {
            // Use i128 to avoid overflow, an unlikely scenario but better safe than sorry.
            let calc_blocks_used = i128::from(stats.blocks_total) - i128::from(stats.blocks_free);
            let calc_blocks_total = i128::from(stats.blocks_total);
            ((100i128 * calc_blocks_used) / calc_blocks_total) as i64
                + i64::from(calc_blocks_used % calc_blocks_total != 0)
        } else {
            100i64
        };

        let blocks_used = blocks_total - blocks_free;

        table.push([
            mount_from,
            blocks_total.to_string(),
            blocks_used.to_string(),
            blocks_free.to_string(),
            format!("{capacity}%"),
            mount_point,
        ]);
    }

    display_table(table);

    Ok(())
}
