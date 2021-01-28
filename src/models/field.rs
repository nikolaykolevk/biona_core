use super::Serialize;

//Field struct is used to store the properties of each field
#[derive(Serialize, Default, Debug, Clone)]
pub struct Field {
    pub priority : usize,
    pub display_name : String,
    pub name : String,
    pub filterable : bool,
    pub updatable : bool,
    pub insertable : bool,
    pub sortable : bool,
    pub hidden : bool,
    pub is_primary_key : bool,
    pub field_type : String,
    pub field_schema : String,
    pub from_system_table : bool
}
