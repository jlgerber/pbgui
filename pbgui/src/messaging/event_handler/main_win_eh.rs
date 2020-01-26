use super::*;
use crate::main_window::InnerMainWindow;
use crate::messaging::{event::main_win::MainWin, incoming::imain_win::IMainWin};
use crate::traits::RowSetterTrait;
//use std::cell::RefCell;
use std::rc::Rc;

pub unsafe fn match_main_win<'a>(
    event: MainWin,
    main_win: Rc<InnerMainWindow<'a>>,
    receiver: &Receiver<IMsg>,
) {
    match event {
        MainWin::GetVpins => {
            if let Ok(IMsg::MainWin(IMainWin::Vpins(vpins))) = receiver.recv() {
                let mut vpin_tablewidget_ptr = main_win.vpin_table();

                let mut cnt = 0;
                vpin_tablewidget_ptr.set_sorting_enabled(false);
                vpin_tablewidget_ptr.set_row_count(0);
                vpin_tablewidget_ptr.set_row_count(vpins.len() as i32);
                //let mut filtered_cnt = 0;
                for result in vpins {
                    result.set_table_row(&mut vpin_tablewidget_ptr, cnt);
                    cnt += 1;
                }

                vpin_tablewidget_ptr.set_sorting_enabled(true);
            } else {
                log::error!("PackagesTree::GetPackages IMsg does not match event state");
            }
        }
        //TODO: finish
        MainWin::GetWithsForVpin => {
            if let Ok(IMsg::MainWin(IMainWin::WithPackages(withs))) = receiver.recv() {
                let withs = withs.iter().map(|x| x.with.as_str()).collect();
                let withs_list = main_win.package_withs_list();
                withs_list.borrow_mut().set_items(withs);
            } else {
                log::error!("PackagesTree::GetPackages IMsg does not match event state");
            }
        }
    }
}
