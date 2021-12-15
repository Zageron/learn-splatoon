#[macro_use]
extern crate maplit;
extern crate dotenv;
extern crate env_logger;

mod authenticate;
mod database;

//use authenticate::KratosIdentity;
use database::connect;

use actix_files::Files;
use actix_http::body::BoxBody;
use actix_web::{
    dev::ServiceResponse,
    guard,
    http::{header, StatusCode},
    middleware::{ErrorHandlerResponse, ErrorHandlers, Logger, NormalizePath, TrailingSlash},
    web, App, HttpRequest, HttpResponse, HttpServer, Result,
};
use dotenv::dotenv;
use handlebars::Handlebars;
use mongodb::bson::doc;
//use ory_kratos_client::apis::configuration::Configuration as KratosConfiguration;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{
    collections::{BTreeMap, HashMap},
    error::Error,
    fs::File,
    io::{self, BufReader},
    path::Path,
};

#[derive(Clone, Debug, Deserialize)]
struct RootData {
    data: BTreeMap<String, String>,
}

#[derive(Clone, Debug, Deserialize)]
struct LearningData {
    data: BTreeMap<String, String>,
}

// Macro documentation can be found in the actix_web_codegen crate
async fn index(
    hb: web::Data<Handlebars<'_>>,
    root_template_data: web::Data<RootData>,
    _req: HttpRequest,
) -> HttpResponse {
    let body = hb.render("pages/index", &root_template_data.data).unwrap();
    HttpResponse::Ok().body(body)
}

#[derive(Deserialize, Debug)]
struct Info {
    entry_id: u32,
}

async fn study(_hb: web::Data<Handlebars<'_>>) -> HttpResponse {
    HttpResponse::Ok().body("You need to pick an entry to study.")
}

async fn study_entry(_hb: web::Data<Handlebars<'_>>, info: web::Path<Info>) -> HttpResponse {
    println!("Entry {:?}", info.entry_id);
    HttpResponse::Ok().body(format!("{:?}", info))
}

async fn study_submit(_hb: web::Data<Handlebars<'_>>, info: web::Path<Info>) -> HttpResponse {
    HttpResponse::Ok().body(format!("{:?}", info))
}

async fn learning_entry(
    hb: web::Data<Handlebars<'_>>,
    root_template_data: web::Data<RootData>,
    learning_data: web::Data<LearningData>,
) -> HttpResponse {
    let mut data: BTreeMap<&String, &String> = BTreeMap::new();
    data.extend(root_template_data.data.iter());
    data.extend(learning_data.data.iter());

    let body = hb.render("pages/learn", &data).unwrap();
    println!("{}", body);
    HttpResponse::Ok().body(body)
}

async fn copyright(hb: web::Data<Handlebars<'_>>) -> HttpResponse {
    let data = btreemap! {
        "author".to_string() => "Zageron".to_string(),
        "year".to_string() => "2021".to_string()
    };

    let body: String = hb.render("copyright", &data).unwrap();
    HttpResponse::Ok().body(body)
}

async fn robots(hb: web::Data<Handlebars<'_>>) -> HttpResponse {
    let data = btreemap! {
        "url".to_string() => "https://www.zageron.com".to_string(),
    };

    let body: String = hb.render("robots", &data).unwrap();

    let mut builder = HttpResponse::Ok();
    builder.insert_header(header::ContentType(mime::TEXT_PLAIN_UTF_8));
    builder.body(body)
}

#[derive(Serialize, Deserialize, Debug)]
struct LanguageLookup {
    #[serde(flatten)]
    inner: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct MainWeapons {
    addition: u32,
    id: u32,
    ink_saver_lv: String,
    lock: String,
    main_up_gear_power_type: String,
    move_vel_lv: String,
    name: String,
    param0: String,
    param1: String,
    param2: String,
    param_value0: u32,
    param_value1: u32,
    param_value2: u32,
    price: u32,
    range: u32,
    rank: u32,
    shot_move_vel_type: String,
    special: String,
    special_cost: u32,
    stealth_move_acc_lv: String,
    sub: String,
}

fn read_language_file<P: AsRef<Path>>(path: P) -> Result<LanguageLookup, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let language: LanguageLookup =
        serde_json::from_reader(reader).expect("JSON was not well-formatted.");
    Ok(language)
}

fn read_mains_file<P: AsRef<Path>>(path: P) -> Result<Vec<MainWeapons>, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mains: Vec<MainWeapons> =
        serde_json::from_reader(reader).expect("JSON was not well-formatted.");
    Ok(mains)
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv().ok();

    #[cfg(debug_assertions)]
    std::env::set_var("RUST_LOG", "actix_web=info");
    #[cfg(debug_assertions)]
    env_logger::init();

    let mut handlebars = Handlebars::new();

    handlebars.set_dev_mode(cfg!(debug_assertions));

    handlebars
        .register_templates_directory(".hbs", "templates/")
        .unwrap();
    let handlebars_ref = web::Data::new(handlebars);

    let _language_file = read_language_file("./data/splatoon/english.json");
    let _mains_file = read_mains_file("./data/splatoon/mains.json");

    let _base_path = std::env::var("ORY_SDK_URL").expect("ORY_SDK_URL is not set.");

    let connection_result = connect().await;
    let connection_success = connection_result.is_ok();
    println!("Connected to Database: {:?}", connection_success);

    let root_template_data = RootData {
        data: btreemap! {
            "title".to_string() => "Learn - Splatoon Callouts".to_string(),
            "author".to_string() => "Zageron".to_string(),
            "url".to_string() => "https://www.zageron.com/learn/splatoon".to_string(),
            "description".to_string() => "A Spaced Repetition site for memorizing Splatoon 2 callouts.".to_string(),
            "parent".to_string() => "root".to_string()
        },
    };

    let learning_data = LearningData {
        data: btreemap! {
            "item_header".to_string() => "Test Item".to_string(),
            "item_title".to_string() => "Sploosh".to_string(),
            "item_description".to_string() => "Splooshes be splooshing.".to_string(),
            "item_footer".to_string() => "You've learned this already.".to_string(),
        },
    };

    HttpServer::new(move || {
        App::new()
            .app_data(handlebars_ref.clone())
            .app_data(web::Data::new(connection_result.clone()))
            .app_data(web::Data::new(root_template_data.clone()))
            .app_data(web::Data::new(learning_data.clone()))
            // .wrap(KratosIdentity {
            //     configuration: KratosConfiguration {
            //         base_path: base_path.clone(),
            //         ..Default::default()
            //     },
            //})
            .wrap(NormalizePath::new(TrailingSlash::Trim))
            .wrap(error_handlers())
            .wrap(Logger::default())
            .service(
                web::scope("/learn").service(
                    web::resource(["/", ""])
                        .guard(guard::Get())
                        .to(learning_entry),
                ),
            )
            .service(
                web::scope("/study")
                    .service(web::resource(["/", ""]).guard(guard::Get()).to(study))
                    .service(
                        web::resource("{entry_id}")
                            .guard(guard::Get())
                            .to(study_entry),
                    )
                    .service(
                        web::resource("{entry_id}")
                            .guard(guard::Post())
                            .to(study_submit),
                    ),
            )
            .service(web::resource("/").guard(guard::Get()).to(index))
            .service(web::resource("copyright").guard(guard::Get()).to(copyright))
            .service(web::resource("robots.txt").guard(guard::Get()).to(robots))
            .service(Files::new("/", "./data/web"))
    })
    .bind("0.0.0.0:8081")?
    .run()
    .await
}

// Custom error handlers, to return HTML responses when an error occurs.
fn error_handlers() -> ErrorHandlers<BoxBody> {
    ErrorHandlers::new().handler(StatusCode::NOT_FOUND, not_found)
}

// Error handler for a 404 Page not found error.
fn not_found<B>(res: ServiceResponse<B>) -> Result<ErrorHandlerResponse<BoxBody>> {
    let response = get_error_response(&res, "Page not found");
    Ok(ErrorHandlerResponse::Response(res.into_response(response)))
}

// Generic error handler.
fn get_error_response<B>(res: &ServiceResponse<B>, error: &str) -> HttpResponse<BoxBody> {
    let request = res.request();

    // Provide a fallback to a simple plain text response in case an error occurs during the
    // rendering of the error page.
    let fallback = |e: &str| {
        HttpResponse::build(res.status())
            .content_type("text/plain")
            .body(e.to_string())
    };

    let hb = request
        .app_data::<web::Data<Handlebars>>()
        .map(|t| t.get_ref());
    match hb {
        Some(hb) => {
            let data = json!({
                "error": error,
                "status_code": res.status().as_str(),
                "page": request.uri().to_string()
            });
            let body = hb.render("404", &data);

            match body {
                Ok(body) => HttpResponse::build(res.status())
                    .content_type("text/html")
                    .body(body),
                Err(_) => fallback(error),
            }
        }
        None => fallback(error),
    }
}

#[cfg(test)]
mod tests {
    use crate::read_language_file;
    use crate::read_mains_file;
    use crate::MainWeapons;

    #[test]
    fn language_english_exists() {
        assert!(read_language_file("./data/splatoon/english.json").is_ok());
    }

    #[test]
    fn mains_exists() {
        assert!(read_mains_file("./data/splatoon/mains.json").is_ok());
    }

    #[test]
    fn test_langauge_lookup() {
        // Testing out splatoon weaponry stuff.
        let english_lang = read_language_file("./data/splatoon/english.json").unwrap();
        let main_weapons = read_mains_file("./data/splatoon/mains.json").unwrap();

        let main: &MainWeapons = &main_weapons[0];
        let name: &String = &english_lang.inner[&main.name];
        assert_eq!(name, "Sploosh-o-matic");
    }
}
