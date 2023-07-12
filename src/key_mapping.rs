use heapless::Vec;

use crate::key::Key;
use crate::key_codes::KeyCode;
use crate::mods::mod_combo::ModCombo;
use crate::mods::mod_tapcom::TapCom;
use crate::mods::rgb_key::RGBKey;
use crate::{key::Default, keyscanning::KeyMatrix, mods::mod_tap::ModTap};

#[rustfmt::skip]
pub const ERGOONE_0: [&str; 80] = [
"df,Fun_Escz",                  "df,Num_1zzz","df,Num_2zzz","df,Num_3zzz","df,Num_4zzz","df,Num_5zzz","rk,0_255_0",          "df,EEEEEEEE","df,EEEEEEEE","df,EEEEEEEE","df,Num_6zzz","df,Num_7zzz","df,Num_8zzz","df,Num_9zzz","df,Num_0zzz","df,Sym_Equz",
"df,Fun_Tabz",                  "df,Ltr_Qzzz","df,Ltr_Wzzz","df,Ltr_Dzzz","df,Ltr_Fzzz","df,Ltr_Zzzz","rk,255_0_0",          "df,EEEEEEEE","df,EEEEEEEE","df,EEEEEEEE","df,Sym_Scln","df,Ltr_Uzzz","df,Ltr_Kzzz","df,Ltr_Yzzz","df,Ltr_Pzzz","df,Sym_BSla",
"mt,Fun_Escz,Mod_LCtl",         "df,Ltr_Azzz","df,Ltr_Szzz","df,Ltr_Ezzz","df,Ltr_Rzzz","df,Ltr_Tzzz","df,Sym_Minz",         "df,Fun_Spcz","df,Fun_Entz","df,Sym_Equz","df,Ltr_Hzzz","df,Ltr_Nzzz","df,Ltr_Izzz","df,Ltr_Ozzz","df,Ltr_Lzzz","df,Sym_SQut",
"tc,Mod_LSft,Mod_LSft,Num_9zzz","df,Ltr_Gzzz","df,Ltr_Xzzz","df,Ltr_Czzz","df,Ltr_Vzzz","df,Sym_FSla","mc,Sym_Minz,Mod_LSft","df,EEEEEEEE","df,EEEEEEEE","df,Fun_Bksp","df,Ltr_Bzzz","df,Ltr_Jzzz","df,Ltr_Mzzz","df,Sym_Coma","df,Sym_Perd","tc,Mod_RSft,Mod_RSft,Num_0zzz",
"df,Mod_LCtl",                  "df,Mod_LAlt","df,Mod_LCmd","df,Fun_Spcz","df,Sym_LBrk","df,Mod_LCmd","df,EEEEEEEE",         "df,EEEEEEEE","df,EEEEEEEE","df,EEEEEEEE","df,EEEEEEEE","df,Sym_RBrk","df,Arw_Left","df,Arw_Down","df,Arw_Upzz","df,Arw_Rght",
];

#[allow(dead_code)]
#[rustfmt::skip]
pub const ERGOONE_1: [&str; 80] = [
"df,________", "df,________","df,________","df,________","df,________","df,________","df,EEEEEEEE","df,EEEEEEEE","df,EEEEEEEE","df,EEEEEEEE","df,________","df,________","df,________","df,________","df,________","df,________",
"df,________", "df,________","df,________","df,________","df,________","df,________","df,EEEEEEEE","df,EEEEEEEE","df,EEEEEEEE","df,EEEEEEEE","df,________","df,________","df,________","df,________","df,________","df,________",
"df,________", "df,________","df,________","df,________","df,________","df,________","df,________","df,________","df,________","df,EEEEEEEE","df,________","df,________","df,________","df,________","df,________","df,________",
"df,________", "df,________","df,________","df,________","df,________","df,________","df,________","df,EEEEEEEE","df,EEEEEEEE","df,________","df,________","df,________","df,________","df,________","df,________","df,________",
"df,________", "df,________","df,________","df,________","df,________","df,________","df,EEEEEEEE","df,EEEEEEEE","df,EEEEEEEE","df,EEEEEEEE","df,EEEEEEEE","df,________","df,________","df,________","df,________","df,________",
];

// TODO use enum or lookup function to get the parsing function for these key strings from the
// modules themselves instead of writing the parsing functions here
impl<const RSIZE: usize, const CSIZE: usize> From<[&str; RSIZE * CSIZE]>
    for KeyMatrix<RSIZE, CSIZE>
{
    fn from(v: [&str; RSIZE * CSIZE]) -> Self {
        let mut m: [[Key; CSIZE]; RSIZE] = [[Default::new(KeyCode::EEEEEEEE, None); CSIZE]; RSIZE];
        let mut r: usize = 0;
        let mut c: usize = 0;
        v.iter().enumerate().for_each(|(i, sel)| {
            if i == (CSIZE * (r + 1)) {
                r += 1;
                c = 0;
            }
            if sel.len() > 0 {
                // TODO use split and join with trim to remove whitespace instead of slicing the
                // string and then parsing it
                if sel.starts_with("df,") {
                    let b: usize = sel.find("df,").unwrap() + 3;
                    m[r][c] = Default::new(sel[b..].into(), None);
                } else if sel.starts_with("mt,") {
                    let b: usize = sel.find("mt,").unwrap_or(0) + 3;
                    let sr = sel[b..]
                        .split(",")
                        .map(|s| s.trim())
                        .collect::<Vec<&str, 2>>();
                    m[r][c] = ModTap::mtnew(sr[0].into(), sr[1].into());
                } else if sel.starts_with("tc,") {
                    let b: usize = sel.find("tc,").unwrap_or(0) + 3;
                    let sr = sel[b..]
                        .split(",")
                        .map(|s| s.trim())
                        .collect::<Vec<&str, 3>>();
                    m[r][c] = TapCom::tcnew(sr[0].into(), (sr[1].into(), sr[2].into()));
                } else if sel.starts_with("mc,") {
                    let b: usize = sel.find("mc,").unwrap_or(0) + 3;
                    let sr = sel[b..]
                        .split(",")
                        .map(|s| s.trim())
                        .collect::<Vec<&str, 2>>();
                    m[r][c] = ModCombo::mcnew(sr[0].into(), sr[1].into());
                } else if sel.starts_with("rk,") {
                    let b: usize = sel.find("rk,").unwrap_or(0) + 3;
                    let sr = &sel[b..].split("_").map(|x| x.trim()).collect::<Vec<&str, 3>>();
                    m[r][c] = RGBKey::rknew(sr[0].parse().unwrap(), sr[1].parse().unwrap(), sr[2].parse().unwrap());
                } else {
                    m[r][c] = Default::new("EEEEEEEE".into(), None);
                }
            }
            c += 1;
        });
        KeyMatrix::new(m)
    }
}
