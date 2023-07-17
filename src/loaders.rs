use ::firestore::FirestoreDb;
use async_trait::async_trait;
use dataloader::non_cached::Loader;
use dataloader::BatchFn;
use log;
use std::collections::HashMap;

use crate::firestore;
use crate::models;

pub struct ProfileBatcher {
    firestore: FirestoreDb,
}

#[async_trait]
impl BatchFn<String, Option<models::Profile>> for ProfileBatcher {
    async fn load(&self, keys: &[String]) -> HashMap<String, Option<models::Profile>> {
        log::debug!("Loading profiles: {:?}", keys);
        let profiles = firestore::get_profiles(&self.firestore, keys.to_vec())
            .await
            .unwrap();
        let mut map = HashMap::new();
        for res in profiles {
            if let Some(profile) = res.1 {
                map.insert(profile.id.clone(), Some(profile));
            }
        }
        map
    }
}

pub type ProfileLoader = Loader<String, Option<models::Profile>, ProfileBatcher>;

pub fn get_loader(firestore: FirestoreDb) -> ProfileLoader {
    Loader::new(ProfileBatcher { firestore }).with_yield_count(100)
}
