mod obj;

use obj::parser;
use std::env;

fn main() {
    let mut db = parser::DBParser::new(
        env::var("TARGET_FILE").unwrap_or_default().to_string());
    let _ = db.parse_database();
    db.display_urls();
    db.display_downloads();
    println!("End");
}