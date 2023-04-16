use mongodb::sync::{Collection, Cursor};
use mongodb::bson::{doc, Document};
use anyhow::{Result};

use crate::models::{DbNameRecord, DbNameData};

struct NameDataIterator {
    cursor: Cursor<Document>
}

impl NameDataIterator {
    fn new(cursor: Cursor<Document>) -> Self {
        Self { cursor: cursor }
    }

    fn document_to_name_data(doc: Document) -> Result<DbNameData> {
        let name = doc.get_str("name")?.to_string();
        let gender_percent_male = doc.get_f64("gender_percent_male")? as f32;

        Ok(DbNameData {
            name: name,
            gender_percent_male: gender_percent_male
        })
    }
}

impl Iterator for NameDataIterator {
    type Item = DbNameData;

    fn next(&mut self) -> Option<Self::Item> {
        match self.cursor.next() {
            Some(Ok(doc)) => {
                match Self::document_to_name_data(doc) {
                    Ok(data) => Some(data),
                    Err(e) => {
                        println!("Error converting document to name data: {:?}", e);
                        None
                    }
                }

            },
            Some(Err(e)) => {
                println!("Error retrieving training data: {:?}", e);
                None
            },
            None => None
        }
    }
}

fn get_name_data(names: &Collection<DbNameRecord>, for_training: bool) -> Result<NameDataIterator> {
    let include_keys = if for_training {
        vec!["0", "1", "2", "3", "4", "5", "6", "7"]
    }
    else {
        vec!["8", "9", "a", "b", "c", "d", "e", "f"]
    };

    let cursor = names.aggregate(vec![
        doc! {
            "$addFields": {
                "key": {
                    "$substrBytes": [{ "$toString": "$_id" }, 23, 1]
                }
            }
        },

        doc! {
            "$addFields": {
                "use_for_training": {
                    "$in": ["$key", include_keys]
                }
            }
        },

        doc! {
            "$match": {
                "use_for_training": true
            }
        },

        doc! {
            "$project": {
                "_id": 0,
                "name": 1,
                "gender_percent_male": 1
            }
        }
    ], None)?;

    Ok(NameDataIterator::new(cursor))
}

pub fn train_neural_network(names: &Collection<DbNameRecord>) -> Result<()> {
    let name_data_iter = get_name_data(&names, true)?;

    println!("Names for training:");
    for name_data in name_data_iter.take(5) {
        println!("Name: {}, percent male: {}", name_data.name, name_data.gender_percent_male);
    }

    Ok(())
}
