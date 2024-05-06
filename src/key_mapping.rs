use defmt::error;
use heapless::Vec;

use crate::key::{Default, Key, Modules};
use crate::keyscanning::KeyMatrix;
use crate::mods::hidvol::HIDVol;
use crate::mods::layer_hold::LayerHold;
use crate::mods::mod_combo::ModCombo;
use crate::mods::mod_tap::ModTap;
use crate::mods::mod_tapcom::TapCom;
use crate::mods::mod_tapstr::TapStr;
use crate::mods::rgb_key::RGBKey;
use crate::mods::sendhid::SendHID;
use crate::mods::sendstring::SendString;
use crate::mods::transparent::Transparent;
use crate::secrets::*;

// TODO: Create a build step for getting the errors out of the strings in mapping, before the
// firmware is running on the controller.

// TODO: find a better way to nest functionality
#[allow(dead_code)]
#[rustfmt::skip]
pub const ERGOONE_RSTLNE: [&str; 80] = [
"dft,Sym_Tild",         "dft,Num_1zzz","dft,Num_2zzz","dft,Num_3zzz","dft,Num_4zzz","dft,Num_5zzz","rgk,0_255_0",          "dft,EEEEEEEE","dft,EEEEEEEE","dft,EEEEEEEE","dft,Num_6zzz","dft,Num_7zzz","dft,Num_8zzz","dft,Num_9zzz","dft,Num_0zzz","dft,Sym_Equz",
"dft,Fun_Tabz",         "dft,Ltr_Qzzz","dft,Ltr_Wzzz","dft,Ltr_Dzzz","dft,Ltr_Fzzz","dft,Ltr_Zzzz","rgk,255_0_0",          "dft,EEEEEEEE","dft,EEEEEEEE","dft,EEEEEEEE","dft,Sym_Scln","dft,Ltr_Uzzz","dft,Ltr_Kzzz","dft,Ltr_Yzzz","dft,Ltr_Pzzz","dft,Sym_BSla",
"mdt,Fun_Escz,Mod_LCtl","dft,Ltr_Azzz","dft,Ltr_Szzz","dft,Ltr_Ezzz","dft,Ltr_Rzzz","dft,Ltr_Tzzz","dft,Sym_Minz",         "dft,Fun_Spcz","dft,Fun_Entz","dft,Sym_Equz","dft,Ltr_Hzzz","dft,Ltr_Nzzz","dft,Ltr_Izzz","dft,Ltr_Ozzz","dft,Ltr_Lzzz","dft,Sym_SQut",
"tps,Mod_LSft,(",       "dft,Ltr_Gzzz","dft,Ltr_Xzzz","dft,Ltr_Czzz","dft,Ltr_Vzzz","dft,Sym_FSla","sst,_",                "dft,Fun_Home","dft,Fun_PgDn","dft,Fun_Bksp","dft,Ltr_Bzzz","dft,Ltr_Jzzz","dft,Ltr_Mzzz","dft,Sym_Coma","dft,Sym_Perd","tps,Mod_RSft,)",
"dft,Mod_LCtl",         "dft,Mod_LAlt","dft,Mod_LCmd","dft,Fun_Spcz","dft,Sym_LBrk","dft,Mod_LCmd","dft,EEEEEEEE",         "dft,Fun_Endz","dft,Fun_PgUp","dft,EEEEEEEE","lyh,1",     "dft,Sym_RBrk","dft,Arw_Left","dft,Arw_Down","dft,Arw_Upzz","dft,Arw_Rght",
];

#[allow(dead_code)]
#[rustfmt::skip]
pub const ERGOONE_1: [&str; 80] = [
"transparent",          "dft,Fun_F1zz","dft,Fun_F2zz","dft,Fun_F3zz","dft,Fun_F4zz","dft,Fun_F5zz","transparent",          "transparent", "transparent", "transparent", "dft,Fun_F6zz","dft,Fun_F7zz","dft,Fun_F8zz","dft,Fun_F9zz","dft,Fun_F10z","dft,Fun_F11z",
"transparent",          "transparent", "dft,Arw_Upzz","transparent", "transparent", "transparent", "transparent",          "transparent", "transparent", "transparent", "transparent", "dft,Num_7zzz","dft,Num_8zzz","dft,Num_9zzz","transparent", "dft,Fun_F12z",
"transparent",          "dft,Arw_Left","dft,Arw_Down","dft,Arw_Rght","transparent", "transparent", "transparent",          WORKUSER,      "transparent", "transparent", "transparent", "dft,Num_4zzz","dft,Num_5zzz","dft,Num_6zzz","transparent", "transparent", 
"transparent",          "vol,set:100:firefox", "vol,set:0:firefox",   "vol,inc:10",  "vol,dec:10",  "vol,togglemute:firefox",PERSONAL,   "transparent", "transparent", "dft,Fun_Delz","transparent", "dft,Num_1zzz","dft,Num_2zzz","dft,Num_3zzz","transparent", "transparent", 
"transparent",          "transparent", "transparent", "transparent", "transparent", WORKADMIN,     "transparent",          "transparent", "transparent", "transparent", "transparent", "dft,Num_0zzz","transparent", "transparent", "transparent", "transparent", 
];

#[allow(dead_code)]
#[rustfmt::skip]
pub const ERGOONE_QWERTY: [&str; 80] = [
"dft,Sym_Tild",         "dft,Num_1zzz","dft,Num_2zzz","dft,Num_3zzz","dft,Num_4zzz","dft,Num_5zzz","rgk,0_255_0",          "dft,EEEEEEEE","dft,EEEEEEEE","dft,EEEEEEEE","dft,Num_6zzz","dft,Num_7zzz","dft,Num_8zzz","dft,Num_9zzz","dft,Num_0zzz","dft,Sym_Equz",
"dft,Fun_Tabz",         "dft,Ltr_Qzzz","dft,Ltr_Wzzz","dft,Ltr_Ezzz","dft,Ltr_Rzzz","dft,Ltr_Tzzz","rgk,255_0_0",          "dft,EEEEEEEE","dft,EEEEEEEE","dft,EEEEEEEE","dft,Ltr_Yzzz","dft,Ltr_Uzzz","dft,Ltr_Izzz","dft,Ltr_Ozzz","dft,Ltr_Pzzz","dft,Sym_BSla",
"mdt,Fun_Escz,Mod_LCtl","dft,Ltr_Azzz","dft,Ltr_Szzz","dft,Ltr_Dzzz","dft,Ltr_Fzzz","dft,Ltr_Gzzz","dft,Sym_Minz",         "dft,Fun_Spcz","dft,Fun_Entz","dft,Sym_Equz","dft,Ltr_Hzzz","dft,Ltr_Jzzz","dft,Ltr_Kzzz","dft,Ltr_Lzzz","dft,Sym_Scln","dft,Sym_SQut",
"tps,Mod_LSft,(",       "dft,Ltr_Zzzz","dft,Ltr_Xzzz","dft,Ltr_Czzz","dft,Ltr_Vzzz","dft,Ltr_Bzzz","sst,_",                "dft,Fun_Home","dft,Fun_PgDn","dft,Fun_Bksp","dft,Ltr_Nzzz","dft,Ltr_Mzzz","dft,Sym_Coma","dft,Sym_Perd","dft,Sym_FSla","tps,Mod_RSft,)",
"dft,Mod_LCtl",         "dft,Mod_LAlt","dft,Mod_LCmd","dft,EEEEEEEE","dft,Sym_LBrk","dft,Mod_LCmd","shr,volume:100",         "dft,Fun_Endz","dft,Fun_PgUp","shr,volume:0","lyh,1",     "dft,Sym_RBrk","dft,Arw_Left","dft,Arw_Down","dft,Arw_Upzz","dft,Arw_Rght",
];

pub fn keymap_from<const RSIZE: usize, const CSIZE: usize>(
  v: [&'static str; RSIZE * CSIZE],
  l: usize,
) -> KeyMatrix<RSIZE, CSIZE> {
  let mut m: [[Key; CSIZE]; RSIZE] =
    [[Default::new(Vec::from_slice(&["EEEEEEEE"]).unwrap()); CSIZE]; RSIZE];
  let mut r: usize = 0;
  let mut c: usize = 0;
  v.iter().enumerate().for_each(|(i, sel)| {
    if i == (CSIZE * (r + 1)) {
      r += 1;
      c = 0;
    }
    if sel.len() > 0 {
      let spt = &sel.split(',').collect::<Vec<&str, 5>>();
      let mdl = spt[0];
      let map = &spt
        .iter()
        .enumerate()
        .filter(|(i, _)| *i > 0)
        .map(|(_, v)| *v)
        .collect::<Vec<&str, 4>>();
      match Modules::try_from_str(mdl) {
        Ok(Modules::Default) => {
          m[r][c] = Default::new(map.clone());
        }
        Ok(Modules::LayerHold) => {
          m[r][c] = LayerHold::new(map.clone(), l);
        }
        Ok(Modules::ModTap) => {
          m[r][c] = ModTap::new(map.clone());
        }
        Ok(Modules::TapCom) => {
          m[r][c] = TapCom::new(map.clone());
        }
        Ok(Modules::ModCombo) => {
          m[r][c] = ModCombo::new(map.clone());
        }
        Ok(Modules::TapStr) => {
          m[r][c] = TapStr::new(map.clone());
        }
        Ok(Modules::RGBKey) => {
          m[r][c] = RGBKey::new(map.clone());
        }
        Ok(Modules::SendHIDRaw) => {
          m[r][c] = SendHID::new(map.clone());
        }
        Ok(Modules::HIDVol) => {
          m[r][c] = HIDVol::new(map.clone());
        }
        Ok(Modules::SendString) => {
          m[r][c] = SendString::new(map.clone());
        }
        Ok(Modules::Transparent) => {
          m[r][c] = Transparent::new();
        }
        _ => {
          error!(
            "Key mapping at ({}, {}) was specified incorrectly as: {}, {}",
            r, c, sel, mdl
          );
          m[r][c] = Default::new(Vec::from_slice(&["EEEEEEEE"]).unwrap());
        }
      }
    }
    c += 1;
  });
  KeyMatrix::new(m)
}
