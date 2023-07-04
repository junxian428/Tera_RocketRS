use rocket::{catch, catchers, get, routes, launch};
use rocket_dyn_templates::Template;
use rocket_dyn_templates::serde::Serialize;
use tera::Context;

#[derive(Serialize)]
struct IndexContext {
    name: &'static str,
}

#[get("/")]
fn index() -> Template {
    let context = IndexContext { name: "John Doe" };

    Template::render("index", &context)
}

#[catch(default)]
fn error_catcher() -> &'static str {
    "An error occurred."
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .register("/", catchers![error_catcher])
        .attach(Template::fairing())
        .manage(tera::Tera::new("templates/**").unwrap())
}