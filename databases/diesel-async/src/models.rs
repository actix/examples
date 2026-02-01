use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::schema::items;

/// Item details.
#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Selectable, Insertable)]
#[diesel(table_name = items)]
pub struct Item {
    pub id: Uuid,
    pub name: String,
}

/// New item details.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NewItem {
    pub name: String,
}

#[cfg(not(feature = "postgres_tests"))]
impl NewItem {
    /// Constructs new item details from name.
    #[cfg(test)] // only needed in tests
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}
