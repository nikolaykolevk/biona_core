use syn::{DeriveInput, ExprMethodCall, Type};

use crate::attribute_types::*;


use crate::impl_table_trait::{Blocks, CommonItems};

pub fn get_blocks(ast: &DeriveInput, common_items: CommonItems) -> Blocks {
    let fields = match &ast.data {
        syn::Data::Struct(dt) => Some(&dt.fields),
        _ => None
    };
    let fields = fields.unwrap();

    let struct_ident = common_items.struct_ident;
    let select_struct_ident = common_items.select_struct_ident;
    let insert_struct_ident = common_items.insert_struct_ident;
    let update_struct_ident = common_items.update_struct_ident;
    let table_schema = common_items.table_schema;
    let table_name = common_items.table_name;
    let tables_join = common_items.tables_join;


    let mut primary_key_schema: syn::Path = parse_quote!(#table_schema::id);
    let mut primary_key_type: syn::Type = parse_quote!(i32);
    let mut already_one_primary_key = false;


    let mut fields_list = quote! {
        use serde::Serialize;
        let mut json: Vec<Field> = Vec::new();
    };

    let mut filter_block_1 = quote!();
    let mut filter_block_2 = quote!();

    let mut fields_select_block = quote!();
    let mut select_struct_block = quote! {};
    let mut insert_struct_block = quote! {};
    let mut update_struct_block = quote! {};

    let mut insert_struct_init_block = quote!();
    let mut update_struct_init_block = quote!();

    match fields {
        syn::Fields::Named(named) => for field in named.named.iter() {
            let field_name = field.clone().ident.unwrap();
            let field_name_string = field_name.to_string();
            let field_type = field.clone().ty;
            let prior_value = int_lit_attr_from_field(field, "priority").unwrap_or_else(|| parse_quote!(0));
            let sortable = bool_attr_from_field(field, "sortable").unwrap_or_else(|| parse_quote!(true));
            let from_system_table = bool_attr_from_field(field, "from_system_table").unwrap_or_else(|| parse_quote!(false)).value;
            let hidden = bool_attr_from_field(field, "hidden").unwrap_or_else(|| parse_quote!(false));
            let filterable = bool_attr_from_field(field, "filterable").unwrap_or_else(|| parse_quote!(true));
            let mut field_schema = path_attr_from_field(field, "field_schema").unwrap_or_else(|| parse_quote!(#table_schema::#field_name));
            if !quote!(#field_schema).to_string().split_whitespace().collect::<String>().contains("crate::schema") { field_schema = parse_quote!(crate::schema::#field_schema); }
            let display_name = string_lit_attr_from_field(field, "display_name").unwrap_or_else(|| parse_quote!(#field_name_string));
            let primary_key = bool_attr_from_field(field, "primary_key").unwrap_or_else(|| parse_quote!(false));
            let selectable = bool_attr_from_field(field, "selectable").unwrap_or_else(|| parse_quote!(true)).value;
            let insertable = bool_attr_from_field(field, "insertable").unwrap_or_else(|| parse_quote!(true)).value;
            let updatable = bool_attr_from_field(field, "updatable").unwrap_or_else(|| parse_quote!(true)).value;


            if primary_key.value {
                if already_one_primary_key {
                    panic!("There can't be more than one primary key");
                }
                primary_key_schema = field_schema.clone();
                primary_key_type = field_type.clone();
                already_one_primary_key = true;
            }


            if selectable {
                select_struct_block.extend(quote! {
                    pub #field_name : #field_type,
                });

                fields_select_block.extend(quote!(#field_schema, ));


                fields_list.extend(quote! {
                json.push(Field {
                    name : String::from(stringify!(#field_name)),
                    display_name : String::from(#display_name),
                    sortable : #sortable,
                    filterable : #filterable,
                    priority : #prior_value,
                    updatable : #updatable,
                    insertable : #insertable,
                    hidden : #hidden,
                    is_primary_key : #primary_key,
                    field_type : String::from(stringify!(#field_type)),
                    field_schema : String::from(stringify!(#field_schema)),
                    from_system_table : #from_system_table
                });
            });

                if filterable.value
                {
                    let field_type_parsed: Type;
                    if quote!(#field_type).to_string().contains("Option") {
                        let tmp_val = field_type.clone();
                        let tmp_val: String = quote!(#tmp_val).to_string().split_whitespace().collect();
                        let tmp_val = &tmp_val[7..tmp_val.len() - 1];
                        let tmp_val = format_ident!("{}", tmp_val);
                        field_type_parsed = parse_quote!(#tmp_val);
                    } else {
                        field_type_parsed = field_type.clone();
                    }

                    let common_filter_operations_block = quote! {
                    "eq" => {
                        if filter.or {
                            result = result.or_filter(#field_schema.eq::<#field_type_parsed>(filter.r_value.parse().unwrap_or_else(|_|  #field_type_parsed::default())))
                        } else {
                            result = result.filter(#field_schema.eq::<#field_type_parsed>(filter.r_value.parse().unwrap_or_else(|_|  #field_type_parsed::default())))
                        }
                    },
                    "ne" => {
                        if filter.or {
                            result = result.or_filter(#field_schema.ne::<#field_type_parsed>(filter.r_value.parse().unwrap_or_else(|_|  #field_type_parsed::default())))
                        } else {
                            result = result.filter(#field_schema.ne::<#field_type_parsed>(filter.r_value.parse().unwrap_or_else(|_|  #field_type_parsed::default())))
                        }
                    },

                };

                    let text_filter_operations_block = quote! {
                    "like" => {
                        if filter.or {
                            let mut tmp_val = String::from("%");
                            tmp_val.push_str(filter.r_value.as_str());
                            tmp_val.push_str("%");
                            result = result.or_filter(#field_schema.ilike(tmp_val))
                        } else {
                            let mut tmp_val = String::from("%");
                            tmp_val.push_str(filter.r_value.as_str());
                            tmp_val.push_str("%");
                            result = result.filter(#field_schema.ilike(tmp_val))
                        }
                    },
                    "not_like" => {
                        if filter.or {
                            let mut tmp_val = String::from("%");
                            tmp_val.push_str(filter.r_value.as_str());
                            tmp_val.push_str("%");
                            result = result.or_filter(#field_schema.not_ilike(tmp_val))
                        } else {
                            let mut tmp_val = String::from("%");
                            tmp_val.push_str(filter.r_value.as_str());
                            tmp_val.push_str("%");
                            result = result.filter(#field_schema.not_ilike(tmp_val))
                        }
                    },

                };

                    let numeral_filter_operations_block = quote! {
                    "gt" => {
                        if filter.or {
                            result = result.or_filter(#field_schema.gt::<#field_type_parsed>(filter.r_value.parse().unwrap_or_else(|_|  #field_type_parsed::default())))
                        } else {
                            result = result.filter(#field_schema.gt::<#field_type_parsed>(filter.r_value.parse().unwrap_or_else(|_|  #field_type_parsed::default())))
                        }
                    },
                    "lt" => {
                        if filter.or {
                            result = result.or_filter(#field_schema.lt::<#field_type_parsed>(filter.r_value.parse().unwrap_or_else(|_|  #field_type_parsed::default())))
                        } else {
                            result = result.filter(#field_schema.lt::<#field_type_parsed>(filter.r_value.parse().unwrap_or_else(|_|  #field_type_parsed::default())))
                        }
                    },

                };

                    let mut filter_operations_block = quote! {
                        #common_filter_operations_block
                };


                    if quote!(#field_type).to_string().to_lowercase().contains("str") {
                        filter_operations_block.extend(quote! {
                            #text_filter_operations_block
                        });
                    } else if vec!["i8", "i16", "i32", "i64", "u8", "u16", "u32", "u64", "f8", "f16", "f32", "f64"].contains(&quote!(#field_type).to_string().as_str()) {
                        filter_operations_block.extend(quote! {
                            #numeral_filter_operations_block
                        });
                    } else if quote!(#field_type).to_string().to_lowercase().contains("date") {
                        filter_operations_block.extend(quote! {
                            #numeral_filter_operations_block
                        });
                    }


                    filter_block_1.extend(quote! {
                    #field_name_string => { match filter.op.as_str() {
                        #filter_operations_block
                        _ => result = result
                    }},
                });
                }

                if sortable.value {
                    filter_block_2.extend(quote! {
                        #field_name_string => {
                            if sort.order_asc {
                                result = result.order_by(#field_schema.asc());
                            } else {
                                result = result.order_by(#field_schema.desc());
                            }
                        },
                    });
                }
            }

            if insertable {
                insert_struct_block.extend(quote! {
                    pub #field_name : #field_type,
                });

                insert_struct_init_block.extend(quote!(#field_name : item.#field_name, ));
            }

            if updatable {
                update_struct_block.extend(quote! {
                    pub #field_name : #field_type,
                });

                update_struct_init_block.extend(quote!(#field_name : item.#field_name, ));
            }
        }
        _ => println!("There are no fields")
    }

    let filter: syn::ExprBlock = parse_quote!({
        let mut result = #tables_join
        .select((#fields_select_block))
        .offset(i64::from(display_count)*i64::from(page-1))
        .limit(i64::from(display_count))
        .internal_into_boxed();

        for filter in filter_sort.filters {
            match filter.field_name.as_str() {
                #filter_block_1
                // "select_all" => return super::#struct_ident::new().select_all(display_count, page),
                "select_all" => {
                    result = #tables_join
                    .select((#fields_select_block))
                    .offset(i64::from(display_count)*i64::from(page-1))
                    .limit(i64::from(display_count))
                    .internal_into_boxed();
                    break;
                }
                "select_none" => return vec![],
                _ => return vec![]
            }
        }

        if let Some(sort) = filter_sort.sort {
            match sort.field_name.as_str() {
                #filter_block_2
                _ => result = result
            }
        }
        result.load::<#select_struct_ident>(&biona_core::establish_connection()).expect("Error Filtering")
    });

    let count_filtered: syn::ExprBlock = parse_quote!({
        let mut result = #tables_join
        .select(count_star())
        .internal_into_boxed();

        for filter in filter_sort.filters {
            match filter.field_name.as_str() {
                #filter_block_1
                _ => result = result
            }
        }
        use std::convert::TryFrom;
        let count = result.first(&biona_core::establish_connection()).unwrap_or_else(|_| 0);
        u64::try_from(count).unwrap_or_else(|_| 0)
    });

    let fields_select_block: ExprMethodCall = parse_quote! {
        #tables_join.select((#fields_select_block))
    };

    let select_all = quote! {
        #fields_select_block
        .offset(i64::from(display_count)*i64::from(page-1))
        .limit(i64::from(display_count))
        .load::<#select_struct_ident>(&biona_core::establish_connection())
        .expect("Error loading")
    };

    let select_by_id_block = quote! {
        #fields_select_block
        .filter(#primary_key_schema.eq::<#primary_key_type>(find_primary_key.parse().unwrap()))
        .load::<#select_struct_ident>(&biona_core::establish_connection())
        .expect("Error loading")
        .pop()
    };

    fields_list.extend(quote! {
        let mut sorted_json: Vec<_> = json.into_iter().collect();
        sorted_json.sort_by(|x,y| x.priority.cmp(&y.priority));
        sorted_json.reverse();

        sorted_json
    });

    let get_table_name: syn::ExprBlock = parse_quote!({
        return String::from(#table_name)
    });

    let impl_from_for_insert_struct_block: syn::ItemImpl = parse_quote! {
        impl From<super::#struct_ident> for #insert_struct_ident {
            fn from(item: super::#struct_ident) -> Self {
                #insert_struct_ident {
                    #insert_struct_init_block
                }
            }
        }
    };

    let impl_from_for_update_struct_block: syn::ItemImpl = parse_quote! {
        impl From<super::#struct_ident> for #update_struct_ident {
            fn from(item: super::#struct_ident) -> Self {
                #update_struct_ident {
                    #update_struct_init_block
                }
            }
        }
    };

    let impl_from_for_table_page_block: syn::ItemImpl = parse_quote!(
        impl From<super::#struct_ident> for biona_core::models::TablePage<#select_struct_ident> {
            fn from(item: super::#struct_ident) -> Self {
                let mut res = biona_core::models::TablePage::new();
                res.set_table_name(&item.get_table_name());
                res.set_fields_list(item.fields_list());
                res
            }
        }
    );


    let insert: syn::ExprBlock = parse_quote!({
            let mut insert_vec : Vec<#insert_struct_ident> = Vec::new();
            for item in new_data {
                insert_vec.push(#insert_struct_ident::from(item));
            }

             diesel::insert_into(#table_schema::table)
            .values(insert_vec)
            .returning(#primary_key_schema)
            .get_results(&biona_core::establish_connection()).unwrap()
    });

    let update_by_id: syn::ExprBlock = parse_quote!({
            let update_struct : #update_struct_ident = #update_struct_ident::from(update_data);

             diesel::update(#table_schema::table)
                .filter(#primary_key_schema.eq::<#primary_key_type>(find_primary_key.parse().unwrap()))
                .set(update_struct)
                .execute(&biona_core::establish_connection())
                .unwrap()
    });

    let delete_by_id: syn::ExprBlock = parse_quote!({
             diesel::delete(#table_schema::table)
            .filter(#primary_key_schema.eq::<#primary_key_type>(find_primary_key.parse().unwrap()))
            .execute(&biona_core::establish_connection())
    });

    let count_all = parse_quote!({
        use std::convert::TryFrom;
        let count = #tables_join.select(count_star()).first(&biona_core::establish_connection()).unwrap_or_else(|_| 0);
        u64::try_from(count).unwrap_or_else(|_| 0)

     });


    Blocks {
        fields_list: parse_quote!({ #fields_list }),
        get_table_name,
        select_all: parse_quote!({ #select_all }),
        select_by_id: parse_quote!({ #select_by_id_block }),
        filter,
        count_all,
        count_filtered,
        insert,
        update_by_id,
        delete_by_id,
        prim_key_type: primary_key_type,
        select_struct: parse_quote!(pub struct #select_struct_ident { #select_struct_block }),
        insert_struct: parse_quote!(pub struct #insert_struct_ident { #insert_struct_block }),
        update_struct: parse_quote!(pub struct #update_struct_ident { #update_struct_block }),
        impl_from_for_insert_struct_block,
        impl_from_for_update_struct_block,
        impl_from_for_table_page_block,
    }
}