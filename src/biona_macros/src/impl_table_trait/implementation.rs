use crate::impl_table_trait::{construct_blocks, CommonItems};
use syn::{DeriveInput};
use crate::attribute_types::*;


pub fn impl_trait(ast: &DeriveInput) -> proc_macro::TokenStream {
    let struct_ident = &ast.ident;
    let select_struct_ident = format_ident!("{}Select", struct_ident);
    let insert_struct_ident = format_ident!("{}Insert", struct_ident);
    let update_struct_ident = format_ident!("{}Update", struct_ident);
    let module_ident = format_ident!("{}_private", struct_ident.to_string().to_lowercase());

    let table_schema = if let Some(t) = path_attr_from_derive_input(ast, "table_name") {
        parse_quote!(crate::schema::#t)
    } else {
        parse_quote!(crate::schema::#struct_ident)
    };

    let tables_join = expr_method_call_from_derive_input(ast, "tables_join").unwrap_or_else(|| parse_quote!{ #table_schema::table .to_owned() });

    let table_name =  path_attr_from_derive_input(ast, "table_name").unwrap_or_else(|| parse_quote!(#struct_ident));
    let table_name = quote!(#table_name).to_string();


    let common_items = CommonItems {
        struct_ident,
        select_struct_ident: &select_struct_ident,
        insert_struct_ident: &insert_struct_ident,
        update_struct_ident: &update_struct_ident,
        table_schema : &table_schema,
        table_name : &table_name,
        tables_join : &tables_join
    };

    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    let blocks = construct_blocks::get_blocks(ast, common_items);
    let fields_list = blocks.fields_list;
    let select_all = blocks.select_all;
    let get_table_name = blocks.get_table_name;
    let select_by_pr_key_block = blocks.select_by_pr_key;
    let filter = blocks.filter;
    let count_all = blocks.count_all;
    let count_filtered = blocks.count_filtered;
    let insert = blocks.insert;
    let get_pr_key = blocks.get_pr_key;
    let update = blocks.update;
    let delete_by_pr_key = blocks.delete_by_pr_key;
    let prim_key_type = blocks.prim_key_type;
    let select_struct_block = blocks.select_struct;
    let insert_struct_block = blocks.insert_struct;
    let update_struct_block = blocks.update_struct;
    let impl_from_for_insert_struct_block = blocks.impl_from_for_insert_struct_block;
    let impl_from_for_update_struct_block = blocks.impl_from_for_update_struct_block;
    let impl_from_for_table_page_block = blocks.impl_from_for_table_page_block;


    quote! (

        mod  #module_ident {

            use diesel::prelude::*;
            use diesel::query_dsl::boxed_dsl::BoxedDsl;
            use diesel::dsl::*;
            use diesel::{Queryable, Insertable, AsChangeset};
            use biona_core::models::*;
            use #table_schema;
            use biona_core::*;
            use crate::schema::*;
            use biona_core::Serialize;

            #[derive(Queryable, Serialize, Debug, Default)]
            #select_struct_block

            #[derive(Insertable, Debug, Default)]
            #[table_name = #table_name]
            #insert_struct_block

            #[derive(AsChangeset, Debug, Default)]
            #[table_name = #table_name]
            #update_struct_block

            #impl_from_for_insert_struct_block
            #impl_from_for_update_struct_block
            #impl_from_for_table_page_block


            impl #impl_generics biona_core::models::TableTrait<#select_struct_ident, #prim_key_type> for super::#struct_ident #ty_generics #where_clause {

                fn new() -> Self where Self: Sized {
                    super::#struct_ident::default()
                }

                fn fields_list(&self) -> Vec<biona_core::models::Field> #fields_list

                fn get_table_name(&self) -> String #get_table_name

                fn select_all(&self, display_count : u32, page: u32) -> Vec<#select_struct_ident> #select_all

                fn select_by_pr_key(&self, find_primary_key: &str) -> Option<#select_struct_ident> #select_by_pr_key_block

                fn filter(&self, filter_sort: biona_core::models::FilterSort, display_count : u32, page: u32) -> Vec<#select_struct_ident> #filter

                fn count_all(&self) -> u64 #count_all

                fn count_filtered(&self, filter_sort : biona_core::models::FilterSort) -> u64 #count_filtered

                fn insert (&self, new_data: Vec<Self>) -> Vec<#prim_key_type> #insert

                fn get_pr_key(&self) -> String #get_pr_key

                fn update(&self, update_data: Self) -> usize #update

                fn delete_by_pr_key(&self, find_primary_key: &str) -> QueryResult<usize> #delete_by_pr_key

            }
        }
    ).into()
}
