//! pbgui-tree
//!
//! Display a treeview of distributions and their platforms for a given site, complete with filtering.
//!
//! # Example Usage
//! ```ignore
//! ...
//! // create the widget, passing it a pointer to its intended parent
//! let mut mytree = tree::DistributionTreeView::create(main_widget_ptr);
//!
//! // set the default stylesheet
//! mytree.set_default_stylesheet();
//!
//! // set the packages to be displayed under the root in the treeview
//! mytree.set_packages(package_list);
//!
//! // Set the sites to be displayed in the sites pulldown, along with the initial
//! // site.
//! mytree.set_sites(site_list, "portland");
//! ```
pub(crate) mod api;
pub(crate) mod inner_tree;
pub mod tree;
