//biona core generates a responder for the page and the data of the page
//uses table templates
//used path: /<table>?<index>&<dc>&<f_names>&<f_op>&<f_values>&<f_or>&<s_name>&<s_asc> GET
//table: name of the table (first argument of the macros)
//index: n-th page
//dc: number of rows to display
//f_names: COMMA SEPARATED STRINGS names of the rows to filter (in the same order)
//f_values: COMMA SEPARATED STRINGS values of the rows to filter (in the same order)
//f_or: COMMA SEPARATED STRINGS whether to use logical OR or AND for every filter (in the same order)
//s_name: STRING name of the row to filter by
//s_asc: BOOL ascending or descending order

#[macro_export]
macro_rules! generate_table_page_and_table_data {
    ($($match_case : expr, $struct : ident, $title : expr ),*) => {

        #[get("/<table>?<index>&<dc>&<f_names>&<f_op>&<f_values>&<f_or>&<s_name>&<s_asc>")]
        pub fn table_page(table : String, dc : Option<String>, index : Option<String>, f_names : Option<String>, f_op : Option<String>, f_values : Option<String>, f_or : Option<String>, s_name : Option<String>, s_asc : Option<String>) -> biona_core::rocket_contrib::templates::Template {

            use biona_core::models::*;

            let filter_sort = FilterSort::from_option_string(f_names, f_op, f_values, f_or, s_name, s_asc);
            let index: u32 = index.unwrap_or_else(|| "1".to_string()).parse().unwrap_or_else(|_| 1);
            let display_count : u32 = dc.unwrap_or_else(|| "50".to_string()).parse().unwrap_or_else(|_| 50);

            let mut page = 1;
            if (index > 0) {
                page = index;
            }

            let max_page;



                return match table.to_lowercase().as_str() {
                    $($match_case => {
                        let table : Box<dyn TableTrait<_, _>> = <$struct>::new_boxed();
                        let mut table_page: TablePage<_> = <$struct>::new_boxed().into();
                        table_page.set_data(table.filter(filter_sort.clone() ,display_count, page));
                        let data_count = table.count_filtered(filter_sort);
                        if (data_count% (display_count as u64) == (0 as u64)) {
                            max_page = data_count / (display_count as u64);
                        }
                        else {
                            max_page = data_count / (display_count as u64) + (1 as u64);
                        }

                        table_page.set_max_page(max_page);
                        table_page.set_data_count(data_count);
                        table_page.set_title($title);
                        biona_core::rocket_contrib::templates::Template::render("table/page", &table_page)
                    },
                    )*
                    _ => {
                        biona_core::rocket_contrib::templates::Template::render("table/not_found", &biona_core::models::table_not_found())
                    }
                }
            }

        #[get("/<table>/data?<index>&<dc>&<f_names>&<f_op>&<f_values>&<f_or>&<s_name>&<s_asc>")]
        pub fn table_data(table : String, dc : Option<String>, index : Option<String>, f_names : Option<String>, f_op : Option<String>, f_values : Option<String>, f_or : Option<String>, s_name : Option<String>, s_asc : Option<String>) -> biona_core::rocket_contrib::templates::Template {

            use biona_core::models::*;

            let filter_sort = FilterSort::from_option_string(f_names, f_op, f_values, f_or, s_name, s_asc);
            let index: u32 = index.unwrap_or_else(|| "1".to_string()).parse().unwrap_or_else(|_| 1);
            let display_count : u32 = dc.unwrap_or_else(|| "50".to_string()).parse().unwrap_or_else(|_| 50);

            let mut page = 1;
            if (index > 0) {
                page = index;
            }

            let max_page;



                return match table.to_lowercase().as_str() {
                    $($match_case => {
                        let table : Box<dyn TableTrait<_, _>> = <$struct>::new_boxed();
                        let mut table_page: TablePage<_> = <$struct>::new().into();
                        table_page.set_data(table.filter(filter_sort.clone() ,display_count, page));
                        let data_count = table.count_filtered(filter_sort);
                        if (data_count% (display_count as u64) == (0 as u64)) {
                            max_page = data_count / (display_count as u64);
                        }
                        else {
                            max_page = data_count / (display_count as u64) + (1 as u64);
                        }

                        table_page.set_max_page(max_page);
                        table_page.set_data_count(data_count);
                        table_page.set_title($title);
                        biona_core::rocket_contrib::templates::Template::render("table/table", &table_page)
                    },
                    )*
                    _ => {
                        let table_page: TablePage<_> = table_not_found();
                        biona_core::rocket_contrib::templates::Template::render("table/table", &table_page)
                    }
                }
        }
    }
}