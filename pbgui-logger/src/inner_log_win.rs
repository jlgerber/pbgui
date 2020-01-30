use chrono::{DateTime, Local};
use log::Level;
use qt_core::{GlobalColor, QString};
use qt_gui::{QBrush, QStandardItem, QStandardItemModel};
use qt_widgets::{
    cpp_core::{CastInto, CppBox, MutPtr, Ref as QRef},
    q_abstract_item_view::SelectionBehavior,
    q_header_view::ResizeMode,
    QFrame, QTableView, QWidget,
};
use rustqt_utils::{create_vlayout, qs, set_stylesheet_from_str};

const STYLE_STR: &'static str = include_str!("../resources/pbgui_logger.qss");
const COL_0_WIDTH: i32 = 60;
const COL_1_WIDTH: i32 = 180;
const COL_2_WIDTH: i32 = 120;
const COL_3_WIDTH: i32 = 260;
const COL_4_WIDTH: i32 = 60;

pub struct LogData<'a> {
    pub level: Level,
    pub target: &'a str,
    pub file: Option<&'a str>,
    pub line: Option<u32>,
}

impl<'a> LogData<'a> {
    pub fn level(&self) -> &Level {
        &self.level
    }

    pub fn target(&self) -> &'a str {
        self.target
    }

    pub fn file(&self) -> &'a str {
        match self.file {
            Some(ref file) => file,
            None => "",
        }
    }

    pub fn line(&self) -> u32 {
        match self.line {
            Some(v) => v,
            None => 0,
        }
    }
}

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
        view.set_object_name(&qs("LoggerTable"));

        //view.set_word_wrap(true);
        view.set_show_grid(false);
        let mut hheader = view.horizontal_header();
        hheader.set_stretch_last_section(true);
        hheader.set_section_resize_mode_1a(ResizeMode::Fixed);
        let view_ptr = view.as_mut_ptr();

        let mut model = QStandardItemModel::new_0a();
        model.set_column_count(6);
        let model_ptr = model.as_mut_ptr();
        view.set_model(model.into_ptr());

        let mut header = view_ptr.vertical_header();
        header.set_section_resize_mode_1a(ResizeMode::Fixed);
        header.set_default_section_size(1);
        view.horizontal_header().hide();

        view.set_selection_behavior(SelectionBehavior::SelectRows);

        view.vertical_header().hide();
        view.set_column_width(0, COL_0_WIDTH);
        view.set_column_width(1, COL_1_WIDTH);
        view.set_column_width(2, COL_2_WIDTH);
        view.set_column_width(3, COL_3_WIDTH);
        view.set_column_width(4, COL_4_WIDTH);

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

    pub fn clear_log(&self) {
        unsafe {
            let mut model = self.model;
            model.clear();
        }
    }
    pub fn set_default_stylesheet(&self) {
        set_stylesheet_from_str(STYLE_STR, self.main);
    }

    pub fn log(&self, log_data: Option<LogData>, msg: &str) {
        unsafe {
            let mut item = QStandardItem::new();
            item.set_editable(false);

            let dt: DateTime<Local> = Local::now();
            let dt_str = dt.format("%a %b %e %T %Y");
            let mut datetime = QStandardItem::new();
            datetime.set_editable(false);
            datetime.set_text(&qs(dt_str.to_string().as_str()));

            let mut loglevel = QStandardItem::new();
            loglevel.set_editable(false);

            let mut target_item = QStandardItem::new();
            target_item.set_editable(false);

            let mut mp_item = QStandardItem::new();
            mp_item.set_editable(false);

            let mut file_item = QStandardItem::new();
            file_item.set_editable(false);

            let mut line_item = QStandardItem::new();
            line_item.set_editable(false);

            let mut model = self.model();
            let rc = model.row_count_0a();
            model.set_row_count(rc + 1);

            match &log_data {
                &Some(LogData {
                    level: Level::Trace,
                    target,
                    file,
                    line,
                    ..
                }) => {
                    item.set_text(&qs(msg));
                    loglevel.set_text(&qs("TRACE"));
                    target_item.set_text(&qs(target));
                    file_item.set_text(&qs(file.unwrap_or("").split("/").last().unwrap_or("")));
                    line_item.set_text(&qs(line.unwrap_or(0).to_string().as_str()));

                    let brush = QBrush::from_global_color(GlobalColor::Cyan);
                    loglevel.set_foreground(brush.as_ref());
                    datetime.set_foreground(brush.as_ref());
                    target_item.set_foreground(brush.as_ref());
                    file_item.set_foreground(brush.as_ref());
                    line_item.set_foreground(brush.as_ref());

                    model.set_item_3a(rc, 0, loglevel.into_ptr());
                    model.set_item_3a(rc, 1, datetime.into_ptr());
                    model.set_item_3a(rc, 2, file_item.into_ptr());
                    model.set_item_3a(rc, 3, target_item.into_ptr());
                    model.set_item_3a(rc, 4, line_item.into_ptr());
                    model.set_item_3a(rc, 5, item.into_ptr());
                }

                &Some(LogData {
                    level: Level::Debug,
                    target,
                    file,
                    line,
                    ..
                }) => {
                    item.set_text(&qs(msg));
                    loglevel.set_text(&qs("DEBUG"));
                    target_item.set_text(&qs(target));
                    file_item.set_text(&qs(file.unwrap_or("").split("/").last().unwrap_or("")));
                    line_item.set_text(&qs(line.unwrap_or(0).to_string().as_str()));

                    let brush = QBrush::from_global_color(GlobalColor::Cyan);
                    loglevel.set_foreground(brush.as_ref());
                    datetime.set_foreground(brush.as_ref());
                    target_item.set_foreground(brush.as_ref());
                    file_item.set_foreground(brush.as_ref());
                    line_item.set_foreground(brush.as_ref());

                    model.set_item_3a(rc, 0, loglevel.into_ptr());
                    model.set_item_3a(rc, 1, datetime.into_ptr());
                    model.set_item_3a(rc, 2, file_item.into_ptr());
                    model.set_item_3a(rc, 3, target_item.into_ptr());
                    model.set_item_3a(rc, 4, line_item.into_ptr());
                    model.set_item_3a(rc, 5, item.into_ptr());
                }
                &Some(LogData {
                    level: Level::Info,
                    target,
                    file,
                    line,
                    ..
                }) => {
                    loglevel.set_text(&qs("INFO"));
                    item.set_text(&qs(msg));
                    target_item.set_text(&qs(target));
                    file_item.set_text(&qs(file.unwrap_or("").split("/").last().unwrap_or("")));
                    line_item.set_text(&qs(line.unwrap_or(0).to_string().as_str()));

                    let brush = QBrush::from_global_color(GlobalColor::Green);
                    loglevel.set_foreground(brush.as_ref());
                    datetime.set_foreground(brush.as_ref());
                    target_item.set_foreground(brush.as_ref());
                    file_item.set_foreground(brush.as_ref());
                    line_item.set_foreground(brush.as_ref());

                    model.set_item_3a(rc, 0, loglevel.into_ptr());
                    model.set_item_3a(rc, 1, datetime.into_ptr());
                    model.set_item_3a(rc, 2, file_item.into_ptr());
                    model.set_item_3a(rc, 3, target_item.into_ptr());
                    model.set_item_3a(rc, 4, line_item.into_ptr());
                    model.set_item_3a(rc, 5, item.into_ptr());
                }
                &Some(LogData {
                    level: Level::Warn,
                    target,
                    file,
                    line,
                    ..
                }) => {
                    loglevel.set_text(&qs("WARN"));
                    item.set_text(&qs(msg));
                    target_item.set_text(&qs(target));
                    file_item.set_text(&qs(file.unwrap_or("").split("/").last().unwrap_or("")));
                    line_item.set_text(&qs(line.unwrap_or(0).to_string().as_str()));

                    let brush = QBrush::from_global_color(GlobalColor::Yellow);
                    loglevel.set_foreground(brush.as_ref());
                    datetime.set_foreground(brush.as_ref());
                    target_item.set_foreground(brush.as_ref());
                    file_item.set_foreground(brush.as_ref());
                    line_item.set_foreground(brush.as_ref());

                    model.set_item_3a(rc, 0, loglevel.into_ptr());
                    model.set_item_3a(rc, 1, datetime.into_ptr());
                    model.set_item_3a(rc, 2, file_item.into_ptr());
                    model.set_item_3a(rc, 3, target_item.into_ptr());
                    model.set_item_3a(rc, 4, line_item.into_ptr());
                    model.set_item_3a(rc, 5, item.into_ptr());
                }
                &Some(LogData {
                    level: Level::Error,
                    target,
                    file,
                    line,
                    ..
                }) => {
                    loglevel.set_text(&qs("ERROR"));
                    item.set_text(&qs(msg));
                    target_item.set_text(&qs(target));
                    file_item.set_text(&qs(file.unwrap_or("").split("/").last().unwrap_or("")));
                    line_item.set_text(&qs(line.unwrap_or(0).to_string().as_str()));

                    let brush = QBrush::from_global_color(GlobalColor::Red);
                    loglevel.set_foreground(brush.as_ref());
                    datetime.set_foreground(brush.as_ref());
                    target_item.set_foreground(brush.as_ref());
                    file_item.set_foreground(brush.as_ref());
                    line_item.set_foreground(brush.as_ref());

                    model.set_item_3a(rc, 0, loglevel.into_ptr());
                    model.set_item_3a(rc, 1, datetime.into_ptr());
                    model.set_item_3a(rc, 2, file_item.into_ptr());
                    model.set_item_3a(rc, 3, target_item.into_ptr());
                    model.set_item_3a(rc, 4, line_item.into_ptr());
                    model.set_item_3a(rc, 5, item.into_ptr());
                }
                &None => {
                    loglevel.set_text(&qs(""));
                    datetime.set_text(&qs(""));
                    target_item.set_text(&qs(""));
                    file_item.set_text(&qs(""));
                    line_item.set_text(&qs(""));

                    item.set_text(&qs(msg));
                    model.set_item_3a(rc, 0, loglevel.into_ptr());
                    model.set_item_3a(rc, 1, datetime.into_ptr());
                    model.set_item_3a(rc, 2, file_item.into_ptr());
                    model.set_item_3a(rc, 3, target_item.into_ptr());
                    model.set_item_3a(rc, 4, line_item.into_ptr());
                    model.set_item_3a(rc, 5, item.into_ptr());
                }
            }
            // we have to reset the sizing once we have cleared the table so we
            // might as well do this when we add our first item
            if rc == 1 {
                let mut view = self.table_view();
                view.set_column_width(0, COL_0_WIDTH);
                view.set_column_width(1, COL_1_WIDTH);
                view.set_column_width(2, COL_2_WIDTH);
                view.set_column_width(3, COL_3_WIDTH);
                view.set_column_width(4, COL_4_WIDTH);
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
        model.set_item_3a(rc, 0, loglevel.into_ptr());
        model.set_item_3a(rc, 1, item.into_ptr());
    }
}
