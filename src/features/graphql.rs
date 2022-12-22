use async_graphql::{EmptySubscription, MergedObject, Schema};

use crate::prisma;

use super::{post::graphql::mutation::PostMutation, user::graphql::mutation::UserMutation};
use super::{post::graphql::query::PostQuery, user::graphql::query::UserQuery};

#[derive(MergedObject, Default, Clone)]
pub struct Query(UserQuery, PostQuery);

#[derive(MergedObject, Default, Clone)]
pub struct Mutation(UserMutation, PostMutation);

pub type AppSchema = Schema<Query, Mutation, EmptySubscription>;

pub async fn build_schema() -> AppSchema {
    let database = prisma::new_client()
        .await
        .expect("Failed to connect to database");

    Schema::build(Query::default(), Mutation::default(), EmptySubscription)
        .data(database)
        .finish()
}
