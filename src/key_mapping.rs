use crate::key_codes::KeyCode::*;
use crate::{
    key::Default,
    keyscanning::KeyMatrix,
};

#[allow(unused_macros)]
macro_rules! df {
    ($code:expr) => {
        Default::new($code,None)
    };
}

// Maybe instead of keycodes we store functions that return keycodes.
// This way we end up making them expandable by nature.

#[rustfmt::skip]
pub fn FancyAlice66() -> KeyMatrix<5, 16> {
    KeyMatrix::new([
        [df!(Fun_Escz), df!(Num_1zzz), df!(Num_2zzz), df!(Num_3zzz), df!(Num_4zzz), df!(Num_5zzz), df!(Num_6zzz), df!(Num_7zzz), df!(Num_8zzz), df!(Num_9zzz), df!(Num_0zzz), df!(Sym_Minz), df!(Sym_Equz), df!(Fun_Bksp), df!(________), df!(Fun_Delz)],
        [df!(Fun_Tabz), df!(Ltr_Qzzz), df!(Ltr_Wzzz), df!(Ltr_Dzzz), df!(Ltr_Fzzz), df!(Ltr_Zzzz), df!(________), df!(Sym_Scln), df!(Ltr_Uzzz), df!(Ltr_Kzzz), df!(Ltr_Yzzz), df!(Ltr_Pzzz), df!(Sym_LBrk), df!(Sym_RBrk), df!(Sym_Bszz), df!(Fun_PgUp)],
        [df!(Mod_LCtl), df!(Ltr_Azzz), df!(Ltr_Szzz), df!(Ltr_Ezzz), df!(Ltr_Rzzz), df!(Ltr_Tzzz), df!(________), df!(Ltr_Hzzz), df!(Ltr_Nzzz), df!(Ltr_Izzz), df!(Ltr_Ozzz), df!(Ltr_Lzzz), df!(Sym_SQut), df!(Fun_Bksp), df!(________), df!(Fun_PgDn)],
        [df!(Mod_LSft), df!(Ltr_Gzzz), df!(Ltr_Xzzz), df!(Ltr_Czzz), df!(Ltr_Vzzz), df!(Sym_FSla), df!(________), df!(Ltr_Bzzz), df!(Ltr_Jzzz), df!(Ltr_Mzzz), df!(Sym_Coma), df!(Sym_Perd), df!(Mod_RSft), df!(Ltr_Gzzz), df!(Arw_Upzz), df!(Ltr_Gzzz)],
        [df!(Mod_LCtl), df!(________), df!(Mod_LCmd), df!(Fun_Spcz), df!(________), df!(________), df!(Sym_Minz), df!(Fun_Entz), df!(Mod_RAlt), df!(________), df!(________), df!(________), df!(________), df!(Arw_Left), df!(Arw_Down), df!(Arw_Rght)],
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
