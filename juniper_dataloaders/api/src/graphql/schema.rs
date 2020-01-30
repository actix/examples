use super::{mutation::Mutation, query::Query};
use crate::data::{CultData, PersonData};

use juniper;

#[derive(Clone)]
pub struct Context {
    pub person_data: PersonData,
    pub cult_data: CultData,
}

impl juniper::Context for Context {}

impl Context {
    pub fn new(person_data: PersonData, cult_data: CultData) -> Self {
        Self {
            person_data,
            cult_data,
        }
    }
}

pub type Schema = juniper::RootNode<'static, Query, Mutation>;

pub fn create_schema() -> Schema {
    Schema::new(Query {}, Mutation {})
}
