use entity::post;
use migration::DbErr;
use sea_orm::EntityTrait;
use sea_orm_example::establish_connection;

#[tokio::main]
async fn main() -> Result<(), DbErr> {
    let db = establish_connection().await.unwrap();

    let posts: Vec<post::Model> = post::Entity::find().all(&db).await.unwrap();

    println!("All the posts in db");
    for post in posts {
        println!("ID: {} TITLE: {}", post.id, post.title);
    }

    Ok(())
}
