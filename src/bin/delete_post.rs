use entity::post;
use migration::DbErr;
use sea_orm::{DeleteResult, EntityTrait, ModelTrait};
use sea_orm_example::establish_connection;

#[tokio::main]
async fn main() -> Result<(), DbErr> {
    let db = establish_connection().await?;

    let post = post::Entity::find_by_id(1).one(&db).await.unwrap();
    let post: post::Model = post.unwrap();

    let res: DeleteResult = post.delete(&db).await?;

    assert_eq!(res.rows_affected, 1);

    println!("{:?}", res);

    Ok(())
}
