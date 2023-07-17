use firestore::*;
use juniper::futures::stream::BoxStream;
use juniper::futures::StreamExt;
use std::collections::HashMap;

use crate::models;

pub type BoxError = Box<dyn std::error::Error + Send + Sync + 'static>;

pub async fn get_client() -> Result<firestore::FirestoreDb, BoxError> {
    let db = FirestoreDb::new("mystvff-a9ad3").await?;
    return Ok(db);
}

pub async fn get_bounties(db: &FirestoreDb) -> Result<Vec<models::Bounty>, BoxError> {
    const COLLECTION_NAME: &'static str = "bounties";

    let snapshot: BoxStream<models::Bounty> = db
        .fluent()
        .select()
        .from(COLLECTION_NAME)
        .obj()
        .stream_query()
        .await?;

    let bounties: Vec<models::Bounty> = snapshot.collect().await;

    Ok(bounties)
}

pub async fn get_profile(
    db: &FirestoreDb,
    profile_id: String,
) -> Result<Option<models::Profile>, BoxError> {
    const COLLECTION_NAME: &'static str = "users";

    let profile: Option<models::Profile> = db
        .fluent()
        .select()
        .by_id_in(COLLECTION_NAME)
        .obj()
        .one(profile_id)
        .await?;

    Ok(profile)
}

pub async fn get_profiles(
    db: &FirestoreDb,
    ids: Vec<String>,
) -> Result<HashMap<String, Option<models::Profile>>, BoxError> {
    const COLLECTION_NAME: &'static str = "users";

    let mut snapshot: BoxStream<(String, Option<models::Profile>)> = db
        .fluent()
        .select()
        .by_id_in(COLLECTION_NAME)
        .obj()
        .batch(ids)
        .await?;

    let mut profiles: HashMap<String, Option<models::Profile>> = HashMap::new();

    while let Some(res) = snapshot.next().await {
        profiles.insert(res.0, res.1);
    }

    Ok(profiles)
}
