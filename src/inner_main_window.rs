use crate::{
    bottom_stacked_widget::create_bottom_stacked_widget,
    center_widget, left_toolbar, package_withs_list, packages_tree,
    utility::{create_vlayout, load_stylesheet, qs, resize_window_to_screen},
    versionpin_table, versionpin_table_splitter, withs_splitter, LeftToolBarActions,
};
//use log;
use pbgui_toolbar::toolbar;
use pbgui_tree::tree;
use pbgui_withs::WithsList;
use qt_core::QString;
use qt_gui::QKeySequence;
use qt_widgets::{
    cpp_core::{CppBox, MutPtr},
    QAction, QMainWindow, QMenu, QMenuBar, QPushButton, QShortcut, QTableWidget, QVBoxLayout,
    QWidget,
};
use std::cell;
use std::cell::RefCell;
use std::rc::Rc;

/// Just as it sounds, MainWindow is the MainWindow struct, holding on
/// to various pointers that need to persist as well as top level slots
pub struct MainWindow<'a> {
    main: MutPtr<QMainWindow>,              //CppBox<QMainWindow>
    main_toolbar: Rc<toolbar::MainToolbar>, //
    packages_tree: Rc<RefCell<tree::DistributionTreeView<'a>>>, //does this have to be a refcell?
    package_withs_list: Rc<RefCell<WithsList<'a>>>, //can this be an RC?
    vpin_table: MutPtr<QTableWidget>,
    pinchanges_list: MutPtr<QTableWidget>,
    save_button: MutPtr<QPushButton>,
    pin_changes_button: MutPtr<QPushButton>,
    history_button: MutPtr<QPushButton>,
    dist_popup_menu: MutPtr<QMenu>,
    dist_popup_action: MutPtr<QAction>,
    _left_toolbar_actions: LeftToolBarActions,
    search_shortcut: MutPtr<QShortcut>,
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

    pub fn new() -> (MainWindow<'a>, CppBox<QMainWindow>, CppBox<QMenu>) {
        unsafe {
            // create the main window, menus, central widget and layout
            let (mut main_window, _main_widget_ptr, mut main_layout_ptr) = create_main_window();
            let mut main_window_ptr = main_window.as_mut_ptr();
            let main_toolbar = Rc::new(create_top_toolbar(main_window_ptr.clone()));
            let _main_toolbar_ptr = main_toolbar.clone();
            // create left toolbar
            let left_toolbar_actions = left_toolbar::create(&mut main_window_ptr);
            let _view_packages = left_toolbar_actions.view_packages;
            let mut view_withs = left_toolbar_actions.view_withs;
            let _view_pin_changes = left_toolbar_actions.view_vpin_changes;
            let _search_shows = left_toolbar_actions.search_shows;

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
                mut _revisions_ptr,
                mut _changes_table_ptr,
                save_button,
                mut _stacked_ptr,
                pinchanges_button_ptr,
                history_button_ptr,
                mut _controls_ptr,
            ) = create_bottom_stacked_widget(&mut vpin_table_splitter);

            // setup popup menu for versionpin table
            let mut dist_popup_menu = QMenu::new();
            let choose_dist_action =
                dist_popup_menu.add_action_q_string(&QString::from_std_str("Change Version"));

            let _choose_withs_action =
                dist_popup_menu.add_action_q_string(&QString::from_std_str("Withs"));

            //let dist_popup_menu_ptr = dist_popup_menu.as_mut_ptr();

            // create the with with package list on the right hand side
            let item_list_ptr = package_withs_list::create(with_splitter_ptr);
            item_list_ptr.borrow_mut().set_add_mode();
            item_list_ptr.borrow_mut().set_cb_max_visible_items(30);

            // shortcuts
            let key_seq = QKeySequence::from_q_string(&qs("Ctrl+Return"));
            let search_shortcut =
                QShortcut::new_2a(key_seq.as_ref(), item_list_ptr.borrow().main());

            // persist data
            //let pinchange_cache = Rc::new(PinChangesCache::new());
            //
            // final housekeeping before showing main window
            //
            versionpin_table_splitter::set_sizes(&mut vpin_table_splitter);
            withs_splitter::set_sizes(&mut with_splitter_ptr);

            resize_window_to_screen(&mut main_window_ptr, 0.8);
            load_stylesheet("/Users/jgerber/bin/pbgui.qss", main_window_ptr);

            let _withpackage_save = item_list_ptr.borrow().save_button().clone();
            let _versionpin_table = vpin_tablewidget_ptr.clone();

            main_window_ptr.show();
            // Create the MainWindow instance, set up signals and slots, and return
            // the newly minted instance. We are done.
            let main_window_inst = MainWindow {
                main: main_window.as_mut_ptr(),
                main_toolbar: main_toolbar,
                packages_tree: packages_ptr,
                package_withs_list: item_list_ptr.clone(),
                vpin_table: vpin_tablewidget_ptr,
                pinchanges_list: pinchanges_ptr,
                save_button: save_button,
                pin_changes_button: pinchanges_button_ptr,
                history_button: history_button_ptr,
                dist_popup_menu: dist_popup_menu.as_mut_ptr(),
                dist_popup_action: choose_dist_action,
                _left_toolbar_actions: left_toolbar_actions,
                search_shortcut: search_shortcut.into_ptr(),
            };

            // configuration
            view_withs.set_checked(false);

            (main_window_inst, main_window, dist_popup_menu)
        }
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

    /// Retrieve a MutPtr to the QMainWindow instance
    ///
    /// # Arguments
    /// * None
    ///
    /// # Returns
    /// * MutPtr<QMainWindow>
    pub unsafe fn main(&mut self) -> MutPtr<QMainWindow> {
        self.main
    }

    pub unsafe fn vpin_table(&self) -> MutPtr<QTableWidget> {
        self.vpin_table
    }

    pub unsafe fn pinchanges_list(&self) -> MutPtr<QTableWidget> {
        self.pinchanges_list
    }

    pub unsafe fn save_button(&self) -> MutPtr<QPushButton> {
        self.save_button
    }

    pub unsafe fn pinchanges_button(&self) -> MutPtr<QPushButton> {
        self.pin_changes_button
    }

    pub unsafe fn history_button(&self) -> MutPtr<QPushButton> {
        self.history_button
    }

    pub unsafe fn dist_popup_menu(&self) -> MutPtr<QMenu> {
        self.dist_popup_menu
    }

    pub unsafe fn dist_popup_action(&self) -> MutPtr<QAction> {
        self.dist_popup_action
    }

    /// Retrieve a MutPTr to the search shortcut
    pub unsafe fn search_shortcut(&self) -> MutPtr<QShortcut> {
        self.search_shortcut
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
