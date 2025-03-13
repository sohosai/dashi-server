# Service and Container

## RDB (Container)

- PostgreSQL

## GraphDB (Container)

- Neo4j

## Meilisearch (Container)

- Meilisearch

## Object Strage (Service)

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

## 開発環境の構築

### 0. .envの設置

シークレットな情報のため、詳細はscrapbox参照

### 1. docker-compose up

```sh
docker-compose -f db.compose.yaml up -d
```

### 2. .envの編集

シークレットな情報のため、詳細はscrapbox参照

### 3. server の起動

```sh
cargo run --bin presentation
```

## 開発環境の削除

```sh
docker-compose -f db.compose.yaml down --rmi all --volumes
sudo rm -rf postgres neo4j meilisearch init
```

# 本番環境の構築

### 0. .envの設置

シークレットな情報のため、詳細はscrapbox参照

### 1. docker-compose up (db)

```sh
docker-compose -f db.compose.yaml up -d
```

### 2. .envの編集

シークレットな情報のため、詳細はscrapbox参照

### 3. docker-compose up (server)

```sh
docker-compose -f server.compose.yaml up -d
```

## 開発環境の削除

```sh
docker-compose -f db.compose.yaml down --rmi all --volumes
docker-compose -f server.compose.yaml down --rmi all --volumes
sudo rm -rf postgres neo4j meilisearch init
```

## テーブルの更新

### 1. docker-compose up

```sh
docker-compose -f entity.compose.yaml up -d
```

### 2. Migration

```sh
cargo run --manifest-path ./migration/Cargo.toml -- refresh -u postgres://<POSTGRES_USER>:<POSTGRES_PASSWORD>@localhost:<POSTGRES_PORT>/<POSTGRES_DB>
```


### 3. Entityの生成

```sh
sea-orm-cli generate entity \
    -u postgres://<POSTGRES_USER>:<POSTGRES_PASSWORD>@localhost:<POSTGRES_PORT>/<POSTGRES_DB> \
    -o entity/src
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
        String Recipient ""
        String RentalDescription ""
        Option_datetime LatestRentAt ""
        Option_datetime ScheduledReplaceAt ""
        Option_datetime LatestReplaceAt ""
    }
    Label {
        String VisibleId UK "0000"
        boolean IsMax "true"
        String Record "Record::Nothing"
    }
    Connector {
        String id PK "autoincrement"
        String name UK "connector.csvのnameが入る"
        String status "Status::Active"
    }
    Color {
        String id PK "autoincrement"
        String name UK "color.csvのnameが入る"
        String hex_color_code UK "color.csvのhex_color_codeが入る"
        String status "Status::Active"
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
        Option_i32 PurchaseYear "None"
        Option_i32 PurchasePrice "None"
        Option_i32 Durability "None"
        boolean IsDepreciation "false"
        Json Connector "vec![]"
        boolean IsRent "false"
        String Color ""
        datetime CreatedAt "Utc::now().naive_utc()"
        datetime UpdatedAt "Utc::now().naive_utc()"
        String Recipient ""
        String RentalDescription ""
        datetime LatestRentAt "None"
        datetime ScheduledReplaceAt "None"
        datetime LatestReplaceAt "None"
    }
    Connector {
        String id PK "autoincrement"
        String name UK "connector.csvのnameが入る"
        String status "Status::Active"
    }
    Color {
        String id PK "autoincrement"
        String name UK "color.csvのnameが入る"
        String hex_color_code UK "color.csvのhex_color_codeが入る"
        String status "Status::Active"
    }
```

## GraphDB

```mermaid
flowchart TD
    id1(("id: 1"))
```
