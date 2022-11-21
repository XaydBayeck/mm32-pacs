#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Unique product indentification register 1"]
    pub uid1: UID1,
    #[doc = "0x02 - Unique product indentification register 2"]
    pub uid2: UID2,
    #[doc = "0x04 - Unique product indentification register 3"]
    pub uid3: UID3,
    #[doc = "0x08 - Unique product indentification register 4"]
    pub uid4: UID4,
}
#[doc = "UID1 (r) register accessor: an alias for `Reg<UID1_SPEC>`"]
pub type UID1 = crate::Reg<uid1::UID1_SPEC>;
#[doc = "Unique product indentification register 1"]
pub mod uid1;
#[doc = "UID2 (r) register accessor: an alias for `Reg<UID2_SPEC>`"]
pub type UID2 = crate::Reg<uid2::UID2_SPEC>;
#[doc = "Unique product indentification register 2"]
pub mod uid2;
#[doc = "UID3 (r) register accessor: an alias for `Reg<UID3_SPEC>`"]
pub type UID3 = crate::Reg<uid3::UID3_SPEC>;
#[doc = "Unique product indentification register 3"]
pub mod uid3;
#[doc = "UID4 (r) register accessor: an alias for `Reg<UID4_SPEC>`"]
pub type UID4 = crate::Reg<uid4::UID4_SPEC>;
#[doc = "Unique product indentification register 4"]
pub mod uid4;
