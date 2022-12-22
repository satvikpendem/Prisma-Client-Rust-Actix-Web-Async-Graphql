use async_graphql::{Context, InputObject, Object, Result};

use crate::prisma::{user, PrismaClient};

use super::types::Post;

#[derive(InputObject)]
pub struct CreatePostInput {
    pub content: String,
    pub user_id: String,
}

#[derive(Default, Clone)]
pub struct PostMutation;

#[Object]
impl PostMutation {
    pub async fn create_post(&self, context: &Context<'_>, input: CreatePostInput) -> Result<Post> {
        let database = context.data::<PrismaClient>().unwrap();

        let created = database
            .post()
            .create(input.content, user::id::equals(input.user_id), vec![])
            .exec()
            .await
            .unwrap();

        Ok(created.into())
    }
}
