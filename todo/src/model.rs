use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;

use crate::schema::{
    tasks,
    tasks::dsl::{completed as task_completed, tasks as all_tasks},
};

#[derive(Debug, Insertable)]
#[table_name = "tasks"]
pub struct NewTask {
    pub description: String,
}

#[derive(Debug, Queryable, Serialize)]
pub struct Task {
    pub id: i32,
    pub description: String,
    pub completed: bool,
}

impl Task {
    pub fn all(conn: &PgConnection) -> QueryResult<Vec<Task>> {
        all_tasks.order(tasks::id.desc()).load::<Task>(conn)
    }

    pub fn insert(todo: NewTask, conn: &PgConnection) -> QueryResult<usize> {
        diesel::insert_into(tasks::table)
            .values(&todo)
            .execute(conn)
    }

    pub fn toggle_with_id(id: i32, conn: &PgConnection) -> QueryResult<usize> {
        let task = all_tasks.find(id).get_result::<Task>(conn)?;

        let new_status = !task.completed;
        let updated_task = diesel::update(all_tasks.find(id));
        updated_task
            .set(task_completed.eq(new_status))
            .execute(conn)
    }

    pub fn delete_with_id(id: i32, conn: &PgConnection) -> QueryResult<usize> {
        diesel::delete(all_tasks.find(id)).execute(conn)
    }
}
