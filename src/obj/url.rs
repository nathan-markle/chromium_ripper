use std::fmt::Display;

#[derive(Debug)]
pub struct URL {
    id: i64,
    url: String,
    title: String,
    last_visit_time: String,
    visit_count: i64,
}

impl URL {
    pub fn new(
        id: i64, 
        url: String, 
        title: String, 
        last_visit_time: String,
        visit_count: i64) -> Self {
        Self { id, url, title, last_visit_time, visit_count }
    }

    pub fn get_id(&self) -> i64 {
        self.id
    }

    pub fn get_url(&self) -> String {
        self.url.clone()
    }

    pub fn get_title(&self) -> String {
        self.title.clone()
    }

    pub fn get_visit_time(&self) -> String {
        self.last_visit_time.clone()
    }    

    pub fn get_visits(&self) -> i64 {
        self.visit_count
    }
}

impl PartialEq for URL {
    fn eq(&self, other: &Self) -> bool {
        self.get_id() == other.get_id() &&
        self.get_url() == other.get_url() &&
        self.get_title() == other.get_title() &&
        self.get_visit_time() == other.get_visit_time() &&
        self.get_visits() == other.get_visits()
    }

    fn ne(&self, other: &Self) -> bool {
        self.get_id() != other.get_id() ||
        self.get_url() != other.get_url() ||
        self.get_title() != other.get_title() ||
        self.get_visit_time() != other.get_visit_time() ||
        self.get_visits() != other.get_visits()
    }
}

impl Display for URL {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{\n\tID: {},\n\tURL: {},\n\tTitle: {},\n\tVisit Time: {},\n\tVisit Count: {}\n}}\n",
            self.get_id(),
            self.get_url(),
            self.get_title(),
            self.get_visit_time(),
            self.get_visits()
        )
    }
}