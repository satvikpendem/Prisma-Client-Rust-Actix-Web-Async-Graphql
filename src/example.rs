mod prisma;

use prisma_client_rust::NewClientError;

use crate::prisma::{post, user};

#[actix_web::main]
async fn main() -> Result<(), NewClientError> {
    let client = prisma::new_client().await?;

    // client.user().delete_many(vec![]).exec().await.unwrap();
    // client.post().delete_many(vec![]).exec().await.unwrap();

    let user = client
        .user()
        .create("Alice".to_string(), vec![])
        .exec()
        .await
        .unwrap();

    client
        .post()
        .create("content".to_string(), user::id::equals(user.id), vec![])
        .exec()
        .await
        .unwrap();

    let users = client
        .user()
        .find_many(vec![])
        .with(user::posts::fetch(vec![]).with(post::user::fetch()))
        .exec()
        .await
        .unwrap();
    println!("{:#?}", users);

    users.into_iter().for_each(|user| {
        println!("user: {}", user.display_name);
        user.posts.unwrap().into_iter().for_each(|post| {
            println!("post: {}", post.content);
            println!("user: {}", post.user.unwrap().display_name);
        });
    });

    let users2 = client
        .user()
        .find_many(vec![])
        .include(user::include!({ posts: include { user: include { posts } } }))
        .exec()
        .await
        .unwrap();
    println!("{:#?}", users2);

    users2.into_iter().for_each(|user| {
        println!("user: {}", user.display_name);
        user.posts.into_iter().for_each(|post| {
            println!("post: {}", post.content);
            println!("user: {}", post.user.display_name);
        });
    });

    Ok(())
}
