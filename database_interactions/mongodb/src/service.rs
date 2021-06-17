use futures::stream::StreamExt;
use mongodb::bson::{doc, Document};
use mongodb::{error::Error, results::InsertOneResult, Collection};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct NewUser {
    pub name: String,
    pub email: String,
}

pub struct UserService {
    collection: Collection,
}
impl UserService {
    pub fn new(collection: Collection) -> Self {
        Self { collection }
    }
    pub async fn create_user(&self, user: NewUser) -> Result<InsertOneResult, Error> {
        let NewUser { name, email, .. } = user;
        self.collection
            .insert_one(doc! {"name": name, "email": email}, None)
            .await
    }

    pub async fn get_all(&self) -> Result<Vec<Document>, Error> {
        let mut cursor = self.collection.find(None, None).await?;
        let mut users: Vec<Document> = Vec::new();

        while let Some(result) = cursor.next().await {
            match result {
                Ok(document) => users.push(document),
                Err(_) => todo!(),
            }
        }

        Ok(users)
    }
}
