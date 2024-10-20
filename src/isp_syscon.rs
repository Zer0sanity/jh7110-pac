#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    isp_syscfg0: IspSyscfg0,
    isp_syscfg1: IspSyscfg1,
    isp_syscfg2: IspSyscfg2,
    isp_syscfg3: IspSyscfg3,
    isp_syscfg4: IspSyscfg4,
    isp_syscfg5: IspSyscfg5,
    isp_syscfg6: IspSyscfg6,
    isp_syscfg7: IspSyscfg7,
    isp_syscfg8: IspSyscfg8,
    isp_syscfg9: IspSyscfg9,
    isp_syscfg10: IspSyscfg10,
}
impl RegisterBlock {
    #[doc = "0x00 - ISP SYSCFG 0: isp_sysconsaif_syscfg_0"]
    #[inline(always)]
    pub const fn isp_syscfg0(&self) -> &IspSyscfg0 {
        &self.isp_syscfg0
    }
    #[doc = "0x04 - ISP SYSCFG 1: isp_sysconsaif_syscfg_4"]
    #[inline(always)]
    pub const fn isp_syscfg1(&self) -> &IspSyscfg1 {
        &self.isp_syscfg1
    }
    #[doc = "0x08 - ISP SYSCFG 2: isp_sysconsaif_syscfg_8"]
    #[inline(always)]
    pub const fn isp_syscfg2(&self) -> &IspSyscfg2 {
        &self.isp_syscfg2
    }
    #[doc = "0x0c - ISP SYSCFG 3: isp_sysconsaif_syscfg_12"]
    #[inline(always)]
    pub const fn isp_syscfg3(&self) -> &IspSyscfg3 {
        &self.isp_syscfg3
    }
    #[doc = "0x10 - ISP SYSCFG 4: isp_sysconsaif_syscfg_16"]
    #[inline(always)]
    pub const fn isp_syscfg4(&self) -> &IspSyscfg4 {
        &self.isp_syscfg4
    }
    #[doc = "0x14 - ISP SYSCFG 5: isp_sysconsaif_syscfg_20"]
    #[inline(always)]
    pub const fn isp_syscfg5(&self) -> &IspSyscfg5 {
        &self.isp_syscfg5
    }
    #[doc = "0x18 - ISP SYSCFG 6: isp_sysconsaif_syscfg_24"]
    #[inline(always)]
    pub const fn isp_syscfg6(&self) -> &IspSyscfg6 {
        &self.isp_syscfg6
    }
    #[doc = "0x1c - ISP SYSCFG 7: isp_sysconsaif_syscfg_28"]
    #[inline(always)]
    pub const fn isp_syscfg7(&self) -> &IspSyscfg7 {
        &self.isp_syscfg7
    }
    #[doc = "0x20 - ISP SYSCFG 8: isp_sysconsaif_syscfg_32"]
    #[inline(always)]
    pub const fn isp_syscfg8(&self) -> &IspSyscfg8 {
        &self.isp_syscfg8
    }
    #[doc = "0x24 - ISP SYSCFG 9: isp_sysconsaif_syscfg_36"]
    #[inline(always)]
    pub const fn isp_syscfg9(&self) -> &IspSyscfg9 {
        &self.isp_syscfg9
    }
    #[doc = "0x28 - ISP SYSCFG 10: isp_sysconsaif_syscfg_40"]
    #[inline(always)]
    pub const fn isp_syscfg10(&self) -> &IspSyscfg10 {
        &self.isp_syscfg10
    }
}
#[doc = "isp_syscfg0 (rw) register accessor: ISP SYSCFG 0: isp_sysconsaif_syscfg_0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isp_syscfg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isp_syscfg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isp_syscfg0`]
module"]
#[doc(alias = "isp_syscfg0")]
pub type IspSyscfg0 = crate::Reg<isp_syscfg0::IspSyscfg0Spec>;
#[doc = "ISP SYSCFG 0: isp_sysconsaif_syscfg_0"]
pub mod isp_syscfg0;
#[doc = "isp_syscfg1 (rw) register accessor: ISP SYSCFG 1: isp_sysconsaif_syscfg_4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isp_syscfg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isp_syscfg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isp_syscfg1`]
module"]
#[doc(alias = "isp_syscfg1")]
pub type IspSyscfg1 = crate::Reg<isp_syscfg1::IspSyscfg1Spec>;
#[doc = "ISP SYSCFG 1: isp_sysconsaif_syscfg_4"]
pub mod isp_syscfg1;
#[doc = "isp_syscfg2 (rw) register accessor: ISP SYSCFG 2: isp_sysconsaif_syscfg_8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isp_syscfg2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isp_syscfg2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isp_syscfg2`]
module"]
#[doc(alias = "isp_syscfg2")]
pub type IspSyscfg2 = crate::Reg<isp_syscfg2::IspSyscfg2Spec>;
#[doc = "ISP SYSCFG 2: isp_sysconsaif_syscfg_8"]
pub mod isp_syscfg2;
#[doc = "isp_syscfg3 (rw) register accessor: ISP SYSCFG 3: isp_sysconsaif_syscfg_12\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isp_syscfg3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isp_syscfg3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isp_syscfg3`]
module"]
#[doc(alias = "isp_syscfg3")]
pub type IspSyscfg3 = crate::Reg<isp_syscfg3::IspSyscfg3Spec>;
#[doc = "ISP SYSCFG 3: isp_sysconsaif_syscfg_12"]
pub mod isp_syscfg3;
#[doc = "isp_syscfg4 (rw) register accessor: ISP SYSCFG 4: isp_sysconsaif_syscfg_16\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isp_syscfg4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isp_syscfg4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isp_syscfg4`]
module"]
#[doc(alias = "isp_syscfg4")]
pub type IspSyscfg4 = crate::Reg<isp_syscfg4::IspSyscfg4Spec>;
#[doc = "ISP SYSCFG 4: isp_sysconsaif_syscfg_16"]
pub mod isp_syscfg4;
#[doc = "isp_syscfg5 (rw) register accessor: ISP SYSCFG 5: isp_sysconsaif_syscfg_20\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isp_syscfg5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isp_syscfg5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isp_syscfg5`]
module"]
#[doc(alias = "isp_syscfg5")]
pub type IspSyscfg5 = crate::Reg<isp_syscfg5::IspSyscfg5Spec>;
#[doc = "ISP SYSCFG 5: isp_sysconsaif_syscfg_20"]
pub mod isp_syscfg5;
#[doc = "isp_syscfg6 (rw) register accessor: ISP SYSCFG 6: isp_sysconsaif_syscfg_24\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isp_syscfg6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isp_syscfg6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isp_syscfg6`]
module"]
#[doc(alias = "isp_syscfg6")]
pub type IspSyscfg6 = crate::Reg<isp_syscfg6::IspSyscfg6Spec>;
#[doc = "ISP SYSCFG 6: isp_sysconsaif_syscfg_24"]
pub mod isp_syscfg6;
#[doc = "isp_syscfg7 (rw) register accessor: ISP SYSCFG 7: isp_sysconsaif_syscfg_28\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isp_syscfg7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isp_syscfg7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isp_syscfg7`]
module"]
#[doc(alias = "isp_syscfg7")]
pub type IspSyscfg7 = crate::Reg<isp_syscfg7::IspSyscfg7Spec>;
#[doc = "ISP SYSCFG 7: isp_sysconsaif_syscfg_28"]
pub mod isp_syscfg7;
#[doc = "isp_syscfg8 (rw) register accessor: ISP SYSCFG 8: isp_sysconsaif_syscfg_32\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isp_syscfg8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isp_syscfg8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isp_syscfg8`]
module"]
#[doc(alias = "isp_syscfg8")]
pub type IspSyscfg8 = crate::Reg<isp_syscfg8::IspSyscfg8Spec>;
#[doc = "ISP SYSCFG 8: isp_sysconsaif_syscfg_32"]
pub mod isp_syscfg8;
#[doc = "isp_syscfg9 (rw) register accessor: ISP SYSCFG 9: isp_sysconsaif_syscfg_36\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isp_syscfg9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isp_syscfg9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isp_syscfg9`]
module"]
#[doc(alias = "isp_syscfg9")]
pub type IspSyscfg9 = crate::Reg<isp_syscfg9::IspSyscfg9Spec>;
#[doc = "ISP SYSCFG 9: isp_sysconsaif_syscfg_36"]
pub mod isp_syscfg9;
#[doc = "isp_syscfg10 (rw) register accessor: ISP SYSCFG 10: isp_sysconsaif_syscfg_40\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isp_syscfg10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isp_syscfg10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isp_syscfg10`]
module"]
#[doc(alias = "isp_syscfg10")]
pub type IspSyscfg10 = crate::Reg<isp_syscfg10::IspSyscfg10Spec>;
#[doc = "ISP SYSCFG 10: isp_sysconsaif_syscfg_40"]
pub mod isp_syscfg10;
