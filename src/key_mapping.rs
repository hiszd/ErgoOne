use crate::key::Key;
use crate::key_codes::KeyCode;
use crate::mods::layer_hold::LayerHold;
use crate::mods::mod_combo::ModCombo;
use crate::mods::mod_tapcom::TapCom;
use crate::mods::rgb_key::RGBKey;
use crate::mods::sendstring::SendString;
use crate::mods::transparent::Transparent;
use crate::{key::Default, keyscanning::KeyMatrix, mods::mod_tap::ModTap};

// TODO: find a better way to nest functionality
#[rustfmt::skip]
pub const ERGOONE_RSTLNE: [&str; 80] = [
"dft,Sym_Tild",                  "dft,Num_1zzz","dft,Num_2zzz","dft,Num_3zzz","dft,Num_4zzz","dft,Num_5zzz","rgk,0_255_0",          "dft,EEEEEEEE","dft,EEEEEEEE","dft,EEEEEEEE","dft,Num_6zzz","dft,Num_7zzz","dft,Num_8zzz","dft,Num_9zzz","dft,Num_0zzz","dft,Sym_Equz",
"dft,Fun_Tabz",                  "dft,Ltr_Qzzz","dft,Ltr_Wzzz","dft,Ltr_Dzzz","dft,Ltr_Fzzz","dft,Ltr_Zzzz","rgk,255_0_0",          "dft,EEEEEEEE","dft,EEEEEEEE","dft,EEEEEEEE","dft,Sym_Scln","dft,Ltr_Uzzz","dft,Ltr_Kzzz","dft,Ltr_Yzzz","dft,Ltr_Pzzz","dft,Sym_BSla",
"mdt,Fun_Escz,Mod_LCtl",         "dft,Ltr_Azzz","dft,Ltr_Szzz","dft,Ltr_Ezzz","dft,Ltr_Rzzz","dft,Ltr_Tzzz","dft,Sym_Minz",         "dft,Fun_Spcz","dft,Fun_Entz","dft,Sym_Equz","dft,Ltr_Hzzz","dft,Ltr_Nzzz","dft,Ltr_Izzz","dft,Ltr_Ozzz","dft,Ltr_Lzzz","dft,Sym_SQut",
"tpc,Mod_LSft,Mod_LSft,Num_9zzz","dft,Ltr_Gzzz","dft,Ltr_Xzzz","dft,Ltr_Czzz","dft,Ltr_Vzzz","dft,Sym_FSla","mdc,Mod_LSft,Sym_Minz","dft,Fun_Home","dft,Fun_PgDn","dft,Fun_Bksp","dft,Ltr_Bzzz","dft,Ltr_Jzzz","dft,Ltr_Mzzz","dft,Sym_Coma","dft,Sym_Perd","tpc,Mod_RSft,Mod_RSft,Num_0zzz",
"dft,Mod_LCtl",                  "dft,Mod_LAlt","dft,Mod_LCmd","dft,Fun_Spcz","dft,Sym_LBrk","dft,Mod_LCmd","dft,EEEEEEEE",         "dft,Fun_Endz","dft,Fun_PgUp","dft,EEEEEEEE","lyh,1,0",     "dft,Sym_RBrk","dft,Arw_Left","dft,Arw_Down","dft,Arw_Upzz","dft,Arw_Rght",
];

#[allow(dead_code)]
#[rustfmt::skip]
pub const ERGOONE_1: [&str; 80] = [
"transparent",                   "dft,Fun_F1zz","dft,Fun_F2zz","dft,Fun_F3zz","dft,Fun_F4zz","dft,Fun_F5zz","transparent",          "transparent", "transparent", "transparent", "dft,Fun_F6zz","dft,Fun_F7zz","dft,Fun_F8zz","dft,Fun_F9zz","dft,Fun_F10z","dft,Fun_F11z",
"transparent",                   "transparent", "dft,Arw_Upzz","transparent", "transparent", "transparent", "transparent",          "transparent", "transparent", "transparent", "transparent", "dft,Num_7zzz","dft,Num_8zzz","dft,Num_9zzz","transparent", "dft,Fun_F12z",
"transparent",                   "dft,Arw_Left","dft,Arw_Down","dft,Arw_Rght","transparent", "transparent", "transparent",          "transparent", "transparent", "transparent", "transparent", "dft,Num_4zzz","dft,Num_5zzz","dft,Num_6zzz","transparent", "transparent", 
"transparent",                   "transparent", "transparent", "transparent", "transparent", "transparent", "transparent",          "transparent", "transparent", "dft,Fun_Delz","transparent", "dft,Num_1zzz","dft,Num_2zzz","dft,Num_3zzz","transparent", "transparent", 
"transparent",                   "transparent", "transparent", "transparent", "transparent", "sst,@zion",    "transparent",         "transparent", "transparent", "transparent", "transparent", "dft,Num_0zzz","transparent", "transparent", "transparent", "transparent", 
];

#[allow(dead_code)]
#[rustfmt::skip]
pub const ERGOONE_QWERTY: [&str; 80] = [
"dft,Fun_Escz",                  "dft,Num_1zzz","dft,Num_2zzz","dft,Num_3zzz","dft,Num_4zzz","dft,Num_5zzz","rgk,0_255_0",          "dft,EEEEEEEE","dft,EEEEEEEE","dft,EEEEEEEE","dft,Num_6zzz","dft,Num_7zzz","dft,Num_8zzz","dft,Num_9zzz","dft,Num_0zzz","dft,Sym_Equz",
"dft,Fun_Tabz",                  "dft,Ltr_Qzzz","dft,Ltr_Wzzz","dft,Ltr_Ezzz","dft,Ltr_Rzzz","dft,Ltr_Tzzz","rgk,255_0_0",          "dft,EEEEEEEE","dft,EEEEEEEE","dft,EEEEEEEE","dft,Ltr_Yzzz","dft,Ltr_Uzzz","dft,Ltr_Kzzz","dft,Ltr_Yzzz","dft,Ltr_Pzzz","dft,Sym_BSla",
"mdt,Fun_Escz,Mod_LCtl",         "dft,Ltr_Azzz","dft,Ltr_Szzz","dft,Ltr_Dzzz","dft,Ltr_Fzzz","dft,Ltr_Gzzz","dft,Sym_Minz",         "dft,Fun_Spcz","dft,Fun_Entz","dft,Sym_Equz","dft,Ltr_Hzzz","dft,Ltr_Jzzz","dft,Ltr_Kzzz","dft,Ltr_Lzzz","dft,Sym_Scln","dft,Sym_SQut",
"dft,Mod_LSft",                  "dft,Ltr_Zzzz","dft,Ltr_Xzzz","dft,Ltr_Czzz","dft,Ltr_Vzzz","dft,Ltr_Bzzz","mdc,Sym_Minz,Mod_LSft","dft,EEEEEEEE","dft,EEEEEEEE","dft,Fun_Bksp","dft,Ltr_Nzzz","dft,Ltr_Mzzz","dft,Sym_Coma","dft,Sym_Perd","dft,Sym_FSla","dft,Mod_RSft",
"dft,Mod_LCtl",                  "dft,Mod_LAlt","dft,Mod_LCmd","dft,Fun_Spcz","dft,Sym_LBrk","dft,Mod_LCmd","dft,EEEEEEEE",         "dft,EEEEEEEE","dft,EEEEEEEE","dft,EEEEEEEE","dft,EEEEEEEE","dft,Sym_RBrk","dft,Arw_Left","dft,Arw_Down","dft,Arw_Upzz","dft,Arw_Rght",
];

impl<const RSIZE: usize, const CSIZE: usize> From<[&'static str; RSIZE * CSIZE]>
  for KeyMatrix<RSIZE, CSIZE>
{
  fn from(v: [&'static str; RSIZE * CSIZE]) -> Self {
    let mut m: [[Key; CSIZE]; RSIZE] = [[Default::new(KeyCode::EEEEEEEE); CSIZE]; RSIZE];
    let mut r: usize = 0;
    let mut c: usize = 0;
    v.iter().enumerate().for_each(|(i, sel)| {
      if i == (CSIZE * (r + 1)) {
        r += 1;
        c = 0;
      }
      if sel.len() > 0 {
        if sel.starts_with("dft,") {
          m[r][c] = Default::new(sel[4..].into());
        } else if sel.starts_with("mdt,") {
          m[r][c] = ModTap::mdtnew(&sel[4..]);
        } else if sel.starts_with("tpc,") {
          m[r][c] = TapCom::tpcnew(&sel[4..]);
        } else if sel.starts_with("mdc,") {
          m[r][c] = ModCombo::mdcnew(&sel[4..]);
        } else if sel.starts_with("rgk,") {
          m[r][c] = RGBKey::rgknew(&sel[4..]);
        } else if sel.starts_with("lyh,") {
          m[r][c] = LayerHold::lyhnew(&sel[4..]);
        } else if sel.starts_with("transparent") {
          m[r][c] = Transparent::tptnew();
        } else if sel.starts_with("sst,") {
          m[r][c] = SendString::sstnew(&sel[4..]);
        } else {
          m[r][c] = Default::new("EEEEEEEE".into());
        }
      }
      c += 1;
    });
    KeyMatrix::new(m)
  }
}
