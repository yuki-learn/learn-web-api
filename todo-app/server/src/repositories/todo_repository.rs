use diesel::prelude::*;
use diesel::pg::PgConnection;
use crate::{models::{NewTodo, Todo}, schema::todos};

type DbError = Box<dyn std::error::Error + Send + Sync>;

/// 指定されたidのtodoを返す
pub fn find_by_todos(conn: &PgConnection, todo_id: &i32) -> Result<Option<Todo>, DbError> {
    use crate::schema::todos::dsl::*;
    let result = todos.find(todo_id).first::<Todo>(conn).optional()?;

    Ok(result)
}

/// 全てのtodoを返す
pub fn find_all_todos(conn: &PgConnection) -> Result<Option<Vec<Todo>>, DbError> {
    use crate::schema::todos::dsl::*;
    let result = todos.load::<Todo>(conn).optional()?;

    Ok(result)
}

/// 新しいtodoを追加する
pub fn add_new_todo(conn: &PgConnection, value: &str) -> Result<Todo, DbError> {
    let new_todo = NewTodo {
        value,
    };

    let todo = diesel::insert_into(todos::table)
        .values(&new_todo)
        .get_result(conn)?;

    Ok(todo)
}

/// todoを更新する
pub fn update_todo(conn: &PgConnection, id: &i32, todo: Todo) -> Result<Todo, DbError> {
    use crate::schema::todos::dsl::{todos, value};
    
    let todo = diesel::update(todos.find(id))
        .set(value.eq(todo.value))
        .get_result::<Todo>(conn)?;

    Ok(todo)
}

/// todoを削除する
pub fn delete_todo(conn: &PgConnection, id: i32) -> Result<usize, DbError> {
    use crate::schema::todos::dsl::{todos};

    let result = diesel::delete(todos.find(id))
        .execute(conn)
        .expect("Error deleting posts");

    Ok(result)
}