use crate::{
    bottom_stacked_widget::create_bottom_stacked_widget,
    choose_distribution::choose_alternative_distribution,
    constants::COL_REV_TXID,
    save_versionpin_changes::save_versionpin_changes,
    select_history::select_history,
    update_changes_table::update_changes_table,
    update_versionpin_table::update_vpin_table,
    update_withpackages::update_withpackages,
    utility::{create_hlayout, load_stylesheet, qs},
    versionpin_table::setup_table,
    withpackage_widget, {combo_boxes, create_query_button},
};
use log;
use packybara::packrat::PackratDb;
use qt_core::{
    ContextMenuPolicy, Orientation, QItemSelection, QListOfInt, QPoint, QString, Slot,
    SlotOfQItemSelectionQItemSelection, WidgetAttribute,
};
use qt_gui::QIcon;
use qt_widgets::{
    cpp_core::{CppBox, MutPtr, Ref},
    q_line_edit::ActionPosition,
    QAction, QLineEdit, QListWidget, QMainWindow, QMenu, QPushButton, QSplitter, QTableWidget,
    QVBoxLayout, QWidget, SlotOfQPoint,
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
    query_button_clicked: Slot<'a>,
    save_clicked: Slot<'a>,
    choose_distribution_triggered: Slot<'a>,
    show_dist_menu: SlotOfQPoint<'a>,
    clear_package: Slot<'a>,
    show_line_edit_menu: SlotOfQPoint<'a>,
    select_pin_changes: Slot<'a>,
    select_history: Slot<'a>,
    revision_changed: SlotOfQItemSelectionQItemSelection<'a>,
    distribution_changed: SlotOfQItemSelectionQItemSelection<'a>,
}

impl<'a> MainWindow<'a> {
    //
    // Create Main Widget
    //
    pub fn new(mut db: &'a mut PackratDb) -> MainWindow<'a> {
        unsafe {
            let mut main_window = QMainWindow::new_0a();
            main_window.set_base_size_2a(1200, 800);
            // parent root_widget
            let mut root_widget = QWidget::new_0a();
            //root_widget.set_base_size_2a(1200, 800);
            let root_widget_ptr = root_widget.as_mut_ptr();
            // top vertical layout
            let mut root_layout = QVBoxLayout::new_0a();
            let mut root_layout_ptr = root_layout.as_mut_ptr();
            root_widget.set_layout(root_layout.into_ptr());
            main_window.set_central_widget(root_widget.into_ptr());
            // header layout
            let mut hlayout = create_hlayout();
            let mut hlayout_ptr = hlayout.as_mut_ptr();
            //root_layout_ptr.add_layout_1a(hlayout.into_ptr());
            // setup comboboxes in header
            let (level_ptr, role_ptr, platform_ptr, site_ptr, dir_ptr) =
                combo_boxes(&mut db, &mut hlayout_ptr);
            // LINE EDIT
            let mut line_edit = QLineEdit::new();
            line_edit.set_attribute_2a(WidgetAttribute::WAMacShowFocusRect, false);
            line_edit.set_object_name(&QString::from_std_str("packageLineEdit"));
            let clear_icon = QIcon::from_q_string(&QString::from_std_str(":/images/clear.png"));
            //println!("icon is null {}", clear_icon.is_null());
            if clear_icon.is_null() {
                log::warn!("The :/images/clear.png icon was unable to be located.");
            }
            let clear_action = line_edit.add_action_q_icon_action_position(
                clear_icon.as_ref(),
                ActionPosition::TrailingPosition,
            );
            line_edit.set_context_menu_policy(ContextMenuPolicy::CustomContextMenu);
            let mut line_edit_popup_menu = QMenu::new();
            let mut line_edit_popup_menu_ptr = line_edit_popup_menu.as_mut_ptr();
            let choose_line_edit_clear_action =
                line_edit_popup_menu.add_action_q_string(&QString::from_std_str("Clear"));

            let mut line_edit_ptr = line_edit.as_mut_ptr();
            //root_layout_ptr.add_widget(line_edit.into_ptr());

            hlayout_ptr.add_widget(line_edit.into_ptr());
            // create query button
            let mut button_ptr = create_query_button(&mut hlayout_ptr);
            button_ptr.set_object_name(&qs("QueryButton"));
            //
            // qtoolbar setup
            //
            let mut top_toolbar = main_window.add_tool_bar_q_string(&qs("TopToolPar"));
            top_toolbar.set_floatable(false);
            top_toolbar.set_movable(false);

            let mut toolbar_widget = QWidget::new_0a();
            toolbar_widget.set_object_name(&qs("ToobarWidget"));
            toolbar_widget.set_layout(hlayout.into_ptr());
            top_toolbar.add_widget(toolbar_widget.into_ptr());
            // Create Splitter between query results and action logger
            let mut vsplit = QSplitter::new();
            let mut vsplit_ptr = vsplit.as_mut_ptr();
            vsplit.set_orientation(Orientation::Vertical);
            // set splitter sizing
            // setup the main table widget
            let mut vpin_tablewidget_ptr = setup_table(&mut vsplit_ptr);
            let (
                pinchanges_ptr,
                mut revisions_ptr,
                mut changes_table_ptr,
                save_button,
                mut stacked_ptr,
                pinchanges_button_ptr,
                history_button_ptr,
                mut controls_ptr,
            ) = create_bottom_stacked_widget(&mut vsplit_ptr);
            //
            // setup popup menu for versionpin table
            //
            let mut dist_popup_menu = QMenu::new();
            let choose_dist_action =
                dist_popup_menu.add_action_q_string(&QString::from_std_str("Change Version"));
            let _choose_withs_action =
                dist_popup_menu.add_action_q_string(&QString::from_std_str("Withs"));
            let mut dist_popup_menu_ptr = dist_popup_menu.as_mut_ptr();
            // set the style sheet
            load_stylesheet(main_window.as_mut_ptr());
            //
            // Setup WithPackage
            //
            //create_withpackage_widget
            let (withpackage_ptr, _with_action) =
                withpackage_widget::create(&mut main_window.as_mut_ptr());
            main_window.show();
            //
            let usage = Rc::new(RefCell::new(HashMap::<i32, i32>::new()));
            let usage_ptr = Rc::clone(&usage);
            let update_cnt = Rc::new(Cell::new(0));
            let update_cnt_ptr = Rc::clone(&update_cnt);
            let mut pinchanges_ptr = pinchanges_ptr.clone();
            let dist_usage_ptr = usage_ptr.clone();
            let dist_update_cnt_ptr = update_cnt_ptr.clone();
            let mut splitter_sizes = QListOfInt::new();
            splitter_sizes.append_int(Ref::from_raw_ref(&(500 as i32)));
            splitter_sizes.append_int(Ref::from_raw_ref(&(300 as i32)));
            vsplit.set_sizes(&splitter_sizes);
            root_layout_ptr.add_widget(vsplit.into_ptr());
            let form = MainWindow {
                distribution_changed: SlotOfQItemSelectionQItemSelection::new(
                    move |selected: Ref<QItemSelection>, _deselected: Ref<QItemSelection>| {
                        let ind = selected.indexes();
                        let mut withpackage: MutPtr<QListWidget> =
                            withpackage_ptr.widget().dynamic_cast_mut();
                        if ind.count_0a() > 0 {
                            let txid = ind.at(COL_REV_TXID);
                            update_withpackages(
                                txid.row(),
                                &mut vpin_tablewidget_ptr,
                                &mut withpackage,
                            );
                        } else {
                            withpackage.clear();
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
                // save clicked
                //
                save_clicked: Slot::new(move || {
                    save_versionpin_changes(root_widget_ptr, &mut pinchanges_ptr);
                }),
                //
                // Add query_button_clicked Slot
                //
                query_button_clicked: Slot::new(move || {
                    update_vpin_table(
                        dir_ptr,
                        line_edit_ptr,
                        level_ptr,
                        role_ptr,
                        platform_ptr,
                        site_ptr,
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
                        root_widget_ptr,
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
                _db: db,
                _main: main_window,
                _vpin_table: vpin_tablewidget_ptr,
                _query_button: button_ptr,
                _save_button: save_button,
                _pkg_line_edit: line_edit_ptr,
                _pinchanges_list: pinchanges_ptr,
                _dist_popup_menu: dist_popup_menu,
                _dist_popup_action: choose_dist_action,
                _package_popup_menu: line_edit_popup_menu,
                _pin_changes_button: pinchanges_button_ptr,
                _history_button: history_button_ptr,
            };
            //
            // connect signals to slots
            //
            pinchanges_button_ptr
                .clicked()
                .connect(&form.select_pin_changes);
            history_button_ptr.clicked().connect(&form.select_history);
            button_ptr.clicked().connect(&form.query_button_clicked);
            save_button.clicked().connect(&form.save_clicked);
            vpin_tablewidget_ptr
                .custom_context_menu_requested()
                .connect(&form.show_dist_menu);
            choose_dist_action
                .triggered()
                .connect(&form.choose_distribution_triggered);
            clear_action.triggered().connect(&form.clear_package);
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
            form
        }
    }
}
