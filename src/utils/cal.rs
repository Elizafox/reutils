/* utils/cal.rs - implementation of cal
 * Copyright (C) 2023 Elizabeth Myers. All rights reserved.
 * SPDX-License-Identifier: GPL-2.0-only
 */

/* TODO: a lot of this code is messy and could use cleaning up
 * The formatting code is especially hairy and not ideal.
 * It does match BSD's behaviour, though.
 * Also, clippy has been told to shut up a lot here, probably unwise, but there's a reason.
 */

use chrono::{Datelike, Local};

use crate::err::{Error, Result};

const fn is_leap_year(year: u64) -> bool {
    if year > 1752 {
        // Gregorian
        !(((year % 4) > 0 && (year % 100) > 0) || ((year % 400) == 0))
    } else {
        // Julian
        (year % 4) == 0
    }
}

// NB: this does not work for September 1752, and that must be special cased
// (3-13 Sep "didn't happen")
fn days_in_month(month: u8, year: u64) -> u8 {
    const DAYS_IN_MONTH: [[u8; 12]; 2] = [
        [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31],
        [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31],
    ];

    assert!(month > 0 && month <= 12);

    let month = month - 1;
    if is_leap_year(year) {
        #[allow(clippy::cast_possible_truncation)]
        DAYS_IN_MONTH[1][month as usize]
    } else {
        #[allow(clippy::cast_possible_truncation)]
        DAYS_IN_MONTH[0][month as usize]
    }
}

fn get_first_day_of_week(month: u8, year: u64) -> u8 {
    let mut month = u64::from(month);
    let mut year = year;
    let day_of_week: u8 = if year > 1752 {
        // Gregorian
        const T: [u64; 12] = [0, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];
        if month < 3 {
            year -= 1;
        }
        #[allow(clippy::cast_possible_truncation)]
        ((year + year / 4 - year / 100 + year / 400 + T[(month - 1) as usize] + 1) % 7)
            .try_into()
            .unwrap()
    } else {
        // Julian
        if month < 3 {
            month += 12;
            year -= 1;
        }
        ((1 + 2 * month + (3 * month + 3) / 5 + year + year / 4 + 6) % 7)
            .try_into()
            .unwrap()
    };
    day_of_week
}

fn pad_line(line: &mut str, do_extra_pad: bool) {
    // XXX we can probably eliminate this{
    let mut line = line.trim_end().to_string();
    if do_extra_pad {
        line.push_str(&" ".repeat(20 - (line.len() - 7)));
    } else {
        line.push_str(&" ".repeat(20 - line.len()));
    }
}

fn vec_month_calendar(month: u8, year: u64, print_year: bool) -> Vec<String> {
    const MONTHS: [&str; 12] = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    let now = Local::now();
    #[allow(clippy::cast_sign_loss)]
    let local_year = now.year() as u64;
    #[allow(clippy::cast_possible_truncation)]
    let local_month = now.month() as u8;
    #[allow(clippy::cast_possible_truncation)]
    let local_day = now.day() as u8;

    let mut ret: Vec<String> = Vec::new();
    let days_in_month = days_in_month(month, year);
    let mut current_day = 1u8;
    let mut day_of_week = get_first_day_of_week(month, year);
    let month_name = MONTHS[(month - 1) as usize];

    if print_year {
        ret.push(format!("{: ^20}", format!("{month_name} {year}")));
    } else {
        ret.push(format!("{month_name: ^20}"));
    }

    ret.push("Su Mo Tu We Th Fr Sa".to_string());
    let mut line: String = "   ".repeat(day_of_week.into());
    let mut do_extra_pad = false;

    loop {
        if year == local_year && month == local_month && current_day == local_day {
            do_extra_pad = true;
            line.push_str(&format!("\x1b[7m{current_day:>2}\x1b[m "));
        } else {
            line.push_str(&format!("{current_day:>2} "));
        }

        current_day += 1;
        if current_day > days_in_month {
            pad_line(&mut line, do_extra_pad);
            ret.push(line.clone());
            break;
        } else if year == 1752 && month == 9 && current_day == 3 {
            /* You may be wondering why this is here.
             * Per POSIX, the transition to the Gregorian calendar should be treated as if it
             * happened on 14 September, 1752. That's when the British Empire adopted the Gregorian
             * calendar empire-wide. The actual implementation was to drop all days 3 thru 13
             * inclusive.
             *
             * If you want to know more:
             *   https://en.wikipedia.org/wiki/Calendar_(New_Style)_Act_1750
             */
            current_day += 13;
        }

        day_of_week += 1;
        if day_of_week > 6 {
            day_of_week = 0;
            pad_line(&mut line, do_extra_pad);
            ret.push(line.clone());
            line.clear();
        }
    }

    // Add blank lines so it formats correctly later
    if ret.len() < 8 {
        for _ in 0..(8 - ret.len()) {
            ret.push(" ".repeat(20).to_string());
        }
    }

    ret
}

fn print_month_calendar(month: u8, year: u64, print_year: bool) {
    let v = vec_month_calendar(month, year, print_year);
    for line in v {
        println!("{line}");
    }

    println!();
}

fn print_year_calendar(year: u64) {
    println!("{year: ^60}");

    // Do this in batches of 3, just like BSD
    for i in (1..12).step_by(3) {
        let v1 = vec_month_calendar(i, year, false);
        let v2 = vec_month_calendar(i + 1, year, false);
        let v3 = vec_month_calendar(i + 2, year, false);

        let iter = v1
            .iter()
            .zip(v2.iter())
            .zip(v3.iter())
            .map(|((a, b), c)| (a, b, c));

        for (l1, l2, l3) in iter {
            println!("{l1}  {l2}  {l3}");
        }

        println!();
    }
}

fn usage(arg0: &str) -> Error {
    eprintln!("Usage: {arg0} [year] [month]");
    Error::new_nomsg(1)
}

pub fn util(args: &[String]) -> Result {
    match args.len() {
        0 => {
            panic!("Got no argv[0]!");
        }
        1 => {
            let now = Local::now();
            #[allow(clippy::cast_sign_loss)]
            let local_year = now.year() as u64;
            #[allow(clippy::cast_possible_truncation)]
            let local_month = now.month() as u8;

            print_month_calendar(local_month, local_year, true);
        }
        2 => {
            let year = args
                .get(1)
                .unwrap()
                .parse::<u64>()
                .map_err(|_| usage(&args[0]))?;

            if year == 0 {
                eprintln!("Only years 1 through 18446744073709551615 accepted");
                return Err(usage(&args[0]));
            }

            print_year_calendar(year);
        }
        _ => {
            let year = args
                .get(1)
                .unwrap()
                .parse::<u64>()
                .map_err(|_| usage(&args[0]))?;

            if year == 0 {
                eprintln!("Only years 1 through 18446744073709551615 accepted");
                return Err(usage(&args[0]));
            }

            let month = args
                .get(2)
                .unwrap()
                .parse::<u8>()
                .map_err(|_| usage(&args[0]))?;

            if month == 0 || month > 12 {
                eprintln!("Only months 1 through 12 accepted");
                return Err(usage(&args[0]));
            }

            print_month_calendar(month, year, true);
        }
    }

    Ok(())
}
