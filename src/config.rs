use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct InputDevicesConfig {
    /// If true, the devices gets detected automatically
    pub autodetect: bool,

    /// The input devices (only if autodetect is false)
    pub input_device: Vec<InputDeviceConfig>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct InputDeviceConfig {
    /// The name of the input device.
    pub name: String,

    /// If false, the input device will be ignored.
    #[serde(default = "default_true")]
    pub active: bool,

    /// The path to the input device, usually /dev/input/by-id/...
    pub path: String,

    /// If true, the keys which are supported by the input device will be automatically detected.
    #[serde(default = "default_true")]
    pub autodetect_keys: bool,

    /// If true, the LEDs which are supported by the input device will be automatically detected.
    #[serde(default = "default_true")]
    pub autodetect_leds: bool,

    /// If true, the relative axes which are supported by the input device will be automatically detected.
    #[serde(default = "default_true")]
    pub autodetect_relative_axes: bool,

    /// If true, the absolute axes which are supported by the input device will be automatically detected.
    #[serde(default = "default_true")]
    pub autodetect_absolute_axes: bool,

    /// If true, the switches which are supported by the input device will be automatically detected.
    #[serde(default = "default_true")]
    pub autodetect_switches: bool,
}

fn default_true() -> bool {
    true
}
