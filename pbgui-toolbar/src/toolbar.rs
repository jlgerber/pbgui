use crate::{combo_boxes, line_edit, query_button};
use qt_core::{AlignmentFlag, QFlags, QString};
use qt_widgets::{
    cpp_core::{CppBox, MutPtr, Ptr},
    QAction, QComboBox, QFrame, QLineEdit, QMainWindow, QMenu, QPushButton, QToolBar,
};

use rustqt_utils::{create_hlayout, qs, set_stylesheet_from_str};

/// The main toolbar structure
pub struct MainToolbar {
    toolbar: MutPtr<QToolBar>,
    query_btn: MutPtr<QPushButton>,
    level: MutPtr<QComboBox>,
    role: MutPtr<QComboBox>,
    platform: MutPtr<QComboBox>,
    site: MutPtr<QComboBox>,
    dir: MutPtr<QComboBox>,
    line_edit: MutPtr<QLineEdit>,
    menu: CppBox<QMenu>,
    clear_line_edit_action: MutPtr<QAction>,
}

/// load style at compile time
const STYLE_STR: &'static str = include_str!("../resources/toolbar.qss");

/// Create the MainToolbar structure
pub fn create(main_window: MutPtr<QMainWindow>) -> MainToolbar {
    unsafe {
        let mut main_window = main_window;
        let mut top_toolbar = main_window.add_tool_bar_q_string(&qs("TopToolBar"));
        top_toolbar.set_floatable(false);
        top_toolbar.set_movable(false);
        let mut hlayout = create_hlayout();
        let mut hlayout_ptr = hlayout.as_mut_ptr();
        let query_btn = query_button::create(None, hlayout.as_mut_ptr());

        let (level, role, platform, site, dir) = combo_boxes::create(&mut hlayout_ptr);

        let (line_edit, menu, clear_line_edit_action) = line_edit::create(hlayout.as_mut_ptr());
        let _align: QFlags<AlignmentFlag> = AlignmentFlag::AlignCenter.into();
        // hlayout.set_alignment_q_layout_q_flags_alignment_flag(hlayout_ptr, align);
        // hlayout.set_alignment_q_widget_q_flags_alignment_flag(level, align);

        let mut toolbar_frame = QFrame::new_0a();
        toolbar_frame.set_object_name(&qs("ToobarFrame"));
        toolbar_frame.set_layout(hlayout.into_ptr());
        top_toolbar.add_widget(toolbar_frame.into_ptr());

        MainToolbar {
            toolbar: top_toolbar,
            query_btn,
            level,
            role,
            platform,
            site,
            dir,
            line_edit,
            menu,
            clear_line_edit_action,
        }
    }
}

impl MainToolbar {
    /// Retrieve a MutPtr to the toolbar
    pub fn toolbar(&self) -> MutPtr<QToolBar> {
        self.toolbar
    }
    /// Retrieve a MutPtr to the query button QPushButton
    ///
    /// # Arguments
    /// * None
    ///
    /// # Returns
    /// * MutPtr to query button QPushButton
    pub fn query_btn(&self) -> MutPtr<QPushButton> {
        self.query_btn
    }

    /// Get a mutable pointer to the level
    /// combobox.
    ///
    /// # Arguments
    /// * None
    ///
    /// # Returns
    /// * MutPtr to level QcomboBox
    pub fn level(&self) -> MutPtr<QComboBox> {
        self.level
    }

    /// Get the current level as a std string
    pub unsafe fn level_string(&self) -> String {
        self.level.current_text().to_std_string()
    }

    /// Retrieve the current show as a string
    pub unsafe fn show_string(&self) -> String {
        self.level_string().split(".").next().unwrap().to_string()
    }

    /// Get the current level as a qstring
    pub unsafe fn level_qstring(&self) -> CppBox<QString> {
        self.level.current_text()
    }

    /// set the levels to choose from in the combobox's dropdown list
    ///
    /// # Arguments
    /// * `inputs` - Vec of types implementing AsRef<str> (so &str, String, etc)
    ///
    /// # Returns
    /// * None
    pub fn set_level_items<I: AsRef<str>>(&self, inputs: Vec<I>) {
        unsafe {
            inputs
                .iter()
                .filter(|s| s.as_ref() != "facility")
                .for_each(|s| {
                    let mut level = self.level;
                    level.add_item_q_string(&QString::from_std_str(s.as_ref()))
                });
        }
    }

    /// add a level to choose from in the combobox's dropdown list
    ///
    /// # Arguments
    /// * `input` - type implementing AsRef<str> (EG &str or String)
    ///
    /// # Returns
    /// * None
    pub fn add_level_item<I: AsRef<str>>(&self, input: I) {
        unsafe {
            let mut level = self.level;
            level.add_item_q_string(&QString::from_std_str(input.as_ref()))
        }
    }

    /// Get a mutable pointer to the role QComboBox.
    ///
    /// # Arguments
    /// * None
    ///
    /// # Returns
    /// * MutPtr to role QCombobBox
    pub fn role(&self) -> MutPtr<QComboBox> {
        self.role
    }

    /// Set role items in the combobox dropdown list
    ///
    /// # Arguments
    /// * `inputs` - A Vec of type that implements AsRef<str> (EG &str, or String)
    ///
    /// # Returns
    /// * None
    pub fn set_role_items<I: AsRef<str>>(&self, inputs: Vec<I>) {
        unsafe {
            inputs.iter().filter(|s| s.as_ref() != "any").for_each(|s| {
                let mut role = self.role;
                role.add_item_q_string(&QString::from_std_str(s.as_ref()))
            });
        }
    }

    /// add a role to choose from in the combobox's dropdown list
    ///
    /// # Arguments
    /// * `input` - A type that implements AsRef<str> (EG &str, or String)
    ///
    /// # Returns
    /// * None
    pub fn add_role_item<I: AsRef<str>>(&self, input: I) {
        unsafe {
            let mut role = self.role;
            role.add_item_q_string(&QString::from_std_str(input.as_ref()))
        }
    }

    /// Get a reference to the mutable pointer to the platform
    /// combobox.
    ///
    /// # Arguments
    /// * None
    ///
    /// # Returns
    /// * A MutPtr to the platform QComboBox
    pub fn platform(&self) -> MutPtr<QComboBox> {
        self.platform
    }

    /// Set the platforms to choose from in the combobox's dropdown list
    ///
    /// # Arguments
    /// * `inputs` - A Vec of type that implements AsRef<str> (EG &str, or String)
    ///
    /// # Returns
    /// * None
    pub fn set_platform_items<I: AsRef<str>>(&self, inputs: Vec<I>) {
        unsafe {
            inputs.iter().filter(|s| s.as_ref() != "any").for_each(|s| {
                let mut platform = self.platform;
                platform.add_item_q_string(&QString::from_std_str(s.as_ref()))
            });
        }
    }

    /// Get a mutable pointer to the site combobox.
    ///
    /// # Arguments
    /// * None
    ///
    /// # Returns
    /// * MutPtr to site QComboBox
    pub fn site(&self) -> MutPtr<QComboBox> {
        self.site
    }

    /// Set the sites to choose from in the combobox's dropdown list
    ///
    /// # Arguments
    /// * `inputs` - A Vec of type that implements AsRef<str> (EG &str or String)
    ///
    /// # Returns
    /// * None
    pub fn set_site_items<I: AsRef<str>>(&self, inputs: Vec<I>) {
        unsafe {
            inputs.iter().filter(|s| s.as_ref() != "any").for_each(|s| {
                let mut site = self.site;
                site.add_item_q_string(&QString::from_std_str(s.as_ref()))
            });
        }
    }

    /// Add a site to choose from in the combobox's dropdown list.
    ///
    /// # Arguments
    /// * `input` - A type which implements AsRef<str> (EG &str or String)
    ///
    /// # Returns
    /// * None
    pub fn add_site_item<I: AsRef<str>>(&self, input: I) {
        unsafe {
            let mut site = self.site;
            site.add_item_q_string(&QString::from_std_str(input.as_ref()))
        }
    }

    /// Get a mutable pointer to the dir combobox.
    ///
    /// # Arguments
    /// * None
    ///
    /// # Returns
    /// * MutPtr to the dir QComboBox
    pub fn dir(&self) -> MutPtr<QComboBox> {
        self.dir
    }

    /// Retrieve a MutPtr to the QLineEdit
    pub fn line_edit(&self) -> MutPtr<QLineEdit> {
        self.line_edit
    }

    /// Retrieve a Ptr to the QMenu. Unlike most items, the
    /// menu is owned by this component; thus, one must use
    /// a `menu_mut` to retrieve a MutPtr
    pub unsafe fn menu(&self) -> Ptr<QMenu> {
        self.menu.as_ptr()
    }
    /// Retrieve a MutPtr to the QMenu
    pub unsafe fn menu_mut(&mut self) -> MutPtr<QMenu> {
        self.menu.as_mut_ptr()
    }
    /// Retrieve a MutPTr to the clear_line_edit_action
    pub unsafe fn clear_line_edit_action(&self) -> MutPtr<QAction> {
        self.clear_line_edit_action
    }

    /// Set the stylesheet to the internal stylesheet
    pub fn set_default_stylesheet(&self) {
        set_stylesheet_from_str(STYLE_STR, self.toolbar);
    }
}
