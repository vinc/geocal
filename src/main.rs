use colored::*;
use geodate::geodate::*;
use geodate::reverse::*;
use geodate::ephemeris::*;
use std::env;

fn main() {
    let mut show_ephemeris = false;
    let args: Vec<String> = env::args().filter(|arg| {
        if arg == "--ephem" {
            show_ephemeris = true
        }
        !arg.starts_with("--")
    }).collect();

    let latitude = args[1].parse::<f64>().unwrap();
    let longitude = args[2].parse::<f64>().unwrap();
    let timestamp = if args.len() == 4 {
        args[3].parse::<i64>().unwrap()
    } else {
        time::get_time().sec
    };

    let format = String::from("%h:%y:%m:%d:%c:%b");
    let formatted_date = get_formatted_date(&format, timestamp, longitude);
    let date: Vec<_> = formatted_date.split(":").collect();

    // Check if the month is short (29 days) or long (30 days)
    // HACK: This rely on an undefined behavior when getting a timestamp for
    // the day #30 of a 29 days month (short month).
    let a = get_formatted_date("%h:%y:%m:29:50:00", timestamp, longitude);
    let t = get_timestamp(format.clone(), a.clone(), longitude);
    let b = get_formatted_date(&format, t, longitude);
    let is_short_month = a != b;

    let line = "+-------------------------+";
    println!("{}", line);

    // Date
    println!("| {:12} {} |", "Date:".bold(), format!("{}{}-{}-{}", date[0], date[1], date[2], date[3]).bold().red());
    println!("{}", line);

    // Calendar
    println!("| {} |", "So Me Ve Te Ma Ju Sa Lu".bold());
    for i in 0..30 {
        if i == 0 || i == 7 || i == 15 || i == 22 {
            print!("| ");
        }
        let mut day = format!("{:02}", i);
        if day == date[3] {
            day = day.bold().red().to_string();
        } else if i == 29 && is_short_month {
            day = format!("  ");
        }
        print!("{} ", day);
        if i == 14 || i == 29 {
            println!("|");
        } else if i == 6 || i == 21 {
            println!("   |");
        }
    }
    println!("{}", line);

    // Time
    println!("| {:17} {} |", "Time:".bold(), format!("{}:{}", date[4], date[5]).bold().red());
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
            println!("| {:17} {} |", format!("{}:", name), time);
        }
        println!("{}", line);
    }
}
