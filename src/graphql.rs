use async_graphql::{FieldError, Result, SimpleObject, Context};
use sqlx::{Pool, Postgres, FromRow};
use async_graphql::dataloader::{DataLoader, Loader};
use async_trait::async_trait;
use std::collections::HashMap;

#[derive(FromRow, Clone, SimpleObject)]
pub struct Item {
    id: i32,
    todo_list_id: i32,
    name: String,
    due_date: String,
}

#[derive(FromRow, Clone, SimpleObject)]
pub struct TodoList {
    id: i32,
    name: String,
    text: String,
    done: bool,
}

pub struct TodoListLoader(Pool<Postgres>);
/*
impl TodoListLoader {
    fn new(postgres_pool: Pool<Postgres>) -> Self {
        Self(postgres_pool)
    }
}
*/


#[async_trait]
impl Loader<i32> for TodoListLoader {
    type Value = TodoList;
    type Error = FieldError;

    async fn load(&self, _keys: &[i32]) -> Result<HashMap<i32, Self::Value>, Self::Error> {
        Ok(sqlx::query_as!(TodoList, "SELECT * FROM todo_lists")
           .fetch_all(&self.0)
           .await?
           .iter()
           .map(|todo: &TodoList| (todo.id, todo.clone()))
           .into_iter()
           .collect()
          )
    }
}

pub struct QueryRoot;

#[async_graphql::Object]
impl QueryRoot {
    async fn todos(&self, ctx: &Context<'_>) -> Option<Vec<TodoList>> {
       Some(ctx
            .data_unchecked::<DataLoader<TodoListLoader>>()
            .load_many(vec![1])
            .await.ok()?
            .values()
            .cloned()
            .collect::<Vec<TodoList>>())

    }

    async fn todo(&self, ctx: &Context<'_>) -> Option<TodoList> {
        ctx
           .data_unchecked::<DataLoader<TodoListLoader>>()
           .load_one(1)
           .await.ok()?
    }
}

pub struct MutationRoot;

#[async_graphql::Object]
impl MutationRoot {
    async fn create_todo(&self, _ctx: &Context<'_>) -> TodoList {
        TodoList {
            id: 1,
            name: "foo".to_string(),
            text: "bar".to_string(),
            done: false,
        }
    }

    async fn add_items(&self, _ctx: &Context<'_>) -> TodoList {
        TodoList {
            id: 1,
            name: "foo".to_string(),
            text: "bar".to_string(),
            done: false,
        }
    }
}
