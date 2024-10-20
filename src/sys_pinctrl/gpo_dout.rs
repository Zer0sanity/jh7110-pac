#[repr(C)]
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO DOUT - The register value indicates the selected GPIO (Digital Output) DOUT index from GPIO DOUT list 0-49. See Table 2-41: GPIO DOUT List for SYS_IOMUX (on page 97) for more information."]
#[doc(alias = "gpo_dout")]
pub struct GpoDout {
    gpo_dout0: GpoDout0,
    gpo_dout1: GpoDout1,
    gpo_dout2: GpoDout2,
    gpo_dout3: GpoDout3,
    gpo_dout4: GpoDout4,
    gpo_dout5: GpoDout5,
    gpo_dout6: GpoDout6,
    gpo_dout7: GpoDout7,
    gpo_dout8: GpoDout8,
    gpo_dout9: GpoDout9,
    gpo_dout10: GpoDout10,
    gpo_dout11: GpoDout11,
    gpo_dout12: GpoDout12,
    gpo_dout13: GpoDout13,
    gpo_dout14: GpoDout14,
    gpo_dout15: GpoDout15,
}
impl GpoDout {
    #[doc = "0x00 - SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 0-3 DOUT"]
    #[inline(always)]
    pub const fn gpo_dout0(&self) -> &GpoDout0 {
        &self.gpo_dout0
    }
    #[doc = "0x04 - SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 4-7 DOUT"]
    #[inline(always)]
    pub const fn gpo_dout1(&self) -> &GpoDout1 {
        &self.gpo_dout1
    }
    #[doc = "0x08 - SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 8-11 DOUT"]
    #[inline(always)]
    pub const fn gpo_dout2(&self) -> &GpoDout2 {
        &self.gpo_dout2
    }
    #[doc = "0x0c - SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 12-15 DOUT"]
    #[inline(always)]
    pub const fn gpo_dout3(&self) -> &GpoDout3 {
        &self.gpo_dout3
    }
    #[doc = "0x10 - SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 16-19 DOUT"]
    #[inline(always)]
    pub const fn gpo_dout4(&self) -> &GpoDout4 {
        &self.gpo_dout4
    }
    #[doc = "0x14 - SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 20-23 DOUT"]
    #[inline(always)]
    pub const fn gpo_dout5(&self) -> &GpoDout5 {
        &self.gpo_dout5
    }
    #[doc = "0x18 - SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 24-27 DOUT"]
    #[inline(always)]
    pub const fn gpo_dout6(&self) -> &GpoDout6 {
        &self.gpo_dout6
    }
    #[doc = "0x1c - SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 28-31 DOUT"]
    #[inline(always)]
    pub const fn gpo_dout7(&self) -> &GpoDout7 {
        &self.gpo_dout7
    }
    #[doc = "0x20 - SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 32-35 DOUT"]
    #[inline(always)]
    pub const fn gpo_dout8(&self) -> &GpoDout8 {
        &self.gpo_dout8
    }
    #[doc = "0x24 - SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 36-39 DOUT"]
    #[inline(always)]
    pub const fn gpo_dout9(&self) -> &GpoDout9 {
        &self.gpo_dout9
    }
    #[doc = "0x28 - SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 40-43 DOUT"]
    #[inline(always)]
    pub const fn gpo_dout10(&self) -> &GpoDout10 {
        &self.gpo_dout10
    }
    #[doc = "0x2c - SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 44-47 DOUT"]
    #[inline(always)]
    pub const fn gpo_dout11(&self) -> &GpoDout11 {
        &self.gpo_dout11
    }
    #[doc = "0x30 - SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 48-51 DOUT"]
    #[inline(always)]
    pub const fn gpo_dout12(&self) -> &GpoDout12 {
        &self.gpo_dout12
    }
    #[doc = "0x34 - SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 52-55 DOUT"]
    #[inline(always)]
    pub const fn gpo_dout13(&self) -> &GpoDout13 {
        &self.gpo_dout13
    }
    #[doc = "0x38 - SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 56-59 DOUT"]
    #[inline(always)]
    pub const fn gpo_dout14(&self) -> &GpoDout14 {
        &self.gpo_dout14
    }
    #[doc = "0x3c - SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 60-63 DOUT"]
    #[inline(always)]
    pub const fn gpo_dout15(&self) -> &GpoDout15 {
        &self.gpo_dout15
    }
}
#[doc = "gpo_dout0 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 0-3 DOUT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_dout0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_dout0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpo_dout0`]
module"]
#[doc(alias = "gpo_dout0")]
pub type GpoDout0 = crate::Reg<gpo_dout0::GpoDout0Spec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 0-3 DOUT"]
pub mod gpo_dout0;
#[doc = "gpo_dout1 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 4-7 DOUT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_dout1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_dout1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpo_dout1`]
module"]
#[doc(alias = "gpo_dout1")]
pub type GpoDout1 = crate::Reg<gpo_dout1::GpoDout1Spec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 4-7 DOUT"]
pub mod gpo_dout1;
#[doc = "gpo_dout2 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 8-11 DOUT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_dout2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_dout2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpo_dout2`]
module"]
#[doc(alias = "gpo_dout2")]
pub type GpoDout2 = crate::Reg<gpo_dout2::GpoDout2Spec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 8-11 DOUT"]
pub mod gpo_dout2;
#[doc = "gpo_dout3 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 12-15 DOUT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_dout3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_dout3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpo_dout3`]
module"]
#[doc(alias = "gpo_dout3")]
pub type GpoDout3 = crate::Reg<gpo_dout3::GpoDout3Spec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 12-15 DOUT"]
pub mod gpo_dout3;
#[doc = "gpo_dout4 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 16-19 DOUT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_dout4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_dout4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpo_dout4`]
module"]
#[doc(alias = "gpo_dout4")]
pub type GpoDout4 = crate::Reg<gpo_dout4::GpoDout4Spec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 16-19 DOUT"]
pub mod gpo_dout4;
#[doc = "gpo_dout5 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 20-23 DOUT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_dout5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_dout5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpo_dout5`]
module"]
#[doc(alias = "gpo_dout5")]
pub type GpoDout5 = crate::Reg<gpo_dout5::GpoDout5Spec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 20-23 DOUT"]
pub mod gpo_dout5;
#[doc = "gpo_dout6 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 24-27 DOUT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_dout6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_dout6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpo_dout6`]
module"]
#[doc(alias = "gpo_dout6")]
pub type GpoDout6 = crate::Reg<gpo_dout6::GpoDout6Spec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 24-27 DOUT"]
pub mod gpo_dout6;
#[doc = "gpo_dout7 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 28-31 DOUT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_dout7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_dout7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpo_dout7`]
module"]
#[doc(alias = "gpo_dout7")]
pub type GpoDout7 = crate::Reg<gpo_dout7::GpoDout7Spec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 28-31 DOUT"]
pub mod gpo_dout7;
#[doc = "gpo_dout8 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 32-35 DOUT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_dout8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_dout8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpo_dout8`]
module"]
#[doc(alias = "gpo_dout8")]
pub type GpoDout8 = crate::Reg<gpo_dout8::GpoDout8Spec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 32-35 DOUT"]
pub mod gpo_dout8;
#[doc = "gpo_dout9 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 36-39 DOUT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_dout9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_dout9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpo_dout9`]
module"]
#[doc(alias = "gpo_dout9")]
pub type GpoDout9 = crate::Reg<gpo_dout9::GpoDout9Spec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 36-39 DOUT"]
pub mod gpo_dout9;
#[doc = "gpo_dout10 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 40-43 DOUT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_dout10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_dout10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpo_dout10`]
module"]
#[doc(alias = "gpo_dout10")]
pub type GpoDout10 = crate::Reg<gpo_dout10::GpoDout10Spec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 40-43 DOUT"]
pub mod gpo_dout10;
#[doc = "gpo_dout11 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 44-47 DOUT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_dout11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_dout11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpo_dout11`]
module"]
#[doc(alias = "gpo_dout11")]
pub type GpoDout11 = crate::Reg<gpo_dout11::GpoDout11Spec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 44-47 DOUT"]
pub mod gpo_dout11;
#[doc = "gpo_dout12 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 48-51 DOUT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_dout12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_dout12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpo_dout12`]
module"]
#[doc(alias = "gpo_dout12")]
pub type GpoDout12 = crate::Reg<gpo_dout12::GpoDout12Spec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 48-51 DOUT"]
pub mod gpo_dout12;
#[doc = "gpo_dout13 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 52-55 DOUT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_dout13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_dout13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpo_dout13`]
module"]
#[doc(alias = "gpo_dout13")]
pub type GpoDout13 = crate::Reg<gpo_dout13::GpoDout13Spec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 52-55 DOUT"]
pub mod gpo_dout13;
#[doc = "gpo_dout14 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 56-59 DOUT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_dout14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_dout14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpo_dout14`]
module"]
#[doc(alias = "gpo_dout14")]
pub type GpoDout14 = crate::Reg<gpo_dout14::GpoDout14Spec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 56-59 DOUT"]
pub mod gpo_dout14;
#[doc = "gpo_dout15 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 60-63 DOUT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpo_dout15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpo_dout15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpo_dout15`]
module"]
#[doc(alias = "gpo_dout15")]
pub type GpoDout15 = crate::Reg<gpo_dout15::GpoDout15Spec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO 60-63 DOUT"]
pub mod gpo_dout15;
