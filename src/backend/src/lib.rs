use ic_cdk::api::caller;
use ic_cdk::storage;
use ic_cdk_macros::*;
use serde_json::json;

mod types;
use types::UploadedCode;

use std::cell::RefCell;

thread_local! {
    static STORED_CODE: RefCell<Option<UploadedCode>> = RefCell::new(None);
}

#[update]
fn upload_code(filename: String, content: String) -> String {
    let code = UploadedCode { filename, content };
    STORED_CODE.with(|storage| *storage.borrow_mut() = Some(code));
    format!("Code uploaded by {:?}!", caller())
}

#[query]
fn get_uploaded_code() -> Option<UploadedCode> {
    STORED_CODE.with(|storage| storage.borrow().clone())
}

