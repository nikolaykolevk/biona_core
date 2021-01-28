use super::{Deserialize, Serialize};

//Used to construct a single filter
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Filter {
    pub field_name : String,
    pub op : String,
    pub r_value : String,
    pub or : bool
}

//Used to construct a sort
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Sort {
    pub field_name : String,
    pub order_asc : bool
}

//combines sort with multiple filters
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct FilterSort {
    pub filters : Vec<Filter>,
    pub sort: Option<Sort>,
}


impl FilterSort {

    pub fn new () -> FilterSort {
        FilterSort {
            filters: vec![],
            sort: None
        }
    }

    //creates a new FilterSort from comma separated Strings in an Option
    pub fn from_option_string (f_names : Option<String>, f_op : Option<String>, f_values : Option<String>, f_or : Option<String>, s_name : Option<String>, s_asc : Option<String>) -> FilterSort {

        let mut filter_err = false;
        let mut fields_none = 0;

        let f_names : String = f_names.unwrap_or_else(|| {
            filter_err = true;
            fields_none +=1;
            "".into() }).split_whitespace().collect::<String>();
        let f_names : Vec<&str> = f_names.split(",").collect::<Vec<&str>>();

        let f_op : String = f_op.unwrap_or_else(|| {
            filter_err = true;
            fields_none +=1;
            "".into() }).split_whitespace().collect::<String>();
        let f_op : Vec<&str> = f_op.split(",").collect::<Vec<&str>>();

        let f_values : String = f_values.unwrap_or_else(|| {
            filter_err = true;
            fields_none +=1;
            "".into() }).split_whitespace().collect::<String>();
        let f_values : Vec<&str> = f_values.split(",").collect::<Vec<&str>>();

        let f_or : String = f_or.unwrap_or_else(|| {
            filter_err = true;
            fields_none +=1;
            "".into() }).split_whitespace().collect::<String>();
        let f_or : Vec<&str> = f_or.split(",").collect::<Vec<&str>>();

        let mut filter_vec : Vec<Filter> = Vec::new();


        if !filter_err && f_names.len() == f_values.len() && f_names.len() == f_op.len() && f_names.len() == f_or.len() {
            for i in 0..f_names.len() {
                filter_vec.push(Filter{
                    field_name: f_names.get(i).unwrap().to_string(),
                    op: f_op.get(i).unwrap().to_string(),
                    r_value: f_values.get(i).unwrap().to_string(),
                    or: f_or.get(i).unwrap().to_string().parse().unwrap_or_else(|_| false)
                });
            }
        } else if fields_none == 4 {
            filter_vec.push(Filter{
                field_name: "select_all".to_string(),
                op: "".to_string(),
                r_value: "".to_string(),
                or: false
            });
        } else {
            filter_vec.push(Filter{
                field_name: "select_none".to_string(),
                op: "".to_string(),
                r_value: "".to_string(),
                or: false
            });
        }


        let mut sort_err : bool = false;
        let s_name : String = s_name.unwrap_or_else(|| {
            sort_err = true;
            "".into() }).split_whitespace().collect::<String>();
        let s_asc : bool = s_asc.unwrap_or_else(|| {
            sort_err = true;
            "".into() }).split_whitespace().collect::<String>().parse().unwrap_or_else(|_| {
            sort_err = true;
            false
        });

        let sort = if sort_err {
            None
        } else {
            Some(Sort { field_name: s_name, order_asc: s_asc })
        };

        FilterSort {
            filters: filter_vec,
            sort
        }

    }
}