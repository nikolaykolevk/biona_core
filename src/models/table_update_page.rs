use super::{Serialize, Field, TableTrait, Meta};
use std::collections::HashMap;


#[derive(Serialize, Debug, Clone)]
pub struct TableUpdatePage <T : Serialize> {
    pub title : String,
    pub messages : HashMap<String, String>,
    pub table_name : String,
    pub data : T,
    pub found : bool,
    pub fields_list : Vec<Field>,
    pub meta : Meta
}

#[derive(Serialize)]
pub struct RowNotFound {}

pub fn row_not_found(meta : Meta) -> TableUpdatePage<RowNotFound>{
    let mut msgs : HashMap<String, String> = HashMap::new();
    msgs.insert("error".to_string(), "Row not found".to_string());

    TableUpdatePage {
        title : String::from("Row not found"),
        messages : msgs,
        table_name : String::new(),
        data : RowNotFound {},
        found : false,
        fields_list : vec![],
        meta
    }
}


impl<T : Default + Serialize> TableUpdatePage<T> {

    pub fn new() -> TableUpdatePage<T>{
        TableUpdatePage {
            title : String::new(),
            messages : HashMap::new(),
            table_name : String::new(),
            data : T::default(),
            found : true,
            fields_list : vec![],
            meta: Meta { app_name: "".to_string(), tables: vec![] }
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

    pub fn set_data(&mut self, data : T) {
        self.data = data;
    }

    pub fn get_data (&self) -> &T {
        &self.data
    }

    pub fn set_found(&mut self, found : bool) {
        self.found = found;
    }

    pub fn get_found (&self) -> &bool {
        &self.found
    }

    pub fn set_fields_list (&mut self, fields_list : Vec<Field>) {
        self.fields_list = fields_list;
    }

    pub fn get_fields_list (&self) -> &Vec<Field> {
        &self.fields_list
    }

}


impl<SelectStruct : Default + Serialize, PrimKey> From<Box<dyn TableTrait<SelectStruct, PrimKey>>> for TableUpdatePage<SelectStruct> {
    fn from(item: Box<dyn TableTrait<SelectStruct, PrimKey>>) -> Self {
        let mut res = TableUpdatePage::new();
        res.set_table_name( & item.get_table_name());
        res.set_fields_list(item.fields_list());
        res
    }
}


