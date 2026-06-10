use serde::Serialize;
use usbd_hid::descriptor::generator_prelude::*;

/// KeyboardReport describes a report and its companion descriptor that can be
/// used to send keyboard button presses to a host and receive the status of the
/// keyboard LEDs.
#[gen_hid_descriptor(
    (collection = APPLICATION, usage_page = GENERIC_DESKTOP, usage = KEYBOARD) = {
        (usage_page = KEYBOARD, usage_min = 0xE0, usage_max = 0xE7) = {
            #[packed_bits 8] #[item_settings data,variable,absolute] modifier=input;
        };
        (logical_min = 0,) = {
            #[item_settings constant,variable,absolute] reserved=input;
        };
        (usage_page = LEDS, usage_min = 0x01, usage_max = 0x05) = {
            #[packed_bits 5] #[item_settings data,variable,absolute] leds=output;
        };
        (usage_page = KEYBOARD, usage_min = 0x00, usage_max = 0xDD) = {
            #[item_settings data,array,absolute] keycodes=input;
        };
    }
)]
#[allow(dead_code)]
#[derive(Default)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct KeyboardReport {
    pub modifier: u8, // ModifierCombination
    pub reserved: u8,
    pub leds: u8, // LedIndicator
    pub keycodes: [u8; 6],
}

#[gen_hid_descriptor(
    (collection = APPLICATION, usage_page = 0xFF60, usage = 0x61) = {
        (usage = 0x62, logical_min = 0x0) = {
            #[item_settings data,variable,absolute] input_data=input;
        };
        (usage = 0x63, logical_min = 0x0) = {
            #[item_settings data,variable,absolute] output_data=output;
        };
    }
)]
#[derive(Default)]
pub struct ViaReport {
    pub(crate) input_data: [u8; 32],
    pub(crate) output_data: [u8; 32],
}

/// Predefined report ids for composite hid report.
/// Should be same with `#[gen_hid_descriptor]`
/// DO NOT EDIT
#[repr(u8)]
#[derive(Debug, Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Serialize)]

pub enum CompositeReportType {
    #[default]
    None = 0x00,
    Mouse = 0x01,
    Media = 0x02,
    System = 0x03,
}

impl CompositeReportType {
    fn from_u8(report_id: u8) -> Self {
        match report_id {
            0x01 => Self::Mouse,
            0x02 => Self::Media,
            0x03 => Self::System,
            _ => Self::None,
        }
    }
}

/// A composite hid report which contains mouse, consumer, system reports.
/// Report id is used to distinguish from them.
#[gen_hid_descriptor(
    (collection = APPLICATION, usage_page = GENERIC_DESKTOP, usage = MOUSE) = {
        (collection = PHYSICAL, usage = POINTER) = {
            (report_id = 0x01,) = {
                (usage_page = BUTTON, usage_min = BUTTON_1, usage_max = BUTTON_8) = {
                    #[packed_bits 8] #[item_settings data,variable,absolute] buttons=input;
                };
                (usage_page = GENERIC_DESKTOP,) = {
                    (usage = X,) = {
                        #[item_settings data,variable,relative] x=input;
                    };
                    (usage = Y,) = {
                        #[item_settings data,variable,relative] y=input;
                    };
                    (usage = WHEEL,) = {
                        #[item_settings data,variable,relative] wheel=input;
                    };
                };
                (usage_page = CONSUMER,) = {
                    (usage = AC_PAN,) = {
                        #[item_settings data,variable,relative] pan=input;
                    };
                };
            };
        };
    },
    (collection = APPLICATION, usage_page = CONSUMER, usage = CONSUMER_CONTROL) = {
        (report_id = 0x02,) = {
            (usage_page = CONSUMER, usage_min = 0x00, usage_max = 0x514) = {
            #[item_settings data,array,absolute,not_null] media_usage_id=input;
            }
        };
    },
    (collection = APPLICATION, usage_page = GENERIC_DESKTOP, usage = SYSTEM_CONTROL) = {
        (report_id = 0x03,) = {
            (usage_min = 0x81, usage_max = 0xB7, logical_min = 1) = {
                #[item_settings data,array,absolute,not_null] system_usage_id=input;
            };
        };
    }
)]
#[derive(Default, Serialize)]
pub struct CompositeReport {
    pub(crate) buttons: u8, // MouseButtons
    pub(crate) x: i8,
    pub(crate) y: i8,
    pub(crate) wheel: i8, // Scroll down (negative) or up (positive) this many units
    pub(crate) pan: i8,   // Scroll left (negative) or right (positive) this many units
    pub(crate) media_usage_id: u16,
    pub(crate) system_usage_id: u8,
}

/// USB-only variant of the composite descriptor that adds the HID Resolution
/// Multiplier (hi-res scrolling, like Logitech mice). Hosts that support it
/// (Windows, Linux) write the multiplier feature report and then divide wheel
/// values by 120, so the firmware can emit fine-grained scroll deltas; hosts
/// that ignore it (macOS) keep classic one-notch scrolling.
///
/// `gen_hid_descriptor` cannot emit Physical Min/Max items (mandatory for the
/// multiplier), so this descriptor is written by hand. The input report layouts
/// are byte-identical to `CompositeReport`'s, only the descriptor differs, and
/// it adds one feature report (id 0x01, 1 byte: bits 0-1 wheel multiplier,
/// bits 2-3 pan multiplier) handled in `usb::UsbRequestHandler`.
///
/// The BLE GATT report map keeps using `CompositeReport::desc()` (it is a fixed
/// `[u8; 111]`), so BLE hosts simply never see the multiplier.
pub struct CompositeReportHiRes;

#[rustfmt::skip]
const COMPOSITE_HI_RES_DESC: [u8; 166] = [
    // --- Mouse (report id 0x01), Microsoft "Enhanced Wheel" pattern ---
    0x05, 0x01,       // Usage Page (Generic Desktop)
    0x09, 0x02,       // Usage (Mouse)
    0xA1, 0x01,       // Collection (Application)
    0x09, 0x01,       //   Usage (Pointer)
    0xA1, 0x00,       //   Collection (Physical)
    0x85, 0x01,       //     Report ID (1)
    0x05, 0x09,       //     Usage Page (Button)
    0x19, 0x01,       //     Usage Minimum (Button 1)
    0x29, 0x08,       //     Usage Maximum (Button 8)
    0x15, 0x00,       //     Logical Minimum (0)
    0x25, 0x01,       //     Logical Maximum (1)
    0x75, 0x01,       //     Report Size (1)
    0x95, 0x08,       //     Report Count (8)
    0x81, 0x02,       //     Input (Data,Var,Abs)            ; buttons
    0x05, 0x01,       //     Usage Page (Generic Desktop)
    0x09, 0x30,       //     Usage (X)
    0x09, 0x31,       //     Usage (Y)
    0x15, 0x81,       //     Logical Minimum (-127)
    0x25, 0x7F,       //     Logical Maximum (127)
    0x75, 0x08,       //     Report Size (8)
    0x95, 0x02,       //     Report Count (2)
    0x81, 0x06,       //     Input (Data,Var,Rel)            ; x, y
    0xA1, 0x02,       //     Collection (Logical)            ; wheel + multiplier
    0x09, 0x48,       //       Usage (Resolution Multiplier)
    0x15, 0x00,       //       Logical Minimum (0)
    0x25, 0x01,       //       Logical Maximum (1)
    0x35, 0x01,       //       Physical Minimum (1)
    0x45, 0x78,       //       Physical Maximum (120)
    0x75, 0x02,       //       Report Size (2)
    0x95, 0x01,       //       Report Count (1)
    0xB1, 0x02,       //       Feature (Data,Var,Abs)        ; wheel multiplier, bits 0-1
    0x09, 0x38,       //       Usage (Wheel)
    0x15, 0x81,       //       Logical Minimum (-127)
    0x25, 0x7F,       //       Logical Maximum (127)
    0x35, 0x00,       //       Physical Minimum (0)
    0x45, 0x00,       //       Physical Maximum (0)          ; reset physical range
    0x75, 0x08,       //       Report Size (8)
    0x81, 0x06,       //       Input (Data,Var,Rel)          ; wheel
    0xC0,             //     End Collection
    0xA1, 0x02,       //     Collection (Logical)            ; AC pan + multiplier
    0x09, 0x48,       //       Usage (Resolution Multiplier)
    0x15, 0x00,       //       Logical Minimum (0)
    0x25, 0x01,       //       Logical Maximum (1)
    0x35, 0x01,       //       Physical Minimum (1)
    0x45, 0x78,       //       Physical Maximum (120)
    0x75, 0x02,       //       Report Size (2)
    0xB1, 0x02,       //       Feature (Data,Var,Abs)        ; pan multiplier, bits 2-3
    0x35, 0x00,       //       Physical Minimum (0)
    0x45, 0x00,       //       Physical Maximum (0)
    0x75, 0x04,       //       Report Size (4)
    0xB1, 0x03,       //       Feature (Const,Var,Abs)       ; padding, bits 4-7
    0x05, 0x0C,       //       Usage Page (Consumer)
    0x0A, 0x38, 0x02, //       Usage (AC Pan)
    0x15, 0x81,       //       Logical Minimum (-127)
    0x25, 0x7F,       //       Logical Maximum (127)
    0x75, 0x08,       //       Report Size (8)
    0x81, 0x06,       //       Input (Data,Var,Rel)          ; pan
    0xC0,             //     End Collection
    0xC0,             //   End Collection (Physical)
    0xC0,             // End Collection (Application)
    // --- Media (report id 0x02), byte-identical to CompositeReport::desc() ---
    0x05, 0x0C,       // Usage Page (Consumer)
    0x09, 0x01,       // Usage (Consumer Control)
    0xA1, 0x01,       // Collection (Application)
    0x85, 0x02,       //   Report ID (2)
    0x05, 0x0C,       //   Usage Page (Consumer)
    0x19, 0x00,       //   Usage Minimum (0)
    0x2A, 0x14, 0x05, //   Usage Maximum (0x514)
    0x15, 0x00,       //   Logical Minimum (0)
    0x27, 0xFF, 0xFF, 0x00, 0x00, // Logical Maximum (65535)
    0x75, 0x10,       //   Report Size (16)
    0x81, 0x00,       //   Input (Data,Array,Abs)            ; media_usage_id
    0xC0,             // End Collection
    // --- System control (report id 0x03), byte-identical to CompositeReport::desc() ---
    0x05, 0x01,       // Usage Page (Generic Desktop)
    0x09, 0x80,       // Usage (System Control)
    0xA1, 0x01,       // Collection (Application)
    0x85, 0x03,       //   Report ID (3)
    0x19, 0x81,       //   Usage Minimum (0x81)
    0x29, 0xB7,       //   Usage Maximum (0xB7)
    0x15, 0x01,       //   Logical Minimum (1)
    0x26, 0xFF, 0x00, //   Logical Maximum (255)
    0x75, 0x08,       //   Report Size (8)
    0x81, 0x00,       //   Input (Data,Array,Abs)            ; system_usage_id
    0xC0,             // End Collection
];

impl SerializedDescriptor for CompositeReportHiRes {
    fn desc() -> &'static [u8] {
        &COMPOSITE_HI_RES_DESC
    }
}
