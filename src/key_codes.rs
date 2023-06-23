use defmt::Format;

#[allow(unused)]
#[repr(u8)]
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Format)]
pub enum KeyCode {
    /// Empty
    ________,
    Ltr_Azzz,
    Ltr_Bzzz,
    Ltr_Czzz,
    Ltr_Dzzz,
    Ltr_Ezzz,
    Ltr_Fzzz,
    Ltr_Gzzz,
    Ltr_Hzzz,
    Ltr_Izzz,
    Ltr_Jzzz,
    Ltr_Kzzz,
    Ltr_Lzzz,
    Ltr_Mzzz,
    Ltr_Nzzz,
    Ltr_Ozzz,
    Ltr_Pzzz,
    Ltr_Qzzz,
    Ltr_Rzzz,
    Ltr_Szzz,
    Ltr_Tzzz,
    Ltr_Uzzz,
    Ltr_Vzzz,
    Ltr_Wzzz,
    Ltr_Xzzz,
    Ltr_Yzzz,
    Ltr_Zzzz,
    Num_1zzz,
    Num_2zzz,
    Num_3zzz,
    Num_4zzz,
    Num_5zzz,
    Num_6zzz,
    Num_7zzz,
    Num_8zzz,
    Num_9zzz,
    Num_0zzz,
    /// Enter
    Fun_Entz,
    /// Escape
    Fun_Escz,
    /// Backspace
    Fun_Bksp,
    /// Tab
    Fun_Tabz,
    /// Space
    Fun_Spcz,
    /// Minus
    Sym_Minz,
    /// Equals
    Sym_Equz,
    /// Left Square Bracket
    Sym_LBrk,
    /// Right Square Bracket
    Sym_RBrk,
    /// Backslash
    Sym_Bszz,
    /// Semicolon
    Sym_Scln,
    /// Single Quote
    Sym_SQut,
    /// Tilde
    Sym_Tild,
    /// Comma
    Sym_Coma,
    /// Period
    Sym_Perd,
    /// Forward Slash
    Sym_FSla,
    /// Capslock
    Fun_Caps,
    Fun_F1zz,
    Fun_F2zz,
    Fun_F3zz,
    Fun_F4zz,
    Fun_F5zz,
    Fun_F6zz,
    Fun_F7zz,
    Fun_F8zz,
    Fun_F9zz,
    Fun_F10z,
    Fun_F11z,
    Fun_F12z,

    /// Right
    Arw_Rght,
    /// Left
    Arw_Left,
    /// Down
    Arw_Down,
    /// Up
    Arw_Upzz,

    /// Home
    Fun_Home,
    /// PageUp
    Fun_PgUp,
    /// Delete
    Fun_Delz,
    /// End
    Fun_Endz,
    /// PageDown
    Fun_PgDn,

    // Media Keys
    /// Volume Mute
    Vol_Mute,
    /// Volume Up
    Vol_Upzz,
    /// Volume Down
    Vol_Down,

    // Keypad keys
    /// Left Paren
    Sym_LPar,
    /// Right Paren
    Sym_RPar,

    // Modifier keys
    Mod_L01z,
    /// Left Shift
    Mod_LSft,
    /// Left Control
    Mod_LCtl,
    /// Left Alt
    Mod_LAlt,
    /// Left Command
    Mod_LCmd,
    /// Right Command
    Mod_RCmd,
    /// Right Alt
    Mod_RAlt,
    /// Right Ctrl
    Mod_RCtl,
    /// Right Shift
    Mod_RSft,
}

impl KeyCode {
    pub fn modifier_bitmask(&self) -> Option<u8> {
        match *self {
            KeyCode::Mod_LCtl => Some(1 << 0),
            KeyCode::Mod_LSft => Some(1 << 1),
            KeyCode::Mod_LAlt => Some(1 << 2),
            KeyCode::Mod_LCmd => Some(1 << 3),
            KeyCode::Mod_RCtl => Some(1 << 4),
            KeyCode::Mod_RSft => Some(1 << 5),
            KeyCode::Mod_RAlt => Some(1 << 6),
            KeyCode::Mod_RCmd => Some(1 << 7),
            _ => None,
        }
    }

    #[allow(dead_code)]
    pub fn is_modifier(&self) -> bool {
        *self == KeyCode::Mod_L01z || self.modifier_bitmask().is_some()
    }
}

impl From<KeyCode> for u8 {
    fn from(val: KeyCode) -> Self {
        match val {
            KeyCode::Ltr_Azzz => 0x04,
            KeyCode::Ltr_Bzzz => 0x05,
            KeyCode::Ltr_Czzz => 0x06,
            KeyCode::Ltr_Dzzz => 0x07,
            KeyCode::Ltr_Ezzz => 0x08,
            KeyCode::Ltr_Fzzz => 0x09,
            KeyCode::Ltr_Gzzz => 0x0A,
            KeyCode::Ltr_Hzzz => 0x0B,
            KeyCode::Ltr_Izzz => 0x0C,
            KeyCode::Ltr_Jzzz => 0x0D,
            KeyCode::Ltr_Kzzz => 0x0E,
            KeyCode::Ltr_Lzzz => 0x0F,
            KeyCode::Ltr_Mzzz => 0x10,
            KeyCode::Ltr_Nzzz => 0x11,
            KeyCode::Ltr_Ozzz => 0x12,
            KeyCode::Ltr_Pzzz => 0x13,
            KeyCode::Ltr_Qzzz => 0x14,
            KeyCode::Ltr_Rzzz => 0x15,
            KeyCode::Ltr_Szzz => 0x16,
            KeyCode::Ltr_Tzzz => 0x17,
            KeyCode::Ltr_Uzzz => 0x18,
            KeyCode::Ltr_Vzzz => 0x19,
            KeyCode::Ltr_Wzzz => 0x1A,
            KeyCode::Ltr_Xzzz => 0x1B,
            KeyCode::Ltr_Yzzz => 0x1C,
            KeyCode::Ltr_Zzzz => 0x1D,
            KeyCode::Num_1zzz => 0x1E,
            KeyCode::Num_2zzz => 0x1F,
            KeyCode::Num_3zzz => 0x20,
            KeyCode::Num_4zzz => 0x21,
            KeyCode::Num_5zzz => 0x22,
            KeyCode::Num_6zzz => 0x23,
            KeyCode::Num_7zzz => 0x24,
            KeyCode::Num_8zzz => 0x25,
            KeyCode::Num_9zzz => 0x26,
            KeyCode::Num_0zzz => 0x27,
            KeyCode::Fun_Entz => 0x28,
            KeyCode::Fun_Escz => 0x29,
            KeyCode::Fun_Bksp => 0x2A,
            KeyCode::Fun_Tabz => 0x2B,
            KeyCode::Fun_Spcz => 0x2C,
            KeyCode::Sym_Minz => 0x2D,
            KeyCode::Sym_Equz => 0x2E,
            KeyCode::Sym_LBrk => 0x2F,
            KeyCode::Sym_RBrk => 0x30,
            KeyCode::Sym_Bszz => 0x31,
            KeyCode::Sym_Scln => 0x33,
            KeyCode::Sym_SQut => 0x34,
            KeyCode::Sym_Tild => 0x35,
            KeyCode::Sym_Coma => 0x36,
            KeyCode::Sym_Perd => 0x37,
            KeyCode::Sym_FSla => 0x38,
            KeyCode::Fun_Caps => 0x39,
            KeyCode::Fun_F1zz => 0x3A,
            KeyCode::Fun_F2zz => 0x3B,
            KeyCode::Fun_F3zz => 0x3C,
            KeyCode::Fun_F4zz => 0x3D,
            KeyCode::Fun_F5zz => 0x3E,
            KeyCode::Fun_F6zz => 0x3F,
            KeyCode::Fun_F7zz => 0x40,
            KeyCode::Fun_F8zz => 0x41,
            KeyCode::Fun_F9zz => 0x42,
            KeyCode::Fun_F10z => 0x43,
            KeyCode::Fun_F11z => 0x44,
            KeyCode::Fun_F12z => 0x45,
            KeyCode::Arw_Rght => 0x4F,
            KeyCode::Arw_Left => 0x50,
            KeyCode::Arw_Down => 0x51,
            KeyCode::Arw_Upzz => 0x52,
            KeyCode::Fun_Home => 0x4A,
            KeyCode::Fun_PgUp => 0x4B,
            KeyCode::Fun_Delz => 0x4C,
            KeyCode::Fun_Endz => 0x4D,
            KeyCode::Fun_PgDn => 0x4E,
            KeyCode::Vol_Mute => 0x7F,
            KeyCode::Vol_Upzz => 0x80,
            KeyCode::Vol_Down => 0x81,
            KeyCode::Sym_LPar => 0xB6,
            KeyCode::Sym_RPar => 0xB7,
            KeyCode::Mod_L01z => 0xF0,
            KeyCode::Mod_LSft => 0xF1,
            KeyCode::Mod_LCtl => 0xF2,
            KeyCode::Mod_LAlt => 0xF3,
            KeyCode::Mod_LCmd => 0xF4,
            KeyCode::Mod_RCmd => 0xF5,
            KeyCode::Mod_RAlt => 0xF6,
            KeyCode::Mod_RCtl => 0xF7,
            KeyCode::Mod_RSft => 0xF8,
            KeyCode::________ => 0x00,
            _ => 0x00,
        }
    }
}
