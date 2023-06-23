use crate::key_codes::KeyCode::*;
use crate::{
    key::Default,
    keyscanning::KeyMatrix,
};

#[allow(unused_macros)]
macro_rules! kc {
    ($code:expr) => {
        Default::new($code,None)
    };
}

// Maybe instead of keycodes we store functions that return keycodes.
// This way we end up making them expandable by nature.

#[rustfmt::skip]
pub fn FancyAlice66() -> KeyMatrix<5, 16> {
    KeyMatrix::new([
        [kc!(Fun_Escz), kc!(Num_1zzz), kc!(Num_2zzz), kc!(Num_3zzz), kc!(Num_4zzz), kc!(Num_5zzz), kc!(Num_6zzz), kc!(Num_7zzz), kc!(Num_8zzz), kc!(Num_9zzz), kc!(Num_0zzz), kc!(Sym_Minz), kc!(Sym_Equz), kc!(Fun_Bksp), kc!(________), kc!(Fun_Delz)],
        [kc!(Fun_Tabz), kc!(Ltr_Qzzz), kc!(Ltr_Wzzz), kc!(Ltr_Dzzz), kc!(Ltr_Fzzz), kc!(Ltr_Zzzz), kc!(________), kc!(Sym_Scln), kc!(Ltr_Uzzz), kc!(Ltr_Kzzz), kc!(Ltr_Yzzz), kc!(Ltr_Pzzz), kc!(Sym_LBrk), kc!(Sym_RBrk), kc!(Sym_Bszz), kc!(Fun_PgUp)],
        [kc!(Mod_LCtl), kc!(Ltr_Azzz), kc!(Ltr_Szzz), kc!(Ltr_Ezzz), kc!(Ltr_Rzzz), kc!(Ltr_Tzzz), kc!(________), kc!(Ltr_Hzzz), kc!(Ltr_Nzzz), kc!(Ltr_Izzz), kc!(Ltr_Ozzz), kc!(Ltr_Lzzz), kc!(Sym_SQut), kc!(Fun_Bksp), kc!(________), kc!(Fun_PgDn)],
        [kc!(Mod_LSft), kc!(Ltr_Gzzz), kc!(Ltr_Xzzz), kc!(Ltr_Czzz), kc!(Ltr_Vzzz), kc!(Sym_FSla), kc!(________), kc!(Ltr_Bzzz), kc!(Ltr_Jzzz), kc!(Ltr_Mzzz), kc!(Sym_Coma), kc!(Sym_Perd), kc!(Mod_RSft), kc!(Ltr_Gzzz), kc!(Arw_Upzz), kc!(Ltr_Gzzz)],
        [kc!(Mod_LCtl), kc!(________), kc!(Mod_LCmd), kc!(Fun_Spcz), kc!(________), kc!(________), kc!(Sym_Minz), kc!(Fun_Entz), kc!(Mod_RAlt), kc!(________), kc!(________), kc!(________), kc!(________), kc!(Arw_Left), kc!(Arw_Down), kc!(Arw_Rght)],
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
