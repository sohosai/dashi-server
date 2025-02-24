# Service and Container

## RDB (Container)

- PostgreSQL

## GraphDB (Container)

- Neo4j

## Meilisearch (Container)

- Meilisearch

## Object Strage (Service)

※ initilize のときのみ使用

- Cloudflare R2

# 構造

server では、Layered Architecture を採用している

※ SeaORM を採用している関係で構造が乱れているが、 SeaORM を使用する場合はこの構造のままの方が使いやすいので、この構造のままにしている

```mermaid
flowchart TD
    presentation --> application
    presentation --> domain
    presentation --> infrastructure
    application --> domain
    application --> entity
    domain --> entity
    infrastructure --> domain
    infrastructure --> entity
    init --> infrastructure
    init --> domain
    migration
```

## presentation

- src/presentation 以下の binary crate

## application

- src/application 以下の library crate

## domain

- src/domain 以下の library crate

## infrastructure

- src/infrastructure 以下の library crate
- migration
- entity

# 開発環境

> [!WARNING]
> 開発環境の構築は、Nix、flakes、direnv、nix-direnvの導入が済んでいることを前提としています。

## `direnv allow`

```sh
direnv allow
```

## 開発環境の構築

```sh
docker-compose -f dev.compose.yaml up -d
```

## server の起動

```sh
cargo run --bin presentation
```

## 開発環境の削除

```sh
docker-compose -f dev.compose.yaml down --rmi all --volumes
sudo rm -rf postgres neo4j meilisearch
```

# 製品版の環境

## 製品版の環境の構築

```sh
docker-compose -f prod.compose.yaml up -d
```

# 初期データ

## RDB

```mermaid
erDiagram
    Item |o--|| Label : "VisibleId<->VisibleId"
    Item {
        i32 Id PK "1"
        String VisibleId FK "0000"
        String Name "筑波大學"
        String ProductNumber ""
        String Description "ルートの物品です"
        Option_i32 PurchaseYear ""
        Option_i32 PurchasePrice ""
        Option_i32 Durability ""
        boolean IsDepreciation "false"
        Json Connector "vec![]"
        boolean IsRent "false"
        String Color ""
        datetime CreatedAt "Utc::now().naive_utc()"
        datetime UpdatedAt "Utc::now().naive_utc()"
    }
    Label {
        String VisibleId UK "0000"
        boolean IsMax "true"
        String Record "Record::Nothing"
    }
```

## Meilisearch

```mermaid
erDiagram
    Item {
        i32 Id PK "1"
        String VisibleId FK "0000"
        String Record "Record::Nothing"
        String Name "筑波大學"
        String ProductNumber ""
        String Description "ルートの物品です"
        Option_i32 PurchaseYear ""
        Option_i32 PurchasePrice ""
        Option_i32 Durability ""
        boolean IsDepreciation "false"
        Json Connector "vec![]"
        boolean IsRent "false"
        String Color ""
        datetime CreatedAt "Utc::now().naive_utc()"
        datetime UpdatedAt "Utc::now().naive_utc()"
    }
```

## GraphDB

```mermaid
flowchart TD
    id1(("id: 1"))
```
