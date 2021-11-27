extern crate env_logger;

#[macro_use]
extern crate maplit;

use actix_files::Files;
use actix_web::http::header;
use actix_web::http::StatusCode;
use actix_web::middleware::Logger;
use actix_web::middleware::{ErrorHandlerResponse, ErrorHandlers};
use actix_web::web::Bytes;
use actix_web::HttpRequest;
use actix_web::{dev, get, http, post, web, App, HttpResponse, HttpServer, Result};

use handlebars::Handlebars;

use serde::Deserialize;
use std::io;

// Macro documentation can be found in the actix_web_codegen crate
#[get("/")]
async fn index(hb: web::Data<Handlebars<'_>>, _req: HttpRequest, body: Bytes) -> HttpResponse {
    println!("{:?}", _req);
    println!("{:?}", body);
    let data = btreemap! {
        "title".to_string() => "Learn - Splatoon Callouts".to_string(),
        "author".to_string() => "Zageron".to_string(),
        "url".to_string() => "https://www.zageron.com/learn/splatoon".to_string(),
        "description".to_string() => "A Spaced Repetition site for memorizing Splatoon 2 callouts.".to_string(),
        "parent".to_string() => "root".to_string()
    };

    let body = hb.render("subentry", &data).unwrap();

    HttpResponse::Ok().body(body)
}

#[derive(Deserialize, Debug)]
struct Info {
    entry_id: u32,
}

#[get("")]
async fn srs(_hb: web::Data<Handlebars<'_>>) -> HttpResponse {
    HttpResponse::Ok().body("You need to pick an entry to study.")
}

#[get("/{entry_id}")]
async fn srs_entry(_hb: web::Data<Handlebars<'_>>, info: web::Path<Info>) -> HttpResponse {
    HttpResponse::Ok().body(format!("{:?}", info))
    //HttpResponse::Ok().body("You need to pick an entry to study.")
}

#[post("/{entry_id}")]
async fn srs_submit(_hb: web::Data<Handlebars<'_>>, info: web::Path<Info>) -> HttpResponse {
    HttpResponse::Ok().body(format!("{:?}", info))
}

#[get("/copyright")]
async fn copyright(hb: web::Data<Handlebars<'_>>) -> HttpResponse {
    let data = btreemap! {
        "author".to_string() => "Zageron".to_string(),
        "year".to_string() => "2021".to_string()
    };

    let body: String = hb.render("copyright", &data).unwrap();
    HttpResponse::Ok().body(body)
}

#[get("/robots.txt")]
async fn robots(hb: web::Data<Handlebars<'_>>) -> HttpResponse {
    let data = btreemap! {
        "url".to_string() => "https://www.zageron.com".to_string(),
    };

    let body: String = hb.render("robots", &data).unwrap();

    let mut builder = HttpResponse::Ok();
    builder.insert_header(header::ContentType(mime::TEXT_PLAIN_UTF_8));
    builder.body(body)
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    #[cfg(debug_assertions)]
    std::env::set_var("RUST_LOG", "actix_web=info");
    #[cfg(debug_assertions)]
    env_logger::init();

    let mut handlebars = Handlebars::new();

    handlebars.set_dev_mode(cfg!(debug_assertions));

    handlebars
        .register_templates_directory(".hbs", "./templates")
        .unwrap();
    let handlebars_ref = web::Data::new(handlebars);

    HttpServer::new(move || {
        App::new()
            .wrap(ErrorHandlers::new().handler(StatusCode::NOT_FOUND, not_found))
            .wrap(Logger::default())
            .app_data(handlebars_ref.clone())
            .service(
                web::scope("/srs")
                    .service(srs)
                    .service(srs_entry)
                    .service(srs_submit),
            )
            .service(index)
            .service(copyright)
            .service(robots)
            .service(Files::new("/", "./data/web"))
    })
    .bind("127.0.0.1:8081")?
    .run()
    .await
}

// Error handler for a 404 Page not found error.
fn not_found<B>(mut res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    res.response_mut().headers_mut().insert(
        http::header::CONTENT_TYPE,
        http::HeaderValue::from_static("Error"),
    );
    Ok(ErrorHandlerResponse::Response(res))
}
