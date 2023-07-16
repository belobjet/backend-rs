use juniper::{EmptyMutation, EmptySubscription, FieldResult, GraphQLObject, RootNode};

#[derive(GraphQLObject)]
pub struct Bounty {
    id: String,
    title: String,
    content: String,
    nb_comments: i32,
}

pub struct QueryRoot;

#[juniper::graphql_object]
impl QueryRoot {
    fn get_bounties() -> FieldResult<Vec<Bounty>> {
        Ok(vec![
            Bounty {
                id: "1".to_owned(),
                title: "Bounty 1".to_owned(),
                content: "Bounty 1 content".to_owned(),
                nb_comments: 0,
            },
            Bounty {
                id: "2".to_owned(),
                title: "Bounty 2".to_owned(),
                content: "Bounty 2 content".to_owned(),
                nb_comments: 0,
            },
        ])
    }
}

pub type Schema = RootNode<'static, QueryRoot, EmptyMutation, EmptySubscription>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot {}, EmptyMutation::new(), EmptySubscription::new())
}
