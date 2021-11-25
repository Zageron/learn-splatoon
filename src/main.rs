extern crate env_logger;

#[macro_use]
extern crate maplit;

use handlebars::Handlebars;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    dbg!("Hello, world!");

    env_logger::init();
    let mut handlebars = Handlebars::new();

    handlebars.register_template_file("site", "./templates/index.hbs")?;
    handlebars.register_template_file("subentry", "./templates/partial_a.hbs")?;

    let data0 = btreemap! {
        "title".to_string() => "Learn - Splatoon Callouts".to_string(),
        "author".to_string() => "Zageron".to_string(),
        "url".to_string() => "https://www.zageron.com/learn/splatoon".to_string(),
        "description".to_string() => "A Spaced Repetition site for memorizing Splatoon 2 callouts.".to_string(),
        "parent".to_string() => "site".to_string()
    };

    println!("{}", handlebars.render("subentry", &data0)?);

    Ok(())
}

#[test]
fn should_fail() {
    unimplemented!();
}
