pub mod api;
pub mod constants;
pub mod slot_functions;
pub mod utility;
pub use api::client_proxy::ClientProxy;
// needed so that qt wont segfault
pub mod components;
pub use components::{
    bottom_stacked_widget, revisions_table,
    search_bar::{combo_boxes, create_query_button},
    table_headers, versionpin_changes_table, versionpin_table,
};
pub use slot_functions::{
    choose_distribution, save_versionpin_changes, select_history, update_changes_table,
    update_versionpin_table,
};
