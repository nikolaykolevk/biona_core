use super::{Serialize, Field, TableTrait, Meta};
use std::collections::HashMap;

//TablePage is used to init a table template
#[derive(Serialize, Debug, Clone)]
pub struct TablePage<T : Serialize> {
    pub title : String,
    pub messages : HashMap<String, String>,
    pub table_name : String,
    pub data : Vec<T>,
    pub data_count : u64,
    pub max_page : u64,
    pub fields_list : Vec<Field>,
    pub meta : Meta
}

#[derive(Serialize)]
pub struct TableNotFound {}

//returns am empty page
pub fn table_not_found(meta : Meta) -> TablePage<TableNotFound>{
    let mut msgs : HashMap<String, String> = HashMap::new();
    msgs.insert("error".to_string(), "Table not found".to_string());

    TablePage{
        title : String::from("Table not found"),
        messages : msgs,
        table_name : String::new(),
        data : vec![],
        data_count : 0,
        max_page: 1,
        fields_list : vec![],
        meta
    }
}


impl<T : Default + Serialize> TablePage<T> {

    pub fn new() -> TablePage<T> {
        TablePage{
            title : String::new(),
            messages : HashMap::new(),
            table_name : String::new(),
            data : vec![],
            data_count : 0,
            max_page: 1,
            fields_list : vec![],
            meta : Meta {
                app_name: "".to_string(),
                tables: vec![]
            }
        }
    }

    pub fn set_title (&mut self, title : &str) {
        self.title = title.to_string();
    }

    pub fn get_title (&self) -> &str {
        &self.title
    }

    pub fn set_meta(&mut self, meta : Meta) {
        self.meta = meta;
    }

    pub fn set_table_name(&mut self, table_name : &str) {
        self.table_name = table_name.to_string();
    }

    pub fn get_table_name (&self) -> &str {
        &self.table_name
    }

    pub fn set_data(&mut self, data : Vec<T>) {
        self.data = data;
    }

    pub fn get_data (&self) -> &Vec<T> {
        &self.data
    }

    pub fn set_data_count(&mut self, data_count : u64) {
        self.data_count = data_count;
    }

    pub fn get_data_count (&self) -> u64 {
        self.data_count
    }

    pub fn set_max_page (&mut self, max_page : u64) {
        self.max_page = max_page;
    }

    pub fn get_max_page (&self) -> u64 {
        self.max_page
    }

    pub fn set_fields_list (&mut self, fields_list : Vec<Field>) {
        self.fields_list = fields_list;
    }

    pub fn get_fields_list (&self) -> &Vec<Field> {
        &self.fields_list
    }

}

//creates new TablePage from a struct implementing TableTrait
impl<SelectStruct : Default + Serialize, PrimKey> From<Box<dyn TableTrait<SelectStruct, PrimKey>>> for TablePage<SelectStruct> {
    fn from(item: Box<dyn TableTrait<SelectStruct, PrimKey>>) -> Self {
        let mut res = TablePage::new();
        res.set_table_name( & item.get_table_name());
        res.set_fields_list(item.fields_list());
        res
    }
}
