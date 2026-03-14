use std::fmt::Display;

#[derive(Debug)]
pub struct Download {
    start_time: String,
    target_path: String,
    referrer_url: String,
    tab_url: String,
    received_bytes: i64,
    mime_type: String,
}

impl Download {
    pub fn new(
        start_time: String, 
        target_path: String, 
        referrer_url: String, 
        tab_url: String, 
        received_bytes: i64, 
        mime_type: String) -> Self {
        Self { start_time, target_path, referrer_url, tab_url, received_bytes,
        mime_type }
    }

    pub fn get_start_time(&self) -> String {
        self.start_time.clone()
    }

    pub fn get_download_path(&self) -> String {
        self.target_path.clone()
    }

    pub fn get_referrer_url(&self) -> String {
        self.referrer_url.clone()
    }

    pub fn get_tab_url(&self) -> String {
        self.tab_url.clone()
    }

    pub fn get_received_bytes(&self) -> i64 {
        self.received_bytes
    }

    pub fn get_mime_type(&self) -> String {
        self.mime_type.clone()
    }
}

impl PartialEq for Download {
    fn eq(&self, other: &Self) -> bool {
        self.get_start_time() == other.get_start_time() &&
        self.get_download_path() == other.get_download_path() &&
        self.get_referrer_url() == other.get_referrer_url() &&
        self.get_tab_url() == other.get_tab_url() &&
        self.get_received_bytes() == other.get_received_bytes() &&
        self.get_mime_type() == other.get_mime_type()
    }

    fn ne(&self, other: &Self) -> bool {
        self.get_start_time() != other.get_start_time() ||
        self.get_download_path() != other.get_download_path() ||
        self.get_referrer_url() != other.get_referrer_url() ||
        self.get_tab_url() != other.get_tab_url() ||
        self.get_received_bytes() != other.get_received_bytes() ||
        self.get_mime_type() != other.get_mime_type()
    }
}

impl Display for Download {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{\n\tTimeline: {},\n\tDownload Path: {},\n\tSite URL: {},\n\tReferrer URL: {},\n\tReceived Bytes: {},\n\tMime Type: {}\n}}\n",
            self.get_start_time(),
            self.get_download_path(),
            self.get_tab_url(),
            self.get_referrer_url(),
            self.get_received_bytes(),
            self.get_mime_type(),
        )
    }
}