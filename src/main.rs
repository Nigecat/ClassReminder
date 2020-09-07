use std::env;
use net::Period;
use std::vec::Vec;
use chrono::Local;
use std::process::exit;
mod net;

/// Given a timetable find the next class after the current point in time
fn next_class<'a>(timetable: &'a Vec<Period>) -> &'a Period {
    // We know our timetable should be ordered in chronological order
    // So we can iterate through it and check if each point is behind the current point in it
    // The first one that is after now should be the next class
    for class in timetable.iter() {
        let now = Local::now().time();
        // If this class is after now
        if class.start >= now {
            return class;
        }
    }

    // If we get this far then we don't have any classes after now
    // So we can just exit
    exit(0);
}

fn main() {
    let username: String = env::args().nth(1).expect("first argument must be a username");
    let password: String = env::args().nth(2).expect("second argument must be a password");
    let timeout: u64 = env::args().nth(3)
        .unwrap_or("5".to_string())
        .parse::<u64>()
        .expect("third argument must be a number");

    // Load the timetable
    let timetable = net::fetch_timetable(username, password);

    println!("{:#?}", next_class(&timetable));
}
