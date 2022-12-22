use async_graphql::{Context, InputObject, Object, Result};

use crate::prisma::PrismaClient;

use super::types::User;

#[derive(InputObject)]
pub struct CreateUserInput {
    pub display_name: String,
}

#[derive(Default, Clone)]
pub struct UserMutation;

#[Object]
impl UserMutation {
    pub async fn create_user(&self, context: &Context<'_>, input: CreateUserInput) -> Result<User> {
        let database = context.data::<PrismaClient>().unwrap();

        let created = database
            .user()
            .create(input.display_name, vec![])
            .exec()
            .await?;

        Ok(created.into())
    }
}
