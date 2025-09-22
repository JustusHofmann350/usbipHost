use crate::BasicApp;
use native_windows_gui as nwg;
use std::process::{Command, Stdio};
use std::io::{self, Read};
use std::os::windows::process::CommandExt;
use std::error::Error;

const CREATE_NO_WINDOW: u32 = 0x08000000;

impl BasicApp {

    pub fn say_hello(&self) {
        nwg::modal_info_message(&self.window, "Hello", &format!("Hello {}", self.name_edit.text()));
    }
    
    pub fn say_goodbye(&self) {
        nwg::stop_thread_dispatch();
    }

    fn ask_user_yes_no(&self, content: &String) -> bool {
    let choice = nwg::message(&nwg::MessageParams {
        title: "Confirmation",
        content: &String::from(content),
        buttons: nwg::MessageButtons::YesNo,
        icons: nwg::MessageIcons::Question
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


    fn install_usbip(&self) -> Result<bool, io::Error>{
        let hash_override = Command::new("cmd")
        .args(["/c", "winget settings --enable InstallerHashOverride"])
        .creation_flags(CREATE_NO_WINDOW)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()?;

        if !hash_override.success() {
            return Ok(false);
        }

        let install = Command::new("cmd")
        .args(["/c", "winget install --accept-source-agreements --ignore-security-hash --nowarn --force --accept-package-agreements --silent --disable-interactivity --exact dorssel.usbipd-win"])
        .creation_flags(CREATE_NO_WINDOW)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()?;

        if !install.success() {
            return Ok(false);
        }

        Ok(true)
    }


    pub fn add_firewall_rule(&self) -> Result<(), Box<dyn Error>> {
        let rule_name = "_Plex (Port 3240)";
        let check_cmd = format!(r#"netsh advfirewall firewall show rule name="{}""#, rule_name);

        let mut child = Command::new("cmd")
            .args(["/C", &check_cmd])
            .creation_flags(CREATE_NO_WINDOW)
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()?;

        let mut stdout = child.stdout.take().ok_or("Failed to capture stdout")?;
        let mut data = String::new();
        stdout.read_to_string(&mut data)?;

        let status = child.wait()?;
        if !status.success() {
            return Err(format!("Check command exited with status: {}", status).into());
        }

        if !data.contains(rule_name) {
            let add_cmd = format!(
                r#"netsh advfirewall firewall add rule name="{}" dir=in action=allow protocol=TCP localport=3240 edge=yes"#,
                rule_name
            );

            let status = Command::new("cmd")
                .args(["/C", &add_cmd])
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
            let accepted = self.ask_user_yes_no(&String::from("USBIP is not installed. Install now?"));

            if accepted {
                match self.install_usbip() {
                    Ok(true) => {
                        nwg::modal_info_message(&self.window, "Successful", "Installation Successful! Close the application and start it again.");
                        nwg::stop_thread_dispatch();
                    },
                    Ok(false) => {
                        nwg::modal_error_message(&self.window, "Error", "Error occured while installing");
                        nwg::stop_thread_dispatch();
                    }
                    Err(_) => {
                        nwg::modal_error_message(&self.window, "Error", "Error occured while installing");
                        nwg::stop_thread_dispatch();
                    }
                }
            } else {
                nwg::modal_info_message(&self.window, "Info", "You can install it later by relaunching the program and accepting the installation.");
                nwg::stop_thread_dispatch();
            }
        }
    }
}