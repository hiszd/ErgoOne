#[allow(unused)]
#[repr(u8)]
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, PartialEq, PartialOrd)]
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
