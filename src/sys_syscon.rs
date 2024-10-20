#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    sys_syscfg0: SysSyscfg0,
    sys_syscfg1: SysSyscfg1,
    sys_syscfg2: SysSyscfg2,
    sys_syscfg3: SysSyscfg3,
    sys_syscfg4: SysSyscfg4,
    sys_syscfg5: SysSyscfg5,
    sys_syscfg6: SysSyscfg6,
    sys_syscfg7: SysSyscfg7,
    sys_syscfg8: SysSyscfg8,
    sys_syscfg9: SysSyscfg9,
    sys_syscfg10: SysSyscfg10,
    sys_syscfg11: SysSyscfg11,
    sys_syscfg12: SysSyscfg12,
    sys_syscfg13: SysSyscfg13,
    sys_syscfg14: SysSyscfg14,
    sys_syscfg_noc_bus_oic_qch_clock_stop: [SysSyscfgNocBusOicQchClockStop; 9],
    sys_syscfg24: SysSyscfg24,
    sys_syscfg25: SysSyscfg25,
    sys_syscfg_reset_vector: SysSyscfgResetVector,
    sys_syscfg33: SysSyscfg33,
    sys_syscfg34: SysSyscfg34,
    sys_syscfg35: SysSyscfg35,
    sys_syscfg36: SysSyscfg36,
    sys_syscfg37: SysSyscfg37,
    sys_syscfg38: SysSyscfg38,
    sys_syscfg39: SysSyscfg39,
}
impl RegisterBlock {
    #[doc = "0x00 - SYS SYSCONSAIF SYSCFG 0"]
    #[inline(always)]
    pub const fn sys_syscfg0(&self) -> &SysSyscfg0 {
        &self.sys_syscfg0
    }
    #[doc = "0x04 - SYS SYSCONSAIF SYSCFG 4"]
    #[inline(always)]
    pub const fn sys_syscfg1(&self) -> &SysSyscfg1 {
        &self.sys_syscfg1
    }
    #[doc = "0x08 - SYS SYSCONSAIF SYSCFG 8"]
    #[inline(always)]
    pub const fn sys_syscfg2(&self) -> &SysSyscfg2 {
        &self.sys_syscfg2
    }
    #[doc = "0x0c - SYS SYSCONSAIF SYSCFG 12"]
    #[inline(always)]
    pub const fn sys_syscfg3(&self) -> &SysSyscfg3 {
        &self.sys_syscfg3
    }
    #[doc = "0x10 - SYS SYSCONSAIF SYSCFG 16"]
    #[inline(always)]
    pub const fn sys_syscfg4(&self) -> &SysSyscfg4 {
        &self.sys_syscfg4
    }
    #[doc = "0x14 - SYS SYSCONSAIF SYSCFG 20"]
    #[inline(always)]
    pub const fn sys_syscfg5(&self) -> &SysSyscfg5 {
        &self.sys_syscfg5
    }
    #[doc = "0x18 - SYS SYSCONSAIF SYSCFG 24"]
    #[inline(always)]
    pub const fn sys_syscfg6(&self) -> &SysSyscfg6 {
        &self.sys_syscfg6
    }
    #[doc = "0x1c - SYS SYSCONSAIF SYSCFG 28"]
    #[inline(always)]
    pub const fn sys_syscfg7(&self) -> &SysSyscfg7 {
        &self.sys_syscfg7
    }
    #[doc = "0x20 - SYS SYSCONSAIF SYSCFG 32"]
    #[inline(always)]
    pub const fn sys_syscfg8(&self) -> &SysSyscfg8 {
        &self.sys_syscfg8
    }
    #[doc = "0x24 - SYS SYSCONSAIF SYSCFG 36"]
    #[inline(always)]
    pub const fn sys_syscfg9(&self) -> &SysSyscfg9 {
        &self.sys_syscfg9
    }
    #[doc = "0x28 - SYS SYSCONSAIF SYSCFG 40"]
    #[inline(always)]
    pub const fn sys_syscfg10(&self) -> &SysSyscfg10 {
        &self.sys_syscfg10
    }
    #[doc = "0x2c - SYS SYSCONSAIF SYSCFG 44"]
    #[inline(always)]
    pub const fn sys_syscfg11(&self) -> &SysSyscfg11 {
        &self.sys_syscfg11
    }
    #[doc = "0x30 - SYS SYSCONSAIF SYSCFG 48"]
    #[inline(always)]
    pub const fn sys_syscfg12(&self) -> &SysSyscfg12 {
        &self.sys_syscfg12
    }
    #[doc = "0x34 - SYS SYSCONSAIF SYSCFG 52"]
    #[inline(always)]
    pub const fn sys_syscfg13(&self) -> &SysSyscfg13 {
        &self.sys_syscfg13
    }
    #[doc = "0x38 - SYS SYSCONSAIF SYSCFG 56"]
    #[inline(always)]
    pub const fn sys_syscfg14(&self) -> &SysSyscfg14 {
        &self.sys_syscfg14
    }
    #[doc = "0x3c..0x60 - SYS SYSCONSAIF SYSCFG 60 - 92: NOC Bus OIC QCH Clock Stop Threshold registers."]
    #[inline(always)]
    pub const fn sys_syscfg_noc_bus_oic_qch_clock_stop(
        &self,
        n: usize,
    ) -> &SysSyscfgNocBusOicQchClockStop {
        &self.sys_syscfg_noc_bus_oic_qch_clock_stop[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x3c..0x60 - SYS SYSCONSAIF SYSCFG 60 - 92: NOC Bus OIC QCH Clock Stop Threshold registers."]
    #[inline(always)]
    pub fn sys_syscfg_noc_bus_oic_qch_clock_stop_iter(
        &self,
    ) -> impl Iterator<Item = &SysSyscfgNocBusOicQchClockStop> {
        self.sys_syscfg_noc_bus_oic_qch_clock_stop.iter()
    }
    #[doc = "0x60 - SYS SYSCONSAIF SYSCFG 96"]
    #[inline(always)]
    pub const fn sys_syscfg24(&self) -> &SysSyscfg24 {
        &self.sys_syscfg24
    }
    #[doc = "0x64 - SYS SYSCONSAIF SYSCFG 96"]
    #[inline(always)]
    pub const fn sys_syscfg25(&self) -> &SysSyscfg25 {
        &self.sys_syscfg25
    }
    #[doc = "0x68..0x84 - SYS SYSCONSAIF SYSCFG 104 - 128: Reset Vector registers."]
    #[inline(always)]
    pub const fn sys_syscfg_reset_vector(&self) -> &SysSyscfgResetVector {
        &self.sys_syscfg_reset_vector
    }
    #[doc = "0x84 - SYS SYSCONSAIF SYSCFG 132"]
    #[inline(always)]
    pub const fn sys_syscfg33(&self) -> &SysSyscfg33 {
        &self.sys_syscfg33
    }
    #[doc = "0x88 - SYS SYSCONSAIF SYSCFG 136"]
    #[inline(always)]
    pub const fn sys_syscfg34(&self) -> &SysSyscfg34 {
        &self.sys_syscfg34
    }
    #[doc = "0x8c - SYS SYSCONSAIF SYSCFG 140"]
    #[inline(always)]
    pub const fn sys_syscfg35(&self) -> &SysSyscfg35 {
        &self.sys_syscfg35
    }
    #[doc = "0x90 - SYS SYSCONSAIF SYSCFG 144"]
    #[inline(always)]
    pub const fn sys_syscfg36(&self) -> &SysSyscfg36 {
        &self.sys_syscfg36
    }
    #[doc = "0x94 - SYS SYSCONSAIF SYSCFG 148"]
    #[inline(always)]
    pub const fn sys_syscfg37(&self) -> &SysSyscfg37 {
        &self.sys_syscfg37
    }
    #[doc = "0x98 - SYS SYSCONSAIF SYSCFG 152"]
    #[inline(always)]
    pub const fn sys_syscfg38(&self) -> &SysSyscfg38 {
        &self.sys_syscfg38
    }
    #[doc = "0x9c - SYS SYSCONSAIF SYSCFG 156"]
    #[inline(always)]
    pub const fn sys_syscfg39(&self) -> &SysSyscfg39 {
        &self.sys_syscfg39
    }
}
#[doc = "sys_syscfg0 (rw) register accessor: SYS SYSCONSAIF SYSCFG 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_syscfg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_syscfg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_syscfg0`]
module"]
#[doc(alias = "sys_syscfg0")]
pub type SysSyscfg0 = crate::Reg<sys_syscfg0::SysSyscfg0Spec>;
#[doc = "SYS SYSCONSAIF SYSCFG 0"]
pub mod sys_syscfg0;
#[doc = "sys_syscfg1 (rw) register accessor: SYS SYSCONSAIF SYSCFG 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_syscfg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_syscfg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_syscfg1`]
module"]
#[doc(alias = "sys_syscfg1")]
pub type SysSyscfg1 = crate::Reg<sys_syscfg1::SysSyscfg1Spec>;
#[doc = "SYS SYSCONSAIF SYSCFG 4"]
pub mod sys_syscfg1;
#[doc = "sys_syscfg2 (rw) register accessor: SYS SYSCONSAIF SYSCFG 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_syscfg2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_syscfg2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_syscfg2`]
module"]
#[doc(alias = "sys_syscfg2")]
pub type SysSyscfg2 = crate::Reg<sys_syscfg2::SysSyscfg2Spec>;
#[doc = "SYS SYSCONSAIF SYSCFG 8"]
pub mod sys_syscfg2;
#[doc = "sys_syscfg3 (rw) register accessor: SYS SYSCONSAIF SYSCFG 12\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_syscfg3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_syscfg3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_syscfg3`]
module"]
#[doc(alias = "sys_syscfg3")]
pub type SysSyscfg3 = crate::Reg<sys_syscfg3::SysSyscfg3Spec>;
#[doc = "SYS SYSCONSAIF SYSCFG 12"]
pub mod sys_syscfg3;
#[doc = "sys_syscfg4 (rw) register accessor: SYS SYSCONSAIF SYSCFG 16\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_syscfg4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_syscfg4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_syscfg4`]
module"]
#[doc(alias = "sys_syscfg4")]
pub type SysSyscfg4 = crate::Reg<sys_syscfg4::SysSyscfg4Spec>;
#[doc = "SYS SYSCONSAIF SYSCFG 16"]
pub mod sys_syscfg4;
#[doc = "sys_syscfg5 (rw) register accessor: SYS SYSCONSAIF SYSCFG 20\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_syscfg5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_syscfg5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_syscfg5`]
module"]
#[doc(alias = "sys_syscfg5")]
pub type SysSyscfg5 = crate::Reg<sys_syscfg5::SysSyscfg5Spec>;
#[doc = "SYS SYSCONSAIF SYSCFG 20"]
pub mod sys_syscfg5;
#[doc = "sys_syscfg6 (rw) register accessor: SYS SYSCONSAIF SYSCFG 24\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_syscfg6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_syscfg6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_syscfg6`]
module"]
#[doc(alias = "sys_syscfg6")]
pub type SysSyscfg6 = crate::Reg<sys_syscfg6::SysSyscfg6Spec>;
#[doc = "SYS SYSCONSAIF SYSCFG 24"]
pub mod sys_syscfg6;
#[doc = "sys_syscfg7 (rw) register accessor: SYS SYSCONSAIF SYSCFG 28\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_syscfg7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_syscfg7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_syscfg7`]
module"]
#[doc(alias = "sys_syscfg7")]
pub type SysSyscfg7 = crate::Reg<sys_syscfg7::SysSyscfg7Spec>;
#[doc = "SYS SYSCONSAIF SYSCFG 28"]
pub mod sys_syscfg7;
#[doc = "sys_syscfg8 (rw) register accessor: SYS SYSCONSAIF SYSCFG 32\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_syscfg8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_syscfg8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_syscfg8`]
module"]
#[doc(alias = "sys_syscfg8")]
pub type SysSyscfg8 = crate::Reg<sys_syscfg8::SysSyscfg8Spec>;
#[doc = "SYS SYSCONSAIF SYSCFG 32"]
pub mod sys_syscfg8;
#[doc = "sys_syscfg9 (rw) register accessor: SYS SYSCONSAIF SYSCFG 36\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_syscfg9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_syscfg9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_syscfg9`]
module"]
#[doc(alias = "sys_syscfg9")]
pub type SysSyscfg9 = crate::Reg<sys_syscfg9::SysSyscfg9Spec>;
#[doc = "SYS SYSCONSAIF SYSCFG 36"]
pub mod sys_syscfg9;
#[doc = "sys_syscfg10 (rw) register accessor: SYS SYSCONSAIF SYSCFG 40\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_syscfg10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_syscfg10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_syscfg10`]
module"]
#[doc(alias = "sys_syscfg10")]
pub type SysSyscfg10 = crate::Reg<sys_syscfg10::SysSyscfg10Spec>;
#[doc = "SYS SYSCONSAIF SYSCFG 40"]
pub mod sys_syscfg10;
#[doc = "sys_syscfg11 (rw) register accessor: SYS SYSCONSAIF SYSCFG 44\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_syscfg11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_syscfg11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_syscfg11`]
module"]
#[doc(alias = "sys_syscfg11")]
pub type SysSyscfg11 = crate::Reg<sys_syscfg11::SysSyscfg11Spec>;
#[doc = "SYS SYSCONSAIF SYSCFG 44"]
pub mod sys_syscfg11;
#[doc = "sys_syscfg12 (rw) register accessor: SYS SYSCONSAIF SYSCFG 48\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_syscfg12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_syscfg12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_syscfg12`]
module"]
#[doc(alias = "sys_syscfg12")]
pub type SysSyscfg12 = crate::Reg<sys_syscfg12::SysSyscfg12Spec>;
#[doc = "SYS SYSCONSAIF SYSCFG 48"]
pub mod sys_syscfg12;
#[doc = "sys_syscfg13 (rw) register accessor: SYS SYSCONSAIF SYSCFG 52\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_syscfg13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_syscfg13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_syscfg13`]
module"]
#[doc(alias = "sys_syscfg13")]
pub type SysSyscfg13 = crate::Reg<sys_syscfg13::SysSyscfg13Spec>;
#[doc = "SYS SYSCONSAIF SYSCFG 52"]
pub mod sys_syscfg13;
#[doc = "sys_syscfg14 (rw) register accessor: SYS SYSCONSAIF SYSCFG 56\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_syscfg14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_syscfg14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_syscfg14`]
module"]
#[doc(alias = "sys_syscfg14")]
pub type SysSyscfg14 = crate::Reg<sys_syscfg14::SysSyscfg14Spec>;
#[doc = "SYS SYSCONSAIF SYSCFG 56"]
pub mod sys_syscfg14;
#[doc = "sys_syscfg_noc_bus_oic_qch_clock_stop (rw) register accessor: SYS SYSCONSAIF SYSCFG 60 - 92: NOC Bus OIC QCH Clock Stop Threshold registers.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_syscfg_noc_bus_oic_qch_clock_stop::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_syscfg_noc_bus_oic_qch_clock_stop::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_syscfg_noc_bus_oic_qch_clock_stop`]
module"]
#[doc(alias = "sys_syscfg_noc_bus_oic_qch_clock_stop")]
pub type SysSyscfgNocBusOicQchClockStop =
    crate::Reg<sys_syscfg_noc_bus_oic_qch_clock_stop::SysSyscfgNocBusOicQchClockStopSpec>;
#[doc = "SYS SYSCONSAIF SYSCFG 60 - 92: NOC Bus OIC QCH Clock Stop Threshold registers."]
pub mod sys_syscfg_noc_bus_oic_qch_clock_stop;
#[doc = "sys_syscfg24 (rw) register accessor: SYS SYSCONSAIF SYSCFG 96\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_syscfg24::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_syscfg24::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_syscfg24`]
module"]
#[doc(alias = "sys_syscfg24")]
pub type SysSyscfg24 = crate::Reg<sys_syscfg24::SysSyscfg24Spec>;
#[doc = "SYS SYSCONSAIF SYSCFG 96"]
pub mod sys_syscfg24;
#[doc = "sys_syscfg25 (rw) register accessor: SYS SYSCONSAIF SYSCFG 96\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_syscfg25::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_syscfg25::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_syscfg25`]
module"]
#[doc(alias = "sys_syscfg25")]
pub type SysSyscfg25 = crate::Reg<sys_syscfg25::SysSyscfg25Spec>;
#[doc = "SYS SYSCONSAIF SYSCFG 96"]
pub mod sys_syscfg25;
#[doc = "SYS SYSCONSAIF SYSCFG 104 - 128: Reset Vector registers."]
pub use self::sys_syscfg_reset_vector::SysSyscfgResetVector;
#[doc = r"Cluster"]
#[doc = "SYS SYSCONSAIF SYSCFG 104 - 128: Reset Vector registers."]
pub mod sys_syscfg_reset_vector;
#[doc = "sys_syscfg33 (rw) register accessor: SYS SYSCONSAIF SYSCFG 132\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_syscfg33::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_syscfg33::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_syscfg33`]
module"]
#[doc(alias = "sys_syscfg33")]
pub type SysSyscfg33 = crate::Reg<sys_syscfg33::SysSyscfg33Spec>;
#[doc = "SYS SYSCONSAIF SYSCFG 132"]
pub mod sys_syscfg33;
#[doc = "sys_syscfg34 (rw) register accessor: SYS SYSCONSAIF SYSCFG 136\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_syscfg34::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_syscfg34::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_syscfg34`]
module"]
#[doc(alias = "sys_syscfg34")]
pub type SysSyscfg34 = crate::Reg<sys_syscfg34::SysSyscfg34Spec>;
#[doc = "SYS SYSCONSAIF SYSCFG 136"]
pub mod sys_syscfg34;
#[doc = "sys_syscfg35 (rw) register accessor: SYS SYSCONSAIF SYSCFG 140\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_syscfg35::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_syscfg35::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_syscfg35`]
module"]
#[doc(alias = "sys_syscfg35")]
pub type SysSyscfg35 = crate::Reg<sys_syscfg35::SysSyscfg35Spec>;
#[doc = "SYS SYSCONSAIF SYSCFG 140"]
pub mod sys_syscfg35;
#[doc = "sys_syscfg36 (rw) register accessor: SYS SYSCONSAIF SYSCFG 144\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_syscfg36::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_syscfg36::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_syscfg36`]
module"]
#[doc(alias = "sys_syscfg36")]
pub type SysSyscfg36 = crate::Reg<sys_syscfg36::SysSyscfg36Spec>;
#[doc = "SYS SYSCONSAIF SYSCFG 144"]
pub mod sys_syscfg36;
#[doc = "sys_syscfg37 (r) register accessor: SYS SYSCONSAIF SYSCFG 148\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_syscfg37::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_syscfg37`]
module"]
#[doc(alias = "sys_syscfg37")]
pub type SysSyscfg37 = crate::Reg<sys_syscfg37::SysSyscfg37Spec>;
#[doc = "SYS SYSCONSAIF SYSCFG 148"]
pub mod sys_syscfg37;
#[doc = "sys_syscfg38 (r) register accessor: SYS SYSCONSAIF SYSCFG 152\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_syscfg38::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_syscfg38`]
module"]
#[doc(alias = "sys_syscfg38")]
pub type SysSyscfg38 = crate::Reg<sys_syscfg38::SysSyscfg38Spec>;
#[doc = "SYS SYSCONSAIF SYSCFG 152"]
pub mod sys_syscfg38;
#[doc = "sys_syscfg39 (rw) register accessor: SYS SYSCONSAIF SYSCFG 156\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_syscfg39::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_syscfg39::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys_syscfg39`]
module"]
#[doc(alias = "sys_syscfg39")]
pub type SysSyscfg39 = crate::Reg<sys_syscfg39::SysSyscfg39Spec>;
#[doc = "SYS SYSCONSAIF SYSCFG 156"]
pub mod sys_syscfg39;
