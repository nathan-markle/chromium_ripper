use std::fs::File;
use std::io::Read;
use std::vec::Vec;
use rusqlite::{Connection, Result};

use crate::obj::download::Download;
use crate::obj::url::URL;

pub struct DBParser {
    db_path: String,
    download_entries: Vec<Download>,
    url_entries: Vec<URL>,
}

#[allow(unused)]
impl DBParser {
    pub fn new(db_path: String) -> Self {
        Self { 
            db_path, 
            download_entries: Vec::new(), 
            url_entries: Vec::new(), 
        }
    }

    pub fn get_download_length(&self) -> usize {
        self.download_entries.len()
    }

    pub fn get_url_entries_length(&self) -> usize {
        self.url_entries.len()
    }

    pub fn is_sqlite(&self) -> std::io::Result<bool> {
        let mut file = File::open(&self.db_path)?;
        let mut header = [0u8; 16];
        match file.read_exact(&mut header) {
            Ok(_) => Ok(&header == b"SQLite format 3\x00"),
            Err(_) => Ok(false),
        }
    }

    pub fn parse_database(&mut self) -> Result<()> {
        if self.is_sqlite().unwrap_or_default() {
            self.parse_url_history();
            self.parse_download_history();
        }
        else {
            println!("Failed to determine database correctly");
        }
        Ok(())
    }

    fn parse_url_history(&mut self) -> Result<(), rusqlite::Error> {
        let conn = Connection::open(&self.db_path)?;
        let mut stmt = conn.prepare("SELECT id, url, title,\n
            datetime((urls.last_visit_time/1000000) - 11644473600, \"unixepoch\")\n
            AS \"last_visit_time\", visit_count FROM urls ORDER BY\n
            last_visit_time DESC;")?;
        let url_iter = stmt.query_map([], |row| {
            Ok(URL::new(
                row.get(0)?, 
                row.get(1)?, 
                row.get(2)?, 
                row.get(3)?, 
                row.get(4)?,
            ))
        })?;
        for url_record in url_iter {
            match url_record {
                Ok(record) => {
                    &self.add_url_entry(record);
                }
                Err(e) => {
                    eprintln!("Error processing URL record row: {}", e);
                }
            }
        }
        Ok(())
    }

    fn parse_download_history(&mut self) -> Result<(), rusqlite::Error> {
        let conn = Connection::open(&self.db_path)?;
        let mut stmt = conn.prepare("SELECT datetime((downloads.start_time/1000000) - 11644473600, \"unixepoch\") AS \"start_time\",\n
            target_path, tab_url, tab_referrer_url, received_bytes, mime_type FROM\n
            downloads ORDER BY downloads.start_time DESC;")?;
        let download_iter = stmt.query_map([], |row| {
            Ok(Download::new(
                row.get(0)?,
                row.get(1)?,
                row.get(3)?,
                row.get(2)?,
                row.get(4)?,
                row.get(5)?,
            ))
        })?;
        for download_record in download_iter {
            match download_record {
                Ok(record) => {
                    &self.add_download(record);
                }
                Err(e) => {
                    eprintln!("Error processing Download record row: {}", e);
                }
            }
        }
        Ok(())
    }

    fn add_download(&mut self, new_entry: Download) {
        self.download_entries.push(new_entry);
    }

    fn add_url_entry(&mut self, new_entry: URL) {
        self.url_entries.push(new_entry);
    }

    pub fn display_urls(&mut self) {
        for entry in &self.url_entries {
            println!("{}", entry);
        }
    }

    pub fn display_downloads(&mut self) {
        for entry in &self.download_entries {
            println!("{}", entry);
        }
    }
}