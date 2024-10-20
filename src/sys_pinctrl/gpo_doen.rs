#[repr(C)]
#[doc = "The register value indicates the selected GPIO (Output Enable) OEN index from GPIO OEN list 0-49. See Table 2-41: GPIO OEN List for SYS_IOMUX (on page 97) for more information."]
#[doc(alias = "gpo_doen")]
pub struct GpoDoen {
    gpo_doen0: GpoDoen0,
    gpo_doen1: GpoDoen1,
    gpo_doen2: GpoDoen2,
    gpo_doen3: GpoDoen3,
    gpo_doen4: GpoDoen4,
    gpo_doen5: GpoDoen5,
    gpo_doen6: GpoDoen6,
    gpo_doen7: GpoDoen7,
    gpo_doen8: GpoDoen8,
    gpo_doen9: GpoDoen9,
    gpo_doen10: GpoDoen10,
    gpo_doen11: GpoDoen11,
    gpo_doen12: GpoDoen12,
    gpo_doen13: GpoDoen13,
    gpo_doen14: GpoDoen14,
    gpo_doen15: GpoDoen15,
}
impl GpoDoen {
    #[doc = "0x00 - SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 0-3 DOEN"]
    #[inline(always)]
    pub const fn gpo_doen0(&self) -> &GpoDoen0 {
        &self.gpo_doen0
    }
    #[doc = "0x04 - SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 4-7 DOEN"]
    #[inline(always)]
    pub const fn gpo_doen1(&self) -> &GpoDoen1 {
        &self.gpo_doen1
    }
    #[doc = "0x08 - SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 8-11 DOEN"]
    #[inline(always)]
    pub const fn gpo_doen2(&self) -> &GpoDoen2 {
        &self.gpo_doen2
    }
    #[doc = "0x0c - SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 12-15 DOEN"]
    #[inline(always)]
    pub const fn gpo_doen3(&self) -> &GpoDoen3 {
        &self.gpo_doen3
    }
    #[doc = "0x10 - SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 16-19 DOEN"]
    #[inline(always)]
    pub const fn gpo_doen4(&self) -> &GpoDoen4 {
        &self.gpo_doen4
    }
    #[doc = "0x14 - SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 20-23 DOEN"]
    #[inline(always)]
    pub const fn gpo_doen5(&self) -> &GpoDoen5 {
        &self.gpo_doen5
    }
    #[doc = "0x18 - SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 24-27 DOEN"]
    #[inline(always)]
    pub const fn gpo_doen6(&self) -> &GpoDoen6 {
        &self.gpo_doen6
    }
    #[doc = "0x1c - SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 28-31 DOEN"]
    #[inline(always)]
    pub const fn gpo_doen7(&self) -> &GpoDoen7 {
        &self.gpo_doen7
    }
    #[doc = "0x20 - SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 32-35 DOEN"]
    #[inline(always)]
    pub const fn gpo_doen8(&self) -> &GpoDoen8 {
        &self.gpo_doen8
    }
    #[doc = "0x24 - SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 36-39 DOEN"]
    #[inline(always)]
    pub const fn gpo_doen9(&self) -> &GpoDoen9 {
        &self.gpo_doen9
    }
    #[doc = "0x28 - SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 40-43 DOEN"]
    #[inline(always)]
    pub const fn gpo_doen10(&self) -> &GpoDoen10 {
        &self.gpo_doen10
    }
    #[doc = "0x2c - SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 44-47 DOEN"]
    #[inline(always)]
    pub const fn gpo_doen11(&self) -> &GpoDoen11 {
        &self.gpo_doen11
    }
    #[doc = "0x30 - SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 48-51 DOEN"]
    #[inline(always)]
    pub const fn gpo_doen12(&self) -> &GpoDoen12 {
        &self.gpo_doen12
    }
    #[doc = "0x34 - SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 52-55 DOEN"]
    #[inline(always)]
    pub const fn gpo_doen13(&self) -> &GpoDoen13 {
        &self.gpo_doen13
    }
    #[doc = "0x38 - SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 56-59 DOEN"]
    #[inline(always)]
    pub const fn gpo_doen14(&self) -> &GpoDoen14 {
        &self.gpo_doen14
    }
    #[doc = "0x3c - SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 60-63 DOEN"]
    #[inline(always)]
    pub const fn gpo_doen15(&self) -> &GpoDoen15 {
        &self.gpo_doen15
    }
}
#[doc = "gpo_doen0 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 0-3 DOEN\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_doen0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_doen0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpo_doen0`]
module"]
#[doc(alias = "gpo_doen0")]
pub type GpoDoen0 = crate::Reg<gpo_doen0::GpoDoen0Spec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 0-3 DOEN"]
pub mod gpo_doen0;
#[doc = "gpo_doen1 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 4-7 DOEN\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_doen1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_doen1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpo_doen1`]
module"]
#[doc(alias = "gpo_doen1")]
pub type GpoDoen1 = crate::Reg<gpo_doen1::GpoDoen1Spec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 4-7 DOEN"]
pub mod gpo_doen1;
#[doc = "gpo_doen2 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 8-11 DOEN\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_doen2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_doen2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpo_doen2`]
module"]
#[doc(alias = "gpo_doen2")]
pub type GpoDoen2 = crate::Reg<gpo_doen2::GpoDoen2Spec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 8-11 DOEN"]
pub mod gpo_doen2;
#[doc = "gpo_doen3 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 12-15 DOEN\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_doen3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_doen3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpo_doen3`]
module"]
#[doc(alias = "gpo_doen3")]
pub type GpoDoen3 = crate::Reg<gpo_doen3::GpoDoen3Spec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 12-15 DOEN"]
pub mod gpo_doen3;
#[doc = "gpo_doen4 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 16-19 DOEN\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_doen4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_doen4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpo_doen4`]
module"]
#[doc(alias = "gpo_doen4")]
pub type GpoDoen4 = crate::Reg<gpo_doen4::GpoDoen4Spec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 16-19 DOEN"]
pub mod gpo_doen4;
#[doc = "gpo_doen5 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 20-23 DOEN\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_doen5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_doen5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpo_doen5`]
module"]
#[doc(alias = "gpo_doen5")]
pub type GpoDoen5 = crate::Reg<gpo_doen5::GpoDoen5Spec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 20-23 DOEN"]
pub mod gpo_doen5;
#[doc = "gpo_doen6 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 24-27 DOEN\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_doen6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_doen6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpo_doen6`]
module"]
#[doc(alias = "gpo_doen6")]
pub type GpoDoen6 = crate::Reg<gpo_doen6::GpoDoen6Spec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 24-27 DOEN"]
pub mod gpo_doen6;
#[doc = "gpo_doen7 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 28-31 DOEN\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_doen7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_doen7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpo_doen7`]
module"]
#[doc(alias = "gpo_doen7")]
pub type GpoDoen7 = crate::Reg<gpo_doen7::GpoDoen7Spec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 28-31 DOEN"]
pub mod gpo_doen7;
#[doc = "gpo_doen8 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 32-35 DOEN\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_doen8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_doen8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpo_doen8`]
module"]
#[doc(alias = "gpo_doen8")]
pub type GpoDoen8 = crate::Reg<gpo_doen8::GpoDoen8Spec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 32-35 DOEN"]
pub mod gpo_doen8;
#[doc = "gpo_doen9 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 36-39 DOEN\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_doen9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_doen9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpo_doen9`]
module"]
#[doc(alias = "gpo_doen9")]
pub type GpoDoen9 = crate::Reg<gpo_doen9::GpoDoen9Spec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 36-39 DOEN"]
pub mod gpo_doen9;
#[doc = "gpo_doen10 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 40-43 DOEN\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_doen10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_doen10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpo_doen10`]
module"]
#[doc(alias = "gpo_doen10")]
pub type GpoDoen10 = crate::Reg<gpo_doen10::GpoDoen10Spec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 40-43 DOEN"]
pub mod gpo_doen10;
#[doc = "gpo_doen11 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 44-47 DOEN\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_doen11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_doen11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpo_doen11`]
module"]
#[doc(alias = "gpo_doen11")]
pub type GpoDoen11 = crate::Reg<gpo_doen11::GpoDoen11Spec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 44-47 DOEN"]
pub mod gpo_doen11;
#[doc = "gpo_doen12 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 48-51 DOEN\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_doen12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_doen12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpo_doen12`]
module"]
#[doc(alias = "gpo_doen12")]
pub type GpoDoen12 = crate::Reg<gpo_doen12::GpoDoen12Spec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 48-51 DOEN"]
pub mod gpo_doen12;
#[doc = "gpo_doen13 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 52-55 DOEN\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_doen13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_doen13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpo_doen13`]
module"]
#[doc(alias = "gpo_doen13")]
pub type GpoDoen13 = crate::Reg<gpo_doen13::GpoDoen13Spec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 52-55 DOEN"]
pub mod gpo_doen13;
#[doc = "gpo_doen14 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 56-59 DOEN\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_doen14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_doen14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpo_doen14`]
module"]
#[doc(alias = "gpo_doen14")]
pub type GpoDoen14 = crate::Reg<gpo_doen14::GpoDoen14Spec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 56-59 DOEN"]
pub mod gpo_doen14;
#[doc = "gpo_doen15 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 60-63 DOEN\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_doen15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_doen15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpo_doen15`]
module"]
#[doc(alias = "gpo_doen15")]
pub type GpoDoen15 = crate::Reg<gpo_doen15::GpoDoen15Spec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 60-63 DOEN"]
pub mod gpo_doen15;
