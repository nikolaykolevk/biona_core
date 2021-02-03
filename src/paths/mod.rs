mod generate_table_page_and_data;
mod generate_table_row_update;



#[macro_export]
macro_rules! load_rocket {
    ($path_to_templates : expr, $meta : expr) => {

        use rocket::config::{Config, Environment};
        use rocket::{Request, Rocket};
        use rocket_contrib::serve::StaticFiles;
        use rocket::response::NamedFile;
        use rocket_contrib::templates::Template;
        use biona_core::models::Page;
        use std::collections::HashMap;

        // static PATH_TO_TEMPLATES: &str = "../biona_templates";
        static PATH_TO_TEMPLATES: &str = $path_to_templates;

        pub fn rocket() -> Rocket {

            let config = Config::build(Environment::Development).extra("template_dir", $path_to_templates).address("0.0.0.0").unwrap();

            rocket::custom(config).attach(Template::fairing()).mount("/src/libs", StaticFiles::from(format!("{}{}", PATH_TO_TEMPLATES, "/sources/node_modules")))
                .mount("/src/css", StaticFiles::from(format!("{}{}", PATH_TO_TEMPLATES, "/sources/css")))
                .mount("/src/js", StaticFiles::from(format!("{}{}", PATH_TO_TEMPLATES, "/sources/js")))
                .mount("/media/", StaticFiles::from(format!("{}{}", PATH_TO_TEMPLATES, "/sources/media")))
                .mount("/", routes![favicon, index, home_page])
                .register(catchers![not_found])
        }

        //favicon responder
        #[get("/favicon.ico")]
        fn favicon() -> NamedFile {
            let file = NamedFile::open(format!("{}{}", PATH_TO_TEMPLATES, "/sources/favicon.ico"));
            file.unwrap()
        }

        //not found responder
        #[catch(404)]
        fn not_found(req: &Request<'_>) -> Template {
            let mut page: Page<HashMap<&str, &str>> = Page::new();

            page.set_title("Error 404");
            page.details.insert("path", req.uri().path());
            page.meta = $meta;

            Template::render("error/404", &page)
        }

        //index responder
        #[get("/")]
        fn index() -> Template {
            let mut page: Page<HashMap<&str, &str>> = Page::new();
            page.set_title("Index");
            page.meta = $meta;
            Template::render("index", &page)
        }

        #[get("/home")]
        fn home_page() -> Template {
            let mut page: Page<HashMap<&str, &str>> = Page::new();
            page.set_title("Index");
            page.meta = $meta;
            Template::render("home", &page)
        }

    };
}