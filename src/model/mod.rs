use async_graphql::SimpleObject;
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, PartialEq, Eq, Clone, SimpleObject)]
pub struct Note {
    pub id: Uuid,
    pub title: String,
    pub contents: Option<String>,
    pub created_at: DateTime<Utc>,
    pub modified_at: Option<DateTime<Utc>>,
}

impl Default for Note {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            title: "".to_string(),
            contents: None,
            created_at: Utc::now(),
            modified_at: None,
        }
    }
}

impl Note {
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
            ..Default::default()
        }
    }
}
