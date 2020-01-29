use log::Level;
use qt_core::{GlobalColor, QString};
use qt_gui::{QBrush, QStandardItem, QStandardItemModel};
use qt_widgets::{
    cpp_core::{CastInto, CppBox, MutPtr, Ref as QRef},
    q_header_view::ResizeMode,
    QFrame, QTableView, QWidget,
};
use rustqt_utils::{create_vlayout, qs, set_stylesheet_from_str};

const STYLE_STR: &'static str = include_str!("../resources/pbgui_logger.qss");

pub struct InnerLogWin {
    main: MutPtr<QFrame>,
    list_view: MutPtr<QTableView>,
    model: MutPtr<QStandardItemModel>,
}

impl InnerLogWin {
    pub unsafe fn new(parent: impl CastInto<MutPtr<QWidget>>) -> Self {
        let mut main_frame = QFrame::new_0a();
        let main_frame_ptr = main_frame.as_mut_ptr();
        main_frame.set_object_name(&qs("LoggerMainFrame"));
        // create main layout
        let mut main_layout = create_vlayout();

        // create the view
        let mut view = QTableView::new_0a();
        //view.set_word_wrap(true);
        view.set_show_grid(false);
        view.horizontal_header().set_stretch_last_section(true);
        let view_ptr = view.as_mut_ptr();

        let mut model = QStandardItemModel::new_0a();
        model.set_column_count(2);
        let model_ptr = model.as_mut_ptr();
        view.set_model(model.into_ptr());

        let mut header = view_ptr.vertical_header();
        header.set_section_resize_mode_2a(1, ResizeMode::Fixed);
        header.set_default_section_size(1);
        view.horizontal_header().hide();
        view.vertical_header().hide();

        // add the view to the main layout
        main_layout.add_widget(view.into_ptr());
        main_frame.set_layout(main_layout.into_ptr());
        // add the main frame to the parent's layout
        let parent = parent.cast_into();
        let mut layout = parent.layout();
        layout.add_widget(main_frame.into_ptr());

        Self {
            main: main_frame_ptr,
            list_view: view_ptr,
            model: model_ptr,
        }
    }

    /// Retrieve a mutable pointer to the root widget of the InnerLogWin
    pub fn main(&self) -> MutPtr<QFrame> {
        self.main
    }

    /// Retrieve a mutable pointer to the main view (qlistview)
    pub fn list_view(&self) -> MutPtr<QTableView> {
        self.list_view
    }

    /// Retrieve a mutable pointer to the model
    pub fn model(&self) -> MutPtr<QStandardItemModel> {
        self.model
    }

    pub fn set_default_stylesheet(&self) {
        set_stylesheet_from_str(STYLE_STR, self.main);
    }

    pub fn trace(&self, msg: &str) {
        self.log(Level::Trace, msg);
    }

    pub fn debug(&self, msg: &str) {
        self.log(Level::Debug, msg);
    }

    pub fn info(&self, msg: &str) {
        self.log(Level::Info, msg);
    }

    pub fn warn(&self, msg: &str) {
        self.log(Level::Warn, msg);
    }

    pub fn error(&self, msg: &str) {
        self.log(Level::Error, msg);
    }

    pub fn log(&self, level: Level, msg: &str) {
        unsafe {
            let mut item = QStandardItem::new();
            let mut loglevel = QStandardItem::new();
            let mut model = self.model();
            let rc = model.row_count_0a();
            model.set_row_count(rc + 1);
            match &level {
                &Level::Trace => {
                    item.set_text(&qs(msg));
                    loglevel.set_text(&qs("[ TRACE ]"));
                    let brush = QBrush::from_global_color(GlobalColor::Cyan);
                    loglevel.set_foreground(brush.as_ref());
                    model.set_item_3a(rc, 0, loglevel.into_ptr());
                    model.set_item_3a(rc, 1, item.into_ptr());
                }
                &Level::Debug => {
                    item.set_text(&qs(msg));
                    loglevel.set_text(&qs("[ DEBUG ]"));
                    let brush = QBrush::from_global_color(GlobalColor::Cyan);
                    loglevel.set_foreground(brush.as_ref());
                    model.set_item_3a(rc, 0, loglevel.into_ptr());
                    model.set_item_3a(rc, 1, item.into_ptr());
                    //let index = model.index_2a(rc,0);
                }
                &Level::Info => {
                    loglevel.set_text(&qs("[ INFO ]"));
                    item.set_text(&qs(msg));
                    let brush = QBrush::from_global_color(GlobalColor::Green);
                    loglevel.set_foreground(brush.as_ref());

                    model.set_item_3a(rc, 0, loglevel.into_ptr());
                    model.set_item_3a(rc, 1, item.into_ptr());
                }
                &Level::Warn => {
                    loglevel.set_text(&qs("[ WARN ]"));
                    item.set_text(&qs(msg));
                    let brush = QBrush::from_global_color(GlobalColor::Yellow);
                    loglevel.set_foreground(brush.as_ref());

                    model.set_item_3a(rc, 0, loglevel.into_ptr());
                    model.set_item_3a(rc, 1, item.into_ptr());
                }
                &Level::Error => {
                    loglevel.set_text(&qs("[ ERROR ]"));
                    item.set_text(&qs(msg));
                    let brush = QBrush::from_global_color(GlobalColor::Red);
                    loglevel.set_foreground(brush.as_ref());

                    model.set_item_3a(rc, 0, loglevel.into_ptr());
                    model.set_item_3a(rc, 1, item.into_ptr())
                }
            }
        }
    }

    pub unsafe fn log_qstrings(
        &self,
        msg: QRef<QString>,
        level: QRef<QString>,
        color: GlobalColor,
    ) {
        let mut item = QStandardItem::new();
        let mut loglevel = QStandardItem::new();
        let mut model = self.model();
        let rc = model.row_count_0a();
        model.set_row_count(rc + 1);
        loglevel.set_text(level);
        item.set_text(msg);
        let brush = QBrush::from_global_color(color);
        loglevel.set_foreground(brush.as_ref());

        model.set_item_3a(rc, 0, loglevel.into_ptr());
        model.set_item_3a(rc, 1, item.into_ptr());
    }

    pub unsafe fn log_items(&self, item: CppBox<QStandardItem>, loglevel: CppBox<QStandardItem>) {
        let mut model = self.model();
        let rc = model.row_count_0a();
        model.set_row_count(rc + 1);
        //let brush = QBrush::from_global_color(GlobalColor::Yellow);
        //loglevel.set_foreground(brush.as_ref());

        model.set_item_3a(rc, 0, loglevel.into_ptr());
        model.set_item_3a(rc, 1, item.into_ptr());
    }
}
