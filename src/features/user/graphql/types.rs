use async_graphql::{ComplexObject, Context, Result, SimpleObject};

use crate::{
    features::post::graphql::types::Post,
    prisma::{post, user, PrismaClient},
};

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct User {
    pub id: String,
    pub display_name: String,
}

#[ComplexObject]
impl User {
    pub async fn posts(&self, context: &Context<'_>) -> Result<Vec<Post>> {
        let database = context.data::<PrismaClient>().unwrap();

        Ok(database
            .post()
            .find_many(vec![post::user_id::equals(self.id.clone())])
            .exec()
            .await?
            .into_iter()
            .map(|p| p.into())
            .collect())
    }
}

impl From<user::Data> for User {
    fn from(val: user::Data) -> Self {
        User {
            id: val.id,
            display_name: val.display_name,
        }
    }
}

impl From<&user::Data> for User {
    fn from(val: &user::Data) -> Self {
        User {
            id: val.id.clone(),
            display_name: val.display_name.clone(),
        }
    }
}
