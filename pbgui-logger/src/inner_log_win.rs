use chrono::{DateTime, Local, TimeZone};
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
    table_view: MutPtr<QTableView>,
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
        view.horizontal_header()
            .set_section_resize_mode_1a(ResizeMode::ResizeToContents);
        let view_ptr = view.as_mut_ptr();

        let mut model = QStandardItemModel::new_0a();
        model.set_column_count(2);
        let model_ptr = model.as_mut_ptr();
        view.set_model(model.into_ptr());

        let mut header = view_ptr.vertical_header();
        header.set_section_resize_mode_1a(ResizeMode::ResizeToContents);
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
            table_view: view_ptr,
            model: model_ptr,
        }
    }

    /// Retrieve a mutable pointer to the root widget of the InnerLogWin
    pub fn main(&self) -> MutPtr<QFrame> {
        self.main
    }

    /// Retrieve a mutable pointer to the main view (qlistview)
    pub fn table_view(&self) -> MutPtr<QTableView> {
        self.table_view
    }

    /// Retrieve a mutable pointer to the model
    pub fn model(&self) -> MutPtr<QStandardItemModel> {
        self.model
    }

    pub fn set_default_stylesheet(&self) {
        set_stylesheet_from_str(STYLE_STR, self.main);
    }

    pub fn trace(&self, msg: &str) {
        self.log(Some(Level::Trace), msg);
    }

    pub fn debug(&self, msg: &str) {
        self.log(Some(Level::Debug), msg);
    }

    pub fn info(&self, msg: &str) {
        self.log(Some(Level::Info), msg);
    }

    pub fn warn(&self, msg: &str) {
        self.log(Some(Level::Warn), msg);
    }

    pub fn error(&self, msg: &str) {
        self.log(Some(Level::Error), msg);
    }

    pub fn log(&self, level: Option<Level>, msg: &str) {
        unsafe {
            let mut item = QStandardItem::new();
            let mut loglevel = QStandardItem::new();
            let mut model = self.model();
            let rc = model.row_count_0a();
            model.set_row_count(rc + 1);
            let dt: DateTime<Local> = Local::now();
            let dt_str = dt.format("%a %b %e %T %Y");
            match &level {
                &Some(Level::Trace) => {
                    item.set_text(&qs(msg));
                    loglevel.set_text(&qs(format!("{} TRACE", dt_str)));
                    let brush = QBrush::from_global_color(GlobalColor::Cyan);
                    loglevel.set_foreground(brush.as_ref());
                    model.set_item_3a(rc, 0, loglevel.into_ptr());
                    model.set_item_3a(rc, 1, item.into_ptr());
                }
                &Some(Level::Debug) => {
                    item.set_text(&qs(msg));
                    loglevel.set_text(&qs(format!("{} DEBUG", dt_str)));
                    let brush = QBrush::from_global_color(GlobalColor::Cyan);
                    loglevel.set_foreground(brush.as_ref());
                    model.set_item_3a(rc, 0, loglevel.into_ptr());
                    model.set_item_3a(rc, 1, item.into_ptr());
                    //let index = model.index_2a(rc,0);
                }
                &Some(Level::Info) => {
                    loglevel.set_text(&qs(format!("{} INFO", dt_str)));
                    item.set_text(&qs(msg));
                    let brush = QBrush::from_global_color(GlobalColor::Green);
                    loglevel.set_foreground(brush.as_ref());

                    model.set_item_3a(rc, 0, loglevel.into_ptr());
                    model.set_item_3a(rc, 1, item.into_ptr());
                }
                &Some(Level::Warn) => {
                    loglevel.set_text(&qs(format!("{} WARN", dt_str)));
                    item.set_text(&qs(msg));
                    let brush = QBrush::from_global_color(GlobalColor::Yellow);
                    loglevel.set_foreground(brush.as_ref());

                    model.set_item_3a(rc, 0, loglevel.into_ptr());
                    model.set_item_3a(rc, 1, item.into_ptr());
                }
                &Some(Level::Error) => {
                    loglevel.set_text(&qs(format!("{} ERROR", dt_str)));
                    item.set_text(&qs(msg));
                    let brush = QBrush::from_global_color(GlobalColor::Red);
                    loglevel.set_foreground(brush.as_ref());

                    model.set_item_3a(rc, 0, loglevel.into_ptr());
                    model.set_item_3a(rc, 1, item.into_ptr())
                }
                &None => {
                    loglevel.set_text(&qs(""));
                    item.set_text(&qs(msg));
                    model.set_item_3a(rc, 0, loglevel.into_ptr());
                    model.set_item_3a(rc, 1, item.into_ptr())
                }
            }
            self.table_view().scroll_to_bottom();
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
