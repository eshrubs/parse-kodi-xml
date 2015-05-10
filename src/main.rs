extern crate xml;

use std::io::BufReader;
use std::fs::File;

use xml::reader::{EventReader, Events};
use xml::reader::events::*;

struct Movie {
    title: String,
    year: i32,
    rating: f32
}

fn get_chars(events: &mut Iterator<Item=XmlEvent>) -> String {
    match events.next() {
        Some(e) => match e {
            XmlEvent::Characters(ref data) => {
                return data.to_string();
            },
            _ => {
                println!("Error");
            }
        },
        None => println!("Error2")
    }
    return "#fail".to_string();
}

fn get_movie(events: &mut Iterator<Item=XmlEvent>) -> Movie {
    let mut title = String::new();
    let mut year = 0;
    let mut rating = 0.0;
    loop {
        match events.next() {
            Some(e) => match e {
                XmlEvent::StartElement { name, attributes, namespace: _ } => {
                    match name.local_name.as_ref() {
                        "title" => {
                            title = get_chars(events);
                        },
                        "year" => {
                            let year_str = get_chars(events);
                            year = year_str.parse::<i32>().unwrap();
                        },
                        "rating" => {
                            let rating_str = get_chars(events);
                            rating = rating_str.parse::<f32>().unwrap();
                        }
                        _ => {}
                    }
                },
                XmlEvent::EndElement { name } => {
                    if name.local_name.eq("movie") {
                        break;
                    }
                }
                _ => {
                }
            },
            None => break,
        }
    }
    return Movie { title: title, year: year, rating: rating };
}

fn main() {
    let file = File::open("videodb.xml").unwrap();
    let reader = BufReader::new(file);

    let mut parser = EventReader::new(reader);

    let mut events = parser.events();
    loop {
        match events.next() {
            Some(x) => match x {
                XmlEvent::StartElement { name, attributes: _, namespace: _ } => {
                    if name.local_name.eq("movie") {
                        let m = get_movie(&mut events);
                        println!("{},{},{}", m.title, m.year, m.rating);
                    }
                },
                _ => {}
            },
            None => {
                break;
            }
        }
    }
}
