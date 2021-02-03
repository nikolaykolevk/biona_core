use super::Serialize;

//contains info about the application
#[derive(Serialize, Default, Debug, Clone)]
pub struct Meta {
    pub app_name : String,
    pub tables : Vec <TableMeta>
}

#[derive(Serialize, Default, Debug, Clone)]
pub struct TableMeta {
    pub name : String,
    pub link : String
}