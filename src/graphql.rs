use async_graphql::{Context, ID};

pub struct Item {
    id: ID,
    todo_list_id: ID,
    name: String,
    due_date: String,
}

#[async_graphql::Object]
impl Item {
    async fn id(&self) -> &str {
        &self.id
    }

    async fn todo_list_id(&self) -> &str {
        &self.todo_list_id
    }

    async fn name(&self) -> &str {
        &self.name
    }

    async fn due_date(&self) -> &str {
        &self.due_date
    }
}

pub struct TodoList {
    id: ID,
    name: String,
    text: String,
    done: bool,
    items: Option<Vec<Option<Item>>>,
}

#[async_graphql::Object]
impl TodoList {
    async fn id(&self) -> &str {
        &self.id
    }

    async fn name(&self) -> &str {
        &self.name
    }

    async fn text(&self) -> &str {
        &self.text
    }

    async fn done(&self) -> &bool {
        &self.done
    }

    async fn items(&self) -> &Option<Vec<Option<Item>>> {
        &self.items
    }
}

pub struct QueryRoot;

#[async_graphql::Object]
impl QueryRoot {
    async fn todos(&self, _ctx: &Context<'_>) -> Vec<TodoList> {
        vec![]
    }

    async fn todo(&self, _ctx: &Context<'_>) -> TodoList {
        TodoList {
            id: ID::from("1"),
            name: "foo".to_string(),
            text: "bar".to_string(),
            done: false,
            items: Some(vec![]),
        }
    }
}

pub struct MutationRoot;

#[async_graphql::Object]
impl MutationRoot {
    async fn create_todo(&self, _ctx: &Context<'_>) -> TodoList {
        TodoList {
            id: ID::from("1"),
            name: "foo".to_string(),
            text: "bar".to_string(),
            done: false,
            items: Some(vec![]),
        }
    }

    async fn add_items(&self, _ctx: &Context<'_>) -> TodoList {
        TodoList {
            id: ID::from(1),
            name: "foo".to_string(),
            text: "bar".to_string(),
            done: false,
            items: Some(vec![]),
        }
    }
}
