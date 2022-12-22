use actix_web::{guard, web, web::Data, App, HttpResponse, HttpServer, Result};
use async_graphql::http::graphiql_source;
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use prisma_client_rust_actix_web_async_graphql::features::graphql::{build_schema, AppSchema};

async fn index(schema: web::Data<AppSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn graphql_playground() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(graphiql_source("/", None)))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let schema = build_schema().await;
    println!("GraphiQL: http://localhost:8000");
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(schema.clone()))
            .service(web::resource("/").guard(guard::Post()).to(index))
            .service(
                web::resource("/")
                    .guard(guard::Get())
                    .to(graphql_playground),
            )
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
