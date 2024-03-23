extern crate self as ft2;

pub mod check;
pub mod errors;
mod file;
mod route;
mod ud;
pub mod validation;
pub mod uuid;
mod member_role;
pub mod site;
pub mod urls;

pub use file::{file_type, File, FileText, FileTextError, FileType};
pub use route::route;
pub use ud::{GetUDError, UserData};
pub use crate::member_role::MemberRole;

#[no_mangle]
pub extern "C" fn main_ft() {
    let req = ft_sdk::http::current_request();
    let resp = ft2::route(req);
    ft_sdk::http::send_response(resp);
}
