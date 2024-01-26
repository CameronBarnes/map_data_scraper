use once_cell::sync::Lazy;
use regex::Regex;
use reqwest::blocking::Client;

use crate::types::{Category, Document, DownloadType, LibraryItem};

pub fn parse_open_street_map() -> LibraryItem {}

fn parse_geofabrik_size(size: &str) -> u64 {
    let (size, unit) = size.split_once(' ').unwrap();
    let mut size: f64 = size.parse().unwrap();
    if unit.eq_ignore_ascii_case("kb") {
        size *= 1024.0;
    } else if unit.eq_ignore_ascii_case("mb") {
        size *= 1_048_576.0;
    } else if unit.eq_ignore_ascii_case("gb") {
        size *= 1_073_741_824.0;
    }
    size as u64
}

fn parse_geofabrik() -> LibraryItem {
    let mut category = Category::new(String::from("Map Data"), vec![], false);

    static MAIN_PATH: &str = "https://download.geofabrik.de";
    // Parse main download page for regions
    let regions = parse_geofabrik_page(&get_page_from_path(MAIN_PATH));
    for (path, name, file, size) in regions {
        let size = parse_geofabrik_size(size);
        // Get the sub regions for each region, in case the user wants a certain sub region instead
        // of the whole region.
        let sub_items = parse_geofabrik_page(&get_page_from_path(&format!("{MAIN_PATH}/{path}/")))
            .into_iter()
            .map(|(_, name, file, size)| {
                let size = parse_geofabrik_size(size);
                let doc = Document::new(
                    name.to_string(),
                    format!("{MAIN_PATH}/{file}"),
                    size,
                    DownloadType::Http,
                );
                LibraryItem::Document(doc)
            })
            .collect();

        let sub_region_cat =
            LibraryItem::Category(Category::new(String::from("Sub Regions"), sub_items, false));
        let region_doc = LibraryItem::Document(Document::new(
            String::from("Single File"),
            format!("{MAIN_PATH}/{file}"),
            size,
            DownloadType::Http,
        ));
        let region_cat = Category::new(name.to_string(), vec![region_doc, sub_region_cat], true);
        category.add(LibraryItem::Category(region_cat));
    }

    LibraryItem::Category(category)
}

fn parse_geofabrik_page(page: &str) -> Vec<(&str, &str, &str, &str)> {
    static GEOFABRIK: Lazy<Regex> = Lazy::new(|| {
        Regex::new("<td class=\"subregion\"><a href=\"(.+?)\">(.+?)</a></td>\n<td style=.+?><a href=\"(.+?)\">\\[\\.osm\\.pbf\\]</a></td><td style=.+?>\\((.+?&nbsp;.+?)\\)/gm").unwrap()
    });
    GEOFABRIK
        .captures_iter(&page)
        .map(|e| e.extract())
        .map(|(_, arr)| arr.into())
        .collect()
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
