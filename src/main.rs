use net::Period;
use std::vec::Vec;
use chrono::Local;
use notify_rust::Notification;
use std::{env, process, time, thread};
mod net;

/// Remind the user about a class
fn remind_class(period: &Period) {
    let now = Local::now().time();
    Notification::new()
        .summary(&format!("{} - {} ({}) T-{} minutes", period.block, period.name, period.loc, (period.start - now).num_minutes()).to_owned()[..])
        .show()
        .unwrap_or_default();
}

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
    process::exit(0);
}

fn main() -> ! {
    let username: String = env::args().nth(1).expect("first argument must be a username");
    let password: String = env::args().nth(2).expect("second argument must be a password");
    let shortcut: String = env::args().nth(3).expect("third argument must be a shortcut");
    let timeout: i64 = env::args().nth(4)
        .unwrap_or("5".to_string())
        .parse::<i64>()
        .expect("fourth argument must be a number");

    // Load the timetable
    // We create two copies, the first of which is moved into the scope of the second thread
    println!("Fetching timetable...");
    let timetable_a = net::fetch_timetable(&username, &password);
    let timetable_b = net::fetch_timetable(&username, &password);
    println!("Retrieved timetable!");

    // Spawn a new thread to handle the keyboard events for the shortcut
    thread::spawn(move || {
        // Calculate the modifiers for this shortcut
        let mut modifiers = 0;
        if shortcut.contains("control") { modifiers |= hotkey::modifiers::CONTROL };
        if shortcut.contains("shift") { modifiers |= hotkey::modifiers::SHIFT };
        if shortcut.contains("alt") { modifiers |= hotkey::modifiers::ALT };
        if shortcut.contains("super") { modifiers |= hotkey::modifiers::SUPER };

        let mut hk = hotkey::Listener::new();
        hk.register_hotkey(
            modifiers,
            // Get the last character of the shortcut for the trigger char
            shortcut.chars().last().unwrap().to_uppercase().next().unwrap() as u32,
            move || {
                let next = next_class(&timetable_a);
                remind_class(next);
            }
        ).unwrap();
        hk.listen();
    });



    // Main reminder loop, this runs every minute and checks the next class
    loop {
        // Get the next class, we do not have to worry about the event where we do not have any more classes
        // As this function will exit the program in the case
        let next = next_class(&timetable_b);

        // Get the current time
        let now = Local::now().time();
    
        // Calculate how many minutes from now until the class, 
        // then subtract the number of minutes before it we want to be reminded
        let expected = next.start - now - chrono::Duration::minutes(timeout);

        // If the time until we should remind is less than one minute then send a reminder
        if expected < chrono::Duration::minutes(1) {
            remind_class(next);
        }

        // Wait one minute before checking again
        thread::sleep(time::Duration::from_millis(1000));
    }
}
