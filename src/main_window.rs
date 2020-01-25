use crate::{
    cache::PinChangesCache, choose_distribution::choose_alternative_distribution,
    constants::COL_REV_TXID, inner_main_window, save_versionpin_changes::save_versionpin_changes,
    select_history::select_history, store_withpackage_changes,
    update_changes_table::update_changes_table, update_versionpin_table::update_vpin_table,
    update_withpackages::update_withpackages, utility::create_vlayout,
};
use log;
use pbgui_toolbar::toolbar;
use pbgui_tree::tree;
use pbgui_withs::WithsList;
use qt_core::{QItemSelection, QPoint, Slot, SlotOfBool, SlotOfQItemSelectionQItemSelection};
use qt_widgets::{
    cpp_core::{CppBox, MutPtr, Ref as QRef},
    QMainWindow, QMenu, QMenuBar, QVBoxLayout, QWidget, SlotOfQPoint,
};
use rustqt_utils::enclose;
use std::cell;
use std::cell::RefCell;
use std::rc::Rc;

/// Just as it sounds, MainWindow is the MainWindow struct, holding on
/// to various pointers that need to persist as well as top level slots
pub struct MainWindow<'a> {
    main: Rc<inner_main_window::InnerMainWindow<'a>>, //
    _main_owned: CppBox<QMainWindow>,
    _dist_popup_menu_owned: CppBox<QMenu>,
    // Slots
    query_button_clicked: Slot<'a>,
    save_clicked: Slot<'a>,
    choose_distribution_triggered: Slot<'a>,
    show_dist_menu: SlotOfQPoint<'a>,
    //clear_package: Slot<'a>,
    // show_line_edit_menu: SlotOfQPoint<'a>,
    select_pin_changes: Slot<'a>,
    select_history: Slot<'a>,
    toggle_packages_tree: SlotOfBool<'a>,
    toggle_withs: SlotOfBool<'a>,
    toggle_vpin_changes: SlotOfBool<'a>,
    revision_changed: SlotOfQItemSelectionQItemSelection<'a>,
    distribution_changed: SlotOfQItemSelectionQItemSelection<'a>,
    save_withpackages: Slot<'a>,
}

impl<'a> MainWindow<'a> {
    /// New up the MainWindow. The MainWindow's primary job is to
    /// ensure that needed widgets and data remain in scope for the
    /// lifetime of the application. Other than that, it is fairly
    /// inert. This is more than in part due to the fact that the
    /// initialization of QT happens during new, including wiring up
    /// of signals and slots. Thus, nothing is relying on MainWindow
    /// methods.
    ///
    /// In order to avoid MainWindow becoming completely unreadable,
    /// MainWindow::new delegates a good deal of construction to
    /// crate::components, and the bulk of the business logic in
    /// slot implementation to crate::slot_functions.
    /// Even so, the main structure is a bit nested.
    ///
    /// ```ignore
    /// main_window (QMainApplication)
    /// - main_window_bar (QMenuBar)
    /// - main_widget (QWidget)
    /// -- main_layout (QHBoxLayout)
    /// --- with_splitter (QSplitter)
    /// ---- center_widget (QWidget)
    /// ----- center_layout (QVBoxLayourt)
    /// ---- item_list_ptr (MutPtr<QListWidget>)
    /// ```
    pub fn new() -> MainWindow<'a> {
        unsafe {
            let (main, main_owned, dist_popup_menu_owned) =
                inner_main_window::InnerMainWindow::new();
            let main_ = Rc::new(main);
            let main = main_.clone();
            // persist data
            // Create the MainWindow instance, set up signals and slots, and return
            // the newly minted instance. We are done.
            let main_window_inst = MainWindow {
                main: main_,
                _main_owned: main_owned,
                _dist_popup_menu_owned: dist_popup_menu_owned,
                // slots
                save_withpackages: Slot::new(enclose! { (main) move || {

                    let mut pinchanges_ptr = main.pinchanges_list();
                    store_withpackage_changes::store_withpackage_changes(
                        main.package_withs_list(),
                        main.vpin_table(),
                        &mut pinchanges_ptr,
                        main.cache(),
                    );
                }}),

                distribution_changed: SlotOfQItemSelectionQItemSelection::new(enclose! { (main)
                move |selected: QRef<QItemSelection>, _deselected: QRef<QItemSelection>| {
                    //let item_list_ptr = None;
                    let mut vpin_tablewidget_ptr = main.vpin_table();
                    let ind = selected.indexes();
                    if ind.count_0a() > 0 {
                        let txid = ind.at(COL_REV_TXID);
                        update_withpackages(
                            txid.row(),
                            &mut vpin_tablewidget_ptr,
                            main.package_withs_list(),//item_list_ptr.clone(),
                            main.cache(),
                        );
                    } else {
                       // item_list_ptr.borrow_mut().clear();
                        main.package_withs_list().borrow().clear();
                    }
                }}),

                revision_changed: SlotOfQItemSelectionQItemSelection::new(enclose! { (main)
                    move |selected: QRef<QItemSelection>, _deselected: QRef<QItemSelection>| {
                        let ind = selected.indexes();
                        if ind.count_0a() > 0 {
                            let txid = ind.at(COL_REV_TXID);
                            update_changes_table(
                                txid.row(),
                                main.revisions_table(), //revisions_ptr,
                                main.changes_table(),   //changes_table_ptr,
                            );
                        } else {
                            main.changes_table().clear_contents();
                            main.changes_table().set_row_count(0);
                        }
                    }
                }),

                // clear_package: Slot::new(move || {
                //     line_edit_ptr.clear();
                // }),

                // show_line_edit_menu: SlotOfQPoint::new(move |pos: QRef<QPoint>| {
                //     let _action = line_edit_popup_menu_ptr
                //         .exec_1a_mut(line_edit_ptr.map_to_global(pos).as_ref());
                // }),
                show_dist_menu: SlotOfQPoint::new(enclose! { (main) move |pos: QRef<QPoint>| {
                    if main.vpin_table().is_null()
                    //vpin_tablewidget_ptr.is_null()
                    {
                        log::error!("vpin_tablewidget_ptr is null");
                        return;
                    }
                    if main.dist_popup_menu().is_null()
                    //dist_popup_menu_ptr.is_null()
                    {
                        log::error!("dist_popup_menu_ptr is null");
                        return;
                    }
                    let _action = main
                        .dist_popup_menu() //dist_popup_menu_ptr
                        .exec_1a_mut(
                            //vpin_tablewidget_ptr
                            main.vpin_table().map_to_global(pos).as_ref(),
                        );
                }}),

                save_clicked: Slot::new(enclose! { (main) move || {
                    //let pinchange_cache = None;
                    //let main_toolbar_ptr = None;
                    let mut pinchanges_ptr = main.changes_table();
                    save_versionpin_changes(
                        main.main_widget(),//main_widget_ptr,
                        &mut pinchanges_ptr,
                        main.main_toolbar(),//main_toolbar_ptr.clone(),
                        main.cache()//pinchange_cache.clone(),
                    );
                } }),

                query_button_clicked: Slot::new(enclose! {(main) move || {
                    update_vpin_table(
                        main.main_toolbar(),
                        &main.left_toolbar_actions().search_shows,
                        main.vpin_table()//vpin_tablewidget_ptr,
                    );
                }}),

                choose_distribution_triggered: Slot::new(enclose! { (main) move || {
                    if main.vpin_table().is_null() {
                        log::error!("Error: attempted to access null pointer in choose_distribution_tribbered");
                        return;
                    }
                    if main.vpin_table().row_count() == 0 {
                        return;
                    }
                    let current_row = main.vpin_table().current_row();
                    choose_alternative_distribution(
                        current_row,
                        main.vpin_table(),//vpin_tablewidget_ptr,
                        main.main_widget(),//main_widget_ptr,
                        main.pinchanges_list(), //pinchanges_ptr,
                        main.cache(), //pinchange_cache.clone(),
                    );
                }}),

                select_pin_changes: Slot::new(enclose! { (main) move || {
                    // todo - move bottom stacked widget into own component
                    main.bottom_stacked_widget().set_current_index(0); //.stacked_ptr.set_current_index(0);
                                                                       //controls_ptr.set_current_index(0);
                    main.bottom_ctrls_stacked_widget().set_current_index(0);
                }}),

                select_history: Slot::new(enclose! { (main) move || {
                    let mut revisions_ptr = main.revisions_table();
                    let mut stacked_ptr = main.bottom_stacked_widget();
                    select_history(
                        &mut revisions_ptr,
                        &mut stacked_ptr);
                    //controls_ptr.set_current_index(1);
                    main.bottom_ctrls_stacked_widget().set_current_index(1);
                }}),

                toggle_packages_tree: SlotOfBool::new(enclose! { (main) move |state: bool| {
                    let mut frame = main.withs_splitter().widget(0);
                    //let mut frame = with_splitter_ptr.widget(0);
                    frame.set_visible(state);
                }}),
                toggle_withs: SlotOfBool::new(enclose! { (main) move |state: bool| {
                    let mut frame = main.withs_splitter().widget(2);
                    //let mut frame = with_splitter_ptr.widget(2);
                    frame.set_visible(state);
                }}),

                toggle_vpin_changes: SlotOfBool::new(enclose! { (main) move |state: bool| {
                    let mut frame = main.vpin_table_splitter().widget(1);
                    frame.set_visible(state);
                }}),
            };

            //
            // connect signals to slots
            //
            //pinchanges_button_ptr
            main.pinchanges_button()
                .clicked()
                .connect(&main_window_inst.select_pin_changes);

            main.history_button()
                //history_button_ptr
                .clicked()
                .connect(&main_window_inst.select_history);

            main.main_toolbar()
                //main_toolbar_ptr
                //.borrow()
                .query_btn()
                .clicked()
                .connect(&main_window_inst.query_button_clicked);

            main.save_button()
                //save_button
                .clicked()
                .connect(&main_window_inst.save_clicked);

            main.vpin_table()
                //vpin_tablewidget_ptr
                .custom_context_menu_requested()
                .connect(&main_window_inst.show_dist_menu);

            main.dist_popup_action()
                //choose_dist_action
                .triggered()
                .connect(&main_window_inst.choose_distribution_triggered);

            main.revisions_table()
                //.revisions_ptr
                .selection_model()
                .selection_changed()
                .connect(&main_window_inst.revision_changed);

            main.vpin_table()
                //vpin_tablewidget_ptr
                .selection_model()
                .selection_changed()
                .connect(&main_window_inst.distribution_changed);

            main.left_toolbar_actions()
                .view_packages
                .toggled()
                .connect(&main_window_inst.toggle_packages_tree);

            main.left_toolbar_actions()
                .view_withs
                .toggled()
                .connect(&main_window_inst.toggle_withs);

            main.left_toolbar_actions()
                .view_vpin_changes
                .toggled()
                .connect(&main_window_inst.toggle_vpin_changes);

            main.package_withs_list_save()
                //withpackage_save
                .clicked()
                .connect(&main_window_inst.save_withpackages);

            main.search_shortcut()
                //main_window_inst
                //    .search_shortcut
                .activated()
                .connect(&main_window_inst.query_button_clicked);

            // configuration
            let mut view_withs = main.left_toolbar_actions().view_withs;
            view_withs.set_checked(false);

            main_window_inst
        }
    }

    pub fn cache(&self) -> Rc<PinChangesCache> {
        self.main.cache()
    }
    /// Retrieve a Ref wrapped DistributionTreeView instance
    ///
    /// # Arguments
    /// * None
    ///
    /// # Returns
    /// * Ref<DistributionTreeView>
    pub unsafe fn packages_tree(&self) -> cell::Ref<tree::DistributionTreeView<'a>> {
        self.main.packages_tree()
    }

    /// Retrieve an shared copy of the DistributionTreeView
    pub unsafe fn tree(&self) -> Rc<RefCell<tree::DistributionTreeView<'a>>> {
        self.main.tree()
    }

    /// Retrieve an shared copy of the DistributionTreeView
    pub unsafe fn package_withs_list(&self) -> Rc<RefCell<WithsList<'a>>> {
        self.main.package_withs_list()
    }
    /// Retrieve a RefMut wrapped DistributionTreeView instance
    ///
    /// # Arguments
    /// * None
    ///
    /// # Returns
    /// * RefMut<DistributionTreeView>
    pub unsafe fn packages_tree_mut(&self) -> cell::RefMut<tree::DistributionTreeView<'a>> {
        self.main.packages_tree_mut()
    }

    /// Retrieve an Rc wrapped MainToolbar instance
    ///
    /// # Arguments
    /// * None
    ///
    /// # Returns
    /// * Rc<MainToolbar>
    pub unsafe fn main_toolbar(&self) -> Rc<toolbar::MainToolbar> {
        self.main_toolbar()
    }

    /// Retrieve a MutPtr to the QMainWindow instance
    ///
    /// # Arguments
    /// * None
    ///
    /// # Returns
    /// * MutPtr<QMainWindow>
    pub unsafe fn main(&self) -> MutPtr<QMainWindow> {
        self.main()
    }
}

// create the main window, the main menubar, and the central widget
fn create_main_window() -> (CppBox<QMainWindow>, MutPtr<QWidget>, MutPtr<QVBoxLayout>) {
    unsafe {
        let mut main_window = QMainWindow::new_0a();
        // the qmainwindow takes ownership of the menubar,
        // even though it takes a MutPtr instead of a Cpp
        let main_menu_bar = QMenuBar::new_0a();
        main_window.set_menu_bar(main_menu_bar.into_ptr());

        // main_widget - central widget of teh main_window

        let mut main_widget = QWidget::new_0a();
        let main_widget_ptr = main_widget.as_mut_ptr();

        // main_layout

        let mut main_layout = create_vlayout();
        let main_layout_ptr = main_layout.as_mut_ptr();
        main_widget.set_layout(main_layout.into_ptr());

        // set main_widget as the central widget in main_window
        main_window.set_central_widget(main_widget.into_ptr());

        (main_window, main_widget_ptr, main_layout_ptr)
    }
}

fn create_top_toolbar(parent: MutPtr<QMainWindow>) -> toolbar::MainToolbar {
    let tb = toolbar::create(parent);
    tb.set_default_stylesheet();
    tb
}
