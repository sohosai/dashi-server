use crate::value_object::error::connection::ConnectionError;
use async_std::future::Future;
use cf_r2_sdk::operator::Operator;
use meilisearch_sdk::client::Client;
use neo4rs::Graph;
use sea_orm::DatabaseConnection;

pub trait ConnectionRepository {
    fn new() -> impl Future<Output = Result<Self, ConnectionError>> + Send
    where
        Self: std::marker::Sized;
    fn connect_graphdb() -> impl Future<Output = Result<Graph, ConnectionError>> + Send;
    fn connect_meilisearch() -> impl Future<Output = Result<Client, ConnectionError>> + Send;
    fn connect_object_strage() -> impl Future<Output = Result<Operator, ConnectionError>> + Send;
    fn connect_rdb() -> impl Future<Output = Result<DatabaseConnection, ConnectionError>> + Send;
}
