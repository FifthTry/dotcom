extern crate self as ft2;

pub mod check;
mod file;
mod route;
mod ud;
pub mod validation;

pub use file::{file_type, File, FileText, FileTextError, FileType};

pub use route::route;
pub use ud::{GetUDError, UserData};

#[no_mangle]
pub extern "C" fn main_ft() {
    let req = ft_sdk::http::current_request();
    let resp = ft2::route(req);
    ft_sdk::http::send_response(resp);
}
