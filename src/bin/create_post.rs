use entity::post;
use migration::DbErr;
use sea_orm::{ActiveModelTrait, Set};
use sea_orm_example::establish_connection;

#[tokio::main]
async fn main() -> Result<(), DbErr> {
    let db = establish_connection().await.unwrap();

    let post = post::ActiveModel {
        title: Set(String::from("할일 1")),
        text: Set(String::from("러스트")),
        ..Default::default()
    };

    let post: post::Model = post.insert(&db).await.unwrap();

    println!("Post created with ID: {}, TITLE: {}", post.id, post.title);

    Ok(())
}
