use crate::messaging::OMsg;
use crate::messaging::Sender;
use crate::{
    bottom_stacked_widget::create_bottom_stacked_widget,
    cache::PinChangesCache,
    center_widget,
    choose_distribution::choose_alternative_distribution,
    constants::COL_REV_TXID,
    left_toolbar, package_withs_list, packages_tree,
    save_packages_xml::save_packages_xml,
    save_versionpin_changes::save_versionpin_changes,
    select_history::select_history,
    store_withpackage_changes,
    update_changes_table::update_changes_table,
    update_versionpin_table::update_vpin_table,
    update_withpackages::update_withpackages,
    utility::{create_vlayout, load_stylesheet, qs, resize_window_to_screen},
    versionpin_table, versionpin_table_splitter, withs_splitter, LeftToolBarActions,
};
use log;
use pbgui_logger::LogWin;
use pbgui_menubar::MenuBar;
use pbgui_toolbar::toolbar;
use pbgui_tree::tree;
use pbgui_withs::WithsList;

use qt_core::{
    QItemSelection, QPoint, QString, Slot, SlotOfBool, SlotOfQItemSelectionQItemSelection,
};
use qt_gui::QIcon;
use qt_gui::QKeySequence;
use qt_widgets::{
    cpp_core::{CppBox, MutPtr, Ref as QRef},
    QAction, QMainWindow, QMenu, QMenuBar, QPushButton, QShortcut, QSplitter, QStackedWidget,
    QTableWidget, QToolButton, QVBoxLayout, QWidget, SlotOfQPoint,
};
use rustqt_utils::enclose;
use std::cell;
use std::cell::RefCell;
use std::rc::Rc;

/// Are we limiting our search to the current show or are we searching
/// globally?
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum SearchMode {
    Show,
    All,
}
/// Just as it sounds, MainWindow is the MainWindow struct, holding on
/// to various pointers that need to persist as well as top level slots
pub struct InnerMainWindow<'a> {
    main: MutPtr<QMainWindow>,
    main_widget: MutPtr<QWidget>,
    main_menubar: MenuBar<'a>,
    main_toolbar: Rc<toolbar::MainToolbar>,
    withs_splitter: MutPtr<QSplitter>,
    packages_tree: Rc<RefCell<tree::DistributionTreeView<'a>>>,
    package_withs_list: Rc<RefCell<WithsList<'a>>>,
    vpin_table: MutPtr<QTableWidget>,
    vpin_table_splitter: MutPtr<QSplitter>,
    vpin_requested_changes_table: MutPtr<QTableWidget>,
    pinchanges_cache: Rc<PinChangesCache>,
    bottom_stacked_widget: MutPtr<QStackedWidget>,
    bottom_ctrls_stacked_widget: MutPtr<QStackedWidget>,
    save_button: MutPtr<QPushButton>,
    pin_changes_button: MutPtr<QToolButton>,
    revision_changes_table: MutPtr<QTableWidget>,
    history_button: MutPtr<QToolButton>,
    revisions_table: MutPtr<QTableWidget>,
    log_win: Rc<LogWin<'a>>,
    log_button: MutPtr<QToolButton>,
    toggle_log_ctrls_button: MutPtr<QPushButton>,
    dist_popup_menu: MutPtr<QMenu>,
    dist_popup_action: MutPtr<QAction>,
    left_toolbar_actions: LeftToolBarActions,
    search_shortcut: MutPtr<QShortcut>,
}

impl<'a> InnerMainWindow<'a> {
    /// New up the MainWindow. The MainWindow hosts the InnerMainWindow,
    /// which houses all of the accessible main window components, along with
    /// private owned components (exposed as MutPtrs on the InnerMainWinodw),
    /// and all of the main window Slots.
    ///
    /// In this way, responibilities are split between the InnerMainWindow, and MainWindow.
    /// This is a common pattern, mainly designed to allow this author to take advantage
    /// of an api in slots. Without the division, we would have to duplicate logic between
    /// slots and external consumers.
    pub fn new() -> (
        InnerMainWindow<'a>,
        CppBox<QMainWindow>,
        CppBox<QMenu>,
        CppBox<QIcon>,
    ) {
        unsafe {
            // create the main window, menus, central widget and layout
            let (mut main_window, main_widget_ptr, mut main_layout_ptr, main_menubar) =
                create_main_window();
            let mut main_window_ptr = main_window.as_mut_ptr();
            let main_toolbar = Rc::new(create_top_toolbar(main_window_ptr.clone()));

            // create left toolbar
            let left_toolbar_actions =
                left_toolbar::create(&mut main_window_ptr, main_menubar.inner());
            let mut view_withs = left_toolbar_actions.view_withs;

            // create the splitter between the center widget and the withs
            let mut with_splitter_ptr = withs_splitter::create(&mut main_layout_ptr);

            // create packages treeview on left side
            let packages_ptr = packages_tree::create(with_splitter_ptr);

            // create the center widget
            let mut center_layout_ptr = center_widget::create(&mut with_splitter_ptr);

            // create the versionpin table splitter
            let mut vpin_table_splitter = versionpin_table_splitter::create(&mut center_layout_ptr);
            // create the versionpin table
            let vpin_tablewidget_ptr = versionpin_table::create(&mut vpin_table_splitter);

            let (
                pinchanges_ptr,
                revisions_ptr,
                changes_table_ptr,
                log_win,
                save_button,
                stacked_ptr,
                pinchanges_button_ptr,
                history_button_ptr,
                log_button,
                toggle_log_ctrls_button,
                controls_ptr,
                mode_icon,
            ) = create_bottom_stacked_widget(vpin_table_splitter.clone(), main_menubar.inner());

            // setup popup menu for versionpin table
            let mut dist_popup_menu = QMenu::new();
            let choose_dist_action =
                dist_popup_menu.add_action_q_string(&QString::from_std_str("Change Version"));

            let _choose_withs_action =
                dist_popup_menu.add_action_q_string(&QString::from_std_str("Withs"));

            let dist_popup_menu_ptr = dist_popup_menu.as_mut_ptr();

            // create the with with package list on the right hand side
            let item_list_ptr = package_withs_list::create(with_splitter_ptr);
            item_list_ptr.borrow_mut().set_add_mode();
            item_list_ptr.borrow_mut().set_cb_max_visible_items(30);

            // shortcuts
            let key_seq = QKeySequence::from_q_string(&qs("Ctrl+Return"));
            let search_shortcut =
                QShortcut::new_2a(key_seq.as_ref(), item_list_ptr.borrow().main());

            // persist data
            let pinchanges_cache = Rc::new(PinChangesCache::new());
            // final housekeeping before showing main window

            versionpin_table_splitter::set_sizes(vpin_table_splitter.clone());
            withs_splitter::set_sizes(&mut with_splitter_ptr);

            resize_window_to_screen(&mut main_window_ptr, 0.8);
            load_stylesheet("/Users/jgerber/bin/pbgui.qss", main_window_ptr.clone());

            main_window_ptr.show();

            // Create the MainWindow instance, set up signals and slots, and return
            // the newly minted instance. We are done.
            let main_window_inst = InnerMainWindow {
                main: main_window_ptr,
                main_widget: main_widget_ptr,
                main_menubar,
                main_toolbar: main_toolbar,
                withs_splitter: with_splitter_ptr,
                packages_tree: packages_ptr,
                package_withs_list: item_list_ptr.clone(),
                vpin_table: vpin_tablewidget_ptr,
                vpin_table_splitter,
                save_button: save_button,
                vpin_requested_changes_table: pinchanges_ptr,
                pinchanges_cache,
                bottom_stacked_widget: stacked_ptr,
                bottom_ctrls_stacked_widget: controls_ptr,
                dist_popup_menu: dist_popup_menu_ptr,
                dist_popup_action: choose_dist_action,
                pin_changes_button: pinchanges_button_ptr,
                revision_changes_table: changes_table_ptr,
                history_button: history_button_ptr,
                revisions_table: revisions_ptr,
                log_win: Rc::new(log_win),
                log_button,
                toggle_log_ctrls_button,
                left_toolbar_actions: left_toolbar_actions,
                search_shortcut: search_shortcut.into_ptr(),
            };

            //
            // connect signals to slots
            //

            // configuration
            view_withs.set_checked(false);

            (main_window_inst, main_window, dist_popup_menu, mode_icon)
        }
    }

    /// Get a cloen fo the LogWin
    pub unsafe fn logger(&self) -> Rc<LogWin> {
        self.log_win.clone()
    }

    /// Returns a mutable pointer to the QMainWindow instance
    ///
    /// # Arguments
    /// * None
    ///
    /// # Returns
    /// * MutPtr<QMainWindow>
    pub unsafe fn main(&self) -> MutPtr<QMainWindow> {
        self.main
    }

    /// Returns a mutable pointer to the main widget under the QMainWindow
    ///
    /// # Arguments
    /// * None
    ///
    /// # Returns
    /// * MutPtr to the main widget, which is the main widget of the QMainWindow
    pub unsafe fn main_widget(&self) -> MutPtr<QWidget> {
        self.main_widget
    }

    pub unsafe fn main_menubar(&self) -> &MenuBar {
        &self.main_menubar
    }
    /// Returns a reference counted pointer to the PinChangedCache. This is not
    /// suitable to be moved between threads as it is `Rc`.
    ///
    /// # Arguments
    /// * None
    ///
    /// # Returns
    /// * Rc<PinChangesCache>
    pub fn cache(&self) -> Rc<PinChangesCache> {
        self.pinchanges_cache.clone()
    }

    /// Returns a mutable pointer to the QStackedWidget used to organize the
    /// bottom widget containing the changes, history, etc
    ///
    /// # Arguments
    /// * None
    ///
    /// # Returns
    /// * MutPtr<QStackedWidget>
    pub unsafe fn bottom_stacked_widget(&self) -> MutPtr<QStackedWidget> {
        self.bottom_stacked_widget
    }

    /// Returns a mutable pointer to the QStackedWidget used to provide specific
    /// controls in coordination with the bottom_stacked_widget. In otherwords, for
    /// each widget displayed in the bottom_stack, one may provide unique controls
    /// via this widget. It is displayed on the right side of the bar above the
    /// bottom_stacked_widget
    ///
    /// # Arguments
    /// * None
    ///
    /// # Returns
    /// * MutPtr<QStackedWidget>
    pub unsafe fn bottom_ctrls_stacked_widget(&self) -> MutPtr<QStackedWidget> {
        self.bottom_ctrls_stacked_widget
    }

    /// Returns a reference (Cell::Ref) wrapped DistributionTreeView instance
    ///
    /// # Arguments
    /// * None
    ///
    /// # Returns
    /// * Ref<DistributionTreeView>
    pub unsafe fn packages_tree(&self) -> cell::Ref<tree::DistributionTreeView<'a>> {
        self.packages_tree.borrow()
    }

    /// Returns a reference counted pointer to a RefCell wrapped DistributionTreeView
    ///
    /// # Arguments
    /// * None
    ///
    /// # Returns
    /// * Rc<RefCell<DistributionTreeView>>>
    pub unsafe fn tree(&self) -> Rc<RefCell<tree::DistributionTreeView<'a>>> {
        self.packages_tree.clone()
    }
    /// Returns an reference counted pointer to a RefCell wrapped WIthsList
    ///
    /// # Arguments
    /// * None
    ///
    /// # Returns
    /// * Rc<RefCell<WithsList>>>
    pub unsafe fn package_withs_list(&self) -> Rc<RefCell<WithsList<'a>>> {
        self.package_withs_list.clone()
    }

    /// Returns a mutable reference to the DistributionTreeView instance
    ///
    /// # Arguments
    /// * None
    ///
    /// # Returns
    /// * RefMut<DistributionTreeView>
    pub unsafe fn packages_tree_mut(&self) -> cell::RefMut<tree::DistributionTreeView<'a>> {
        self.packages_tree.borrow_mut()
    }

    /// returns a mutable pointer to the versionpin table widget
    ///
    /// # Arguments
    /// * None
    ///
    /// # Returns
    /// * MutPtr<QTableWidget>
    pub unsafe fn vpin_table(&self) -> MutPtr<QTableWidget> {
        self.vpin_table
    }

    /// Returns a mutable pointer to the versionpin table qsplitter
    ///
    /// # Arguments
    /// * None
    ///
    /// # Returns
    /// * MutPtr<QSplitter>
    pub unsafe fn vpin_table_splitter(&self) -> MutPtr<QSplitter> {
        self.vpin_table_splitter
    }

    /// Returns a mutable pointer to the table of requested vpin changes. These must be
    /// finalized by hitting the "save" button in order to be persisted to the database.
    ///
    /// # Arguments
    /// * None
    ///
    /// # Returns
    /// * MutPtr<QTableWidget>
    pub unsafe fn vpin_requested_changes_table(&self) -> MutPtr<QTableWidget> {
        self.vpin_requested_changes_table
    }

    /// Returns a mutable pointer to the pinchanges pushbutton
    ///
    /// # Arguments
    /// * None
    ///
    /// # Returns
    /// * MutPtr<QToolButton>
    pub unsafe fn pinchanges_button(&self) -> MutPtr<QToolButton> {
        self.pin_changes_button
    }

    /// Returns a reference counted pointer to the MainToolbar instance
    ///
    /// # Arguments
    /// * None
    ///
    /// # Returns
    /// * Rc<MainToolbar>
    pub unsafe fn main_toolbar(&self) -> Rc<toolbar::MainToolbar> {
        self.main_toolbar.clone()
    }

    /// Returns the a mutable pointer to the splitter between the main table widget and the
    /// withs list
    ///
    /// # Arguments
    /// * None
    ///
    /// # Returns
    /// * MutPtr<QSplitter>
    pub unsafe fn withs_splitter(&self) -> MutPtr<QSplitter> {
        self.withs_splitter
    }

    /// Return the current SearchMode
    pub fn search_mode(&self) -> SearchMode {
        unsafe {
            if self.left_toolbar_actions.search_shows.is_checked() {
                SearchMode::Show
            } else {
                SearchMode::All
            }
        }
    }
    /// Returns a mutable pointer to the save button
    ///
    /// # Arguments
    /// * None
    ///
    /// # Returns
    /// * MutPtr<QPushButton>
    pub unsafe fn save_button(&self) -> MutPtr<QPushButton> {
        self.save_button
    }

    /// Returns a mutable pointer to the history button
    ///
    /// # Arguments
    /// * None
    ///
    /// # Returns
    /// * MutPtr<QToolButton>
    pub unsafe fn history_button(&self) -> MutPtr<QToolButton> {
        self.history_button
    }
    /// Returns a mutable pointer to the log button
    ///
    /// # Arguments
    /// * None
    ///
    /// # Returns
    /// * MutPtr<QToolButton>
    pub unsafe fn log_button(&self) -> MutPtr<QToolButton> {
        self.log_button
    }

    /// Returns a mutable pointer to the toggle log controls button
    ///
    /// # Arguments
    /// * None
    ///
    /// # Returns
    /// * MutPtr<QPushButton>
    pub unsafe fn toggle_log_ctrls_button(&self) -> MutPtr<QPushButton> {
        self.toggle_log_ctrls_button
    }
    /// Returns a mutable pointer to the revisions table
    ///
    /// # Arguments
    /// * None
    ///
    /// # Returns
    /// * MutPtr<QTableWidget>
    pub unsafe fn revisions_table(&self) -> MutPtr<QTableWidget> {
        self.revisions_table
    }

    /// Returns a reference to the LeftToolBarActions instance, which collects
    /// all of the left toolbar's QActions and makes them available.
    ///
    /// # Arguments
    /// * None
    ///
    /// # Returns
    /// * &LeftToolBarActions
    pub fn left_toolbar_actions(&self) -> &LeftToolBarActions {
        &self.left_toolbar_actions
    }

    /// Returns a mutable pointer to the changes tablewidget. A revsion, may
    /// have one or more associated changes. These are they.
    ///
    /// # Arguments
    /// * None
    ///
    /// # Returns
    /// * MutPtr<QTableWidget>
    pub unsafe fn revision_changes_table(&self) -> MutPtr<QTableWidget> {
        self.revision_changes_table
    }

    /// Returns a mutable pointer to the distribution popup qmenu
    ///
    /// # Arguments
    /// * None
    ///
    /// # Returns
    /// * MutPtr<QMenu>
    pub unsafe fn dist_popup_menu(&self) -> MutPtr<QMenu> {
        self.dist_popup_menu
    }

    /// Returns a mutable pointer to the distribution popup action
    ///
    /// # Arguments
    /// * None
    ///
    /// # Returns
    /// * MutPtr<QAction>
    pub unsafe fn dist_popup_action(&self) -> MutPtr<QAction> {
        self.dist_popup_action
    }
}

// create the main window, the main menubar, and the central widget
fn create_main_window<'a>() -> (
    CppBox<QMainWindow>,
    MutPtr<QWidget>,
    MutPtr<QVBoxLayout>,
    MenuBar<'a>,
) {
    unsafe {
        let mut main_window = QMainWindow::new_0a();
        // the qmainwindow takes ownership of the menubar,
        // even though it takes a MutPtr instead of a Cpp
        let main_menu_bar = QMenuBar::new_0a();
        main_window.set_menu_bar(main_menu_bar.into_ptr());
        let main_window_ptr = main_window.as_mut_ptr();
        let main_menu_bar = MenuBar::create(main_window_ptr);
        // main_widget - central widget of teh main_window

        let mut main_widget = QWidget::new_0a();
        let main_widget_ptr = main_widget.as_mut_ptr();

        // main_layout

        let mut main_layout = create_vlayout();
        let main_layout_ptr = main_layout.as_mut_ptr();
        main_widget.set_layout(main_layout.into_ptr());

        // set main_widget as the central widget in main_window
        main_window.set_central_widget(main_widget.into_ptr());

        (main_window, main_widget_ptr, main_layout_ptr, main_menu_bar)
    }
}

fn create_top_toolbar(parent: MutPtr<QMainWindow>) -> toolbar::MainToolbar {
    let tb = toolbar::create(parent);
    tb.set_default_stylesheet();
    tb
}

/// Holds a reference counted pointer to the InnerMainWindow instance, along with
/// pointers to owned CppBoxed items, and all of the slots that call on the InnerMainWindow
pub struct MainWindow<'a> {
    main: Rc<InnerMainWindow<'a>>,
    _main_box: CppBox<QMainWindow>,
    _dist_popup_menu_box: CppBox<QMenu>,
    _logger_icon: CppBox<QIcon>,
    //
    // slots
    //
    query_button_clicked: Slot<'a>,
    save_clicked: Slot<'a>,
    choose_distribution_triggered: Slot<'a>,
    show_dist_menu: SlotOfQPoint<'a>,
    select_pin_changes: Slot<'a>,
    select_history: Slot<'a>,
    select_log: Slot<'a>,
    toggle_log_ctrls: SlotOfBool<'a>,
    toggle_packages_tree: SlotOfBool<'a>,
    toggle_withs: SlotOfBool<'a>,
    toggle_vpin_changes: SlotOfBool<'a>,
    revision_changed: SlotOfQItemSelectionQItemSelection<'a>,
    distribution_changed: SlotOfQItemSelectionQItemSelection<'a>,
    save_withpackages: Slot<'a>,
    save_packages_xml: Slot<'a>,
}

impl<'a> MainWindow<'a> {
    /// New up the MainWindow instance
    ///
    /// # Arguments
    /// * None
    ///
    /// # Returns
    /// * MainWindow instance
    pub unsafe fn new(to_thread_sender: Sender<OMsg>) -> MainWindow<'a> {
        let (pbgui_root, pbgui_main_cppbox, dist_popup_menu_box, logger_icon) =
            InnerMainWindow::new();
        let main = Rc::new(pbgui_root);
        let main_win = MainWindow {
            main: main.clone(),
            _main_box: pbgui_main_cppbox,
            _dist_popup_menu_box: dist_popup_menu_box,
            _logger_icon: logger_icon,
            //
            // slots
            //
            query_button_clicked: Slot::new(enclose! {(main, to_thread_sender) move || {
                update_vpin_table(
                    main.clone(),
                    to_thread_sender.clone(),
                );
            }}),

            save_clicked: Slot::new(enclose! { (main, to_thread_sender) move || {
                save_versionpin_changes(
                    main.main_widget(),
                    main.cache(),
                    to_thread_sender.clone()
                );
            } }),

            choose_distribution_triggered: Slot::new(enclose! { (main, to_thread_sender) move || {
                let vpin_tablewidget_ptr = main.vpin_table();
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
                    main.main_widget(),
                    main.vpin_requested_changes_table(),
                    to_thread_sender.clone()
                );
            }}),

            show_dist_menu: SlotOfQPoint::new(enclose! { (main) move |pos: QRef<QPoint>| {

                if main.vpin_table().is_null() {
                    log::error!("vpin_tablewidget_ptr is null");
                    return;
                }
                if main.dist_popup_menu().is_null() {
                    log::error!("dist_popup_menu_ptr is null");
                    return;
                }
                let _action = main.dist_popup_menu()
                    .exec_1a_mut(main.vpin_table().map_to_global(pos).as_ref());
            }}),

            select_pin_changes: Slot::new(enclose! { (main) move || {
                main.bottom_stacked_widget().set_current_index(0);
                main.bottom_ctrls_stacked_widget().set_current_index(0);
            }}),

            select_history: Slot::new(enclose! { (main, to_thread_sender) move || {
                let mut revisions_ptr = main.revisions_table();
                let mut stacked_ptr = main.bottom_stacked_widget();
                let mut controls_ptr = main.bottom_ctrls_stacked_widget();
                select_history(&mut revisions_ptr, &mut stacked_ptr, to_thread_sender.clone());
                controls_ptr.set_current_index(1);
            }}),

            select_log: Slot::new(enclose! { (main) move || {
                main.bottom_stacked_widget().set_current_index(2);
                main.bottom_ctrls_stacked_widget().set_current_index(2);
            }}),

            toggle_log_ctrls: SlotOfBool::new(enclose! { (main) move |state: bool| {
                main.logger().inner().set_ctrls_visible(state);
            }}),

            toggle_packages_tree: SlotOfBool::new(enclose! { (main) move |state: bool| {
                let mut frame = main.withs_splitter().widget(0);
                frame.set_visible(state);
            }}),

            toggle_withs: SlotOfBool::new(enclose! { (main) move |state: bool| {
                let mut frame = main.withs_splitter().widget(2);
                frame.set_visible(state);
            }}),

            toggle_vpin_changes: SlotOfBool::new(enclose! { (main) move |state: bool| {
                let mut frame = main.vpin_table_splitter().widget(1);
                frame.set_visible(state);
            }}),

            revision_changed: SlotOfQItemSelectionQItemSelection::new(
                enclose! { (main, to_thread_sender)
                move |selected: QRef<QItemSelection>, _deselected: QRef<QItemSelection>| {
                    let ind = selected.indexes();
                    if ind.count_0a() > 0 {
                        let txid = ind.at(COL_REV_TXID);
                        update_changes_table(
                            txid.row(),
                            main.revisions_table(),
                            main.revision_changes_table(),
                            to_thread_sender.clone())
                        ;
                    } else {
                        main.revision_changes_table().clear_contents();
                        main.revision_changes_table().set_row_count(0);
                    }
                }},
            ),

            distribution_changed: SlotOfQItemSelectionQItemSelection::new(
                enclose! { (main, to_thread_sender)
                move |selected: QRef<QItemSelection>, _deselected: QRef<QItemSelection>| {
                    let ind = selected.indexes();
                    if ind.count_0a() > 0 {
                        let mut vpin_tablewidget_ptr = main.vpin_table();
                        let txid = ind.at(COL_REV_TXID);
                        update_withpackages(
                            txid.row(),
                            &mut vpin_tablewidget_ptr,
                            main.package_withs_list(),
                            main.cache(),
                            to_thread_sender.clone()
                        );
                    } else {
                        main.package_withs_list().borrow().clear()
                    }
                }},
            ),
            save_withpackages: Slot::new(enclose! { (main) move || {
                let mut pinchanges_ptr = main.vpin_requested_changes_table();
                store_withpackage_changes::store_withpackage_changes(
                    main.package_withs_list(),
                    main.vpin_table(),
                    &mut pinchanges_ptr,
                    main.cache(),
                );
            }}),
            save_packages_xml: Slot::new(enclose! { (main, to_thread_sender) move || {
                let toolbar = main.main_toolbar();
                let level_cb = toolbar.level();
                save_packages_xml(main.main(), level_cb, to_thread_sender.clone());
            }}),
        };

        //
        // Wire up signals and slots
        //
        main.main_toolbar()
            .query_btn()
            .clicked()
            .connect(&main_win.query_button_clicked);

        main.search_shortcut
            .activated()
            .connect(&main_win.query_button_clicked);

        main.save_button().clicked().connect(&main_win.save_clicked);

        main.dist_popup_action()
            .triggered()
            .connect(&main_win.choose_distribution_triggered);

        main.vpin_table()
            .custom_context_menu_requested()
            .connect(&main_win.show_dist_menu);

        main.pinchanges_button()
            .default_action()
            .toggled()
            .connect(&main_win.select_pin_changes);

        main.history_button()
            .default_action()
            .toggled()
            .connect(&main_win.select_history);

        main.log_button()
            .default_action()
            .toggled()
            .connect(&main_win.select_log);

        main.toggle_log_ctrls_button()
            .clicked()
            .connect(&main_win.toggle_log_ctrls);

        main.left_toolbar_actions()
            .view_packages
            .toggled()
            .connect(&main_win.toggle_packages_tree);

        main.left_toolbar_actions()
            .view_withs
            .toggled()
            .connect(&main_win.toggle_withs);

        main.left_toolbar_actions()
            .view_vpin_changes
            .toggled()
            .connect(&main_win.toggle_vpin_changes);

        main.revisions_table()
            .selection_model()
            .selection_changed()
            .connect(&main_win.revision_changed);

        main.vpin_table()
            .selection_model()
            .selection_changed()
            .connect(&main_win.distribution_changed);

        main.package_withs_list()
            .borrow()
            .save_button()
            .clone()
            .clicked()
            .connect(&main_win.save_withpackages);

        // set initial state of with button to on
        let mut button = main.left_toolbar_actions().view_withs;
        button.toggle();

        main.main_menubar()
            .inner()
            .save_packages_action()
            .triggered()
            .connect(&main_win.save_packages_xml);

        main_win
    }

    /// Returns a reference counted ponter to the InnerMainWindow instance
    ///
    /// # Arguments
    /// * None
    ///
    /// # Returns
    /// * Rc<InnerMainWindow>>
    pub fn main_win(&self) -> Rc<InnerMainWindow<'a>> {
        self.main.clone()
    }

    /// Returns a mutable pointer to the QMainWindow instance
    ///
    /// # Arguments
    /// * None
    ///
    /// # Returns
    /// * MutPtr<QMainWindow>
    pub unsafe fn main(&self) -> MutPtr<QMainWindow> {
        let main = self.main_win();
        main.main()
    }
}
