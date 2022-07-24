use std::process::{Command, Stdio};
use std::cell::{RefCell, RefMut};
use std::rc::Rc;

use gtk4::prelude::*;
use gtk4::subclass::prelude::*;
use gtk4::{gio, glib, CompositeTemplate};
use serde_json;
use unicode_truncate::UnicodeTruncateStr;

use crate::config::APP_ID;
use crate::structs::*;
use crate::tablet::Device;

mod imp {
    use super::*;
#[derive(Debug, CompositeTemplate)] #[template(resource = "/com/github/avargas05/tabletsettings/window.ui")]
    // Declare all application variables
    pub struct TabletsettingsWindow {

        // The settings for the application
        pub settings: gio::Settings,

        // Vector with the computer monitors names and properties
        pub monitors: Rc<RefCell<Vec<Monitor>>>,

        // Vector with the application window names and properties
        pub windows: Rc<RefCell<Vec<AppWindow>>>,

        // Vector for tablet devices names and properties
        pub tablet_devices: Rc<RefCell<Vec<TabletDevice>>>,

        #[template_child]
        pub header_bar: TemplateChild<gtk4::HeaderBar>,
        #[template_child]
        pub devices_combobox: TemplateChild<gtk4::ComboBoxText>,
        #[template_child]
        pub monitors_combobox: TemplateChild<gtk4::ComboBoxText>,
        #[template_child]
        pub windows_combobox: TemplateChild<gtk4::ComboBoxText>,
        #[template_child]
        pub rotation_combobox: TemplateChild<gtk4::ComboBoxText>,
        #[template_child]
        pub reset_all_button: TemplateChild<gtk4::Button>,
        #[template_child]
        pub aspect_ratio_checkbutton: TemplateChild<gtk4::CheckButton>,
        #[template_child]
        pub apply_button: TemplateChild<gtk4::Button>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for TabletsettingsWindow {
        const NAME: &'static str = "TabletsettingsWindow";
        type Type = super::TabletsettingsWindow;
        type ParentType = gtk4::ApplicationWindow;

        // Initialize all application variables
        fn new() -> Self {
            Self {
                settings: gio::Settings::new(APP_ID),
                header_bar: TemplateChild::default(),
                devices_combobox: TemplateChild::default(),
                monitors_combobox: TemplateChild::default(),
                windows_combobox: TemplateChild::default(),
                rotation_combobox: TemplateChild::default(),
                reset_all_button: TemplateChild::default(),
                aspect_ratio_checkbutton: TemplateChild::default(),
                apply_button: TemplateChild::default(),
                monitors: Rc::new(RefCell::new(Vec::new())),
                windows: Rc::new(RefCell::new(Vec::new())),
                tablet_devices: Rc::new(RefCell::new(Vec::new())),
            }
        }

        fn class_init(klass: &mut Self::Class) {
            Self::bind_template(klass);
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for TabletsettingsWindow {
        // Construct all the application widgets
        fn constructed(&self, obj: &Self::Type) {
            self.parent_constructed(obj);
            obj.load_window_size();

            // Add tablet devices to devices comboboxtext
            obj.load_devices();

            // Add computer monitors to monitors comboboxtext
            obj.load_monitors_and_windows();

            // Add options avaible for rotating the tablet
            obj.load_rotation_options();

            obj.connect_signals();
        }
    }
    impl WidgetImpl for TabletsettingsWindow {}
    impl WindowImpl for TabletsettingsWindow {
        // Function for closing the window
        fn close_request(&self, window: &Self::Type) -> glib::signal::Inhibit {
            window.save_window_size().expect("Failed to save window state");
            glib::signal::Inhibit(false)
        }
    }
    impl ApplicationWindowImpl for TabletsettingsWindow {}
}

glib::wrapper! {
    pub struct TabletsettingsWindow(ObjectSubclass<imp::TabletsettingsWindow>)
        @extends gtk4::Widget, gtk4::Window, gtk4::ApplicationWindow,
        @implements gio::ActionGroup, gio::ActionMap;
}

impl TabletsettingsWindow {
    // Create new window for tablet settings
    pub fn new<P: glib::IsA<gtk4::Application>>(application: &P) -> Self {
        glib::Object::new(&[("application", application)])
            .expect("Failed to create TabletsettingsWindow")
    }

    // Saves the size of the window
    fn save_window_size(&self) -> Result<(), glib::BoolError> {
        let settings = &self.imp().settings;
        let size = self.default_size();

        settings.set_int("window-width", size.0)?;
        settings.set_int("window-height", size.1)?;
        settings.set_boolean("is-maximized", self.is_maximized())?;

        Ok(())
    }

    // Load the window size
    fn load_window_size(&self) {
        let settings = &self.imp().settings;

        let width = settings.int("window-width");
        let height = settings.int("window-height");
        let is_maximized = settings.boolean("is-maximized");

        self.set_default_size(width, height);

        if is_maximized {
            self.maximize();
        }
    }

    // Load all wacom devices to the devices comboboxtext
    fn load_devices(&self) {
        // ComboBoxText for wacom devices
        let devices = &self.imp().devices_combobox;
        self.imp().aspect_ratio_checkbutton.set_active(true);

        // Vector for devices
        let mut device_list: RefMut<Vec<TabletDevice>> = self.imp().tablet_devices.borrow_mut();

        // Shell command for getting inputs through swaymsg in json format
        let command = Command::new("sh")
            .arg("-c")
            .arg("swaymsg -t get_inputs")
            .stdout(Stdio::piped())
            .output()
            .unwrap();

        // Parse json output
        let stdout = String::from_utf8(command.stdout).unwrap();
        let inputs: Vec<Input> = serde_json::from_str(&stdout).unwrap();

        // Insert blank option into ComboBoxText
        devices.append_text("");

        // Device types to search for
        let device_type = ["tablet_tool", "tablet_pad", "touchpad"];

        // Iterate through all inputs available for device types
        for input in inputs.iter() {
            // If device type in list
            if device_type.contains(&&input.r#type[..]) {
                // Name of device
                let name = &input.name;
                devices.append_text(name);

                // Create device struct and add to vector
                let device = TabletDevice {
                    name: name.to_string(),
                    identifier: String::from(&input.identifier),
                    vendor: input.vendor,
                    product: input.product,
                };

                device_list.push(device);
            }
        }
    }

    // Load all computer monitors and application windows to their respective comboboxtext
    fn load_monitors_and_windows(&self) {
        // ComboBoxText for monitors and windows
        let monitor_names = &self.imp().monitors_combobox;
        let windows = &self.imp().windows_combobox;

        // Vector for monitors
        let mut monitors: RefMut<Vec<Monitor>> = self.imp().monitors.borrow_mut();

        // Shell command for getting tree from swaymsg in json format
        let command = Command::new("sh")
            .arg("-c")
            .arg("swaymsg -t get_tree")
            .stdout(Stdio::piped())
            .output()
            .unwrap();

        // Parse json output
        let stdout = String::from_utf8(command.stdout).unwrap();
        let root: WaylandOutput = serde_json::from_str(&stdout).unwrap();

        // Insert blank option to ComboBoxText
        monitor_names.append_text("");
        windows.append_text("");

        // Iterate through all outputs and add to ComboBoxText
        let displays = root.nodes.unwrap();
        for display in displays {
            let name = display.name.unwrap();

            // Monitors will contain underscore in name
            if !name.contains("_") {
                // Add to ComboBoxText
                monitor_names.append_text(&name);

                // Add to vector
                monitors.push(
                    Monitor {
                        name,
                        width: display.rect.width,
                        height: display.rect.height,
                        x: display.rect.x,
                        y: display.rect.y,
                    }
                );

                // Iterate through workspaces and add windows to windows' ComboBoxText
                let workspaces = display.nodes.unwrap();
                if workspaces.len() > 0 {
                    for workspace in workspaces {
                        // Recursive function for iterating through json tree
                        scan_tree(&workspace, windows);
                    }
                }
            }
        }
    }

    // Sway only has left and right-handed options
    fn load_rotation_options(&self) {
        let rotation = &self.imp().rotation_combobox;

        rotation.append_text("");
        rotation.append_text("Left-handed");
        rotation.append_text("Right-handed");
    }

    // Connect on-click functions to buttons
    fn connect_signals(&self) {
        // Get apply button
        let apply_button = &self.imp().apply_button;

        // Connect the signals
        apply_button.connect_clicked(glib::clone!(@weak self as app => move |_| {
            // Get the Vectors
            let devices: Vec<TabletDevice> = app.imp().tablet_devices.borrow().to_vec();
            let monitors: Vec<Monitor> = app.imp().monitors.borrow().to_vec();

            // Get the ComboBoxTexts' and CheckButton's values
            let device: glib::GString = app.imp().devices_combobox.active_text().unwrap();
            let monitor: Option<glib::GString> = app.imp().monitors_combobox.active_text();
            let window: Option<glib::GString> = app.imp().windows_combobox.active_text();
            let rotation: Option<glib::GString> = app.imp().rotation_combobox.active_text();
            let keep = app.imp().aspect_ratio_checkbutton.is_active();

            // Initialize empty struct for tablet device
            let mut tablet_device: &TabletDevice = &TabletDevice {
                identifier: String::from(""),
                name: String::from(""),
                vendor: 0,
                product: 0
            };

            for input in &devices {
                if input.name == device {
                    tablet_device = input;
                    break;
                }
            }

            // Handle rotation options
            if let Some(rotation) = rotation {
                let rotation_selected: &str = rotation.as_str();
                let mut rotation_option = "disabled";

                // Do nothing if blank selected
                if rotation_selected != "" {
                    if rotation_selected == "Right-handed" {
                        rotation_option = "disabled";
                    } else if rotation_selected == "Left-handed" {
                        rotation_option = "enabled";
                    }

                    let command: String = format!(
                        "swaymsg input {} left_handed {}",
                        tablet_device.identifier, rotation_option
                    );

                    Command::new("sh")
                        .arg("-c")
                        .arg(command)
                        .stdout(Stdio::piped())
                        .output()
                        .unwrap();
                }
            }

            // Check which boxes contain values. Ignore monitor if window is chosen
            if let Some(window) = window {
                // Get value from window ComboBoxText
                let name: &str = window.as_str();
                if name != "" {
                    apply_to_window(name, &tablet_device, keep);
                } else {
                    if let Some(monitor) = monitor {
                        apply_to_monitor(&monitor, &monitors, &tablet_device, keep);
                    } else {
                        // TODO: Add pop-up message to select a monitor or window
                    }
                }
            } else {
                // Monitor selected
                if let Some(monitor) = monitor {
                    apply_to_monitor(&monitor, &monitors, &tablet_device, keep);
                } else {
                    // TODO: Add pop-up message to select a monitor or window
                }
            }
        }));

        let reset_all_button = &self.imp().reset_all_button;
        reset_all_button.connect_clicked(glib::clone!(@weak self as app => move |_| {
            let devices: Vec<TabletDevice> = app.imp().tablet_devices.borrow().to_vec();
            let monitors: Vec<Monitor> = app.imp().monitors.borrow().to_vec();
            let device: glib::GString = app.imp().devices_combobox.active_text().unwrap();
            let monitor: Option<glib::GString> = app.imp().monitors_combobox.active_text();
            let window: Option<glib::GString> = app.imp().windows_combobox.active_text();
            let rotation: Option<glib::GString> = app.imp().rotation_combobox.active_text();
            let keep = app.imp().aspect_ratio_checkbutton.is_active();
        }));
    }
}

fn adjust_tablet_ratio(tablet_device: &TabletDevice, window_width: i32, window_height: i32)
{
    // Get wacom dimensions
    // Will only work with wacom devices since sway doesn't query for the dimensions
    // of the device through swaymsg -t get_inputs
    let wacom_device = Device::new(tablet_device.vendor, tablet_device.product);

    if let Ok(wacom_device) = wacom_device {
        // Calculate the ratios
        let display_ratio: f32 = window_width as f32 / window_height as f32;
        let tablet_ratio: f32 = wacom_device.width as f32 / wacom_device.height as f32;

        // Initialize variables for x and y positions
        let mut x1: f32 = 0.0;
        let mut x2: f32 = wacom_device.width as f32;
        let mut y1: f32 = 0.0;
        let mut y2: f32 = wacom_device.height as f32;

        // Redefine x's and y's according to which ratio is greater
        if tablet_ratio > display_ratio {
            let width: f32 = y2 * display_ratio;
            x1 = ((x2 - width) / 2.0) / x2;
            x2 = 1.0 - x1;
            y2 = 1.0;
        } else {
            let height: f32 = x2 / display_ratio;
            y1 = ((y2 - height) / 2.0) / y2;
            y2 = 1.0 - y1;
            x2 = 1.0;
        }

        // Shell command for mapping the tablet's active region
        let ratio_command: String = format!(
            "swaymsg input {0} map_from_region {1:.6}x{2:.6} {3:.6}x{4:.6}",
            tablet_device.identifier, x1, y1, x2, y2
        );

        Command::new("sh")
            .arg("-c")
            .arg(ratio_command)
            .stdout(Stdio::piped())
            .output()
            .unwrap();

    }
}

// Recursive function for iterating through the output tree for application window names
// and adding to the ComboBoxText

fn scan_tree(container: &WaylandOutput, windows: &TemplateChild<gtk4::ComboBoxText>)
{
    // Get nodes from containers
    let containers = &container.nodes;

    if let Some(nodes) = containers {
        // If more nodes iterate through nodes
        if nodes.len() > 0 {
            for node in nodes {
                scan_tree(&node, windows);
            }
        } else {
            // If no nodes, get name
            let app_name = container.name.as_ref().unwrap();
            let mut name: &str = &app_name.to_string().clone()[..];

            // Clip if too long
            if name.len() > 30 {
                // Added truncate function in case strings are not UTF-8
                let mut _i: usize = 0;
                (name, _i) = name.unicode_truncate(30);
            }

            // Add to ComboBoxText
            windows.append_text(name);
        }
    }
}

// Recursive function for iterating through and searching for matching name selection
fn scan_tree_for_window(container: &WaylandOutput, selection: &str) -> AppWindow {
    let containers = &container.nodes;

    // Create mutable blank struct to bring variable into outer scope
    let mut window: AppWindow = AppWindow {
        name: "blank".to_string(),
        width: 0,
        height: 0,
        x: 0,
        y: 0,
    };

    if let Some(nodes) = containers {
        // If more nodes iterate through nodes
        if nodes.len() > 0 {
            for node in nodes {
                window = scan_tree_for_window(&node, selection);

                // When match found break and return window
                if window.name != "blank" {
                    break;
                }
            }
        } else {
            // If no nodes, get name
            let app_name = container.name.as_ref().unwrap();
            let mut name: &str = &app_name.to_string().clone()[..];

            // Clip if too long
            if name.len() > 30 {
                // Added truncate function in case strings are not UTF-8
                let mut _i: usize = 0;
                (name, _i) = name.unicode_truncate(30);
            }

            // If name matches return window struct otherwise returns blank struct
            if name == selection {
                // Define variables for application window
                let width: i32 = container.window_rect.width;
                let height: i32 = container.window_rect.height;
                let x: i32 = container.rect.x + container.window_rect.x;
                let y: i32 = container.rect.y + container.window_rect.y;

                // Create struct for application window
                window = AppWindow {
                    name: name.to_string().clone(),
                    width,
                    height,
                    x,
                    y,
                };
            }
        }
    }
    return window;
}

fn apply_to_window(name: &str, tablet_device: &TabletDevice, keep: bool) {
    // Initialize app struct
    let mut app_window: AppWindow = AppWindow {
        name: "blank".to_string(),
        width: 0,
        height: 0,
        x: 0,
        y: 0,
    };

    // Shell command for getting tree from swaymsg in json format
    // Sway is a tiling window manager so chances are the window dimensions will
    // change from opening this program. Getting the names first then matching and
    // getting the dimensions after will allow the user to change to float or move
    // elsewhere before applying the selected window's dimensions.
    let command = Command::new("sh")
        .arg("-c")
        .arg("swaymsg -t get_tree")
        .stdout(Stdio::piped())
        .output()
        .unwrap();

    // Parse json output
    let stdout = String::from_utf8(command.stdout).unwrap();
    let root: WaylandOutput = serde_json::from_str(&stdout).unwrap();
    let displays = root.nodes.unwrap();

    // Iterate through windows for name match and return struct
    for display in displays {
        if !display.name.unwrap().contains("_") {
            let workspaces = display.nodes.unwrap();
            if workspaces.len() > 0 {
                for workspace in workspaces {
                    // Recursive function to get the struct with matching name
                    app_window = scan_tree_for_window(&workspace, name);
                    if app_window.name != "blank" {
                        break;
                    }
                }
            }
        }

        // Break if app found
        if app_window.name != "blank" {
            break;
        }
    }

    // Shell command to map the region through swaymsg
    let command: String = format!(
        "swaymsg input {} map_to_region {} {} {} {}",
        tablet_device.identifier, app_window.x, app_window.y, app_window.width, app_window.height
    );

    Command::new("sh")
        .arg("-c")
        .arg(command)
        .stdout(Stdio::piped())
        .output()
        .unwrap();


    // Adjust the ratio for the region if keep is checked
    if keep {
        adjust_tablet_ratio(&tablet_device, app_window.width, app_window.height);
    }
}

fn apply_to_monitor(monitor: &glib::GString, monitors: &Vec<Monitor>,tablet_device: &TabletDevice, keep: bool) {
    for display in monitors {
        if display.name == monitor.as_str() {
            // Shell command for mapping to output
            let command: String = format!("swaymsg input {} map_to_output {}",
                tablet_device.identifier, display.name
            );

            Command::new("sh")
                .arg("-c")
                .arg(command)
                .stdout(Stdio::piped())
                .output()
                .unwrap();

            // Adjust tablet ratio if keep ratio selected
            if keep {
                adjust_tablet_ratio(&tablet_device, display.width, display.height);
            }
            break;
        }
    }
}
