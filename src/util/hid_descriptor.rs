use usbd_hid::descriptor::gen_hid_descriptor;
use usbd_hid::descriptor::generator_prelude::*;

#[rustfmt::skip]
pub const KEYBOARD_REPORT_DESCRIPTOR: &[u8] = &[
    0x05, 0x01,        // Usage Page (Generic Desktop Ctrls)
    0x09, 0x06,        // Usage (Keyboard)
    0xA1, 0x01,        // Collection (Application)

    // Modifier Keys
    0x05, 0x07,        //   Usage Page (Kbrd/Keypad)
    0x19, 0xE0,        //   Usage Minimum (0xE0)
    0x29, 0xE7,        //   Usage Maximum (0xE7)
    0x15, 0x00,        //   Logical Minimum (0)
    0x25, 0x01,        //   Logical Maximum (1)
    0x95, 0x08,        //   Report Count (8)
    0x75, 0x01,        //   Report Size (1)
    0x81, 0x02,        //   Input (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position)

    // Reserved Byte
    0x95, 0x01,        //   Report Count (1)
    0x75, 0x08,        //   Report Size (8)
    0x81, 0x01,        //   Input (Const,Var,Abs,No Wrap,Linear,Preferred State,No Null Position)

    // LEDs
    0x05, 0x08,        //   Usage Page (LEDs)
    0x19, 0x01,        //   Usage Minimum (Num Lock)
    0x29, 0x05,        //   Usage Maximum (Kana)
    0x95, 0x05,        //   Report Count (5)
    0x75, 0x01,        //   Report Size (1)
    0x91, 0x02,        //   Output (Data,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)

    // LED Padding
    0x95, 0x01,        //   Report Count (1)
    0x75, 0x03,        //   Report Size (3)
    0x91, 0x01,        //   Output (Const,Var,Abs,No Wrap,Linear,Preferred State,No Null Position,Non-volatile)

    // Keycodes
    0x05, 0x07,        //   Usage Page (Kbrd/Keypad)
    0x19, 0x00,        //   Usage Minimum (0x00)
    0x29, 0xDD,        //   Usage Maximum (0xDD) - TODO - double check this
    0x15, 0x00,        //   Logical Minimum (0)
    0x26, 0xFF, 0x00,  //   Logical Maximum (255) - TOOD - double check max and trailing 0x00 byte
    0x95, 0x06,        //   Report Count (6)
    0x75, 0x08,        //   Report Size (8)
    0x81, 0x00,        //   Input (Data,Array,Abs,No Wrap,Linear,Preferred State,No Null Position)

    0xC0,              // End Collection
];

pub const ZKEY_DESCRIPTOR: &[u8] = &[
    0x6, 0x0, 0xff,      // USEGE PAGE Vendor
    0x9, 0x1,             // USAGE Vendor
    0xa1, 0x1,           // Collection (Application)
    0x15, 0x0,          // Logical minimum
    0x26, 0xff, 0x0,   // Logical maximum
    0x75, 0x8,          // Report size 8
    0x95, 0x1,          // Report count 1
    0x09, 0x01,        // USAGE Vendor <---- manually added here
    0x81, 0x0,          // Input
    0xc0,                 // End application collection
];

/// NKRO Keyboard - HID Bitmap
///
/// This is a simplified NKRO descriptor as comparied to kiibohd/controller.
/// It uses 1 extra byte in each packet, but easier to understand and parse.
///
/// NOTES:
/// Supports all keys defined by the spec.
/// 0 represents "no keys pressed" so it is excluded.
/// Supports all keys defined by the spec, except 1-3 which define error events
///  and 0 which is "no keys pressed"
/// See <https://usb.org/sites/default/files/hut1_22.pdf> Chapter 10
///
/// Special bits:
/// 0x00 - Reserved (represents no keys pressed, not useful in a bitmap)
/// 0x01 - ErrorRollOver
/// 0x02 - POSTFail
/// 0x03 - ErrorUndefined
/// 0xA5..0xAF - Reserved
/// 0xDE..0xDF - Reserved
/// 0xE8..0xFFFF - Not specified (Reserved in protocol)
///
/// Compatibility Notes:
///  - Using a second endpoint for a boot mode device helps with compatibility
///  - DO NOT use Padding in the descriptor for bitfields
///    (Mac OSX silently fails... Windows/Linux work correctly)
///  - DO NOT use Report IDs (to split the keyboard report), Windows 8.1 will not update
///    keyboard correctly (modifiers disappear)
///    (all other OSs, including OSX work fine...)
///    (you can use them *iff* you only have 1 per collection)
///  - Mac OSX and Windows 8.1 are extremely picky about padding
#[gen_hid_descriptor(
    (collection = APPLICATION, usage_page = GENERIC_DESKTOP, usage = KEYBOARD) = {
        // LED Report
        (usage_page = LEDS, usage_min = 0x01, usage_max = 0x05) = {
            #[packed_bits 8] #[item_settings data,variable,absolute] leds=output;
        };

        // 1-231 (29 bytes/232 bits)
        (usage_page = KEYBOARD, usage_min = 0x01, usage_max = 0xE7) = {
            #[packed_bits 232] #[item_settings data,variable,absolute] keybitmap=input;
        };
    }
)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct KeyboardNkroReport {
    pub leds: u8,
    pub keybitmap: [u8; 29],
}
