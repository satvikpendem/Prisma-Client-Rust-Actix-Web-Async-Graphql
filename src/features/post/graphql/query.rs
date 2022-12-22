use async_graphql::{Context, Object, Result};

use crate::prisma::PrismaClient;

use super::types::Post;

#[derive(Default, Clone)]
pub struct PostQuery;

#[Object]
impl PostQuery {
    async fn get_posts(&self, context: &Context<'_>) -> Result<Vec<Post>> {
        let database = context.data::<PrismaClient>().unwrap();

        Ok(database
            .post()
            .find_many(vec![])
            .exec()
            .await?
            .into_iter()
            .map(|p| p.into())
            .collect())
    }
}
