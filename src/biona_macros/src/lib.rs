#![recursion_limit = "128"]

#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;

mod impl_table_trait;
mod attribute_types;

//Implements TableTrait
// table_name:          STRUCT ATTR      EXPRESSION               name of the table in the db                                   EX: #[table_name(my_table)]                                                 Default: name of the struct
// table_join:          STRUCT ATTR      EXPRESSION METHOD CALL   used to create a join of multiple tables                      EX: #[tables_join(my_table::table.inner_join(table_to_join::table))]        Default: none
// primary_key:         FIELD ATTR       true/false               Indicates primary key                                         Note: Only one per struct                                                   Default: false (one true is mandatory)
// priority:            FIELD ATTR       integer                  Indicates display order on the client                         Note: Higher priorities are displayed first                                 Default: before prioritized
// filterable:          FIELD ATTR       true/false               Indicates if you should be able to filter by this field                                                                                   Default: true
// sortable:            FIELD ATTR       true/false               Indicates if you should be able to sort by this field                                                                                     Default: true
// selectable:          FIELD ATTR       true/false               Indicates if the field is selectable                                                                                                      Default: true
// insertable:          FIELD ATTR       true/false               Indicates if the field is insertable                                                                                                      Default: true
// filterable:          FIELD ATTR       true/false               Indicates if the field is updatable                                                                                                       Default: true
// hidden:              FIELD ATTR       true/false               Indicates if the field should be hidden by default                                                                                        Default: false
// from_system_table:   FIELD ATTR       true/false               Indicates if the field belongs to a system table                                                                                          Default: false
// field_schema:        FIELD ATTR       PATH                     Indicates the path of a field                                                                                                             Default: uses the name of the field
// display_name:        FIELD ATTR       STRING LITERAL           How the field should be displayed as                          EX: #[display_name("FieldNameToDisplay")]                                   Default: name of the field



#[proc_macro_derive(TableTrait, attributes(table_name, tables_join, primary_key ,priority, filterable, sortable, field_schema, display_name, selectable, insertable, updatable, hidden, from_system_table))]
pub fn table_trait_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_table_trait::impl_trait(&ast)
}