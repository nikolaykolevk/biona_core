mod table_trait;
mod table_page;
mod filter_sort;
mod field;
mod default_date;
mod table_update_page;
mod metainfo;

use serde::{Serialize, Deserialize};
use diesel::QueryResult;

pub use table_trait::TableTrait;
pub use table_page::{TablePage, table_not_found};
pub use table_update_page::{TableUpdatePage, row_not_found};
pub use filter_sort::{FilterSort, Filter, Sort};
pub use field::Field;
pub use metainfo::Meta;
pub use metainfo::TableMeta;
pub use default_date::DefaultDate;
pub use chrono::naive::NaiveDate;

#[derive(Serialize, Default, Debug)]
pub struct Page<T : Default> {
    pub details: T,
    pub meta : Meta,
    title: String,
}


impl<T : Default + Serialize> Page<T> {
    pub fn new() -> Page<T> {
        Page::default()
    }

    pub fn set_title(&mut self, title: &str) {
        self.title = title.to_string();
    }

    pub fn get_title(&self,) -> &str {
        &self.title
    }
}