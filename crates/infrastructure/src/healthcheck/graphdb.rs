use domain::value_object::error::healthcheck::HealthCheckError;
use neo4rs::{query, Graph, Node};

pub(super) async fn healthcheck_graphdb(graphdb: Graph) -> Result<(), HealthCheckError> {
    //* test *//
    // get (item:Item {id: 1}) test
    let _ = graphdb
        .execute(query("MATCH (item:Item {id: $id}) RETURN item").param("id", 1))
        .await?;

    //* check *//
    // get node
    let mut root_item = match graphdb
        .execute(query("MATCH (item:Item {id: $id}) RETURN item").param("id", 1))
        .await
    {
        Ok(item) => item,
        Err(e) => {
            tracing::error!("Failed to get item node.");
            tracing::error!("{}", e.to_string());
            return Err(HealthCheckError::GraphDBError(e));
        }
    };
    // parse node
    loop {
        let item = match root_item.next().await {
            Ok(item) => item,
            Err(_) => {
                tracing::error!("Failed to get item.");
                return Err(HealthCheckError::RootItemNotFoundError);
            }
        };
        let row = match item {
            Some(row) => row,
            None => break,
        };
        let node: Node = match row.get::<Node>("item") {
            Ok(node) => node,
            Err(e) => {
                tracing::error!("Failed to get node.");
                tracing::error!("{}", e.to_string());
                return Err(HealthCheckError::GraphDBDeError(e));
            }
        };
        let id: i64 = match node.get::<i64>("id") {
            Ok(id) => id,
            Err(e) => {
                tracing::error!("Failed to get id.");
                tracing::error!("{}", e.to_string());
                return Err(HealthCheckError::GraphDBDeError(e));
            }
        };
        // implement first item check
        if id != 1 {
            tracing::error!("Failed to get id.");
            return Err(HealthCheckError::IncompatibleInGraphDBError);
        }
    }
    tracing::info!("GraphDB is healthy.");
    Ok(())
}
