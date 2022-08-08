use std::vec;

use actix_web::{web, HttpResponse, Error, ResponseError, http::{StatusCode, header::ContentType} };
use diesel::{r2d2::{ConnectionManager, self}, PgConnection};
use serde::{Deserialize, Serialize};
use crate::{repositories::todo_repository::{find_all_todos, update_todo, add_new_todo, find_by_todos}, models::Todo};

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Debug, Serialize)]
enum MyError {
    Error(Vec<String>),
    NotExists(Vec<String>)
}

#[derive(Debug, Serialize)]
struct ErrorResponse<'a> {
    error_messages: &'a Vec<String>
}

impl std::fmt::Display for MyError {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl ResponseError for MyError {
    fn error_response(&self) -> HttpResponse {
        let error_messages = match self {
            Self::Error(e) => e,
            Self::NotExists(e) => e,
        };

        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .json(ErrorResponse { error_messages: &error_messages })
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            MyError::Error(_) => StatusCode::BAD_REQUEST,
            MyError::NotExists(_) => StatusCode::NOT_FOUND,
        }
    }
}

#[derive(Deserialize)]
pub struct NewTodo {
    value: String
}

async fn validate_todo(todo: &Todo) -> Result<(), MyError> {
    let errors = todo.execute_validation();

    match errors {
        Some(e) => Err(MyError::Error(e)),
        None => Ok(())
    }
}

pub async fn find_by(pool: web::Data<DbPool>, id: web::Path<i32>) -> Result<HttpResponse, Error> {
    let todo = web::block(move || {
        let conn = pool.get()?;
        find_by_todos(&conn, &id.into_inner())
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    match todo {
        Some(t) => Ok(HttpResponse::Ok().json(t)),
        None =>    Err(Error::from(MyError::NotExists(vec![String::from("存在しないIDです。")]))),
    }
}

pub async fn find_all(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let todos = web::block(move || {
        let conn = pool.get()?;
        find_all_todos(&conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    // id順にソート
    let sorted_todos: Vec<Todo> = match todos {
        Some(mut s) => {
            s.sort_by(|a, b| a.id.cmp(&b.id));
            s
        },
        None => Vec::new()
    };

    Ok(HttpResponse::Ok().json(sorted_todos))
}

pub async fn post_todo(pool: web::Data<DbPool>, request: web::Json<NewTodo>) -> Result<HttpResponse, Error> {
    let s = String::from(&request.0.value);
    validate_todo(&Todo::new(s)).await?;

    let todo = web::block(move || {
        let conn = pool.get()?;
        add_new_todo(&conn, &request.value)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;
    
    Ok(HttpResponse::Created().json(todo))
}

pub async fn put_todo(pool: web::Data<DbPool>, id: web::Path<i32>, request: web::Json<NewTodo>) -> Result<HttpResponse, Error> {
    let s = String::from(&request.0.value);
    let after_todo = Todo::new(s);
    validate_todo(&after_todo).await?;

    let todo = web::block(move || {
        let conn = pool.get()?;
        let todo_id = &id.into_inner();
        update_todo(&conn, todo_id, after_todo)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;
    
    Ok(HttpResponse::Ok().json(todo))
}

pub async fn delete_todo(pool: web::Data<DbPool>, id: web::Path<i32>) -> Result<HttpResponse, Error> {
    use crate::repositories::todo_repository::delete_todo;

    web::block(move || {
        let conn = pool.get()?;
        delete_todo(&conn, id.into_inner())
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::NoContent().json(""))
}