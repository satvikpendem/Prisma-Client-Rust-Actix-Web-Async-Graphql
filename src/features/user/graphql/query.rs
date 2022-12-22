use async_graphql::{Context, Object, Result};

use crate::prisma::{user, PrismaClient};

use super::types::User;

#[derive(Default, Clone)]
pub struct UserQuery;

#[Object]
impl UserQuery {
    async fn get_users(&self, context: &Context<'_>) -> Result<Vec<User>> {
        let database = context.data::<PrismaClient>().unwrap();

        Ok(database
            .user()
            .find_many(vec![])
            .exec()
            .await?
            .into_iter()
            .map(|u| u.into())
            .collect())
    }

    async fn get_user(&self, context: &Context<'_>, id: String) -> Result<Option<User>> {
        let database = context.data::<PrismaClient>().unwrap();

        Ok(database
            .user()
            .find_unique(user::id::equals(id))
            .exec()
            .await?
            .map(|u| u.into()))
    }
}
