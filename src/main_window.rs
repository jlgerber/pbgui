use crate::{
    bottom_stacked_widget::create_bottom_stacked_widget,
    cache::PinChangesCache,
    center_widget,
    choose_distribution::choose_alternative_distribution,
    constants::COL_REV_TXID,
    left_toolbar, package_withs_list, packages_tree,
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
use pbgui_toolbar::toolbar;
use pbgui_tree::tree;
use pbgui_withs::WithsList;
use qt_core::{
    QItemSelection, QPoint, QString, Slot, SlotOfBool, SlotOfQItemSelectionQItemSelection,
};
use qt_gui::QKeySequence;
use qt_widgets::{
    cpp_core::{CppBox, MutPtr, Ref as QRef},
    QAction, QMainWindow, QMenu, QMenuBar, QPushButton, QShortcut, QSplitter, QStackedWidget,
    QTableWidget, QVBoxLayout, QWidget, SlotOfQPoint,
};
use rustqt_utils::{enclose, enclose_all};
use std::cell;
use std::cell::RefCell;
use std::rc::Rc;

/// Just as it sounds, MainWindow is the MainWindow struct, holding on
/// to various pointers that need to persist as well as top level slots
pub struct InnerMainWindow<'a> {
    main: MutPtr<QMainWindow>,
    main_widget: MutPtr<QWidget>,
    main_toolbar: Rc<toolbar::MainToolbar>,
    withs_splitter: MutPtr<QSplitter>,
    packages_tree: Rc<RefCell<tree::DistributionTreeView<'a>>>,
    package_withs_list: Rc<RefCell<WithsList<'a>>>,
    vpin_table: MutPtr<QTableWidget>,
    pinchanges_list: MutPtr<QTableWidget>,
    pinchanges_cache: Rc<PinChangesCache>,
    bottom_stacked_widget: MutPtr<QStackedWidget>,
    bottom_ctrls_stacked_widget: MutPtr<QStackedWidget>,
    save_button: MutPtr<QPushButton>,
    pin_changes_button: MutPtr<QPushButton>,
    changes_table: MutPtr<QTableWidget>,
    history_button: MutPtr<QPushButton>,
    revisions_table: MutPtr<QTableWidget>,
    dist_popup_menu: MutPtr<QMenu>,
    dist_popup_action: MutPtr<QAction>,
    left_toolbar_actions: LeftToolBarActions,
    search_shortcut: MutPtr<QShortcut>,
    // Slots
    toggle_withs: SlotOfBool<'a>,
    toggle_vpin_changes: SlotOfBool<'a>,
    revision_changed: SlotOfQItemSelectionQItemSelection<'a>,
    distribution_changed: SlotOfQItemSelectionQItemSelection<'a>,
    save_withpackages: Slot<'a>,
}

impl<'a> InnerMainWindow<'a> {
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
    pub fn new() -> (InnerMainWindow<'a>, CppBox<QMainWindow>, CppBox<QMenu>) {
        unsafe {
            // create the main window, menus, central widget and layout
            let (mut main_window, main_widget_ptr, mut main_layout_ptr) = create_main_window();
            let mut main_window_ptr = main_window.as_mut_ptr();
            let main_toolbar = Rc::new(create_top_toolbar(main_window_ptr.clone()));
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
                pinchanges_ptr,
                revisions_ptr,
                mut changes_table_ptr,
                save_button,
                stacked_ptr,
                pinchanges_button_ptr,
                history_button_ptr,
                controls_ptr,
            ) = create_bottom_stacked_widget(&mut vpin_table_splitter);

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
            let cache = pinchanges_cache.clone();
            //
            // final housekeeping before showing main window
            //
            versionpin_table_splitter::set_sizes(&mut vpin_table_splitter);
            withs_splitter::set_sizes(&mut with_splitter_ptr);

            resize_window_to_screen(&mut main_window_ptr, 0.8);
            load_stylesheet("/Users/jgerber/bin/pbgui.qss", main_window_ptr.clone());

            let withpackage_save = item_list_ptr.borrow().save_button().clone();
            let versionpin_table = vpin_tablewidget_ptr.clone();

            main_window_ptr.show();

            // Create the MainWindow instance, set up signals and slots, and return
            // the newly minted instance. We are done.
            let main_window_inst = InnerMainWindow {
                main: main_window_ptr,
                main_widget: main_widget_ptr,
                main_toolbar: main_toolbar,
                withs_splitter: with_splitter_ptr,
                packages_tree: packages_ptr,
                package_withs_list: item_list_ptr.clone(),
                vpin_table: vpin_tablewidget_ptr,
                save_button: save_button,
                pinchanges_list: pinchanges_ptr,
                pinchanges_cache,
                bottom_stacked_widget: stacked_ptr,
                bottom_ctrls_stacked_widget: controls_ptr,
                dist_popup_menu: dist_popup_menu_ptr,
                dist_popup_action: choose_dist_action,
                // _package_popup_menu: line_edit_popup_menu,
                pin_changes_button: pinchanges_button_ptr,
                changes_table: changes_table_ptr,
                history_button: history_button_ptr,
                revisions_table: revisions_ptr,
                left_toolbar_actions: left_toolbar_actions,
                search_shortcut: search_shortcut.into_ptr(),
                // slots
                save_withpackages: Slot::new(
                    enclose_all! { (cache, item_list_ptr) (mut pinchanges_ptr) move || {
                        store_withpackage_changes::store_withpackage_changes(
                            item_list_ptr.clone(),
                            versionpin_table,
                            &mut pinchanges_ptr,
                            cache.clone(),
                        );
                    }},
                ),

                distribution_changed: SlotOfQItemSelectionQItemSelection::new(
                    enclose! { (cache, item_list_ptr)
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
            revisions_ptr
                .selection_model()
                .selection_changed()
                .connect(&main_window_inst.revision_changed);

            vpin_tablewidget_ptr
                .selection_model()
                .selection_changed()
                .connect(&main_window_inst.distribution_changed);

            view_withs.toggled().connect(&main_window_inst.toggle_withs);

            view_pin_changes
                .toggled()
                .connect(&main_window_inst.toggle_vpin_changes);

            withpackage_save
                .clicked()
                .connect(&main_window_inst.save_withpackages);

            // configuration
            view_withs.set_checked(false);

            (main_window_inst, main_window, dist_popup_menu)
        }
    }

    /// Retrieve a MutPtr to the QMainWindow instance
    ///
    /// # Arguments
    /// * None
    ///
    /// # Returns
    /// * MutPtr<QMainWindow>
    pub unsafe fn main(&self) -> MutPtr<QMainWindow> {
        self.main
    }

    /// Retrieve a pointer to the main widget under the QMainWindow
    pub unsafe fn main_widget(&self) -> MutPtr<QWidget> {
        self.main_widget
    }

    pub fn cache(&self) -> Rc<PinChangesCache> {
        self.pinchanges_cache.clone()
    }

    pub unsafe fn bottom_stacked_widget(&self) -> MutPtr<QStackedWidget> {
        self.bottom_stacked_widget
    }

    pub unsafe fn bottom_ctrls_stacked_widget(&self) -> MutPtr<QStackedWidget> {
        self.bottom_ctrls_stacked_widget
    }

    /// Retrieve a Ref wrapped DistributionTreeView instance
    ///
    /// # Arguments
    /// * None
    ///
    /// # Returns
    /// * Ref<DistributionTreeView>
    pub unsafe fn packages_tree(&self) -> cell::Ref<tree::DistributionTreeView<'a>> {
        self.packages_tree.borrow()
    }

    /// Retrieve an shared copy of the DistributionTreeView
    pub unsafe fn tree(&self) -> Rc<RefCell<tree::DistributionTreeView<'a>>> {
        self.packages_tree.clone()
    }
    /// Retrieve an shared copy of the DistributionTreeView
    pub unsafe fn package_withs_list(&self) -> Rc<RefCell<WithsList<'a>>> {
        self.package_withs_list.clone()
    }
    /// Retrieve a RefMut wrapped DistributionTreeView instance
    ///
    /// # Arguments
    /// * None
    ///
    /// # Returns
    /// * RefMut<DistributionTreeView>
    pub unsafe fn packages_tree_mut(&self) -> cell::RefMut<tree::DistributionTreeView<'a>> {
        self.packages_tree.borrow_mut()
    }

    pub unsafe fn vpin_table(&self) -> MutPtr<QTableWidget> {
        self.vpin_table
    }

    pub unsafe fn pinchanges_list(&self) -> MutPtr<QTableWidget> {
        self.pinchanges_list
    }

    pub unsafe fn pinchanges_button(&self) -> MutPtr<QPushButton> {
        self.pin_changes_button
    }
    /// Retrieve an Rc wrapped MainToolbar instance
    ///
    /// # Arguments
    /// * None
    ///
    /// # Returns
    /// * Rc<MainToolbar>
    pub unsafe fn main_toolbar(&self) -> Rc<toolbar::MainToolbar> {
        self.main_toolbar.clone()
    }

    /// Retrieve the splitter between the main table widget and the
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
    pub unsafe fn save_button(&self) -> MutPtr<QPushButton> {
        self.save_button
    }

    pub unsafe fn history_button(&self) -> MutPtr<QPushButton> {
        self.history_button
    }

    pub unsafe fn revisions_table(&self) -> MutPtr<QTableWidget> {
        self.revisions_table
    }

    pub fn left_toolbar_actions(&self) -> &LeftToolBarActions {
        &self.left_toolbar_actions
    }

    pub unsafe fn changes_table(&self) -> MutPtr<QTableWidget> {
        self.changes_table
    }

    pub unsafe fn dist_popup_menu(&self) -> MutPtr<QMenu> {
        self.dist_popup_menu
    }

    pub unsafe fn dist_popup_action(&self) -> MutPtr<QAction> {
        self.dist_popup_action
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

/// Holder of
pub struct MainWindow<'a> {
    main: Rc<InnerMainWindow<'a>>,
    _main_box: CppBox<QMainWindow>,
    _dist_popup_menu_box: CppBox<QMenu>,
    // slots
    query_button_clicked: Slot<'a>,
    save_clicked: Slot<'a>,
    choose_distribution_triggered: Slot<'a>,
    show_dist_menu: SlotOfQPoint<'a>,
    select_pin_changes: Slot<'a>,
    select_history: Slot<'a>,
    toggle_packages_tree: SlotOfBool<'a>,
}

impl<'a> MainWindow<'a> {
    pub unsafe fn new() -> MainWindow<'a> {
        let (pbgui_root, pbgui_main_cppbox, dist_popup_menu_box) = InnerMainWindow::new();
        let main = Rc::new(pbgui_root);
        let main_win = MainWindow {
            main: main.clone(),
            _main_box: pbgui_main_cppbox,
            _dist_popup_menu_box: dist_popup_menu_box,
            // slots
            query_button_clicked: Slot::new(enclose! {(main) move || {
                let search_shows = main.left_toolbar_actions().search_shows;
                update_vpin_table(
                    main.main_toolbar(),
                    &search_shows,
                    main.vpin_table(),
                );
            }}),

            save_clicked: Slot::new(enclose! { (main) move || {
                let mut pinchanges_ptr = main.changes_table();
                save_versionpin_changes(
                    main.main_widget(),//main_widget_ptr,
                    &mut pinchanges_ptr,
                    main.main_toolbar(),//main_toolbar_ptr.clone(),
                    main.cache()//pinchange_cache.clone(),
                );
            } }),

            choose_distribution_triggered: Slot::new(enclose! { (main) move || {
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
                    main.pinchanges_list(),//pinchanges_ptr,
                    main.cache(),
                );
            }}),

            show_dist_menu: SlotOfQPoint::new(enclose! { (main) move |pos: QRef<QPoint>| {

                if main.vpin_table().is_null() {
                    log::error!("vpin_tablewidget_ptr is null");
                    return;
                }
                if main.dist_popup_menu().is_null() { //dist_popup_menu_item()
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

            select_history: Slot::new(enclose! { (main) move || {
                let mut revisions_ptr = main.revisions_table();
                let mut stacked_ptr = main.bottom_stacked_widget();
                let mut controls_ptr = main.bottom_ctrls_stacked_widget();
                select_history(&mut revisions_ptr, &mut stacked_ptr);
                controls_ptr.set_current_index(1);
            }}),

            toggle_packages_tree: SlotOfBool::new(enclose! { (main) move |state: bool| {
                let mut frame = main.withs_splitter().widget(0);
                frame.set_visible(state);
            }}),
        };

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
            .clicked()
            .connect(&main_win.select_pin_changes);

        main.history_button()
            .clicked()
            .connect(&main_win.select_history);

        main.left_toolbar_actions()
            .view_packages
            .toggled()
            .connect(&main_win.toggle_packages_tree);

        main_win
    }

    pub fn main_win(&self) -> Rc<InnerMainWindow<'a>> {
        self.main.clone()
    }
    pub unsafe fn main(&self) -> MutPtr<QMainWindow> {
        let main = self.main_win();
        main.main()
    }
}
