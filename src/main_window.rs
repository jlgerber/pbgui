use crate::{
    bottom_stacked_widget::create_bottom_stacked_widget,
    center_widget,
    choose_distribution::choose_alternative_distribution,
    constants::COL_REV_TXID,
    left_toolbar, package_lineedit, query_button,
    save_versionpin_changes::save_versionpin_changes,
    search_comboboxes,
    select_history::select_history,
    top_toolbar,
    update_changes_table::update_changes_table,
    update_versionpin_table::update_vpin_table,
    update_withpackages::update_withpackages,
    utility::{create_hlayout, create_vlayout, load_stylesheet, qs, resize_window_to_screen},
    versionpin_table, versionpin_table_splitter, withpackage_widget, withs_splitter,
    LeftToolBarActions,
};
use log;
use packybara::packrat::PackratDb;
use qt_core::{
    QItemSelection, QPoint, QString, Slot, SlotOfBool, SlotOfQItemSelectionQItemSelection,
};
use qt_widgets::{
    cpp_core::{CppBox, MutPtr, Ref},
    QAction, QLineEdit, QMainWindow, QMenu, QMenuBar, QPushButton, QTableWidget, QVBoxLayout,
    QWidget, SlotOfQPoint,
};
use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::rc::Rc;

pub struct MainWindow<'a> {
    _db: &'a mut PackratDb,
    _main: CppBox<QMainWindow>,
    _query_button: MutPtr<QPushButton>,
    _pkg_line_edit: MutPtr<QLineEdit>,
    _vpin_table: MutPtr<QTableWidget>,
    _pinchanges_list: MutPtr<QTableWidget>,
    _save_button: MutPtr<QPushButton>,
    _pin_changes_button: MutPtr<QPushButton>,
    _history_button: MutPtr<QPushButton>,
    _dist_popup_menu: CppBox<QMenu>,
    _package_popup_menu: CppBox<QMenu>,
    _dist_popup_action: MutPtr<QAction>,
    _left_toolbar_actions: LeftToolBarActions,
    // Slots
    query_button_clicked: Slot<'a>,
    save_clicked: Slot<'a>,
    choose_distribution_triggered: Slot<'a>,
    show_dist_menu: SlotOfQPoint<'a>,
    clear_package: Slot<'a>,
    show_line_edit_menu: SlotOfQPoint<'a>,
    select_pin_changes: Slot<'a>,
    select_history: Slot<'a>,
    _toggle_withs: SlotOfBool<'a>,
    _toggle_vpin_changes: SlotOfBool<'a>,
    revision_changed: SlotOfQItemSelectionQItemSelection<'a>,
    distribution_changed: SlotOfQItemSelectionQItemSelection<'a>,
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
    /// ```
    /// main_window (QMainApplication)
    /// - main_window_bar (QMenuBar)
    /// - main_widget (QWidget)
    /// -- main_layout (QHBoxLayout)
    /// --- with_splitter (QSplitter)
    /// ---- center_widget (QWidget)
    /// ----- center_layout (QVBoxLayourt)
    /// ---- withpackage_ptr (MutPtr<QListWidget>)
    /// ```
    pub fn new(mut db: &'a mut PackratDb) -> MainWindow<'a> {
        unsafe {
            // create the main window, menus, central widget and layout
            let (mut main_window, main_widget_ptr, mut main_layout_ptr) = create_main_window();
            let mut main_window_ptr = main_window.as_mut_ptr();
            //
            // Create the top toolbar which contains the search controls
            //
            // create the horizontal layout
            let mut top_toolbar_hlayout = create_hlayout();
            let mut top_toolbar_hlayout_ptr = top_toolbar_hlayout.as_mut_ptr();
            // create the comboboxes
            let mut combobox_ctrls =
                search_comboboxes::create(&mut db, &mut top_toolbar_hlayout_ptr);
            // create the package line edit
            let (mut line_edit_ptr, mut line_edit_popup_menu, choose_line_edit_clear_action) =
                package_lineedit::create(&mut top_toolbar_hlayout_ptr);
            let mut line_edit_popup_menu_ptr = line_edit_popup_menu.as_mut_ptr();
            // create query button
            let mut query_button_ptr = query_button::create(&mut top_toolbar_hlayout_ptr);
            query_button_ptr.set_object_name(&qs("QueryButton"));
            // create the toolbar, passing in its layout with previosly registered
            // widgets. The QToolBar will assume ownership
            top_toolbar::create(&mut main_window_ptr, top_toolbar_hlayout);
            //
            // create left toolbar
            //
            let left_toolbar_actions = left_toolbar::create(&mut main_window_ptr);
            let view_withs = left_toolbar_actions.view_withs;
            let view_pin_changes = left_toolbar_actions.view_vpin_changes;
            let search_shows = left_toolbar_actions.search_shows;
            //
            // create the splitter between the center widget and the withs
            //
            let mut with_splitter_ptr = withs_splitter::create(&mut main_layout_ptr);
            // let mut with_splitter = QSplitter::new();
            // let mut with_splitter_ptr = with_splitter.as_mut_ptr();
            // with_splitter.set_orientation(Orientation::Horizontal);
            // // add the splitter into the main layout
            // main_layout_ptr.add_widget(with_splitter.into_ptr());

            //
            // create the center widget
            //
            let mut center_layout_ptr = center_widget::create(&mut with_splitter_ptr);
            //
            // Versionpin Table
            //
            // create the versionpin table splitter
            let mut vpin_table_splitter = versionpin_table_splitter::create(&mut center_layout_ptr);
            // create the versionpin table
            let mut vpin_tablewidget_ptr = versionpin_table::create(&mut vpin_table_splitter);
            let (
                pinchanges_ptr,
                mut revisions_ptr,
                mut changes_table_ptr,
                save_button,
                mut stacked_ptr,
                pinchanges_button_ptr,
                history_button_ptr,
                mut controls_ptr,
            ) = create_bottom_stacked_widget(&mut vpin_table_splitter);
            //
            // setup popup menu for versionpin table
            //
            let mut dist_popup_menu = QMenu::new();
            let choose_dist_action =
                dist_popup_menu.add_action_q_string(&QString::from_std_str("Change Version"));
            let _choose_withs_action =
                dist_popup_menu.add_action_q_string(&QString::from_std_str("Withs"));
            let mut dist_popup_menu_ptr = dist_popup_menu.as_mut_ptr();
            //
            // create the WithPackage
            //
            let mut withpackage_ptr = withpackage_widget::create(&mut with_splitter_ptr);
            // prepare data for slot closures
            let usage = Rc::new(RefCell::new(HashMap::<i32, i32>::new()));
            let usage_ptr = Rc::clone(&usage);
            let update_cnt = Rc::new(Cell::new(0));
            let update_cnt_ptr = Rc::clone(&update_cnt);
            let mut pinchanges_ptr = pinchanges_ptr.clone();
            let dist_usage_ptr = usage_ptr.clone();
            let dist_update_cnt_ptr = update_cnt_ptr.clone();
            //
            // final housekeeping before showing main window
            //
            versionpin_table_splitter::set_sizes(&mut vpin_table_splitter);
            withs_splitter::set_sizes(&mut with_splitter_ptr);
            resize_window_to_screen(&mut main_window_ptr, 0.8);
            load_stylesheet(main_window_ptr);
            main_window_ptr.show();
            //
            // Create the MainWindow instance, set up signals and slots, and return
            // the newly minted instance. We are done.
            let form = MainWindow {
                distribution_changed: SlotOfQItemSelectionQItemSelection::new(
                    move |selected: Ref<QItemSelection>, _deselected: Ref<QItemSelection>| {
                        let ind = selected.indexes();
                        // dont need this anymore. However, this is how you go about
                        // casting...
                        //let mut withpackage: MutPtr<QListWidget> =
                        //    withpackage_ptr.widget().dynamic_cast_mut();
                        if ind.count_0a() > 0 {
                            let txid = ind.at(COL_REV_TXID);
                            update_withpackages(
                                txid.row(),
                                &mut vpin_tablewidget_ptr,
                                &mut withpackage_ptr,
                            );
                        } else {
                            withpackage_ptr.clear();
                        }
                    },
                ),
                revision_changed: SlotOfQItemSelectionQItemSelection::new(
                    move |selected: Ref<QItemSelection>, _deselected: Ref<QItemSelection>| {
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
                clear_package: Slot::new(move || {
                    line_edit_ptr.clear();
                }),
                show_line_edit_menu: SlotOfQPoint::new(move |pos: Ref<QPoint>| {
                    let _action = line_edit_popup_menu_ptr
                        .exec_1a_mut(line_edit_ptr.map_to_global(pos).as_ref());
                }),
                show_dist_menu: SlotOfQPoint::new(move |pos: Ref<QPoint>| {
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
                //
                // The save button governing versionpin distribution changes has been clicked.
                // Gather distribution updates from the versionpin table and update the
                // database
                //
                save_clicked: Slot::new(move || {
                    save_versionpin_changes(
                        main_widget_ptr,
                        &mut pinchanges_ptr,
                        &mut query_button_ptr,
                    );
                }),
                //
                // Add query_button_clicked Slot
                //
                query_button_clicked: Slot::new(move || {
                    update_vpin_table(
                        *combobox_ctrls.dir(),
                        line_edit_ptr,
                        *combobox_ctrls.level(),
                        *combobox_ctrls.role(),
                        *combobox_ctrls.platform(),
                        *combobox_ctrls.site(),
                        &search_shows,
                        vpin_tablewidget_ptr,
                    );
                }),
                //
                // choose_distribution_triggered slot.
                //
                choose_distribution_triggered: Slot::new(move || {
                    if vpin_tablewidget_ptr.row_count() == 0 {
                        return;
                    }
                    let current_row = vpin_tablewidget_ptr.current_row();

                    choose_alternative_distribution(
                        current_row,
                        vpin_tablewidget_ptr,
                        dist_usage_ptr.clone(),
                        main_widget_ptr,
                        pinchanges_ptr,
                        dist_update_cnt_ptr.clone(),
                    );
                }),
                select_pin_changes: Slot::new(move || {
                    stacked_ptr.set_current_index(0);
                    controls_ptr.set_current_index(0);
                }),
                select_history: Slot::new(move || {
                    select_history(&mut revisions_ptr, &mut stacked_ptr);
                    controls_ptr.set_current_index(1);
                }),
                // We have a problem here. i have no way of adding
                _toggle_withs: SlotOfBool::new(move |state: bool| {
                    let mut frame = with_splitter_ptr.widget(1);
                    frame.set_visible(state);
                    //withpackage_ptr.set_visible(state);
                }),
                _toggle_vpin_changes: SlotOfBool::new(move |state: bool| {
                    let mut frame = vpin_table_splitter.widget(1);
                    frame.set_visible(state);
                }),
                _db: db,
                _main: main_window,
                _vpin_table: vpin_tablewidget_ptr,
                _query_button: query_button_ptr,
                _save_button: save_button,
                _pkg_line_edit: line_edit_ptr,
                _pinchanges_list: pinchanges_ptr,
                _dist_popup_menu: dist_popup_menu,
                _dist_popup_action: choose_dist_action,
                _package_popup_menu: line_edit_popup_menu,
                _pin_changes_button: pinchanges_button_ptr,
                _history_button: history_button_ptr,
                _left_toolbar_actions: left_toolbar_actions,
            };
            //
            // connect signals to slots
            //
            //toggle_withs_action.toggled().connect(&form.toggle_withs);
            pinchanges_button_ptr
                .clicked()
                .connect(&form.select_pin_changes);
            history_button_ptr.clicked().connect(&form.select_history);
            query_button_ptr
                .clicked()
                .connect(&form.query_button_clicked);
            save_button.clicked().connect(&form.save_clicked);
            vpin_tablewidget_ptr
                .custom_context_menu_requested()
                .connect(&form.show_dist_menu);
            choose_dist_action
                .triggered()
                .connect(&form.choose_distribution_triggered);
            // clear_action.triggered().connect(&form.clear_package);
            line_edit_ptr
                .custom_context_menu_requested()
                .connect(&form.show_line_edit_menu);
            choose_line_edit_clear_action
                .triggered()
                .connect(&form.clear_package);
            revisions_ptr
                .selection_model()
                .selection_changed()
                .connect(&form.revision_changed);
            vpin_tablewidget_ptr
                .selection_model()
                .selection_changed()
                .connect(&form.distribution_changed);
            view_withs.toggled().connect(&form._toggle_withs);
            view_pin_changes
                .toggled()
                .connect(&form._toggle_vpin_changes);
            form
        }
    }
}
/*
let slot = SlotOfBool::new(move |on: bool| {
            println!("toggled");
        });
        withs_action.toggled().connect(&slot);
*/

// create the main window, the main menubar, and the central widget
fn create_main_window() -> (CppBox<QMainWindow>, MutPtr<QWidget>, MutPtr<QVBoxLayout>) {
    unsafe {
        let mut main_window = QMainWindow::new_0a();
        // the qmainwindow takes ownership of the menubar,
        // even though it takes a MutPtr instead of a Cpp
        let main_menu_bar = QMenuBar::new_0a();
        main_window.set_menu_bar(main_menu_bar.into_ptr());
        //
        // main_widget - central widget of teh main_window
        //
        let mut main_widget = QWidget::new_0a();
        let main_widget_ptr = main_widget.as_mut_ptr();
        //
        // main_layout
        //
        let mut main_layout = create_vlayout();
        let main_layout_ptr = main_layout.as_mut_ptr();
        main_widget.set_layout(main_layout.into_ptr());
        // set main_widget as the central widget in main_window
        main_window.set_central_widget(main_widget.into_ptr());

        (main_window, main_widget_ptr, main_layout_ptr)
    }
}
