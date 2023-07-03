use crate::key_codes::KeyCode::*;
use crate::{key::Default, keyscanning::KeyMatrix, mods::mod_tap::ModTap};

use crate::t;

#[allow(unused_macros)]
macro_rules! x {
    ($code:expr) => {
        Default::new($code, None)
    };
}

// Maybe instead of keycodes we store functions that return keycodes.
// This way we end up making them expandable by nature.

#[rustfmt::skip]
pub fn FancyAlice66() -> KeyMatrix<5, 16> {
    KeyMatrix::new([
        [x!(Fun_Escz),           x!(Num_1zzz), x!(Num_2zzz), x!(Num_3zzz), x!(Num_4zzz), x!(Num_5zzz), x!(EEEEEEEE), x!(EEEEEEEE), x!(EEEEEEEE), x!(EEEEEEEE), x!(Num_6zzz), x!(Num_7zzz), x!(Num_8zzz), x!(Num_9zzz), x!(Num_0zzz), x!(Fun_Delz)],
        [x!(Fun_Tabz),           x!(Ltr_Qzzz), x!(Ltr_Wzzz), x!(Ltr_Dzzz), x!(Ltr_Fzzz), x!(Ltr_Zzzz), x!(EEEEEEEE), x!(EEEEEEEE), x!(EEEEEEEE), x!(EEEEEEEE), x!(Sym_Scln), x!(Ltr_Uzzz), x!(Ltr_Kzzz), x!(Ltr_Yzzz), x!(Ltr_Pzzz), x!(Sym_LBrk)],
        [t!(Fun_Escz, Mod_LCtl), x!(Ltr_Azzz), x!(Ltr_Szzz), x!(Ltr_Ezzz), x!(Ltr_Rzzz), x!(Ltr_Tzzz), x!(Sym_Minz), x!(Fun_Spcz), x!(Fun_Entz), x!(EEEEEEEE), x!(Ltr_Hzzz), x!(Ltr_Nzzz), x!(Ltr_Izzz), x!(Ltr_Ozzz), x!(Ltr_Lzzz), x!(Sym_SQut)],
        [x!(Mod_LSft),           x!(Ltr_Gzzz), x!(Ltr_Xzzz), x!(Ltr_Czzz), x!(Ltr_Vzzz), x!(Sym_FSla), x!(Fun_Tabz), x!(EEEEEEEE), x!(EEEEEEEE), x!(Fun_Bksp), x!(Ltr_Bzzz), x!(Ltr_Jzzz), x!(Ltr_Mzzz), x!(Sym_Coma), x!(Sym_Perd), x!(Mod_RSft)],
        [x!(Mod_LCtl),           x!(Num_9zzz), x!(Mod_LCmd), x!(Fun_Spcz), x!(Sym_LBrk), x!(Mod_LAlt), x!(EEEEEEEE), x!(EEEEEEEE), x!(EEEEEEEE), x!(EEEEEEEE), x!(EEEEEEEE), x!(Sym_RBrk), x!(Arw_Left), x!(Arw_Down), x!(Arw_Upzz), x!(Arw_Rght)],
    ])
}

// #[rustfmt::skip]
// pub const FN_LAYER_MAPPING: [[Key; 16]; 5] = [
//     [kc!(Ltr_Azzz), kc!(Ltr_Bzzz), kc!(Ltr_Czzz), kc!(Ltr_Dzzz), kc!(Ltr_Ezzz), kc!(Ltr_Fzzz), kc!(Ltr_Gzzz), kc!(Ltr_Gzzz), kc!(Ltr_Gzzz), kc!(Ltr_Gzzz), kc!(Ltr_Gzzz), kc!(Ltr_Gzzz), kc!(Ltr_Gzzz), kc!(Ltr_Gzzz), kc!(Ltr_Gzzz), kc!(Ltr_Gzzz)],
//     [kc!(Ltr_Azzz), kc!(Ltr_Bzzz), kc!(Ltr_Czzz), kc!(Ltr_Dzzz), kc!(Ltr_Ezzz), kc!(Ltr_Fzzz), kc!(Ltr_Gzzz), kc!(Ltr_Gzzz), kc!(Ltr_Gzzz), kc!(Ltr_Gzzz), kc!(Ltr_Gzzz), kc!(Ltr_Gzzz), kc!(Ltr_Gzzz), kc!(Ltr_Gzzz), kc!(Ltr_Gzzz), kc!(Ltr_Gzzz)],
//     [kc!(Ltr_Azzz), kc!(Ltr_Bzzz), kc!(Ltr_Czzz), kc!(Ltr_Dzzz), kc!(Ltr_Ezzz), kc!(Ltr_Fzzz), kc!(Ltr_Gzzz), kc!(Ltr_Gzzz), kc!(Ltr_Gzzz), kc!(Ltr_Gzzz), kc!(Ltr_Gzzz), kc!(Ltr_Gzzz), kc!(Ltr_Gzzz), kc!(Ltr_Gzzz), kc!(Ltr_Gzzz), kc!(Ltr_Gzzz)],
//     [kc!(Ltr_Azzz), kc!(Ltr_Bzzz), kc!(Ltr_Czzz), kc!(Ltr_Dzzz), kc!(Ltr_Ezzz), kc!(Ltr_Fzzz), kc!(Ltr_Gzzz), kc!(Ltr_Gzzz), kc!(Ltr_Gzzz), kc!(Ltr_Gzzz), kc!(Ltr_Gzzz), kc!(Ltr_Gzzz), kc!(Ltr_Gzzz), kc!(Ltr_Gzzz), kc!(Ltr_Gzzz), kc!(Ltr_Gzzz)],
//     [kc!(Ltr_Azzz), kc!(Ltr_Bzzz), kc!(Ltr_Czzz), kc!(Ltr_Dzzz), kc!(Ltr_Ezzz), kc!(Ltr_Fzzz), kc!(Ltr_Gzzz), kc!(Ltr_Gzzz), kc!(Ltr_Gzzz), kc!(Ltr_Gzzz), kc!(Ltr_Gzzz), kc!(Ltr_Gzzz), kc!(Ltr_Gzzz), kc!(Ltr_Gzzz), kc!(Ltr_Gzzz), kc!(Ltr_Gzzz)],
// ];

impl<const RSIZE: usize, const CSIZE: usize> From<[[&str; CSIZE]; RSIZE]>
    for KeyMatrix<RSIZE, CSIZE>
{
    // create KeyMatrix from a two dimensional array of strings
    fn from(v: [[&str; CSIZE]; RSIZE]) -> Self {}
}
