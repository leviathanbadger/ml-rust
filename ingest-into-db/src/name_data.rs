use std::{
    fs,
    collections::{HashMap},
    path::{PathBuf}
};
use anyhow::{Result};
use mongodb::sync::{Database, Collection};
use mongodb::bson::{doc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
enum Gender {
    #[serde(rename = "M")]
    Male,
    #[serde(rename = "F")]
    Female
}

#[derive(Debug, Serialize, Deserialize)]
struct CsvNameRecord {
    name: String,
    gender: Gender,
    count: i32
}

struct NameData {
    male: i32,
    female: i32
}

#[derive(Debug, Serialize, Deserialize)]
struct DbNameRecord {
    name: String,
    total_count: i32,
    male_count: i32,
    female_count: i32,
    gender_percent_male: f32
}

fn count_names(names: &Collection<DbNameRecord>) -> Result<i32> {
    let mut count_cursor = names.aggregate(vec![
        doc! {
            "$count": "name"
        }
    ], None)?;

    if let Some(result) = count_cursor.next() {
        let doc = result?;
        let count = doc.get_i32("name")?;
        Ok(count)
    }
    else {
        Ok(0)
    }
}

fn aggregate_name_data(path: PathBuf, name_map: &mut HashMap::<String, NameData>) -> Result<()> {
    println!("Aggregating data from {:?}...", path.file_name().unwrap());
    let mut reader = csv::Reader::from_path(path)?;

    for result in reader.records() {
        let record: CsvNameRecord = result?.deserialize(None)?;

        if let Some(name_data) = name_map.get_mut(&record.name) {
            match record.gender {
                Gender::Male => name_data.male += record.count,
                Gender::Female => name_data.female += record.count,
            };
        }
        else {
            let mut name_data = NameData { male: 0, female: 0 };
            match record.gender {
                Gender::Male => name_data.male += record.count,
                Gender::Female => name_data.female += record.count,
            };
            name_map.insert(record.name.clone(), name_data);
        }
    }

    Ok(())
}

fn save_name_data(name_map: &HashMap::<String, NameData>, names: Collection<DbNameRecord>) -> Result<()> {
    println!("Saving name data to mongo database...");

    for name in name_map.keys() {
        if let Some(value) = name_map.get(name) {
            let db_record = DbNameRecord {
                name: name.clone(),
                total_count: value.male + value.female,
                male_count: value.male,
                female_count: value.female,
                gender_percent_male: (value.male as f32) / ((value.male + value.female) as f32)
            };
            names.insert_one(db_record, None)?;
        }
    }

    Ok(())
}

pub fn ingest_name_data(database: Database) -> Result<()> {
    let mut name_map = HashMap::new();

    let names = database.collection::<DbNameRecord>("names");
    let count = count_names(&names)?;
    if count > 0 {
        println!("Skipping name data ingestion. There is already data in the database.");
        return Ok(());
    }

    let paths = fs::read_dir("./name-data")?;
    for path in paths {
        aggregate_name_data(path?.path(), &mut name_map)?;
    }

    save_name_data(&name_map, names)?;

    Ok(())
}
