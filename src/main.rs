#![windows_subsystem = "windows"]
/*!
    A very simple application that shows your name in a message box.
    Unlike `basic_d`, this example uses layout to position the controls in the window
*/
mod menu_handlers;
mod windows;
mod device_list;

extern crate native_windows_derive as nwd;
extern crate native_windows_gui as nwg;

use nwd::NwgUi;
use nwg::NativeUi;
use nwg::InsertListViewItem;

use crate::device_list::UsbipDevice;
use crate::device_list::list_devices;

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
    #[nwg_events( OnMenuItemSelected: [BasicApp::upgrade_usbipd] )]
    upgrade_menu: nwg::MenuItem,

    #[nwg_control(parent: service_menu, text: "Uninstall")]
    #[nwg_events( OnMenuItemSelected: [BasicApp::uninstall_usbipd] )]
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
    #[nwg_control(text: "Help")]
    #[nwg_events()]
    help_menu: nwg::Menu,

    #[nwg_control(parent: help_menu, text: "About")]
    #[nwg_events( OnMenuItemSelected: [BasicApp::show_about] )]
    about_menu: nwg::MenuItem,

    #[nwg_layout(parent: window, spacing: 1)]
    layout: nwg::GridLayout,

    // ListView
    #[nwg_control(parent: window, list_style: nwg::ListViewStyle::Detailed, size: (940, 200), position: (10, 40))]
    #[nwg_events( OnListViewDoubleClick: [] )]
    #[nwg_layout_item(layout: layout, col: 0, row: 1, col_span: 4)]
    list: nwg::ListView,
}

impl BasicApp {
    fn setup_columns(&self) {
        if self.list.column_len() == 0 {
            self.list.insert_column(nwg::InsertListViewColumn {
                index: Some(0),
                text: Some("BUSID".to_string()),
                width: Some(150),
                fmt: Some(nwg::ListViewColumnFlags::LEFT),
            });
            self.list.insert_column(nwg::InsertListViewColumn {
                index: Some(1),
                text: Some("VID:PID".to_string()),
                width: Some(200),
                fmt: Some(nwg::ListViewColumnFlags::LEFT),
            });
            self.list.insert_column(nwg::InsertListViewColumn {
                index: Some(2),
                text: Some("Device".to_string()),
                width: Some(400),
                fmt: Some(nwg::ListViewColumnFlags::LEFT),
            });
            self.list.insert_column(nwg::InsertListViewColumn {
                index: Some(3),
                text: Some("STATE".to_string()),
                width: Some(200),
                fmt: Some(nwg::ListViewColumnFlags::LEFT),
            });
        }
    }

    fn show_devices(&self) {
        self.setup_columns();
        self.list.clear();
        let devices: Vec<UsbipDevice> = list_devices();

        for usb_device in devices.iter() {
            // 1. Insert the first column at the end of the list
            self.list.insert_item(nwg::InsertListViewItem {
                index: None, // None = append as new row
                column_index: 0,
                text: Some(usb_device.busid.clone()),
                image: None,
            });
            
            // 2. Get the newly inserted item's index
            let row_index = self.list.len() as i32 - 1;
            
            // 3. Insert the other columns for this row
            self.list.insert_item(nwg::InsertListViewItem {
                index: Some(row_index), // Specify row
                column_index: 1,
                text: Some(usb_device.vidpid.clone()),
                image: None,
            });
            self.list.insert_item(nwg::InsertListViewItem {
                index: Some(row_index),
                column_index: 2,
                text: Some(usb_device.device.clone()),
                image: None,
            });
            self.list.insert_item(nwg::InsertListViewItem {
                index: Some(row_index),
                column_index: 3,
                text: Some(usb_device.state.clone()),
                image: None,
            });
        }
    }
}




fn main() {
    nwg::init().expect("Failed to init Native Windows GUI");
    nwg::Font::set_global_family("Segoe UI").expect("Failed to set default font");
    let _app = BasicApp::build_ui(Default::default()).expect("Failed to build UI");
    match windows::is_app_elevated() {
        true => {
            ();
        }
        false => {
            nwg::modal_error_message(&_app.window, "Error", "Administrator rights needed!");
            nwg::stop_thread_dispatch();
        }
    }
    _app.add_firewall_rule()
        .expect("Failed to add firewall rule!");
    _app.install_if_needed();
    _app.show_devices();
    nwg::dispatch_thread_events();
}
