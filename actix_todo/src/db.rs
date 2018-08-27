use std::ops::Deref;

use actix::prelude::{Actor, Handler, Message, SyncContext};
use actix_web::{error, Error};
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PoolError, PooledConnection};

use model::{NewTask, Task};

type PgPool = Pool<ConnectionManager<PgConnection>>;
type PgPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

pub fn init_pool(database_url: &str) -> Result<PgPool, PoolError> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder().build(manager)
}

pub struct DbExecutor(pub PgPool);

impl DbExecutor {
    pub fn get_conn(&self) -> Result<PgPooledConnection, Error> {
        self.0.get().map_err(|e| error::ErrorInternalServerError(e))
    }
}

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}

pub struct AllTasks;

impl Message for AllTasks {
    type Result = Result<Vec<Task>, Error>;
}

impl Handler<AllTasks> for DbExecutor {
    type Result = Result<Vec<Task>, Error>;

    fn handle(&mut self, _: AllTasks, _: &mut Self::Context) -> Self::Result {
        Task::all(self.get_conn()?.deref())
            .map_err(|_| error::ErrorInternalServerError("Error inserting task"))
    }
}

pub struct CreateTask {
    pub description: String,
}

impl Message for CreateTask {
    type Result = Result<(), Error>;
}

impl Handler<CreateTask> for DbExecutor {
    type Result = Result<(), Error>;

    fn handle(&mut self, todo: CreateTask, _: &mut Self::Context) -> Self::Result {
        let new_task = NewTask {
            description: todo.description,
        };
        Task::insert(new_task, self.get_conn()?.deref())
            .map(|_| ())
            .map_err(|_| error::ErrorInternalServerError("Error inserting task"))
    }
}

pub struct ToggleTask {
    pub id: i32,
}

impl Message for ToggleTask {
    type Result = Result<(), Error>;
}

impl Handler<ToggleTask> for DbExecutor {
    type Result = Result<(), Error>;

    fn handle(&mut self, task: ToggleTask, _: &mut Self::Context) -> Self::Result {
        Task::toggle_with_id(task.id, self.get_conn()?.deref())
            .map(|_| ())
            .map_err(|_| error::ErrorInternalServerError("Error inserting task"))
    }
}

pub struct DeleteTask {
    pub id: i32,
}

impl Message for DeleteTask {
    type Result = Result<(), Error>;
}

impl Handler<DeleteTask> for DbExecutor {
    type Result = Result<(), Error>;

    fn handle(&mut self, task: DeleteTask, _: &mut Self::Context) -> Self::Result {
        Task::delete_with_id(task.id, self.get_conn()?.deref())
            .map(|_| ())
            .map_err(|_| error::ErrorInternalServerError("Error inserting task"))
    }
}
