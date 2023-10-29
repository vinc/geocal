use colored::*;
use geodate::geodate::*;
use geodate::reverse::*;
use geodate::ephemeris::*;
use std::env;
use std::time::SystemTime;

// A lunisolar month can be 29 or 30 days long
fn last_day_of_lunisolar_month(timestamp: i64, longitude: f64) -> usize {
    // HACK: This rely on an undefined behavior when getting a timestamp for
    // day following the last day of the month.
    let format = String::from("%h:%y:%m:%d:%c:%b");
    let a = get_formatted_date("%h:%y:%m:29:50:00", timestamp, longitude);
    let t = get_timestamp(format.clone(), a.clone(), longitude);
    let b = get_formatted_date(&format, t, longitude);
    if a == b {
        29
    } else {
        28
    }
}

// A solar month can be 88 to 94 days long
fn last_day_of_solar_month(timestamp: i64, longitude: f64) -> usize {
    // HACK: This rely on an undefined behavior when getting a timestamp for
    // day following the last day of the month.
    let format = String::from("%h:%y:%s:%d:%c:%b");
    for i in 88..100 {
        let a = get_formatted_date(&format!("%h:%y:%s:{:02}:50:00", i), timestamp, longitude);
        let t = get_timestamp(format.clone(), a.clone(), longitude);
        let b = get_formatted_date(&format, t, longitude);
        if a != b {
            return i - 1;
        }
    }
    unreachable!();
}

fn main() {
    let mut show_ephemeris = false;
    let mut solar_calendar = false;
    let args: Vec<String> = env::args().filter(|arg| {
        if arg == "--ephem" {
            show_ephemeris = true
        } else if arg == "--solar" {
            solar_calendar = true
        }
        !arg.starts_with("--")
    }).collect();

    let latitude = args[1].parse::<f64>().unwrap();
    let longitude = args[2].parse::<f64>().unwrap();
    let timestamp = if args.len() == 4 {
        args[3].parse::<i64>().unwrap()
    } else {
        match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(time) => time.as_secs() as i64,
            Err(_) => 0
        }
    };

    let week;
    let format;
    if solar_calendar {
        week = 10;
        format = String::from("%h:%y:%s:%d:%c:%b");
    } else {
        week = 8;
        format = String::from("%h:%y:%m:%d:%c:%b");
    };
    let formatted_date = get_formatted_date(&format, timestamp, longitude);
    let date: Vec<_> = formatted_date.split(":").collect();

    println!("");
    let line = format!("  +-{}+", "-".repeat(3 * week));
    let sep = "|";
    println!("{}", line);

    // Date
    let is_negative = date[0].starts_with('-');
    let colored_title = "Date:".bold();
    let colored_date = format!("{}{}-{}-{}", date[0], date[1], date[2], date[3]).bold().red();
    let mut spacing = (3 * week) - 12;
    if is_negative {
        spacing -= 1;
    }
    println!("  {} {:spacing$} {} {}", sep, colored_title, colored_date, sep);
    println!("{}", line);

    // Calendar
    let last_day;
    if solar_calendar {
        print!("  {} {} ", sep, "So Me Ve Te Ma Ju Sa Ur Ne Lu".bold());
        last_day = last_day_of_solar_month(timestamp, longitude);
    } else {
        print!("  {} {} ", sep, "So Me Ve Te Ma Ju Sa Lu".bold());
        last_day = last_day_of_lunisolar_month(timestamp, longitude);
    }
    let n = last_day + 1;
    for i in 0..n {
        // Weekend
        if solar_calendar {
            if i % week == 0 {
                print!("|\n  {} ", sep);
            }
        } else if i == 0 || i == 7 || i == 15 || i == 22 {
            // The lunisolar calendar has a leap day at the end of the
            // second week and another at the end of the last week if
            // the month is long (30 days).
            if i == 7 || i == 22 {
                print!("   ");
            }
            print!("|\n  {} ", sep);
        }

        let mut day = format!("{:02}", i);
        if day == date[3] {
            day = day.bold().red().to_string();
        }
        print!("{} ", day);
    }
    if solar_calendar {
        if last_day > 89 {
            print!("{}", "   ".repeat(99 - last_day));
        } else {
            print!("{}", "   ".repeat(89 - last_day));
        }
    } else if last_day == 28 {
        print!("   ");
    }
    println!("|");
    println!("{}", line);

    // Time
    let colored_title = "Time:".bold();
    let colored_time = format!("{}:{}", date[4], date[5]).bold().red();
    println!("  {} {:spacing$} {} {}", sep, colored_title, colored_time, sep, spacing = (3 * week) - 7);
    println!("{}", line);

    // Ephemeris
    if show_ephemeris {
        let events = get_ephemeris(timestamp, longitude, latitude);
        for (&t, e) in &events {
            let name = match e.as_str() {
                "Current" => continue,
                "First Quarter Moon" => "First Quarter",
                "Last Quarter Moon" => "Last Quarter",
                _ => e
            };
            let time = get_formatted_date("%c:%b", t, longitude);
            let spacing = (3 * week) - 7;
            println!("  {} {:spacing$} {} {}", sep, format!("{}:", name), time, sep);
        }
        println!("{}", line);
    }
}
