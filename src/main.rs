mod graphql;

use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql::{EmptySubscription, Schema};
use async_std::task;
use graphql::{MutationRoot, QueryRoot};
use sqlx::{Pool, Postgres};
use std::env;
use tide::{http::mime, Body, Response, StatusCode};
type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[derive(Clone, Copy)]
struct AppState {
    schema: &'static Schema<QueryRoot, MutationRoot, EmptySubscription>,
    pool: &'static Pool<Postgres>,
}

fn main() -> Result<()> {
    task::block_on(run())
}

async fn run() -> Result<()> {
    let listen_addr = env::var("LISTEN_ADDR").unwrap_or_else(|_| "localhost:8000".to_owned());
    let pool: Pool<Postgres> = Pool::connect(&env::var("DATABASE_URL")?).await?;

    let schema = Schema::build(QueryRoot, MutationRoot, EmptySubscription).finish();

    println!("Playground: http://{}", listen_addr);

    //let app_state: AppState = AppState { schema: &schema, pool: &pool };
    //let mut app = tide::with_state(&app_state);
    let mut app = tide::new();

    /*
    async fn graphql(req: Request<AppState>) -> tide::Result<Response> {
        let schema = req.state().schema.clone();
        async_graphql_tide::graphql(req, schema, |query_builder| query_builder).await
    }
    */

    app.at("/graphql").post(async_graphql_tide::endpoint(schema));
    
    app.at("/").get(|_| async move {
        let mut resp = Response::new(StatusCode::Ok);
        resp.set_body(Body::from_string(playground_source(
            GraphQLPlaygroundConfig::new("/graphql"),
        )));
        resp.set_content_type(mime::HTML);
        Ok(resp)
    });

    app.listen(listen_addr).await?;

    Ok(())
}
