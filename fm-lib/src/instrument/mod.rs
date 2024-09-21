use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use fm_macros::EnumI64Derive;

#[derive(Hash,Debug, Clone, Copy, Serialize, Deserialize, EnumI64Derive, Eq, PartialEq, Ord, PartialOrd)]
// #[cfg_attr(feature = "std", derive(clap::ValueEnum))]
// #[near(serializers = [borsh])]
pub enum Instrument {
    AudCad,
    AudChf,
    AudHkd,
    AudJpy,
    AudNzd,
    AudSgd,
    AudUsd,
    CadChf,
    CadHkd,
    CadJpy,
    CadSgd,
    ChfHkd,
    ChfJpy,
    EurAud,
    EurCad,
    EurChf,
    EurGbp,
    EurHkd,
    EurJpy,
    EurNzd,
    EurSgd,
    EurUsd,
    GbpAud,
    GbpCad,
    GbpChf,
    GbpHkd,
    GbpJpy,
    GbpNzd,
    GbpSgd,
    GbpUsd,
    HkdJpy,
    NzdCad,
    NzdChf,
    NzdHkd,
    NzdJpy,
    NzdSgd,
    NzdUsd,
    SgdChf,
    SgdHkd,
    SgdJpy,
    UsdCad,
    UsdChf,
    UsdHkd,
    UsdJpy,
    UsdSgd,
    Jpy,
    NonFarmUsPayroll,
    Usd,
    Eur
}

impl Instrument {
    pub fn pip_i64(&self)->i64{
        match self {
            Instrument::EurUsd => 1,
            Instrument::UsdJpy => 100,
            _ => panic!()
        }
    }

}