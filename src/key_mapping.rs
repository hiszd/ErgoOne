use defmt::{println, warn};
use heapless::Vec;

use crate::key::Key;
use crate::key_codes::KeyCode;
use crate::{key::Default, keyscanning::KeyMatrix, mods::mod_tap::ModTap};

// Maybe instead of keycodes we store functions that return keycodes.
// This way we end up making them expandable by nature.

// #[rustfmt::skip]
// pub fn FancyAlice66() -> KeyMatrix<5, 16> {
//     KeyMatrix::new([
//         [x!(Fun_Escz),           x!(Num_1zzz), x!(Num_2zzz), x!(Num_3zzz), x!(Num_4zzz), x!(Num_5zzz), x!(EEEEEEEE), x!(EEEEEEEE), x!(EEEEEEEE), x!(EEEEEEEE), x!(Num_6zzz), x!(Num_7zzz), x!(Num_8zzz), x!(Num_9zzz), x!(Num_0zzz), x!(Fun_Delz)],
//         [x!(Fun_Tabz),           x!(Ltr_Qzzz), x!(Ltr_Wzzz), x!(Ltr_Dzzz), x!(Ltr_Fzzz), x!(Ltr_Zzzz), x!(EEEEEEEE), x!(EEEEEEEE), x!(EEEEEEEE), x!(EEEEEEEE), x!(Sym_Scln), x!(Ltr_Uzzz), x!(Ltr_Kzzz), x!(Ltr_Yzzz), x!(Ltr_Pzzz), x!(Sym_LBrk)],
//         [t!(Fun_Escz, Mod_LCtl), x!(Ltr_Azzz), x!(Ltr_Szzz), x!(Ltr_Ezzz), x!(Ltr_Rzzz), x!(Ltr_Tzzz), x!(Sym_Minz), x!(Fun_Spcz), x!(Fun_Entz), x!(EEEEEEEE), x!(Ltr_Hzzz), x!(Ltr_Nzzz), x!(Ltr_Izzz), x!(Ltr_Ozzz), x!(Ltr_Lzzz), x!(Sym_SQut)],
//         [x!(Mod_LSft),           x!(Ltr_Gzzz), x!(Ltr_Xzzz), x!(Ltr_Czzz), x!(Ltr_Vzzz), x!(Sym_FSla), x!(Fun_Tabz), x!(EEEEEEEE), x!(EEEEEEEE), x!(Fun_Bksp), x!(Ltr_Bzzz), x!(Ltr_Jzzz), x!(Ltr_Mzzz), x!(Sym_Coma), x!(Sym_Perd), x!(Mod_RSft)],
//         [x!(Mod_LCtl),           x!(Num_9zzz), x!(Mod_LCmd), x!(Fun_Spcz), x!(Sym_LBrk), x!(Mod_LAlt), x!(EEEEEEEE), x!(EEEEEEEE), x!(EEEEEEEE), x!(EEEEEEEE), x!(EEEEEEEE), x!(Sym_RBrk), x!(Arw_Left), x!(Arw_Down), x!(Arw_Upzz), x!(Arw_Rght)],
//     ])
// }

#[rustfmt::skip]
pub const ERGOONE: [&str; 80] = [
"df_Fun_Escz",         "df_Num_1zzz","df_Num_2zzz","df_Num_3zzz","df_Num_4zzz","df_Num_5zzz","df_EEEEEEEE","df_EEEEEEEE","df_EEEEEEEE","df_EEEEEEEE","df_Num_6zzz","df_Num_7zzz","df_Num_8zzz","df_Num_9zzz","df_Num_0zzz","df_Fun_Delz",
"df_Fun_Tabz",         "df_Ltr_Qzzz","df_Ltr_Wzzz","df_Ltr_Dzzz","df_Ltr_Fzzz","df_Ltr_Zzzz","df_EEEEEEEE","df_EEEEEEEE","df_EEEEEEEE","df_EEEEEEEE","df_Sym_Scln","df_Ltr_Uzzz","df_Ltr_Kzzz","df_Ltr_Yzzz","df_Ltr_Pzzz","df_Sym_LBrk",
"mt_Fun_Escz,Mod_LCtl","df_Ltr_Azzz","df_Ltr_Szzz","df_Ltr_Ezzz","df_Ltr_Rzzz","df_Ltr_Tzzz","df_Sym_Minz","df_Fun_Spcz","df_Fun_Entz","df_EEEEEEEE","df_Ltr_Hzzz","df_Ltr_Nzzz","df_Ltr_Izzz","df_Ltr_Ozzz","df_Ltr_Lzzz","df_Sym_SQut",
"df_Mod_LSft",         "df_Ltr_Gzzz","df_Ltr_Xzzz","df_Ltr_Czzz","df_Ltr_Vzzz","df_Sym_FSla","df_Fun_Tabz","df_EEEEEEEE","df_EEEEEEEE","df_Fun_Bksp","df_Ltr_Bzzz","df_Ltr_Jzzz","df_Ltr_Mzzz","df_Sym_Coma","df_Sym_Perd","df_Mod_RSft",
"df_Mod_LCtl",         "df_Num_9zzz","df_Mod_LCmd","df_Fun_Spcz","df_Sym_LBrk","df_Mod_LAlt","df_EEEEEEEE","df_EEEEEEEE","df_EEEEEEEE","df_EEEEEEEE","df_EEEEEEEE","df_Sym_RBrk","df_Arw_Left","df_Arw_Down","df_Arw_Upzz","df_Arw_Rght",
];

// #[rustfmt::skip]
// pub const FN_LAYER_MAPPING: [[Key; 16]; 5] = [
//     [kc!(Ltr_Azzz), kc!(Ltr_Bzzz), kc!(Ltr_Czzz), kc!(Ltr_Dzzz), kc!(Ltr_Ezzz), kc!(Ltr_Fzzz), kc!(Ltr_Gzzz), kc!(Ltr_Gzzz), kc!(Ltr_Gzzz), kc!(Ltr_Gzzz), kc!(Ltr_Gzzz), kc!(Ltr_Gzzz), kc!(Ltr_Gzzz), kc!(Ltr_Gzzz), kc!(Ltr_Gzzz), kc!(Ltr_Gzzz)],
//     [kc!(Ltr_Azzz), kc!(Ltr_Bzzz), kc!(Ltr_Czzz), kc!(Ltr_Dzzz), kc!(Ltr_Ezzz), kc!(Ltr_Fzzz), kc!(Ltr_Gzzz), kc!(Ltr_Gzzz), kc!(Ltr_Gzzz), kc!(Ltr_Gzzz), kc!(Ltr_Gzzz), kc!(Ltr_Gzzz), kc!(Ltr_Gzzz), kc!(Ltr_Gzzz), kc!(Ltr_Gzzz), kc!(Ltr_Gzzz)],
//     [kc!(Ltr_Azzz), kc!(Ltr_Bzzz), kc!(Ltr_Czzz), kc!(Ltr_Dzzz), kc!(Ltr_Ezzz), kc!(Ltr_Fzzz), kc!(Ltr_Gzzz), kc!(Ltr_Gzzz), kc!(Ltr_Gzzz), kc!(Ltr_Gzzz), kc!(Ltr_Gzzz), kc!(Ltr_Gzzz), kc!(Ltr_Gzzz), kc!(Ltr_Gzzz), kc!(Ltr_Gzzz), kc!(Ltr_Gzzz)],
//     [kc!(Ltr_Azzz), kc!(Ltr_Bzzz), kc!(Ltr_Czzz), kc!(Ltr_Dzzz), kc!(Ltr_Ezzz), kc!(Ltr_Fzzz), kc!(Ltr_Gzzz), kc!(Ltr_Gzzz), kc!(Ltr_Gzzz), kc!(Ltr_Gzzz), kc!(Ltr_Gzzz), kc!(Ltr_Gzzz), kc!(Ltr_Gzzz), kc!(Ltr_Gzzz), kc!(Ltr_Gzzz), kc!(Ltr_Gzzz)],
//     [kc!(Ltr_Azzz), kc!(Ltr_Bzzz), kc!(Ltr_Czzz), kc!(Ltr_Dzzz), kc!(Ltr_Ezzz), kc!(Ltr_Fzzz), kc!(Ltr_Gzzz), kc!(Ltr_Gzzz), kc!(Ltr_Gzzz), kc!(Ltr_Gzzz), kc!(Ltr_Gzzz), kc!(Ltr_Gzzz), kc!(Ltr_Gzzz), kc!(Ltr_Gzzz), kc!(Ltr_Gzzz), kc!(Ltr_Gzzz)],
// ];

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
                if sel.starts_with("df_") {
                    let b: usize = sel.find("df_").unwrap() + 3;
                    m[r][c] = Default::new(sel[b..].into(), None);
                } else if sel.starts_with("mt_") {
                    let b: usize = sel.find("df_").unwrap_or(0) + 3;
                    let sr = sel[b..]
                        .split(",")
                        .map(|s| s.trim())
                        .collect::<Vec<&str, 2>>();
                    m[r][c] = ModTap::mtnew(sr[0].into(), sr[1].into());
                } else {
                    m[r][c] = Default::new("EEEEEEEE".into(), None);
                }
            }
            c += 1;
        });
        KeyMatrix::new(m)
    }
}
