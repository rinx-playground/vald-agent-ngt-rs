use ngt::{DistanceType, Index, ObjectType, Properties};

use std::collections::HashMap;
use std::io;
use std::sync::Mutex;

use super::vald::payload::v1::object::Distance;

#[derive(Debug)]
pub struct NGT {
    index: Option<Index>,

    id_oid: Mutex<HashMap<String, u32>>,
    oid_id: Mutex<HashMap<u32, String>>,

    dimension: usize,
    distance_type: DistanceType,
    object_type: ObjectType,
    index_path: String,
}

impl Default for NGT {
    fn default() -> Self {
        NGT {
            index: None,
            id_oid: Mutex::new(HashMap::new()),
            oid_id: Mutex::new(HashMap::new()),
            dimension: 784,
            distance_type: DistanceType::L2,
            object_type: ObjectType::Float,
            index_path: "index".to_string(),
        }
    }
}

impl NGT {
    pub fn initialize(&mut self) -> Result<(), ngt::Error> {
        let prop = Properties::dimension(self.dimension)?
            .distance_type(self.distance_type)?
            .object_type(self.object_type)?;
        let index = Index::create("index", prop)?;

        self.index = Some(index);

        Ok(())
    }

    pub fn insert(&mut self, id: &str, vec: Vec<f32>) -> Result<u32, io::Error> {
        let index = match &mut self.index {
            Some(index) => index,
            None => {
                panic!("NGT index is not opened");
            }
        };

        let oid = index.insert(vec).unwrap();

        self.id_oid.lock().unwrap().insert(id.to_string(), oid);
        self.oid_id.lock().unwrap().insert(oid, id.to_string());

        Ok(oid)
    }

    pub fn search(
        &self,
        vec: Vec<f64>,
        num: u64,
        epsilon: f32,
    ) -> Result<Vec<Distance>, io::Error> {
        let index = match &self.index {
            Some(index) => index,
            None => {
                panic!("NGT index is not opened");
            }
        };

        let results = index
            .search(&vec, num, epsilon)
            .unwrap()
            .iter()
            .map(|r| {
                let id = match self.oid_id.lock().unwrap().get(&r.id) {
                    Some(id) => id.to_string(),
                    None => "".to_string(),
                };

                Distance {
                    id,
                    distance: r.distance,
                }
            })
            .collect();

        Ok(results)
    }

    pub fn create_index(&mut self) -> Result<(), io::Error> {
        let index = match &mut self.index {
            Some(index) => index,
            None => {
                panic!("NGT index is not opened");
            }
        };

        index.build(1).unwrap();

        Ok(())
    }
}
