use crate::BasicApp;
use native_windows_gui as nwg;
use std::error::Error;
use std::io::{self, Read};
use std::os::windows::process::CommandExt;
use std::process::{Command, Stdio};

const CREATE_NO_WINDOW: u32 = 0x08000000;

impl BasicApp {
    pub fn say_hello(&self) {
        nwg::modal_info_message(
            &self.window,
            "Hello",
            &format!("Hello {}", self.name_edit.text()),
        );
    }

    pub fn say_goodbye(&self) {
        nwg::stop_thread_dispatch();
    }

    fn ask_user_yes_no(&self, content: &String) -> bool {
        let choice = nwg::message(&nwg::MessageParams {
            title: "Confirmation",
            content: &String::from(content),
            buttons: nwg::MessageButtons::YesNo,
            icons: nwg::MessageIcons::Question,
        });

        match choice {
            nwg::MessageChoice::Yes => true,
            _ => false,
        }
    }

    fn usbipd_installed(&self) -> bool {
        let status = Command::new("where.exe")
            .arg("usbipd.exe")
            .creation_flags(CREATE_NO_WINDOW)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status();

        match status {
            Ok(s) => s.success(), // returns true if exit code == 0
            Err(_) => false,
        }
    }

    fn install_usbip(&self) -> Result<bool, io::Error> {
        let hash_override = Command::new("winget")
            .args(["settings", "--enable", "InstallerHashOverride"])
            .creation_flags(CREATE_NO_WINDOW)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()?;

        if !hash_override.success() {
            return Ok(false);
        }

        let install = Command::new("winget")
            .args([
                "install",
                "--accept-source-agreements",
                "--ignore-security-hash",
                "--nowarn",
                "--force",
                "--accept-package-agreements",
                "--silent",
                "--disable-interactivity",
                "--ignore-security-hash",
                "--exact",
                "dorssel.usbipd-win",
            ])
            .creation_flags(CREATE_NO_WINDOW)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()?;

        Ok(install.success())
    }


    pub fn add_firewall_rule(&self) -> Result<(), Box<dyn Error>> {
        let rule_name = "_Plex (Port 3240)";

        let output = Command::new("netsh")
            .args([
                "advfirewall",
                "firewall",
                "show",
                "rule",
                "name=_Plex (Port 3240)",
            ])
            .creation_flags(CREATE_NO_WINDOW)
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .output()?;

        let data = String::from_utf8_lossy(&output.stdout);

        if !data.contains(rule_name) {
            let status = Command::new("netsh")
                .args([
                    "advfirewall",
                    "firewall",
                    "add",
                    "rule",
                    &format!("name={}", rule_name),
                    "dir=in",
                    "action=allow",
                    "protocol=TCP",
                    "localport=3240",
                    "edge=yes",
                ])
                .creation_flags(CREATE_NO_WINDOW)
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .status()?;

            if !status.success() {
                return Err(format!("Add rule command failed with status: {}", status).into());
            }
        }

        Ok(())
    }

    pub fn install_if_needed(&self) {
        if !self.usbipd_installed() {
            let accepted =
                self.ask_user_yes_no(&String::from("USBIP is not installed. Install now?"));

            if accepted {
                match self.install_usbip() {
                    Ok(true) => {
                        nwg::modal_info_message(
                            &self.window,
                            "Successful",
                            "Installation Successful! Close the application and start it again.",
                        );
                        nwg::stop_thread_dispatch();
                    }
                    Ok(false) => {
                        nwg::modal_error_message(
                            &self.window,
                            "Error",
                            "Error while installing.",
                        );
                        nwg::stop_thread_dispatch();
                    }
                    Err(_) => {
                        nwg::modal_error_message(
                            &self.window,
                            "Error",
                            "Winget can't be found. Is it installed?",
                        );
                        nwg::stop_thread_dispatch();
                    }
                }
            } else {
                nwg::modal_info_message(
                    &self.window,
                    "Info",
                    "You can install it later by relaunching the program and accepting the installation.",
                );
                nwg::stop_thread_dispatch();
            }
        }
    }

    fn get_usbipd_version(&self) -> String {
        let mut output = String::new();

        let mut child = match Command::new("cmd")
            .args(["/C", "usbipd --version"])
            .creation_flags(CREATE_NO_WINDOW)
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
        {
            Ok(child) => child,
            Err(_) => return String::from("Error: Could not execute usbipd"),
        };

        if let Some(mut stdout) = child.stdout.take() {
            let _ = stdout.read_to_string(&mut output);
        }

        let _ = child.wait();

        let search = "Branch.master";
        if let Some(pos) = output.find(search) {
            let end = pos + search.len();
            output = output[..end].to_string();
        }

        output.trim().to_string()
    }

    pub fn show_about(&self) {
        let message = format!(
            "Tool for sharing USB IP bus via IP\n\n\
            Author: Justus Hofmann\n\
            Program Version: {}\n\
            usbipd-win Version: {}",
            env!("CARGO_PKG_VERSION"),
            self.get_usbipd_version()
        );

        nwg::modal_info_message(
                            &self.window,
                            "About",
                            &message,
                        );
    }


    pub fn upgrade_usbipd(&self) {
        match self.usbipd_installed() {
            true => {
                let upgrade_result = Command::new("winget")
                .args([
                    "upgrade",
                    "--silent",
                    "--disable-interactivity",
                    "--exact",
                    "dorssel.usbipd-win"
                ])
                .creation_flags(CREATE_NO_WINDOW)
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .status();

            match upgrade_result {
                Ok(status) => {
                    if status.success() {
                        nwg::modal_info_message(
                            &self.window,
                            "Upgrade",
                            "Upgrade successful.",
                        );
                    } else {
                        if self.usbipd_installed() {
                            nwg::modal_error_message(
                            &self.window,
                            "Upgrade failed",
                            "Winget failed to upgrade the package. The package is probably already installed.",
                        );
                        }
                        else {
                            nwg::modal_error_message(
                            &self.window,
                            "Upgrade failed",
                            "Winget failed to upgrade the package.",
                        );
                        }
                    }
                }
                Err(_) => {
                    nwg::modal_error_message(
                        &self.window,
                        "Error",
                        "Failed to run winget. Is it installed?",
                    );
                }
            }
            }
            false => {
                let _ = self.install_usbip();
            }
        }
    }


    pub fn uninstall_usbip(&self) {
        let accepted = self.ask_user_yes_no(&String::from(
            "Are you sure you want to uninstall the pacakage?",
        ));

        if accepted {
            let uninstall_result = Command::new("winget")
                .args([
                    "uninstall",
                    "-h",
                    "dorssel.usbipd-win"
                ])
                .creation_flags(CREATE_NO_WINDOW)
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .status();

            match uninstall_result {
                Ok(status) => {
                    if status.success() {
                        nwg::modal_info_message(
                            &self.window,
                            "Uninstall",
                            "Uninstallation successful.",
                        );
                    } else {
                        nwg::modal_error_message(
                            &self.window,
                            "Uninstall Failed",
                            "Winget failed to uninstall the package.",
                        );
                    }
                }
                Err(_) => {
                    nwg::modal_error_message(
                        &self.window,
                        "Error",
                        "Failed to run winget. Is it installed?",
                    );
                }
            }
        }
    }
}
