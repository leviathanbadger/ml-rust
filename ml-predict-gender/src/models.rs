use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DbNameRecord {
    pub name: String,
    pub total_count: i32,
    pub male_count: i32,
    pub female_count: i32,
    pub gender_percent_male: f32
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DbNameData {
    pub name: String,
    pub gender_percent_male: f32
}
