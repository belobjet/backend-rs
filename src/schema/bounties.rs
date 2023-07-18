use crate::models::{Bounty, BountyComment, Profile};
use crate::schema::Database;
use juniper::graphql_object;

use crate::firestore::get_comments;

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

    async fn comments(&self, ctx: &Database) -> Vec<BountyComment> {
        get_comments(&ctx.firebase, self.id.clone()).await.unwrap()
    }
}

#[graphql_object(context = Database)]
impl BountyComment {
    fn id(&self) -> &str {
        &self.id
    }

    fn created_at(&self) -> &chrono::DateTime<chrono::Utc> {
        &self.created_at
    }

    fn updated_at(&self) -> &Option<chrono::DateTime<chrono::Utc>> {
        &self.updated_at
    }

    fn content(&self) -> &str {
        &self.content
    }

    async fn created_by(&self, ctx: &Database) -> Option<Profile> {
        ctx.profile_loader.load(self.author.clone()).await
    }
}
