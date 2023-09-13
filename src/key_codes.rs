use defmt::Format;

#[allow(unused)]
#[repr(u8)]
#[allow(non_camel_case_types)]
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug, Format)]
pub enum KeyCode {
  EEEEEEEE,
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
  Sym_BSla,
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

  Led_Col1,
  Led_Col2,
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

  pub fn from_char(val: char) -> ([Option<KeyCode>; 4], usize) {
    match val {
      'a' => ([Some(KeyCode::Ltr_Azzz), None, None, None], 1),
      'A' => ([Some(KeyCode::Ltr_Azzz), None, None, None], 1),
      'b' => ([Some(KeyCode::Ltr_Bzzz), None, None, None], 1),
      'B' => ([Some(KeyCode::Ltr_Bzzz), None, None, None], 1),
      'c' => ([Some(KeyCode::Ltr_Czzz), None, None, None], 1),
      'C' => ([Some(KeyCode::Ltr_Czzz), None, None, None], 1),
      'd' => ([Some(KeyCode::Ltr_Dzzz), None, None, None], 1),
      'D' => ([Some(KeyCode::Ltr_Dzzz), None, None, None], 1),
      'e' => ([Some(KeyCode::Ltr_Ezzz), None, None, None], 1),
      'E' => ([Some(KeyCode::Ltr_Ezzz), None, None, None], 1),
      'f' => ([Some(KeyCode::Ltr_Fzzz), None, None, None], 1),
      'F' => ([Some(KeyCode::Ltr_Fzzz), None, None, None], 1),
      'g' => ([Some(KeyCode::Ltr_Gzzz), None, None, None], 1),
      'G' => ([Some(KeyCode::Ltr_Gzzz), None, None, None], 1),
      'h' => ([Some(KeyCode::Ltr_Hzzz), None, None, None], 1),
      'H' => ([Some(KeyCode::Ltr_Hzzz), None, None, None], 1),
      'i' => ([Some(KeyCode::Ltr_Izzz), None, None, None], 1),
      'I' => ([Some(KeyCode::Ltr_Izzz), None, None, None], 1),
      'j' => ([Some(KeyCode::Ltr_Jzzz), None, None, None], 1),
      'J' => ([Some(KeyCode::Ltr_Jzzz), None, None, None], 1),
      'k' => ([Some(KeyCode::Ltr_Kzzz), None, None, None], 1),
      'K' => ([Some(KeyCode::Ltr_Kzzz), None, None, None], 1),
      'l' => ([Some(KeyCode::Ltr_Lzzz), None, None, None], 1),
      'L' => ([Some(KeyCode::Ltr_Lzzz), None, None, None], 1),
      'm' => ([Some(KeyCode::Ltr_Mzzz), None, None, None], 1),
      'M' => ([Some(KeyCode::Ltr_Mzzz), None, None, None], 1),
      'n' => ([Some(KeyCode::Ltr_Nzzz), None, None, None], 1),
      'N' => ([Some(KeyCode::Ltr_Nzzz), None, None, None], 1),
      'o' => ([Some(KeyCode::Ltr_Ozzz), None, None, None], 1),
      'O' => ([Some(KeyCode::Ltr_Ozzz), None, None, None], 1),
      'p' => ([Some(KeyCode::Ltr_Pzzz), None, None, None], 1),
      'P' => ([Some(KeyCode::Ltr_Pzzz), None, None, None], 1),
      'q' => ([Some(KeyCode::Ltr_Qzzz), None, None, None], 1),
      'Q' => ([Some(KeyCode::Ltr_Qzzz), None, None, None], 1),
      'r' => ([Some(KeyCode::Ltr_Rzzz), None, None, None], 1),
      'R' => ([Some(KeyCode::Ltr_Rzzz), None, None, None], 1),
      's' => ([Some(KeyCode::Ltr_Szzz), None, None, None], 1),
      'S' => ([Some(KeyCode::Ltr_Szzz), None, None, None], 1),
      't' => ([Some(KeyCode::Ltr_Tzzz), None, None, None], 1),
      'T' => ([Some(KeyCode::Ltr_Tzzz), None, None, None], 1),
      'u' => ([Some(KeyCode::Ltr_Uzzz), None, None, None], 1),
      'U' => ([Some(KeyCode::Ltr_Uzzz), None, None, None], 1),
      'v' => ([Some(KeyCode::Ltr_Vzzz), None, None, None], 1),
      'V' => ([Some(KeyCode::Ltr_Vzzz), None, None, None], 1),
      'w' => ([Some(KeyCode::Ltr_Wzzz), None, None, None], 1),
      'W' => ([Some(KeyCode::Ltr_Wzzz), None, None, None], 1),
      'x' => ([Some(KeyCode::Ltr_Xzzz), None, None, None], 1),
      'X' => ([Some(KeyCode::Ltr_Xzzz), None, None, None], 1),
      'y' => ([Some(KeyCode::Ltr_Yzzz), None, None, None], 1),
      'Y' => ([Some(KeyCode::Ltr_Yzzz), None, None, None], 1),
      'z' => ([Some(KeyCode::Ltr_Zzzz), None, None, None], 1),
      'Z' => ([Some(KeyCode::Ltr_Zzzz), None, None, None], 1),
      '1' => ([Some(KeyCode::Num_1zzz), None, None, None], 1),
      '2' => ([Some(KeyCode::Num_2zzz), None, None, None], 1),
      '3' => ([Some(KeyCode::Num_3zzz), None, None, None], 1),
      '4' => ([Some(KeyCode::Num_4zzz), None, None, None], 1),
      '5' => ([Some(KeyCode::Num_5zzz), None, None, None], 1),
      '6' => ([Some(KeyCode::Num_6zzz), None, None, None], 1),
      '7' => ([Some(KeyCode::Num_7zzz), None, None, None], 1),
      '8' => ([Some(KeyCode::Num_8zzz), None, None, None], 1),
      '9' => ([Some(KeyCode::Num_9zzz), None, None, None], 1),
      '0' => ([Some(KeyCode::Num_0zzz), None, None, None], 1),
      _ => ([None;4], 0),
    }
  }
}

impl From<&KeyCode> for u8 {
  fn from(val: &KeyCode) -> Self {
    match val {
      KeyCode::Ltr_Azzz => 0x04,
      KeyCode::Ltr_Bzzz => 0x05,
      KeyCode::Ltr_Czzz => 0x06,
      KeyCode::Ltr_Dzzz => 0x07,
      KeyCode::Ltr_Ezzz => 0x08,
      KeyCode::Ltr_Fzzz => 0x09,
      KeyCode::Ltr_Gzzz => 0x0a,
      KeyCode::Ltr_Hzzz => 0x0b,
      KeyCode::Ltr_Izzz => 0x0c,
      KeyCode::Ltr_Jzzz => 0x0d,
      KeyCode::Ltr_Kzzz => 0x0e,
      KeyCode::Ltr_Lzzz => 0x0f,
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
      KeyCode::Ltr_Wzzz => 0x1a,
      KeyCode::Ltr_Xzzz => 0x1b,
      KeyCode::Ltr_Yzzz => 0x1c,
      KeyCode::Ltr_Zzzz => 0x1d,
      KeyCode::Num_1zzz => 0x1e,
      KeyCode::Num_2zzz => 0x1f,
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
      KeyCode::Fun_Bksp => 0x2a,
      KeyCode::Fun_Tabz => 0x2b,
      KeyCode::Fun_Spcz => 0x2c,
      KeyCode::Sym_Minz => 0x2d,
      KeyCode::Sym_Equz => 0x2e,
      KeyCode::Sym_LBrk => 0x2f,
      KeyCode::Sym_RBrk => 0x30,
      KeyCode::Sym_BSla => 0x31,
      KeyCode::Sym_Scln => 0x33,
      KeyCode::Sym_SQut => 0x34,
      KeyCode::Sym_Tild => 0x35,
      KeyCode::Sym_Coma => 0x36,
      KeyCode::Sym_Perd => 0x37,
      KeyCode::Sym_FSla => 0x38,
      KeyCode::Fun_Caps => 0x39,
      KeyCode::Fun_F1zz => 0x3a,
      KeyCode::Fun_F2zz => 0x3b,
      KeyCode::Fun_F3zz => 0x3c,
      KeyCode::Fun_F4zz => 0x3d,
      KeyCode::Fun_F5zz => 0x3e,
      KeyCode::Fun_F6zz => 0x3f,
      KeyCode::Fun_F7zz => 0x40,
      KeyCode::Fun_F8zz => 0x41,
      KeyCode::Fun_F9zz => 0x42,
      KeyCode::Fun_F10z => 0x43,
      KeyCode::Fun_F11z => 0x44,
      KeyCode::Fun_F12z => 0x45,
      KeyCode::Arw_Rght => 0x4f,
      KeyCode::Arw_Left => 0x50,
      KeyCode::Arw_Down => 0x51,
      KeyCode::Arw_Upzz => 0x52,
      KeyCode::Fun_Home => 0x4a,
      KeyCode::Fun_PgUp => 0x4b,
      KeyCode::Fun_Delz => 0x4c,
      KeyCode::Fun_Endz => 0x4d,
      KeyCode::Fun_PgDn => 0x4e,
      KeyCode::Vol_Mute => 0x7f,
      KeyCode::Vol_Upzz => 0x80,
      KeyCode::Vol_Down => 0x81,
      KeyCode::Sym_LPar => 0xb6,
      KeyCode::Sym_RPar => 0xb7,
      KeyCode::Mod_L01z => 0xf0,
      KeyCode::Mod_LSft => 0xe1,
      KeyCode::Mod_LCtl => 0xe0,
      KeyCode::Mod_LAlt => 0xe2,
      KeyCode::Mod_LCmd => 0xe3,
      KeyCode::Mod_RCmd => 0xe7,
      KeyCode::Mod_RAlt => 0xe6,
      KeyCode::Mod_RCtl => 0xe4,
      KeyCode::Mod_RSft => 0xe5,
      KeyCode::________ => 0x00,
      _ => 0x00,
    }
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
      KeyCode::Ltr_Gzzz => 0x0a,
      KeyCode::Ltr_Hzzz => 0x0b,
      KeyCode::Ltr_Izzz => 0x0c,
      KeyCode::Ltr_Jzzz => 0x0d,
      KeyCode::Ltr_Kzzz => 0x0e,
      KeyCode::Ltr_Lzzz => 0x0f,
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
      KeyCode::Ltr_Wzzz => 0x1a,
      KeyCode::Ltr_Xzzz => 0x1b,
      KeyCode::Ltr_Yzzz => 0x1c,
      KeyCode::Ltr_Zzzz => 0x1d,
      KeyCode::Num_1zzz => 0x1e,
      KeyCode::Num_2zzz => 0x1f,
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
      KeyCode::Fun_Bksp => 0x2a,
      KeyCode::Fun_Tabz => 0x2b,
      KeyCode::Fun_Spcz => 0x2c,
      KeyCode::Sym_Minz => 0x2d,
      KeyCode::Sym_Equz => 0x2e,
      KeyCode::Sym_LBrk => 0x2f,
      KeyCode::Sym_RBrk => 0x30,
      KeyCode::Sym_BSla => 0x31,
      KeyCode::Sym_Scln => 0x33,
      KeyCode::Sym_SQut => 0x34,
      KeyCode::Sym_Tild => 0x35,
      KeyCode::Sym_Coma => 0x36,
      KeyCode::Sym_Perd => 0x37,
      KeyCode::Sym_FSla => 0x38,
      KeyCode::Fun_Caps => 0x39,
      KeyCode::Fun_F1zz => 0x3a,
      KeyCode::Fun_F2zz => 0x3b,
      KeyCode::Fun_F3zz => 0x3c,
      KeyCode::Fun_F4zz => 0x3d,
      KeyCode::Fun_F5zz => 0x3e,
      KeyCode::Fun_F6zz => 0x3f,
      KeyCode::Fun_F7zz => 0x40,
      KeyCode::Fun_F8zz => 0x41,
      KeyCode::Fun_F9zz => 0x42,
      KeyCode::Fun_F10z => 0x43,
      KeyCode::Fun_F11z => 0x44,
      KeyCode::Fun_F12z => 0x45,
      KeyCode::Arw_Rght => 0x4f,
      KeyCode::Arw_Left => 0x50,
      KeyCode::Arw_Down => 0x51,
      KeyCode::Arw_Upzz => 0x52,
      KeyCode::Fun_Home => 0x4a,
      KeyCode::Fun_PgUp => 0x4b,
      KeyCode::Fun_Delz => 0x4c,
      KeyCode::Fun_Endz => 0x4d,
      KeyCode::Fun_PgDn => 0x4e,
      KeyCode::Vol_Mute => 0x7f,
      KeyCode::Vol_Upzz => 0x80,
      KeyCode::Vol_Down => 0x81,
      KeyCode::Sym_LPar => 0xb6,
      KeyCode::Sym_RPar => 0xb7,
      KeyCode::Mod_L01z => 0xf0,
      KeyCode::Mod_LSft => 0xe1,
      KeyCode::Mod_LCtl => 0xe0,
      KeyCode::Mod_LAlt => 0xe2,
      KeyCode::Mod_LCmd => 0xe3,
      KeyCode::Mod_RCmd => 0xe7,
      KeyCode::Mod_RAlt => 0xe6,
      KeyCode::Mod_RCtl => 0xe4,
      KeyCode::Mod_RSft => 0xe5,
      KeyCode::________ => 0x00,
      _ => 0x00,
    }
  }
}

impl From<KeyCode> for &str {
  fn from(val: KeyCode) -> Self {
    match val {
      KeyCode::Ltr_Azzz => "Ltr_Azzz",
      KeyCode::Ltr_Bzzz => "Ltr_Bzzz",
      KeyCode::Ltr_Czzz => "Ltr_Czzz",
      KeyCode::Ltr_Dzzz => "Ltr_Dzzz",
      KeyCode::Ltr_Ezzz => "Ltr_Ezzz",
      KeyCode::Ltr_Fzzz => "Ltr_Fzzz",
      KeyCode::Ltr_Gzzz => "Ltr_Gzzz",
      KeyCode::Ltr_Hzzz => "Ltr_Hzzz",
      KeyCode::Ltr_Izzz => "Ltr_Izzz",
      KeyCode::Ltr_Jzzz => "Ltr_Jzzz",
      KeyCode::Ltr_Kzzz => "Ltr_Kzzz",
      KeyCode::Ltr_Lzzz => "Ltr_Lzzz",
      KeyCode::Ltr_Mzzz => "Ltr_Mzzz",
      KeyCode::Ltr_Nzzz => "Ltr_Nzzz",
      KeyCode::Ltr_Ozzz => "Ltr_Ozzz",
      KeyCode::Ltr_Pzzz => "Ltr_Pzzz",
      KeyCode::Ltr_Qzzz => "Ltr_Qzzz",
      KeyCode::Ltr_Rzzz => "Ltr_Rzzz",
      KeyCode::Ltr_Szzz => "Ltr_Szzz",
      KeyCode::Ltr_Tzzz => "Ltr_Tzzz",
      KeyCode::Ltr_Uzzz => "Ltr_Uzzz",
      KeyCode::Ltr_Vzzz => "Ltr_Vzzz",
      KeyCode::Ltr_Wzzz => "Ltr_Wzzz",
      KeyCode::Ltr_Xzzz => "Ltr_Xzzz",
      KeyCode::Ltr_Yzzz => "Ltr_Yzzz",
      KeyCode::Ltr_Zzzz => "Ltr_Zzzz",
      KeyCode::Num_1zzz => "Num_1zzz",
      KeyCode::Num_2zzz => "Num_2zzz",
      KeyCode::Num_3zzz => "Num_3zzz",
      KeyCode::Num_4zzz => "Num_4zzz",
      KeyCode::Num_5zzz => "Num_5zzz",
      KeyCode::Num_6zzz => "Num_6zzz",
      KeyCode::Num_7zzz => "Num_7zzz",
      KeyCode::Num_8zzz => "Num_8zzz",
      KeyCode::Num_9zzz => "Num_9zzz",
      KeyCode::Num_0zzz => "Num_0zzz",
      KeyCode::Fun_Entz => "Fun_Entz",
      KeyCode::Fun_Escz => "Fun_Escz",
      KeyCode::Fun_Bksp => "Fun_Bksp",
      KeyCode::Fun_Tabz => "Fun_Tabz",
      KeyCode::Fun_Spcz => "Fun_Spcz",
      KeyCode::Sym_Minz => "Sym_Minz",
      KeyCode::Sym_Equz => "Sym_Equz",
      KeyCode::Sym_LBrk => "Sym_LBrk",
      KeyCode::Sym_RBrk => "Sym_RBrk",
      KeyCode::Sym_BSla => "Sym_BSla",
      KeyCode::Sym_Scln => "Sym_Scln",
      KeyCode::Sym_SQut => "Sym_SQut",
      KeyCode::Sym_Tild => "Sym_Tild",
      KeyCode::Sym_Coma => "Sym_Coma",
      KeyCode::Sym_Perd => "Sym_Perd",
      KeyCode::Sym_FSla => "Sym_FSla",
      KeyCode::Fun_Caps => "Fun_Caps",
      KeyCode::Fun_F1zz => "Fun_F1zz",
      KeyCode::Fun_F2zz => "Fun_F2zz",
      KeyCode::Fun_F3zz => "Fun_F3zz",
      KeyCode::Fun_F4zz => "Fun_F4zz",
      KeyCode::Fun_F5zz => "Fun_F5zz",
      KeyCode::Fun_F6zz => "Fun_F6zz",
      KeyCode::Fun_F7zz => "Fun_F7zz",
      KeyCode::Fun_F8zz => "Fun_F8zz",
      KeyCode::Fun_F9zz => "Fun_F9zz",
      KeyCode::Fun_F10z => "Fun_F10z",
      KeyCode::Fun_F11z => "Fun_F11z",
      KeyCode::Fun_F12z => "Fun_F12z",
      KeyCode::Arw_Rght => "Arw_Rght",
      KeyCode::Arw_Left => "Arw_Left",
      KeyCode::Arw_Down => "Arw_Down",
      KeyCode::Arw_Upzz => "Arw_Upzz",
      KeyCode::Fun_Home => "Fun_Home",
      KeyCode::Fun_PgUp => "Fun_PgUp",
      KeyCode::Fun_Delz => "Fun_Delz",
      KeyCode::Fun_Endz => "Fun_Endz",
      KeyCode::Fun_PgDn => "Fun_PgDn",
      KeyCode::Vol_Mute => "Vol_Mute",
      KeyCode::Vol_Upzz => "Vol_Upzz",
      KeyCode::Vol_Down => "Vol_Down",
      KeyCode::Sym_LPar => "Sym_LPar",
      KeyCode::Sym_RPar => "Sym_RPar",
      KeyCode::Mod_L01z => "Mod_L01z",
      KeyCode::Mod_LSft => "Mod_LSft",
      KeyCode::Mod_LCtl => "Mod_LCtl",
      KeyCode::Mod_LAlt => "Mod_LAlt",
      KeyCode::Mod_LCmd => "Mod_LCmd",
      KeyCode::Mod_RCmd => "Mod_RCmd",
      KeyCode::Mod_RAlt => "Mod_RAlt",
      KeyCode::Mod_RCtl => "Mod_RCtl",
      KeyCode::Mod_RSft => "Mod_RSft",
      KeyCode::Led_Col1 => "Led_Col1",
      KeyCode::Led_Col2 => "Led_Col2",
      KeyCode::________ => "________",
      KeyCode::EEEEEEEE => "EEEEEEEE",
    }
  }
}

impl From<&str> for KeyCode {
  fn from(val: &str) -> Self {
    match val {
      "Ltr_Azzz" => KeyCode::Ltr_Azzz,
      "Ltr_Bzzz" => KeyCode::Ltr_Bzzz,
      "Ltr_Czzz" => KeyCode::Ltr_Czzz,
      "Ltr_Dzzz" => KeyCode::Ltr_Dzzz,
      "Ltr_Ezzz" => KeyCode::Ltr_Ezzz,
      "Ltr_Fzzz" => KeyCode::Ltr_Fzzz,
      "Ltr_Gzzz" => KeyCode::Ltr_Gzzz,
      "Ltr_Hzzz" => KeyCode::Ltr_Hzzz,
      "Ltr_Izzz" => KeyCode::Ltr_Izzz,
      "Ltr_Jzzz" => KeyCode::Ltr_Jzzz,
      "Ltr_Kzzz" => KeyCode::Ltr_Kzzz,
      "Ltr_Lzzz" => KeyCode::Ltr_Lzzz,
      "Ltr_Mzzz" => KeyCode::Ltr_Mzzz,
      "Ltr_Nzzz" => KeyCode::Ltr_Nzzz,
      "Ltr_Ozzz" => KeyCode::Ltr_Ozzz,
      "Ltr_Pzzz" => KeyCode::Ltr_Pzzz,
      "Ltr_Qzzz" => KeyCode::Ltr_Qzzz,
      "Ltr_Rzzz" => KeyCode::Ltr_Rzzz,
      "Ltr_Szzz" => KeyCode::Ltr_Szzz,
      "Ltr_Tzzz" => KeyCode::Ltr_Tzzz,
      "Ltr_Uzzz" => KeyCode::Ltr_Uzzz,
      "Ltr_Vzzz" => KeyCode::Ltr_Vzzz,
      "Ltr_Wzzz" => KeyCode::Ltr_Wzzz,
      "Ltr_Xzzz" => KeyCode::Ltr_Xzzz,
      "Ltr_Yzzz" => KeyCode::Ltr_Yzzz,
      "Ltr_Zzzz" => KeyCode::Ltr_Zzzz,
      "Num_1zzz" => KeyCode::Num_1zzz,
      "Num_2zzz" => KeyCode::Num_2zzz,
      "Num_3zzz" => KeyCode::Num_3zzz,
      "Num_4zzz" => KeyCode::Num_4zzz,
      "Num_5zzz" => KeyCode::Num_5zzz,
      "Num_6zzz" => KeyCode::Num_6zzz,
      "Num_7zzz" => KeyCode::Num_7zzz,
      "Num_8zzz" => KeyCode::Num_8zzz,
      "Num_9zzz" => KeyCode::Num_9zzz,
      "Num_0zzz" => KeyCode::Num_0zzz,
      "Fun_Entz" => KeyCode::Fun_Entz,
      "Fun_Escz" => KeyCode::Fun_Escz,
      "Fun_Bksp" => KeyCode::Fun_Bksp,
      "Fun_Tabz" => KeyCode::Fun_Tabz,
      "Fun_Spcz" => KeyCode::Fun_Spcz,
      "Sym_Minz" => KeyCode::Sym_Minz,
      "Sym_Equz" => KeyCode::Sym_Equz,
      "Sym_LBrk" => KeyCode::Sym_LBrk,
      "Sym_RBrk" => KeyCode::Sym_RBrk,
      "Sym_BSla" => KeyCode::Sym_BSla,
      "Sym_Scln" => KeyCode::Sym_Scln,
      "Sym_SQut" => KeyCode::Sym_SQut,
      "Sym_Tild" => KeyCode::Sym_Tild,
      "Sym_Coma" => KeyCode::Sym_Coma,
      "Sym_Perd" => KeyCode::Sym_Perd,
      "Sym_FSla" => KeyCode::Sym_FSla,
      "Fun_Caps" => KeyCode::Fun_Caps,
      "Fun_F1zz" => KeyCode::Fun_F1zz,
      "Fun_F2zz" => KeyCode::Fun_F2zz,
      "Fun_F3zz" => KeyCode::Fun_F3zz,
      "Fun_F4zz" => KeyCode::Fun_F4zz,
      "Fun_F5zz" => KeyCode::Fun_F5zz,
      "Fun_F6zz" => KeyCode::Fun_F6zz,
      "Fun_F7zz" => KeyCode::Fun_F7zz,
      "Fun_F8zz" => KeyCode::Fun_F8zz,
      "Fun_F9zz" => KeyCode::Fun_F9zz,
      "Fun_F10z" => KeyCode::Fun_F10z,
      "Fun_F11z" => KeyCode::Fun_F11z,
      "Fun_F12z" => KeyCode::Fun_F12z,
      "Arw_Rght" => KeyCode::Arw_Rght,
      "Arw_Left" => KeyCode::Arw_Left,
      "Arw_Down" => KeyCode::Arw_Down,
      "Arw_Upzz" => KeyCode::Arw_Upzz,
      "Fun_Home" => KeyCode::Fun_Home,
      "Fun_PgUp" => KeyCode::Fun_PgUp,
      "Fun_Delz" => KeyCode::Fun_Delz,
      "Fun_Endz" => KeyCode::Fun_Endz,
      "Fun_PgDn" => KeyCode::Fun_PgDn,
      "Vol_Mute" => KeyCode::Vol_Mute,
      "Vol_Upzz" => KeyCode::Vol_Upzz,
      "Vol_Down" => KeyCode::Vol_Down,
      "Sym_LPar" => KeyCode::Sym_LPar,
      "Sym_RPar" => KeyCode::Sym_RPar,
      "Mod_L01z" => KeyCode::Mod_L01z,
      "Mod_LSft" => KeyCode::Mod_LSft,
      "Mod_LCtl" => KeyCode::Mod_LCtl,
      "Mod_LAlt" => KeyCode::Mod_LAlt,
      "Mod_LCmd" => KeyCode::Mod_LCmd,
      "Mod_RCmd" => KeyCode::Mod_RCmd,
      "Mod_RAlt" => KeyCode::Mod_RAlt,
      "Mod_RCtl" => KeyCode::Mod_RCtl,
      "Mod_RSft" => KeyCode::Mod_RSft,
      "Led_Col1" => KeyCode::Led_Col1,
      "Led_Col2" => KeyCode::Led_Col2,
      "________" => KeyCode::________,
      "EEEEEEEE" => KeyCode::EEEEEEEE,
      _ => KeyCode::EEEEEEEE,
    }
  }
}
