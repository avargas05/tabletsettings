use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Area {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Mode {
    width: i32,
    height: i32,
    refresh: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct Inhibitor {
    user: String,
    application: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct WindowProperties {
    class: String,
    instance: String,
    title: String,
    transient_for: Option<i32>,
    window_role: Option<String>,
    window_type: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WaylandOutput {
    id: i32,
    pub r#type: String,
    orientation: String,
    percent: Option<f64>,
    urgent: bool,
    marks: Vec<Option<i32>>,
    focused: bool,
    layout: Option<String>,
    border: String,
    current_border_width: i32,
    pub rect: Area,
    deco_rect: Area,
    pub window_rect: Area,
    geometry: Area,
    pub name: Option<String>,
    window: Option<i32>,
    pub nodes: Option<Vec<WaylandOutput>>,
    floating_nodes: Option<Vec<WaylandOutput>>,
    focus: Option<Vec<i64>>,
    fullscreen_mode: i8,
    sticky: Option<bool>,
    num: Option<i32>,
    output: Option<String>,
    representation: Option<String>,
    pid: Option<i32>,
    app_id: Option<String>,
    visible: Option<bool>,
    active: Option<bool>,
    dpms: Option<bool>,
    primary: Option<bool>,
    make: Option<String>,
    model: Option<String>,
    serial: Option<String>,
    scale: Option<f64>,
    scale_filter: Option<String>,
    transform: Option<String>,
    adaptive_sync_status: Option<String>,
    current_workspace: Option<String>,
    modes: Option<Vec<Mode>>,
    current_mode: Option<Mode>,
    max_render_time: Option<i32>,
    shell: Option<String>,
    inhibit_idle: Option<bool>,
    idle_inhibitors: Option<Inhibitor>,
    window_properties: Option<WindowProperties>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Libinput {
    send_events: String,
    pub left_handed: Option<String>,
    tap: Option<String>,
    tap_button_map: Option<String>,
    tap_drag: Option<String>,
    tap_drag_lock: Option<String>,
    accel_profile: Option<String>,
    natural_scroll: Option<String>,
    scroll_method: Option<String>,
    scroll_button: Option<i32>,
    middle_emulation: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Input {
    pub identifier: String,
    pub name: String,
    pub vendor: i32,
    pub product: i32,
    pub r#type: String,
    pub libinput: Libinput,
    scroll_factor: Option<f32>,
    xkb_layout_names: Option<Vec<String>>,
    xkb_active_layout_index: Option<i32>,
    xkb_active_layout_name: Option<String>,
}

#[derive(Debug, Clone)]
pub struct TabletDevice {
    pub identifier: String,
    pub name: String,
    pub vendor: i32,
    pub product: i32,
}

#[derive(Debug, Clone)]
pub struct Monitor {
    pub name: String,
    pub width: i32,
    pub height: i32,
    pub x: i32,
    pub y: i32,
}


#[derive(Debug, Clone)]
pub struct AppWindow {
    pub name: String,
    pub width: i32,
    pub height: i32,
    pub x: i32,
    pub y: i32,
}
