use crate::{
    bottom_stacked_widget::create_bottom_stacked_widget,
    cache::PinChangesCache,
    center_widget,
    choose_distribution::choose_alternative_distribution,
    constants::COL_REV_TXID,
    left_toolbar, packages_tree,
    save_versionpin_changes::save_versionpin_changes,
    select_history::select_history,
    store_withpackage_changes,
    update_changes_table::update_changes_table,
    update_versionpin_table::update_vpin_table,
    update_withpackages::update_withpackages,
    utility::{create_vlayout, load_stylesheet, qs, resize_window_to_screen},
    versionpin_table, versionpin_table_splitter, withpackage_widget, withs_splitter, ClientProxy,
    LeftToolBarActions,
};
use packybara::traits::*;

use log;
use packybara::packrat::PackratDb;
use pbgui_toolbar::toolbar;
use pbgui_tree::tree;
use qt_core::{
    QItemSelection, QPoint, QString, Slot, SlotOfBool, SlotOfQItemSelectionQItemSelection,
};
use qt_gui::QKeySequence;
use qt_widgets::{
    cpp_core::{CppBox, MutPtr, Ref as QRef},
    QAction, QMainWindow, QMenu, QMenuBar, QPushButton, QShortcut, QTableWidget, QVBoxLayout,
    QWidget, SlotOfQPoint,
};
use std::cell::RefCell;
use std::rc::Rc;

// makes it simpler to deal with the need to clone. Saw this here:
// https://github.com/rust-webplatform/rust-todomvc/blob/master/src/main.rs#L142
macro_rules! enclose {
    ( ($(  $x:ident ),*) $y:expr ) => {
        {
            $(let $x = $x.clone();)*
            $y
        }
    };
}

#[allow(unused_macros)]
macro_rules! enclose_mut {
    ( ($( mut $x:ident ),*) $y:expr ) => {
        {
            $(let mut $x = $x.clone();)*
            $y
        }
    };
}

/// clone both immutable and mutable vars. Useful for
/// qt, which has a lot more mutable
/// use like so:
/// ```ignore
/// Slot::,new(enclose_all!{ (foo, bar) (mut bla) move || {}}),
/// ```
macro_rules! enclose_all {
    ( ($(  $x:ident ),*) ($( mut $mx:ident ),*) $y:expr ) => {
        {
            $(let $x = $x.clone();)*
            $(let mut $mx = $mx.clone();)*
            $y
        }
    };
}

/// Just as it sounds, MainWindow is the MainWindow struct, holding on
/// to various pointers that need to persist as well as top level slots
pub struct MainWindow<'a> {
    _db: &'a mut PackratDb,
    _main: CppBox<QMainWindow>,
    _main_toolbar: Rc<RefCell<toolbar::MainToolbar>>,
    _packages_tree: Rc<RefCell<tree::DistributionTreeView<'a>>>,
    _vpin_table: MutPtr<QTableWidget>,
    _pinchanges_list: MutPtr<QTableWidget>,
    _save_button: MutPtr<QPushButton>,
    _pin_changes_button: MutPtr<QPushButton>,
    _history_button: MutPtr<QPushButton>,
    _dist_popup_menu: CppBox<QMenu>,
    _dist_popup_action: MutPtr<QAction>,
    _left_toolbar_actions: LeftToolBarActions,
    search_shortcut: MutPtr<QShortcut>,
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
    pub fn new(db: &'a mut PackratDb) -> MainWindow<'a> {
        unsafe {
            // create the main window, menus, central widget and layout
            let (mut main_window, main_widget_ptr, mut main_layout_ptr) = create_main_window();
            let mut main_window_ptr = main_window.as_mut_ptr();
            let main_toolbar = Rc::new(RefCell::new(create_top_toolbar(main_window_ptr.clone())));
            let main_toolbar_ptr = main_toolbar.clone();
            // create left toolbar
            let left_toolbar_actions = left_toolbar::create(&mut main_window_ptr);
            let view_packages = left_toolbar_actions.view_packages;
            let mut view_withs = left_toolbar_actions.view_withs;
            let view_pin_changes = left_toolbar_actions.view_vpin_changes;
            let search_shows = left_toolbar_actions.search_shows;

            // create the splitter between the center widget and the withs
            let mut with_splitter_ptr = withs_splitter::create(&mut main_layout_ptr);

            // create packages treeview on left side
            let packages_ptr = packages_tree::create(with_splitter_ptr);
            // create the center widget
            let mut center_layout_ptr = center_widget::create(&mut with_splitter_ptr);

            // create the versionpin table splitter
            let mut vpin_table_splitter = versionpin_table_splitter::create(&mut center_layout_ptr);

            // create the versionpin table
            let mut vpin_tablewidget_ptr = versionpin_table::create(&mut vpin_table_splitter);

            let (
                mut pinchanges_ptr,
                mut revisions_ptr,
                mut changes_table_ptr,
                save_button,
                mut stacked_ptr,
                pinchanges_button_ptr,
                history_button_ptr,
                mut controls_ptr,
            ) = create_bottom_stacked_widget(&mut vpin_table_splitter);

            // setup popup menu for versionpin table
            let mut dist_popup_menu = QMenu::new();
            let choose_dist_action =
                dist_popup_menu.add_action_q_string(&QString::from_std_str("Change Version"));

            let _choose_withs_action =
                dist_popup_menu.add_action_q_string(&QString::from_std_str("Withs"));

            let mut dist_popup_menu_ptr = dist_popup_menu.as_mut_ptr();

            // create the with with package list on the right hand side
            let item_list_ptr = withpackage_widget::create(with_splitter_ptr);
            item_list_ptr.borrow_mut().set_add_mode();
            item_list_ptr.borrow_mut().set_cb_max_visible_items(30);

            // shortcuts
            let key_seq = QKeySequence::from_q_string(&qs("Ctrl+Return"));
            let search_shortcut =
                QShortcut::new_2a(key_seq.as_ref(), item_list_ptr.borrow_mut().main);

            // persist data
            let pinchange_cache = Rc::new(PinChangesCache::new());
            let cache = pinchange_cache.clone();
            //
            // final housekeeping before showing main window
            //
            versionpin_table_splitter::set_sizes(&mut vpin_table_splitter);
            withs_splitter::set_sizes(&mut with_splitter_ptr);

            resize_window_to_screen(&mut main_window_ptr, 0.8);
            load_stylesheet("/Users/jgerber/bin/pbgui.qss", main_window_ptr);

            let withpackage_save = item_list_ptr.borrow_mut().save_button.clone();
            let versionpin_table = vpin_tablewidget_ptr.clone();

            main_window_ptr.show();
            // Create the MainWindow instance, set up signals and slots, and return
            // the newly minted instance. We are done.
            let main_window_inst = MainWindow {
                _db: db,
                _main: main_window,
                _main_toolbar: main_toolbar,
                _packages_tree: packages_ptr,
                _vpin_table: vpin_tablewidget_ptr,
                _save_button: save_button,
                _pinchanges_list: pinchanges_ptr,
                _dist_popup_menu: dist_popup_menu,
                _dist_popup_action: choose_dist_action,
                // _package_popup_menu: line_edit_popup_menu,
                _pin_changes_button: pinchanges_button_ptr,
                _history_button: history_button_ptr,
                _left_toolbar_actions: left_toolbar_actions,
                search_shortcut: search_shortcut.into_ptr(),
                // slots
                save_withpackages: Slot::new(
                    enclose_all! { (pinchange_cache, item_list_ptr) (mut pinchanges_ptr) move || {
                        store_withpackage_changes::store_withpackage_changes(
                            item_list_ptr.clone(),
                            versionpin_table,
                            &mut pinchanges_ptr,
                            pinchange_cache.clone(),
                        );
                    }},
                ),

                distribution_changed: SlotOfQItemSelectionQItemSelection::new(
                    enclose! { (item_list_ptr)
                    move |selected: QRef<QItemSelection>, _deselected: QRef<QItemSelection>| {
                        let ind = selected.indexes();
                        if ind.count_0a() > 0 {
                            let txid = ind.at(COL_REV_TXID);
                            update_withpackages(
                                txid.row(),
                                &mut vpin_tablewidget_ptr,
                                item_list_ptr.clone(),
                                cache.clone(),
                            );
                        } else {
                            item_list_ptr.borrow_mut().clear();
                        }
                    }},
                ),

                revision_changed: SlotOfQItemSelectionQItemSelection::new(
                    move |selected: QRef<QItemSelection>, _deselected: QRef<QItemSelection>| {
                        let ind = selected.indexes();
                        if ind.count_0a() > 0 {
                            let txid = ind.at(COL_REV_TXID);
                            update_changes_table(txid.row(), revisions_ptr, changes_table_ptr);
                        } else {
                            changes_table_ptr.clear_contents();
                            changes_table_ptr.set_row_count(0);
                        }
                    },
                ),

                // clear_package: Slot::new(move || {
                //     line_edit_ptr.clear();
                // }),

                // show_line_edit_menu: SlotOfQPoint::new(move |pos: QRef<QPoint>| {
                //     let _action = line_edit_popup_menu_ptr
                //         .exec_1a_mut(line_edit_ptr.map_to_global(pos).as_ref());
                // }),
                show_dist_menu: SlotOfQPoint::new(move |pos: QRef<QPoint>| {
                    if vpin_tablewidget_ptr.is_null() {
                        log::error!("vpin_tablewidget_ptr is null");
                        return;
                    }
                    if dist_popup_menu_ptr.is_null() {
                        log::error!("dist_popup_menu_ptr is null");
                        return;
                    }
                    let _action = dist_popup_menu_ptr
                        .exec_1a_mut(vpin_tablewidget_ptr.map_to_global(pos).as_ref());
                }),

                save_clicked: Slot::new(enclose! { (pinchange_cache, main_toolbar_ptr) move || {
                    save_versionpin_changes(
                        main_widget_ptr,
                        &mut pinchanges_ptr,
                        main_toolbar_ptr.clone(),
                        pinchange_cache.clone(),
                    );
                } }),

                query_button_clicked: Slot::new(enclose! {(main_toolbar_ptr) move || {
                    update_vpin_table(
                        main_toolbar_ptr.clone(),
                        &search_shows,
                        vpin_tablewidget_ptr,
                    );
                }}),

                choose_distribution_triggered: Slot::new(enclose! { (pinchange_cache) move || {
                    if vpin_tablewidget_ptr.is_null() {
                        log::error!("Error: attempted to access null pointer in choose_distribution_tribbered");
                        return;
                    }
                    if vpin_tablewidget_ptr.row_count() == 0 {
                        return;
                    }
                    let current_row = vpin_tablewidget_ptr.current_row();
                    choose_alternative_distribution(
                        current_row,
                        vpin_tablewidget_ptr,
                        main_widget_ptr,
                        pinchanges_ptr,
                        pinchange_cache.clone(),
                    );
                }}),

                select_pin_changes: Slot::new(move || {
                    stacked_ptr.set_current_index(0);
                    controls_ptr.set_current_index(0);
                }),

                select_history: Slot::new(move || {
                    select_history(&mut revisions_ptr, &mut stacked_ptr);
                    controls_ptr.set_current_index(1);
                }),
                toggle_packages_tree: SlotOfBool::new(move |state: bool| {
                    let mut frame = with_splitter_ptr.widget(0);
                    frame.set_visible(state);
                }),
                toggle_withs: SlotOfBool::new(move |state: bool| {
                    let mut frame = with_splitter_ptr.widget(2);
                    frame.set_visible(state);
                }),

                toggle_vpin_changes: SlotOfBool::new(move |state: bool| {
                    let mut frame = vpin_table_splitter.widget(1);
                    frame.set_visible(state);
                }),
            };

            //
            // connect signals to slots
            //
            pinchanges_button_ptr
                .clicked()
                .connect(&main_window_inst.select_pin_changes);

            history_button_ptr
                .clicked()
                .connect(&main_window_inst.select_history);

            main_toolbar_ptr
                .borrow()
                .query_btn
                .clicked()
                .connect(&main_window_inst.query_button_clicked);

            save_button
                .clicked()
                .connect(&main_window_inst.save_clicked);

            vpin_tablewidget_ptr
                .custom_context_menu_requested()
                .connect(&main_window_inst.show_dist_menu);

            choose_dist_action
                .triggered()
                .connect(&main_window_inst.choose_distribution_triggered);

            // main_toolbar.borrow().line_edit
            //     .custom_context_menu_requested()
            //     .connect(&main_window_inst.show_line_edit_menu);

            // choose_line_edit_clear_action
            //     .triggered()
            //     .connect(&main_window_inst.clear_package);

            revisions_ptr
                .selection_model()
                .selection_changed()
                .connect(&main_window_inst.revision_changed);

            vpin_tablewidget_ptr
                .selection_model()
                .selection_changed()
                .connect(&main_window_inst.distribution_changed);

            view_packages
                .toggled()
                .connect(&main_window_inst.toggle_packages_tree);

            view_withs.toggled().connect(&main_window_inst.toggle_withs);

            view_pin_changes
                .toggled()
                .connect(&main_window_inst.toggle_vpin_changes);

            withpackage_save
                .clicked()
                .connect(&main_window_inst.save_withpackages);
            main_window_inst
                .search_shortcut
                .activated()
                .connect(&main_window_inst.query_button_clicked);

            // configuration
            view_withs.set_checked(false);

            main_window_inst
        }
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

fn create_top_toolbar(mut parent: MutPtr<QMainWindow>) -> toolbar::MainToolbar {
    let mut tb = toolbar::create(&mut parent);
    tb.set_default_stylesheet();
    let client = ClientProxy::connect().expect("Unable to connect via ClientProxy");
    let mut db = PackratDb::new(client);

    let results = db
        .find_all_levels()
        .query()
        .expect("unable to find_all_levels");
    let results = results.iter().map(|s| s.level.as_str()).collect::<Vec<_>>();
    tb.set_level_items(results);

    let results = db
        .find_all_roles()
        .query()
        .expect("unable to find_all_roless");
    let results = results.iter().map(|s| s.role.as_str()).collect::<Vec<_>>();
    tb.set_role_items(results);

    let results = db
        .find_all_platforms()
        .query()
        .expect("unable to find_all_platforms");
    let results = results.iter().map(|s| s.name.as_str()).collect::<Vec<_>>();
    tb.set_platform_items(results);

    let results = db
        .find_all_sites()
        .query()
        .expect("unable to find_all_platforms");
    let results = results.iter().map(|s| s.name.as_str()).collect::<Vec<_>>();
    tb.set_site_items(results);
    tb
}
