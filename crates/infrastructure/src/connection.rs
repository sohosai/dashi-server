use crate::connection::graphdb::connect_neo4j;
use crate::connection::meilisearch::connect_meilisearch;
use crate::connection::object_strage::connect_r2;
use crate::connection::rdb::connect_postgres;
use cf_r2_sdk::operator::Operator;
use domain::repository::connection::ConnectionRepository;
use domain::value_object::error::connection::ConnectionError;
use meilisearch_sdk::client::Client;
use neo4rs::Graph;
use sea_orm::DatabaseConnection;

pub mod auth0;
pub mod discord;
pub mod graphdb;
pub mod meilisearch;
pub mod object_strage;
pub mod rdb;

#[derive(Clone)]
pub struct CollectConnection {
    pub graphdb: Graph,
    pub meilisearch: Client,
    pub rdb: DatabaseConnection,
}

impl ConnectionRepository for CollectConnection {
    async fn new() -> Result<Self, ConnectionError>
    where
        Self: std::marker::Sized,
    {
        let graphdb = connect_neo4j().await?;
        let meilisearch = connect_meilisearch().await?;
        let rdb = connect_postgres().await?;
        Ok(CollectConnection {
            graphdb,
            meilisearch,
            rdb,
        })
    }
    async fn connect_graphdb() -> Result<Graph, ConnectionError> {
        connect_neo4j().await
    }
    async fn connect_meilisearch() -> Result<Client, ConnectionError> {
        connect_meilisearch().await
    }
    async fn connect_object_strage() -> Result<Operator, ConnectionError> {
        connect_r2().await
    }
    async fn connect_rdb() -> Result<DatabaseConnection, ConnectionError> {
        connect_postgres().await
    }
}
