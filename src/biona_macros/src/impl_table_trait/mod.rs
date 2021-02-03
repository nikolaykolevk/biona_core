mod construct_blocks;
mod implementation;

pub use implementation::impl_trait;

//used to share common properties between the two functions
pub struct CommonItems<'a> {
    struct_ident : &'a syn::Ident,
    select_struct_ident : &'a syn::Ident,
    insert_struct_ident : &'a syn::Ident,
    update_struct_ident : &'a syn::Ident,
    table_schema : &'a syn::Path,
    table_name : &'a str,
    tables_join : &'a syn::ExprMethodCall
}

//stores the tokens of the bodies of the functions to be implemented
pub struct Blocks {
    pub fields_list : syn::ExprBlock,
    pub get_table_name : syn::ExprBlock,
    pub select_all : syn::ExprBlock,
    pub select_by_pr_key : syn::ExprBlock,
    pub filter : syn::ExprBlock,
    pub count_all : syn::ExprBlock,
    pub count_filtered : syn::ExprBlock,
    pub insert : syn::ExprBlock,
    pub update : syn::ExprBlock,
    pub get_pr_key : syn::ExprBlock,
    pub delete_by_pr_key : syn::ExprBlock,
    pub prim_key_type : syn::Type,
    pub select_struct : syn::ItemStruct,
    pub insert_struct : syn::ItemStruct,
    pub update_struct : syn::ItemStruct,
    pub impl_from_for_insert_struct_block : syn::ItemImpl,
    pub impl_from_for_update_struct_block : syn::ItemImpl,
    pub impl_from_for_table_page_block : syn::ItemImpl
}
