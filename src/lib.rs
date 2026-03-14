mod obj;

#[allow(unused)]
use obj::{parser, download, url};
#[allow(unused)]
use std::env;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_database() {
        let db = parser::DBParser::new(env::var("TARGET_FILE").unwrap_or_default().to_string());
        assert_eq!(true, db.is_sqlite().unwrap_or_default());
    }

    #[test]
    fn test_url() {
        let u = url::URL::new(
            1234 as i64,
            "evil.com".to_string(),
            "Evil Site".to_string(),
            "2026-02-21T17:24:23Z".to_string(),
            5 as i64);
        assert_eq!(1234 as i64, u.get_id());
        assert_eq!("evil.com".to_string(), u.get_url());
        assert_eq!("Evil Site".to_string(), u.get_title());
        assert_eq!("2026-02-21T17:24:23Z".to_string(), u.get_visit_time());
        assert_eq!(5 as i64, u.get_visits());
    }

    #[test]
    fn test_download() {
        let d= download::Download::new(
            "2026-02-21T17:25:10Z".to_string(), 
            "C:\\Users\\nathan\\Downloads\\evil.exe".to_string(),
            "https://evil[.]com/downloads".to_string(),
            "https://evil[.]com/downloads".to_string(),
            22304 as i64,
            "executable".to_string());
        assert_eq!("2026-02-21T17:25:10Z".to_string(), d.get_start_time());
        assert_eq!("C:\\Users\\nathan\\Downloads\\evil.exe".to_string(), d.get_download_path());
        assert_eq!("https://evil[.]com/downloads".to_string(), d.get_referrer_url());
        assert_eq!("https://evil[.]com/downloads".to_string(), d.get_tab_url());
        assert_eq!(22304 as i64, d.get_received_bytes());
        assert_eq!("executable".to_string(), d.get_mime_type());
    }

    #[test]
    fn compare_equal_downloads() {
        let d1= download::Download::new(
            "2026-02-21T17:25:10Z".to_string(), 
            "C:\\Users\\nathan\\Downloads\\evil.exe".to_string(),
            "https://evil[.]com/downloads".to_string(),
            "https://evil[.]com/downloads".to_string(),
            22304 as i64,
            "executable".to_string());
        let d2= download::Download::new(
            "2026-02-21T17:25:10Z".to_string(), 
            "C:\\Users\\nathan\\Downloads\\evil.exe".to_string(),
            "https://evil[.]com/downloads".to_string(),
            "https://evil[.]com/downloads".to_string(),
            22304 as i64,
            "executable".to_string());
            assert_eq!(d1, d2);
    }

    #[test]
    fn compare_different_downloads() {
        let d1= download::Download::new(
            "2026-02-21T17:25:10Z".to_string(), 
            "C:\\Users\\nathan\\Downloads\\evil.exe".to_string(),
            "https://evil[.]com/downloads".to_string(),
            "https://evil[.]com/downloads".to_string(),
            22304 as i64,
            "executable".to_string());
        let d2= download::Download::new(
            "2026-02-21T17:25:10Z".to_string(), 
            "C:\\Users\\nathan\\Downloads\\evil.exe".to_string(),
            "https://evil[.]com/downloads".to_string(),
            "https://evil[.]com/cart".to_string(),
            22304 as i64,
            "executable".to_string());
            assert_ne!(d1, d2);
    }

    #[test]
    fn compare_equal_urls() {
        let u1 = url::URL::new(
            1234 as i64,
            "evil.com".to_string(),
            "Evil Site".to_string(),
            "2026-02-21T17:24:23Z".to_string(),
            5 as i64);
        let u2 = url::URL::new(
            1234 as i64,
            "evil.com".to_string(),
            "Evil Site".to_string(),
            "2026-02-21T17:24:23Z".to_string(),
            5 as i64);
        assert_eq!(u1, u2);
    }

    #[test]
    fn compare_different_urls() {
        let u1 = url::URL::new(
            1234 as i64,
            "evil.com".to_string(),
            "Evil Site".to_string(),
            "2026-02-21T17:24:23Z".to_string(),
            5 as i64);
        let u2 = url::URL::new(
            1234 as i64,
            "evil.com".to_string(),
            "The Evil Site".to_string(),
            "2026-02-21T17:24:23Z".to_string(),
            5 as i64);
        assert_ne!(u1, u2);
    }

    #[test]
    fn acquire_database_contents() {
        let mut db = parser::DBParser::new("data\\History".to_string());
        _ = db.parse_database();
        assert!(db.get_url_entries_length() > 0);
        assert!(db.get_download_length() > 0);
    }
}