mod geofabrik;

use once_cell::sync::Lazy;
use reqwest::blocking::Client;

use crate::types::{LibraryItem, Category};

pub fn parse_open_street_map() -> LibraryItem {
    LibraryItem::Category(Category::new(
        String::from("Open Street Map"),
        vec![geofabrik::parse()],
        false,
    ))
}

pub fn get_page_from_path(path: &str) -> String {
    static CLIENT: Lazy<Client> = Lazy::new(|| {
        reqwest::blocking::ClientBuilder::new()
            .user_agent("Mozilla/5.0 (X11; Linux x86_64; rv:109.0) Gecko/20100101 Firefox/117.0")
            .build()
            .unwrap()
    });
    CLIENT.get(path).send().unwrap().text().unwrap()
}
