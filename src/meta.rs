// Meta struct
//
// Meta struct conatins all metafields for an Item (Page)

use chrono::{DateTime,Local};
use dateparser;
use regex::Regex;
use serde_derive::Deserialize;
use serde_yaml;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub struct Metadata {
    title: String,
    date: Option<String>, // Need to convert to DateTime<Timezone>
    nodate: Option<bool>,
    public: Option<bool>,
    disqus: Option<bool>,
    tags: Option<Vec<String>>,
    slug: Option<String>,
}

impl Metadata {
    fn new() -> Self {
        Metadata {
            title: "".to_string(),
            date: Some("".to_string()),
            nodate: None,
            public: Some(false),
            disqus: Some(false),
            tags: None,
            slug: None,
        }
    }
}

pub struct Meta {
    pub metadata: Metadata,
    pub id: String, // Identifier, It is file path.
    pub title: String,
    pub public: bool,
    pub slug: String,
    pub date: Option<DateTime<Local>>,
}

impl Meta {
    pub fn new(id: &String) -> Self {
        Meta {
            metadata: Metadata::new(),
            id: id.clone(),
            title: "".to_string(),
            public: false,
            slug: "".to_string(),
            date: None,
        }
    }

    pub fn from_vec(&mut self, v: Vec<String>) {
        let meta_str = v.join("\n");

        self.metadata = serde_yaml::from_str(&meta_str).unwrap();
        self.title = self.metadata.title.clone();
        self.public = self.metadata.public.unwrap_or(false);
        self.slug = self.metadata.slug.clone()
                                      .unwrap_or(_convert_to_slug(&self.title));
        // DateTime parse from String, if not, parse from file. If nodate, any
        // value.
        let nodate = self.metadata.nodate.unwrap_or(false);
        let mut date;
        if nodate {
            self.date = None;
            date = "".to_string();
        } else {
            date = match &self.metadata.date {
                Some(d) => d.clone(),
                None => _guess_date(&(self.id)),
            };
        }
        if ! nodate {
            // TODO: Support more than %B %e, %Y
            date.push_str(" 00:00:00");
            let parsed = dateparser::parse(&date).unwrap();
            self.date = Some(parsed.into());
        }
    }
}


fn _guess_date(id: &String) -> String {
    // Date can be guessed by the filename
    // If filename starts with YYYY-mm-dd , then it is being used as a date.
    let re = Regex::new(r"^(?<date>[1-2][0-9][0-9][0-9]-[0-1][0-9]-[0-3][0-9])").unwrap();
    let Some(caps) = re.captures(id) else {
        println!("Cannot guess date: {}", id);
        panic!();
    };
    caps["date"].to_string()
}

fn _convert_to_slug(title: &String) -> String {
    // TODO: Find if any lib does this
    // to lowercase
    let mut result = title.clone();
    result.replace(' ', "-");
    result.replace('\'', "");
    result.replace('"', "");
    result.to_ascii_lowercase();

    result
}