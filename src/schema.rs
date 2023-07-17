use juniper::{graphql_object, EmptyMutation, EmptySubscription, FieldResult, RootNode};

use crate::firestore::get_bounties;
use crate::loaders;
use crate::models::{Bounty, Profile};
use firestore::FirestoreDb;

#[derive(Clone)]
pub struct Database {
    firebase: FirestoreDb,
    profile_loader: loaders::ProfileLoader,
}

impl Database {
    pub fn new(firebase: FirestoreDb, profile_loader: loaders::ProfileLoader) -> Self {
        Self {
            firebase,
            profile_loader,
        }
    }
}

impl juniper::Context for Database {}
pub struct QueryRoot;

#[graphql_object(context = Database)]
impl Bounty {
    fn id(&self) -> &str {
        &self.id
    }

    fn created_at(&self) -> &chrono::DateTime<chrono::Utc> {
        &self.created_at
    }

    fn updated_at(&self) -> &Option<chrono::DateTime<chrono::Utc>> {
        &self.updated_at
    }

    fn title(&self) -> &str {
        &self.title
    }

    fn content(&self) -> &str {
        &self.content
    }

    fn image_urls(&self) -> &Vec<String> {
        &self.image_urls
    }

    async fn created_by(&self, ctx: &Database) -> Option<Profile> {
        ctx.profile_loader.load(self.created_by.clone()).await
    }
}

#[juniper::graphql_object(context = Database)]
impl QueryRoot {
    async fn get_bounties(context: &Database) -> FieldResult<Vec<Bounty>> {
        let bounties = get_bounties(&context.firebase).await?;

        Ok(bounties)
    }
}

pub type Schema =
    RootNode<'static, QueryRoot, EmptyMutation<Database>, EmptySubscription<Database>>;

pub fn create_schema() -> Schema {
    Schema::new(
        QueryRoot,
        EmptyMutation::<Database>::new(),
        EmptySubscription::<Database>::new(),
    )
}
