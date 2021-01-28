// biona core generates a responder for the page and the data of the page
// uses update templates
// used path: /<table>/update/<id>
// table: name of the table (first argument of the macros)
// id: id of the row in the table
#[macro_export]
macro_rules! generate_row_update {
    ($($match_case : expr, $struct : ident),*) => {

        #[get("/<table>/update/<id>")]
        pub fn table_update_page(table : String, id : String) -> biona_core::rocket_contrib::templates::Template {

        use biona_core::models::*;

                return match table.to_lowercase().as_str() {
                    $($match_case => {
                        let table : Box<dyn TableTrait<_, _>> = <$struct>::new_boxed();
                        let mut table_update_page : biona_core::models::TableUpdatePage<_> = <$struct>::new_boxed().into();
                        if let Some(data) = table.select_by_pr_key(id.as_str()) {
                            table_update_page.set_data(data);
                        } else {
                            table_update_page.set_found(false);
                        }

                        biona_core::rocket_contrib::templates::Template::render("update/page", &table_update_page)
                    },
                    )*
                    _ => {
                        biona_core::rocket_contrib::templates::Template::render("update/row_not_found", &biona_core::models::row_not_found())
                    }
                }
            }

    }
}