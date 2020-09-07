use std::env;
mod net;

fn main() {
    let username: String = env::args().nth(1).expect("first argument must be a username");
    let password: String = env::args().nth(2).expect("second argument must be a password");

    let timetable = net::fetch_timetable(username, password);
    println!("{:#?}", timetable);
}
