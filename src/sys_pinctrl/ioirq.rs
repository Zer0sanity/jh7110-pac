#[repr(C)]
#[doc = "GPIO Interrupt Request configuration"]
#[doc(alias = "ioirq")]
pub struct Ioirq {
    ioirq0: Ioirq0,
    ioirq1: Ioirq1,
    ioirq2: Ioirq2,
    ioirq3: Ioirq3,
    ioirq4: Ioirq4,
    ioirq5: Ioirq5,
    ioirq6: Ioirq6,
    ioirq7: Ioirq7,
    ioirq8: Ioirq8,
    ioirq9: Ioirq9,
    ioirq10: Ioirq10,
    ioirq11: Ioirq11,
    ioirq12: Ioirq12,
    ioirq13: Ioirq13,
    ioirq14: Ioirq14,
    ioirq15: Ioirq15,
    ioirq16: Ioirq16,
}
impl Ioirq {
    #[doc = "0x00 - Enable IRQ function"]
    #[inline(always)]
    pub const fn ioirq0(&self) -> &Ioirq0 {
        &self.ioirq0
    }
    #[doc = "0x04 - SYS IOMUX CFGSAIF SYSCFG IOIRQ 4: GPIO Interrupt Edge Trigger Selector"]
    #[inline(always)]
    pub const fn ioirq1(&self) -> &Ioirq1 {
        &self.ioirq1
    }
    #[doc = "0x08 - SYS IOMUX CFGSAIF SYSCFG IOIRQ 8: GPIO Interrupt Edge Trigger Selector"]
    #[inline(always)]
    pub const fn ioirq2(&self) -> &Ioirq2 {
        &self.ioirq2
    }
    #[doc = "0x0c - SYS IOMUX CFGSAIF SYSCFG IOIRQ 12: GPIO Interrupt Clear"]
    #[inline(always)]
    pub const fn ioirq3(&self) -> &Ioirq3 {
        &self.ioirq3
    }
    #[doc = "0x10 - SYS IOMUX CFGSAIF SYSCFG IOIRQ 16: GPIO Interrupt Clear"]
    #[inline(always)]
    pub const fn ioirq4(&self) -> &Ioirq4 {
        &self.ioirq4
    }
    #[doc = "0x14 - SYS IOMUX CFGSAIF SYSCFG IOIRQ 20: GPIO Interrupt Both Edge Trigger Selector"]
    #[inline(always)]
    pub const fn ioirq5(&self) -> &Ioirq5 {
        &self.ioirq5
    }
    #[doc = "0x18 - SYS IOMUX CFGSAIF SYSCFG IOIRQ 24: GPIO Interrupt Both Edge Trigger Selector"]
    #[inline(always)]
    pub const fn ioirq6(&self) -> &Ioirq6 {
        &self.ioirq6
    }
    #[doc = "0x1c - SYS IOMUX CFGSAIF SYSCFG IOIRQ 28: GPIO Interrupt Edge Value"]
    #[inline(always)]
    pub const fn ioirq7(&self) -> &Ioirq7 {
        &self.ioirq7
    }
    #[doc = "0x20 - SYS IOMUX CFGSAIF SYSCFG IOIRQ 32: GPIO Interrupt Edge Value"]
    #[inline(always)]
    pub const fn ioirq8(&self) -> &Ioirq8 {
        &self.ioirq8
    }
    #[doc = "0x24 - SYS IOMUX CFGSAIF SYSCFG IOIRQ 36: GPIO Interrupt Edge Mask Selector"]
    #[inline(always)]
    pub const fn ioirq9(&self) -> &Ioirq9 {
        &self.ioirq9
    }
    #[doc = "0x28 - SYS IOMUX CFGSAIF SYSCFG IOIRQ 40: GPIO Interrupt Edge Mask Selector"]
    #[inline(always)]
    pub const fn ioirq10(&self) -> &Ioirq10 {
        &self.ioirq10
    }
    #[doc = "0x2c - SYS IOMUX CFGSAIF SYSCFG IOIRQ 44: GPIO Register Interrupt Status"]
    #[inline(always)]
    pub const fn ioirq11(&self) -> &Ioirq11 {
        &self.ioirq11
    }
    #[doc = "0x30 - SYS IOMUX CFGSAIF SYSCFG IOIRQ 48: GPIO Register Interrupt Status"]
    #[inline(always)]
    pub const fn ioirq12(&self) -> &Ioirq12 {
        &self.ioirq12
    }
    #[doc = "0x34 - SYS IOMUX CFGSAIF SYSCFG IOIRQ 52: GPIO Masked Interrupt Status"]
    #[inline(always)]
    pub const fn ioirq13(&self) -> &Ioirq13 {
        &self.ioirq13
    }
    #[doc = "0x38 - SYS IOMUX CFGSAIF SYSCFG IOIRQ 56: GPIO Masked Interrupt Status"]
    #[inline(always)]
    pub const fn ioirq14(&self) -> &Ioirq14 {
        &self.ioirq14
    }
    #[doc = "0x3c - SYS IOMUX CFGSAIF SYSCFG IOIRQ 60: GPIO Synchronization Status"]
    #[inline(always)]
    pub const fn ioirq15(&self) -> &Ioirq15 {
        &self.ioirq15
    }
    #[doc = "0x40 - SYS IOMUX CFGSAIF SYSCFG IOIRQ 64: GPIO Synchronization Status"]
    #[inline(always)]
    pub const fn ioirq16(&self) -> &Ioirq16 {
        &self.ioirq16
    }
}
#[doc = "ioirq0 (rw) register accessor: Enable IRQ function\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioirq0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioirq0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ioirq0`]
module"]
#[doc(alias = "ioirq0")]
pub type Ioirq0 = crate::Reg<ioirq0::Ioirq0Spec>;
#[doc = "Enable IRQ function"]
pub mod ioirq0;
#[doc = "ioirq1 (rw) register accessor: SYS IOMUX CFGSAIF SYSCFG IOIRQ 4: GPIO Interrupt Edge Trigger Selector\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioirq1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioirq1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ioirq1`]
module"]
#[doc(alias = "ioirq1")]
pub type Ioirq1 = crate::Reg<ioirq1::Ioirq1Spec>;
#[doc = "SYS IOMUX CFGSAIF SYSCFG IOIRQ 4: GPIO Interrupt Edge Trigger Selector"]
pub mod ioirq1;
#[doc = "ioirq2 (rw) register accessor: SYS IOMUX CFGSAIF SYSCFG IOIRQ 8: GPIO Interrupt Edge Trigger Selector\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioirq2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioirq2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ioirq2`]
module"]
#[doc(alias = "ioirq2")]
pub type Ioirq2 = crate::Reg<ioirq2::Ioirq2Spec>;
#[doc = "SYS IOMUX CFGSAIF SYSCFG IOIRQ 8: GPIO Interrupt Edge Trigger Selector"]
pub mod ioirq2;
#[doc = "ioirq3 (rw) register accessor: SYS IOMUX CFGSAIF SYSCFG IOIRQ 12: GPIO Interrupt Clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioirq3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioirq3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ioirq3`]
module"]
#[doc(alias = "ioirq3")]
pub type Ioirq3 = crate::Reg<ioirq3::Ioirq3Spec>;
#[doc = "SYS IOMUX CFGSAIF SYSCFG IOIRQ 12: GPIO Interrupt Clear"]
pub mod ioirq3;
#[doc = "ioirq4 (rw) register accessor: SYS IOMUX CFGSAIF SYSCFG IOIRQ 16: GPIO Interrupt Clear\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioirq4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioirq4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ioirq4`]
module"]
#[doc(alias = "ioirq4")]
pub type Ioirq4 = crate::Reg<ioirq4::Ioirq4Spec>;
#[doc = "SYS IOMUX CFGSAIF SYSCFG IOIRQ 16: GPIO Interrupt Clear"]
pub mod ioirq4;
#[doc = "ioirq5 (rw) register accessor: SYS IOMUX CFGSAIF SYSCFG IOIRQ 20: GPIO Interrupt Both Edge Trigger Selector\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioirq5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioirq5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ioirq5`]
module"]
#[doc(alias = "ioirq5")]
pub type Ioirq5 = crate::Reg<ioirq5::Ioirq5Spec>;
#[doc = "SYS IOMUX CFGSAIF SYSCFG IOIRQ 20: GPIO Interrupt Both Edge Trigger Selector"]
pub mod ioirq5;
#[doc = "ioirq6 (rw) register accessor: SYS IOMUX CFGSAIF SYSCFG IOIRQ 24: GPIO Interrupt Both Edge Trigger Selector\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioirq6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioirq6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ioirq6`]
module"]
#[doc(alias = "ioirq6")]
pub type Ioirq6 = crate::Reg<ioirq6::Ioirq6Spec>;
#[doc = "SYS IOMUX CFGSAIF SYSCFG IOIRQ 24: GPIO Interrupt Both Edge Trigger Selector"]
pub mod ioirq6;
#[doc = "ioirq7 (rw) register accessor: SYS IOMUX CFGSAIF SYSCFG IOIRQ 28: GPIO Interrupt Edge Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioirq7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioirq7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ioirq7`]
module"]
#[doc(alias = "ioirq7")]
pub type Ioirq7 = crate::Reg<ioirq7::Ioirq7Spec>;
#[doc = "SYS IOMUX CFGSAIF SYSCFG IOIRQ 28: GPIO Interrupt Edge Value"]
pub mod ioirq7;
#[doc = "ioirq8 (rw) register accessor: SYS IOMUX CFGSAIF SYSCFG IOIRQ 32: GPIO Interrupt Edge Value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioirq8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioirq8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ioirq8`]
module"]
#[doc(alias = "ioirq8")]
pub type Ioirq8 = crate::Reg<ioirq8::Ioirq8Spec>;
#[doc = "SYS IOMUX CFGSAIF SYSCFG IOIRQ 32: GPIO Interrupt Edge Value"]
pub mod ioirq8;
#[doc = "ioirq9 (rw) register accessor: SYS IOMUX CFGSAIF SYSCFG IOIRQ 36: GPIO Interrupt Edge Mask Selector\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioirq9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioirq9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ioirq9`]
module"]
#[doc(alias = "ioirq9")]
pub type Ioirq9 = crate::Reg<ioirq9::Ioirq9Spec>;
#[doc = "SYS IOMUX CFGSAIF SYSCFG IOIRQ 36: GPIO Interrupt Edge Mask Selector"]
pub mod ioirq9;
#[doc = "ioirq10 (rw) register accessor: SYS IOMUX CFGSAIF SYSCFG IOIRQ 40: GPIO Interrupt Edge Mask Selector\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioirq10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioirq10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ioirq10`]
module"]
#[doc(alias = "ioirq10")]
pub type Ioirq10 = crate::Reg<ioirq10::Ioirq10Spec>;
#[doc = "SYS IOMUX CFGSAIF SYSCFG IOIRQ 40: GPIO Interrupt Edge Mask Selector"]
pub mod ioirq10;
#[doc = "ioirq11 (r) register accessor: SYS IOMUX CFGSAIF SYSCFG IOIRQ 44: GPIO Register Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioirq11::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ioirq11`]
module"]
#[doc(alias = "ioirq11")]
pub type Ioirq11 = crate::Reg<ioirq11::Ioirq11Spec>;
#[doc = "SYS IOMUX CFGSAIF SYSCFG IOIRQ 44: GPIO Register Interrupt Status"]
pub mod ioirq11;
#[doc = "ioirq12 (r) register accessor: SYS IOMUX CFGSAIF SYSCFG IOIRQ 48: GPIO Register Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioirq12::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ioirq12`]
module"]
#[doc(alias = "ioirq12")]
pub type Ioirq12 = crate::Reg<ioirq12::Ioirq12Spec>;
#[doc = "SYS IOMUX CFGSAIF SYSCFG IOIRQ 48: GPIO Register Interrupt Status"]
pub mod ioirq12;
#[doc = "ioirq13 (r) register accessor: SYS IOMUX CFGSAIF SYSCFG IOIRQ 52: GPIO Masked Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioirq13::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ioirq13`]
module"]
#[doc(alias = "ioirq13")]
pub type Ioirq13 = crate::Reg<ioirq13::Ioirq13Spec>;
#[doc = "SYS IOMUX CFGSAIF SYSCFG IOIRQ 52: GPIO Masked Interrupt Status"]
pub mod ioirq13;
#[doc = "ioirq14 (r) register accessor: SYS IOMUX CFGSAIF SYSCFG IOIRQ 56: GPIO Masked Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioirq14::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ioirq14`]
module"]
#[doc(alias = "ioirq14")]
pub type Ioirq14 = crate::Reg<ioirq14::Ioirq14Spec>;
#[doc = "SYS IOMUX CFGSAIF SYSCFG IOIRQ 56: GPIO Masked Interrupt Status"]
pub mod ioirq14;
#[doc = "ioirq15 (r) register accessor: SYS IOMUX CFGSAIF SYSCFG IOIRQ 60: GPIO Synchronization Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioirq15::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ioirq15`]
module"]
#[doc(alias = "ioirq15")]
pub type Ioirq15 = crate::Reg<ioirq15::Ioirq15Spec>;
#[doc = "SYS IOMUX CFGSAIF SYSCFG IOIRQ 60: GPIO Synchronization Status"]
pub mod ioirq15;
#[doc = "ioirq16 (r) register accessor: SYS IOMUX CFGSAIF SYSCFG IOIRQ 64: GPIO Synchronization Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioirq16::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ioirq16`]
module"]
#[doc(alias = "ioirq16")]
pub type Ioirq16 = crate::Reg<ioirq16::Ioirq16Spec>;
#[doc = "SYS IOMUX CFGSAIF SYSCFG IOIRQ 64: GPIO Synchronization Status"]
pub mod ioirq16;
