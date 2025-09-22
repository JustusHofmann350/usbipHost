use crate::BasicApp;
use native_windows_gui as nwg;
use std::process::Command;

impl BasicApp {

    pub fn say_hello(&self) {
        nwg::modal_info_message(&self.window, "Hello", &format!("Hello {}", self.name_edit.text()));
    }
    
    pub fn say_goodbye(&self) {
        nwg::stop_thread_dispatch();
    }

    pub fn is_installed(&self) {
        let mut 
        nwg::modal_info_message(&self.window, "Title", "This is an info message");
    }
}