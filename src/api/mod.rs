mod app_id;
pub mod listing;
mod request;
pub mod search;

pub static BACKEND_URL: &str = "https://api.homegate.ch";
pub static API_USERNAME: &str = "hg_android";
pub static API_PASSWORD: &str = "6VcGU6ceCFTk8dFm";
pub static SECRET: [u8; 21] = [
    65, 66, 117, 84, 90, 114, 99, 84, 71, 75, 78, 52, 65, 119, 106, 72, 101, 100, 51, 72, 106,
];
pub static USER_AGENT: &str = "hoemgate.ch App Android";
