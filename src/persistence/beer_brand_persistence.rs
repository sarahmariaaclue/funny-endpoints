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
}
