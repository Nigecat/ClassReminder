/* Anything network related goes here */
use std::io::Read;
use std::vec::Vec;
use select::node::Node;
use select::predicate::Class;
use reqwest::blocking::Client;
use select::document::Document;

/// Fetch a user's timetable
pub fn fetch_timetable(username: String, password: String) -> Vec<Period> {
    let client = Client::new();
    // Fetch the careylink homepage, this will have the today's timetable on it
    let mut resp = client.get("https://www.careylink.com.au/Pages/home.aspx")
        .basic_auth(&username, Some(&password)) 
        .send()
        .expect("unable to fetch careylink, maybe check your authentication details");

    // Extract the html content of the homepage
    let mut body = String::new();
    resp.read_to_string(&mut body).unwrap();
    let document = Document::from(body.as_str());
    
    // Extract the timetable    
    let timetable = document.find(Class("ip_timetable_table")).next().unwrap();
    return format_timetable(timetable);
}

#[derive(Debug)]
pub struct Period {
    /// The time this period starts at, should be a string like '10:35 AM'
    start: String,
    /// The location of this class, for online learning this is either 'STUDY' or 'CONF'
    loc: String,
    /// The block this class is in, this is usually in the format 'PERIOD 1' but sometimes may be others such as 'ROLL CALL'
    block: String,
    /// The name of this class, e.g 'GEOGRAPHY'
    name: String,
}

/// Format the raw html of a timetable into something we can process
fn format_timetable(raw: Node) -> Vec<Period> {
    let mut timetable: Vec<Period> = Vec::new();

    // Iterate over the rows in the timetable
    // (skip the first row since it is the headers)
    for row in raw.children().nth(1).unwrap().children().skip(1) {
        // Ignore any non row elements
        if row.name() != Some("tr") {
            continue;
        }
        let mut data: Vec<String> = Vec::new();
        // Extract the data we need from this table row
        for block in row.children() {
            let content = block.children().next();
            // Ignore any invalid elements
            if let Some(node) = content {
                // Add this data to our data for this period
                data.push(node.text());
            }
        }
        
        // Put the data into our struct
        //  0 - Period Number/Name
        //  1 - Start Time
        //  2 - Class Name
        //  3 - Location (CONF/STUDY)
        timetable.push(Period {
            block: data[0].clone(),
            start: data[1].clone(),
            name: data[2].clone(),
            loc: data[3].clone(),
        });
    }

    return timetable;
}
