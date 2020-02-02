pub mod inner_log_win;
pub use inner_log_win::InnerLogWin;

pub mod log_win;
pub use log_win::LogWin;
pub mod output;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
