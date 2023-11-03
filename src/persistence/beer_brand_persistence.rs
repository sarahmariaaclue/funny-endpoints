use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, sqlx::FromRow)]
pub struct BeerBrandEntity {
    id: i64,
    name: String,
    city: String,
}

impl BeerBrandEntity {
    pub fn get_id(&self) -> i64 {
        self.id
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_city(&self) -> &str {
        &self.city
    }
    // TODO nur für tests, remove
    pub fn create(id: i64, name: String, city: String) -> Self {
        Self { id, name, city }
    }
}

pub mod repository {

    use super::*;
    use crate::FunnyEndpointsDB;
    use rocket_db_pools::Connection;
    use sqlx::{self};

    pub async fn find_beer_brands(
        mut connection: Connection<FunnyEndpointsDB>,
    ) -> Result<Vec<BeerBrandEntity>, DatabaseError> {
        let find_all_statement = "SELECT * FROM beer_brand";
        sqlx::query_as::<_, BeerBrandEntity>(find_all_statement)
            .fetch_all(&mut *connection)
            .await
            .map_err(DatabaseError)
    }

    pub async fn find_beer_brand_by_id(
        mut connection: Connection<FunnyEndpointsDB>,
        id: i64,
    ) -> Result<Option<BeerBrandEntity>, DatabaseError> {
        let find_by_id_statement = "SELECT * FROM beer_brand WHERE id = $1";
        sqlx::query_as::<_, BeerBrandEntity>(find_by_id_statement)
            .bind(id)
            .fetch_optional(&mut *connection)
            .await
            .map_err(DatabaseError)
    }

    pub async fn save_beer_brand(
        mut connection: Connection<FunnyEndpointsDB>,
        beer_brand_name: &str, // doch dto übergeben?
        beer_brand_city: &str,
    ) -> Result<BeerBrandEntity, DatabaseError> {
        let save_new_statement = "INSERT INTO beer_brand (name, city) VALUES ($1, $2) RETURNING *";
        sqlx::query_as::<_, BeerBrandEntity>(save_new_statement)
            .bind(beer_brand_name)
            .bind(beer_brand_city)
            .fetch_one(&mut *connection)
            .await
            .map_err(DatabaseError)
    }

    pub async fn save_updated_beer_brand(
        mut connection: Connection<FunnyEndpointsDB>,
        beer_brand_entity: BeerBrandEntity,
    ) -> Result<Option<BeerBrandEntity>, DatabaseError> {
        let update_statement =
            "UPDATE beer_brand SET name = $2, city = $3 WHERE id= $1 RETURNING *";
        sqlx::query_as::<_, BeerBrandEntity>(update_statement)
            .bind(beer_brand_entity.get_id())
            .bind(beer_brand_entity.get_name())
            .bind(beer_brand_entity.get_city())
            .fetch_optional(&mut *connection)
            .await
            .map_err(DatabaseError)
    }

    pub async fn delete_beer_brand_by_id(
        mut connection: Connection<FunnyEndpointsDB>,
        id: i64,
    ) -> Result<Option<BeerBrandEntity>, DatabaseError> {
        let delete_statement = "DELETE FROM beer_brand WHERE id = $1 RETURNING *";
        sqlx::query_as::<_, BeerBrandEntity>(delete_statement)
            .bind(id)
            .fetch_optional(&mut *connection)
            .await
            .map_err(DatabaseError)
    }

    #[derive(Debug)]
    pub struct DatabaseError(sqlx::Error);

    impl DatabaseError {
        pub fn get_message(&self) -> String {
            format!("Database error: {}", self.0)
        }
    }
}
