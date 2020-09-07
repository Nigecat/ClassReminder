/* Anything network related goes here */
use std::io::Read;
use std::vec::Vec;
use reqwest::blocking::Client;
use select::document::Document;
use select::predicate::{Class, Name};

/// Fetch a user's timetable
pub fn fetch_timetable(username: String, password: String) -> Vec<Period> {
    let client = Client::new();
    // The url to the full timetable differs from user to user,
    // so we first fetch the homepage so we can follow a link to it.
    let mut resp = client.get("https://www.careylink.com.au/Pages/home.aspx")
        .basic_auth(&username, Some(&password)) 
        .send()
        .expect("unable to fetch careylink, maybe check your authentication details");

    // Extract the html content of the homepage
    let mut body = String::new();
    resp.read_to_string(&mut body).unwrap();

    // Find the link to the full timetable
    let document = Document::from(body.as_str());
    let mut timetable_link = document.find(Class("ip_timetable_link")).collect::<Vec<_>>()[0].attr("href")
        .expect("unable to find link to full timetable, maybe you don't have classes today")
        .to_string();
    timetable_link.insert_str(0, "https://www.careylink.com.au");

    // Fetch the actual timetable
    resp = client.get(&timetable_link)
        .basic_auth(username, Some(password)) 
        .send()
        .expect("unable to fetch timetable, maybe check your authentication details");

    body = String::new();
    resp.read_to_string(&mut body).unwrap();
    let timetable = format_timetable(Document::from(body.as_str()));
    return timetable;
}

#[derive(Debug)]
pub struct Period {
    /// The time this period starts at, should be a string like '10:35 AM'
    start_time: String,
    /// The time this period ends at, should be a string like '10:35 AM'
    end_time: String,
    /// The teacher code of this class, e.g 'THTR'
    teacher: String,
    /// The location of this class, for online learning this is either 'STUDY' or 'CONF'
    loc: String,
    /// The block this class is in, this is usually in the format 'PERIOD 1' but sometimes may be others such as 'ROLL CALL'
    block: String,
    /// The name of this class, e.g 'GEOGRAPHY'
    name: String,
}

/// Format the raw html of a timetable into something we can process
fn format_timetable(raw: Document) -> Vec<Period> {
    let timetable: Vec<Period> = vec![];

    println!("{:#?}", raw.find(Name("table")).last().unwrap());

    return timetable;
}
