use async_graphql::{extensions::Logger, EmptyMutation, EmptySubscription, MergedObject, Schema};
use sqlx::{Pool, Postgres};

mod hello;
mod world;

#[derive(MergedObject, Default)]
pub struct Query(hello::Hello, world::World);

pub type SchemaType = Schema<Query, EmptyMutation, EmptySubscription>;

pub fn create_schema(database_pool: Pool<Postgres>) -> SchemaType {
    Schema::build(Query::default(), EmptyMutation, EmptySubscription)
        .extension(Logger)
        .data(database_pool)
        .finish()
}
