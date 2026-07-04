use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Note {
    pub id: String,
    pub title: String,
    pub content: String,
    pub tags: Vec<String>,
    pub category: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Note {
    pub fn new(title: &str, content: &str, category: &str, tags: Vec<String>) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            title: title.to_string(),
            content: content.to_string(),
            tags,
            category: category.to_string(),
            created_at: now,
            updated_at: now,
        }
    }

    pub fn update(&mut self, title: &str, content: &str, category: &str, tags: Vec<String>) {
        self.title = title.to_string();
        self.content = content.to_string();
        self.category = category.to_string();
        self.tags = tags;
        self.updated_at = Utc::now();
    }
}
