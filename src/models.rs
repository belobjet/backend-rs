use chrono::{DateTime, Utc};
use juniper::GraphQLObject;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Bounty {
    #[serde(alias = "_firestore_id")]
    pub id: String,

    #[serde(
        alias = "_firestore_created",
        with = "firestore::serialize_as_timestamp"
    )]
    pub created_at: DateTime<Utc>,

    #[serde(
        default,
        alias = "_firestore_updated",
        with = "firestore::serialize_as_optional_timestamp"
    )]
    pub updated_at: Option<DateTime<Utc>>,

    pub title: String,
    pub content: String,

    #[serde(alias = "nbComments")]
    pub nb_comments: i32,

    #[serde(default, alias = "imageURLs")]
    pub image_urls: Vec<String>,

    #[serde(alias = "author")]
    pub created_by: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BountyComment {
    #[serde(alias = "_firestore_id")]
    pub id: String,

    #[serde(
        alias = "_firestore_created",
        with = "firestore::serialize_as_timestamp"
    )]
    pub created_at: DateTime<Utc>,

    #[serde(
        default,
        alias = "_firestore_updated",
        with = "firestore::serialize_as_optional_timestamp"
    )]
    pub updated_at: Option<DateTime<Utc>>,

    pub content: String,
    pub author: String,

    #[serde(alias = "bountyID")]
    pub bounty_id: String,

    #[serde(default, alias = "upvotedBy")]
    pub upvoted_by: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, GraphQLObject, Clone)]
pub struct Profile {
    #[serde(alias = "_firestore_id")]
    pub id: String,

    #[serde(
        alias = "_firestore_created",
        with = "firestore::serialize_as_timestamp"
    )]
    pub created_at: DateTime<Utc>,

    #[serde(
        default,
        alias = "_firestore_updated",
        with = "firestore::serialize_as_optional_timestamp"
    )]
    pub updated_at: Option<DateTime<Utc>>,

    #[serde(alias = "displayName")]
    pub username: String,

    #[serde(alias = "profilePicture")]
    pub profile_picture: Option<String>,
}
