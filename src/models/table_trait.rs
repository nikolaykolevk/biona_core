use super::{Serialize, Field, FilterSort, QueryResult};

//TableTrait is used to work with the tables in the database
pub trait TableTrait<SelectStruct : Serialize, PrimKey>
{
    fn new() -> Self where Self: Sized;

    fn new_boxed() -> Box<dyn TableTrait<SelectStruct, PrimKey>> where Self : Sized + 'static{
        Box::from(Self::new())
    }

    fn get_table_name(&self) -> String;

    fn fields_list(&self) -> Vec<Field>;

    fn select_all(&self, display_count : u32, page : u32) -> Vec<SelectStruct>;

    fn select_by_pr_key(&self, find_primary_key: &str) -> Option<SelectStruct>;

    fn filter(&self, filter_sort : FilterSort, display_count : u32, page : u32) -> Vec<SelectStruct>;

    fn count_all (&self) -> u64;

    fn count_filtered (&self, filter_sort : FilterSort) -> u64;

    fn insert (&self, new_data : Vec<Self>) -> Vec<PrimKey> where Self : Sized;

    fn update_by_id(&self, update_data : Self, find_primary_key: &str) -> usize where Self : Sized;

    fn delete_by_id(&self, find_primary_key: &str) -> QueryResult<usize>;


    fn select_by_id_as_json(&self, find_primary_key: &str) -> String where Self : Sized + Serialize{
        serde_json::to_string(&self.select_by_pr_key(find_primary_key)).unwrap()
    }

    fn select_all_as_json(&self, display_count : u32, page:u32) -> String where Self : Sized + Serialize{
        serde_json::to_string(&self.select_all(display_count, page)).unwrap()
    }

    fn filter_as_json(&self, filter_sort : FilterSort , display_count : u32, page:u32) -> String where Self : Sized + Serialize{
        serde_json::to_string(&self.filter(filter_sort, display_count, page)).unwrap()
    }

}