#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    aon_syscfg_0: AonSyscfg0,
    aon_syscfg_1: AonSyscfg1,
    aon_syscfg_2: AonSyscfg2,
    aon_syscfg_3: AonSyscfg3,
    aon_syscfg_4: AonSyscfg4,
    aon_syscfg_5: AonSyscfg5,
    aon_syscfg_6: AonSyscfg6,
    aon_syscfg_7: AonSyscfg7,
    aon_syscfg_8: AonSyscfg8,
    aon_syscfg_9: AonSyscfg9,
    aon_syscfg_10: AonSyscfg10,
}
impl RegisterBlock {
    #[doc = "0x00 - AON SYSCONSAIF SYSCFG 0"]
    #[inline(always)]
    pub const fn aon_syscfg_0(&self) -> &AonSyscfg0 {
        &self.aon_syscfg_0
    }
    #[doc = "0x04 - AON SYSCONSAIF SYSCFG 4"]
    #[inline(always)]
    pub const fn aon_syscfg_1(&self) -> &AonSyscfg1 {
        &self.aon_syscfg_1
    }
    #[doc = "0x08 - AON SYSCONSAIF SYSCFG 8"]
    #[inline(always)]
    pub const fn aon_syscfg_2(&self) -> &AonSyscfg2 {
        &self.aon_syscfg_2
    }
    #[doc = "0x0c - AON SYSCONSAIF SYSCFG 12"]
    #[inline(always)]
    pub const fn aon_syscfg_3(&self) -> &AonSyscfg3 {
        &self.aon_syscfg_3
    }
    #[doc = "0x10 - AON SYSCONSAIF SYSCFG 16"]
    #[inline(always)]
    pub const fn aon_syscfg_4(&self) -> &AonSyscfg4 {
        &self.aon_syscfg_4
    }
    #[doc = "0x14 - AON SYSCONSAIF SYSCFG 20"]
    #[inline(always)]
    pub const fn aon_syscfg_5(&self) -> &AonSyscfg5 {
        &self.aon_syscfg_5
    }
    #[doc = "0x18 - AON SYSCONSAIF SYSCFG 24"]
    #[inline(always)]
    pub const fn aon_syscfg_6(&self) -> &AonSyscfg6 {
        &self.aon_syscfg_6
    }
    #[doc = "0x1c - AON SYSCONSAIF SYSCFG 28"]
    #[inline(always)]
    pub const fn aon_syscfg_7(&self) -> &AonSyscfg7 {
        &self.aon_syscfg_7
    }
    #[doc = "0x20 - AON SYSCONSAIF SYSCFG 32"]
    #[inline(always)]
    pub const fn aon_syscfg_8(&self) -> &AonSyscfg8 {
        &self.aon_syscfg_8
    }
    #[doc = "0x24 - AON SYSCONSAIF SYSCFG 36"]
    #[inline(always)]
    pub const fn aon_syscfg_9(&self) -> &AonSyscfg9 {
        &self.aon_syscfg_9
    }
    #[doc = "0x28 - AON SYSCONSAIF SYSCFG 40"]
    #[inline(always)]
    pub const fn aon_syscfg_10(&self) -> &AonSyscfg10 {
        &self.aon_syscfg_10
    }
}
#[doc = "aon_syscfg_0 (rw) register accessor: AON SYSCONSAIF SYSCFG 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aon_syscfg_0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aon_syscfg_0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aon_syscfg_0`]
module"]
#[doc(alias = "aon_syscfg_0")]
pub type AonSyscfg0 = crate::Reg<aon_syscfg_0::AonSyscfg0Spec>;
#[doc = "AON SYSCONSAIF SYSCFG 0"]
pub mod aon_syscfg_0;
#[doc = "aon_syscfg_1 (r) register accessor: AON SYSCONSAIF SYSCFG 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aon_syscfg_1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aon_syscfg_1`]
module"]
#[doc(alias = "aon_syscfg_1")]
pub type AonSyscfg1 = crate::Reg<aon_syscfg_1::AonSyscfg1Spec>;
#[doc = "AON SYSCONSAIF SYSCFG 4"]
pub mod aon_syscfg_1;
#[doc = "aon_syscfg_2 (r) register accessor: AON SYSCONSAIF SYSCFG 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aon_syscfg_2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aon_syscfg_2`]
module"]
#[doc(alias = "aon_syscfg_2")]
pub type AonSyscfg2 = crate::Reg<aon_syscfg_2::AonSyscfg2Spec>;
#[doc = "AON SYSCONSAIF SYSCFG 8"]
pub mod aon_syscfg_2;
#[doc = "aon_syscfg_3 (rw) register accessor: AON SYSCONSAIF SYSCFG 12\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aon_syscfg_3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aon_syscfg_3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aon_syscfg_3`]
module"]
#[doc(alias = "aon_syscfg_3")]
pub type AonSyscfg3 = crate::Reg<aon_syscfg_3::AonSyscfg3Spec>;
#[doc = "AON SYSCONSAIF SYSCFG 12"]
pub mod aon_syscfg_3;
#[doc = "aon_syscfg_4 (r) register accessor: AON SYSCONSAIF SYSCFG 16\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aon_syscfg_4::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aon_syscfg_4`]
module"]
#[doc(alias = "aon_syscfg_4")]
pub type AonSyscfg4 = crate::Reg<aon_syscfg_4::AonSyscfg4Spec>;
#[doc = "AON SYSCONSAIF SYSCFG 16"]
pub mod aon_syscfg_4;
#[doc = "aon_syscfg_5 (r) register accessor: AON SYSCONSAIF SYSCFG 20\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aon_syscfg_5::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aon_syscfg_5`]
module"]
#[doc(alias = "aon_syscfg_5")]
pub type AonSyscfg5 = crate::Reg<aon_syscfg_5::AonSyscfg5Spec>;
#[doc = "AON SYSCONSAIF SYSCFG 20"]
pub mod aon_syscfg_5;
#[doc = "aon_syscfg_6 (r) register accessor: AON SYSCONSAIF SYSCFG 24\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aon_syscfg_6::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aon_syscfg_6`]
module"]
#[doc(alias = "aon_syscfg_6")]
pub type AonSyscfg6 = crate::Reg<aon_syscfg_6::AonSyscfg6Spec>;
#[doc = "AON SYSCONSAIF SYSCFG 24"]
pub mod aon_syscfg_6;
#[doc = "aon_syscfg_7 (r) register accessor: AON SYSCONSAIF SYSCFG 28\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aon_syscfg_7::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aon_syscfg_7`]
module"]
#[doc(alias = "aon_syscfg_7")]
pub type AonSyscfg7 = crate::Reg<aon_syscfg_7::AonSyscfg7Spec>;
#[doc = "AON SYSCONSAIF SYSCFG 28"]
pub mod aon_syscfg_7;
#[doc = "aon_syscfg_8 (r) register accessor: AON SYSCONSAIF SYSCFG 32\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aon_syscfg_8::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aon_syscfg_8`]
module"]
#[doc(alias = "aon_syscfg_8")]
pub type AonSyscfg8 = crate::Reg<aon_syscfg_8::AonSyscfg8Spec>;
#[doc = "AON SYSCONSAIF SYSCFG 32"]
pub mod aon_syscfg_8;
#[doc = "aon_syscfg_9 (r) register accessor: AON SYSCONSAIF SYSCFG 36\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aon_syscfg_9::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aon_syscfg_9`]
module"]
#[doc(alias = "aon_syscfg_9")]
pub type AonSyscfg9 = crate::Reg<aon_syscfg_9::AonSyscfg9Spec>;
#[doc = "AON SYSCONSAIF SYSCFG 36"]
pub mod aon_syscfg_9;
#[doc = "aon_syscfg_10 (rw) register accessor: AON SYSCONSAIF SYSCFG 40\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aon_syscfg_10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aon_syscfg_10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aon_syscfg_10`]
module"]
#[doc(alias = "aon_syscfg_10")]
pub type AonSyscfg10 = crate::Reg<aon_syscfg_10::AonSyscfg10Spec>;
#[doc = "AON SYSCONSAIF SYSCFG 40"]
pub mod aon_syscfg_10;
