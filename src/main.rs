#![windows_subsystem = "windows"]
/*!
    A very simple application that shows your name in a message box.
    Unlike `basic_d`, this example uses layout to position the controls in the window
*/
mod menu_handlers;

extern crate native_windows_gui as nwg;
extern crate native_windows_derive as nwd;

use nwd::NwgUi;
use nwg::NativeUi;


#[derive(Default, NwgUi)]
pub struct BasicApp {
    #[nwg_control(size: (940, 530), position: (300, 300), title: "Basic example", flags: "WINDOW|VISIBLE")]
    #[nwg_events( OnWindowClose: [BasicApp::say_goodbye] )]
    window: nwg::Window,
    
    // File Menu
    #[nwg_control(text: "File")]
    #[nwg_events()]
    file_menu: nwg::Menu,

    #[nwg_control(parent: file_menu, text: "Persisted Devices")]
    #[nwg_events()]
    persisted_menu: nwg::MenuItem,

    #[nwg_control(parent: file_menu, text: "Exit")]
    #[nwg_events(OnMenuItemSelected: [nwg::stop_thread_dispatch()])]
    exit_menu: nwg::MenuItem,

    // Service Menu
    #[nwg_control(text: "Service")]
    #[nwg_events()]
    service_menu: nwg::Menu,

    #[nwg_control(parent: service_menu, text: "Upgrade")]
    #[nwg_events()]
    upgrade_menu: nwg::MenuItem,

    #[nwg_control(parent: service_menu, text: "Uninstall")]
    #[nwg_events()]
    uninstall_menu: nwg::MenuItem,

    #[nwg_control(parent: service_menu, text: "Update ID-List")]
    #[nwg_events()]
    update_id_menu: nwg::MenuItem,

    // View Menu
    #[nwg_control(text: "View")]
    #[nwg_events()]
    view_menu: nwg::Menu,

    #[nwg_control(parent: view_menu, text: "Refresh")]
    #[nwg_events()]
    refresh_menu: nwg::MenuItem,

    // Help Menu
    #[nwg_control(text: "About")]
    #[nwg_events()]
    help_menu: nwg::Menu,

    #[nwg_control(parent: help_menu, text: "About")]
    #[nwg_events()]
    about_menu: nwg::MenuItem,

    #[nwg_layout(parent: window, spacing: 1)]
    grid: nwg::GridLayout,

    // ListView

    #[nwg_control(text: "Heisenberg", focus: true)]
    #[nwg_layout_item(layout: grid, row: 0, col: 0)]
    name_edit: nwg::TextInput,

    #[nwg_control(text: "Say my name")]
    #[nwg_layout_item(layout: grid, col: 0, row: 1, row_span: 2)]
    #[nwg_events( OnButtonClick: [BasicApp::say_hello] )]
    hello_button: nwg::Button

}



fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");
    let _app = BasicApp::build_ui(Default::default()).expect("Failed to build UI");
    _app.add_firewall_rule().expect("Failed to add firewall rule!");
    _app.install_if_needed();
    nwg::dispatch_thread_events();
}