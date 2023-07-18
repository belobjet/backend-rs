use juniper::{graphql_object, EmptyMutation, EmptySubscription, FieldResult, RootNode};

use crate::firestore::get_bounties;
use crate::loaders;
use crate::models::Bounty;
use firestore::FirestoreDb;

mod bounties;

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
