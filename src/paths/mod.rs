mod generate_table_page_and_data;
mod generate_table_row_update;

use rocket::{Request, Rocket};
use rocket_contrib::serve::StaticFiles;
use rocket::response::NamedFile;
use rocket_contrib::templates::Template;
use crate::models::Page;
use std::collections::HashMap;


//predefined paths to use local libs
pub fn mount_paths(mut r: Rocket) -> Rocket {
    r = r.mount("/src/libs", StaticFiles::from("templates/sources/node_modules"))
        .mount("/src/css", StaticFiles::from("templates/sources/css"))
        .mount("/src/js", StaticFiles::from("templates/sources/js"))
        .mount("/media/", StaticFiles::from("templates/sources/media"))
        .mount("/", routes![favicon, index])
        .register(catchers![not_found]);

    r
}

//favicon responder
#[get("/favicon.ico")]
fn favicon() -> NamedFile {
    let file = NamedFile::open("templates/sources/favicon.ico");
    file.unwrap()
}

//not found responder
#[catch(404)]
fn not_found(req: &Request<'_>) -> Template {
    let mut page: Page<HashMap<&str, &str>> = Page::new();

    page.set_title("Error 404");
    page.details.insert("path", req.uri().path());

    Template::render("error/404", &page)
}

//index responder
#[get("/")]
fn index() -> Template {
    let mut page: Page<HashMap<&str, &str>> = Page::new();
    page.set_title("Index");
    Template::render("index", &page)
}
