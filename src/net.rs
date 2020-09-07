/* Anything network related goes here */
use std::io::Read;
use select::predicate::Class;
use reqwest::blocking::Client;
use select::document::Document;

pub fn fetch_timetable(username: String, password: String) -> Document {
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
    return Document::from(body.as_str());
}