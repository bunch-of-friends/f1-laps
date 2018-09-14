use bincode;
use serde::de::DeserializeOwned;
use serde::Serialize;
use sled::{ConfigBuilder, Tree};
use std::marker::PhantomData;

pub struct Repository<T: Serialize + DeserializeOwned> {
    pub tree: Tree,
    pub marker: PhantomData<T>,
}

#[allow(dead_code)]
impl<T: Serialize + DeserializeOwned> Repository<T> {
    pub fn new(path: &str) -> Repository<T> {
        let config = ConfigBuilder::new().path(&path).build();

        let tree = match Tree::start(config) {
            Ok(t) => t,
            Err(e) => panic!("failed to build tree from path: {:?}, error: {:?}", path, e),
        };

        Repository {
            tree,
            marker: PhantomData,
        }
    }

    pub fn set(&self, key: &str, value: &T) -> Option<()> {
        let bytes = bincode::serialize(value).expect("failed to serialise");
        match self.tree.set(key.as_bytes().to_vec(), bytes) {
            Ok(_) => Some(()),
            Err(_) => None,
        }
    }

    pub fn get(&self, key: &str) -> Option<T> {
        match self.tree.get(key.as_bytes()) {
            Ok(result) => {
                if let Some(bytes) = result {
                    bincode::deserialize::<T>(&bytes).ok()
                } else {
                    None
                }
            }
            Err(_) => None,
        }
    }

    pub fn delete(&self, key: &str) -> Option<()> {
        match self.tree.del(key.as_bytes()) {
            Ok(_) => Some(()),
            Err(_) => None,
        }
    }

    pub fn get_all(&self) -> Vec<T> {
        self.tree
            .scan(&[0])
            .map(|x| match x {
                Ok((_key, value)) => Some(value),
                Err(_) => None,
            })
            .filter(|x| x.is_some())
            .map(|x| bincode::deserialize::<T>(&x.unwrap()).ok())
            .filter(|x| x.is_some())
            .map(|x| x.unwrap())
            .collect()
    }
}
