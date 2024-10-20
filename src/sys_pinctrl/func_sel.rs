#[repr(C)]
#[doc = "Registers used to configure the function selector of the system signal indicated by the register name."]
#[doc(alias = "func_sel")]
pub struct FuncSel {
    func_sel0: FuncSel0,
    func_sel1: FuncSel1,
    func_sel2: FuncSel2,
    func_sel3: FuncSel3,
    func_sel4: FuncSel4,
    func_sel5: FuncSel5,
    func_sel6: FuncSel6,
}
impl FuncSel {
    #[doc = "0x00 - SYS IOMUX CFG SAIF SYSCFG Function Selector 0"]
    #[inline(always)]
    pub const fn func_sel0(&self) -> &FuncSel0 {
        &self.func_sel0
    }
    #[doc = "0x04 - SYS IOMUX CFG SAIF SYSCFG Function Selector 4"]
    #[inline(always)]
    pub const fn func_sel1(&self) -> &FuncSel1 {
        &self.func_sel1
    }
    #[doc = "0x08 - SYS IOMUX CFG SAIF SYSCFG Function Selector 8"]
    #[inline(always)]
    pub const fn func_sel2(&self) -> &FuncSel2 {
        &self.func_sel2
    }
    #[doc = "0x0c - SYS IOMUX CFG SAIF SYSCFG Function Selector 12"]
    #[inline(always)]
    pub const fn func_sel3(&self) -> &FuncSel3 {
        &self.func_sel3
    }
    #[doc = "0x10 - SYS IOMUX CFG SAIF SYSCFG Function Selector 16"]
    #[inline(always)]
    pub const fn func_sel4(&self) -> &FuncSel4 {
        &self.func_sel4
    }
    #[doc = "0x14 - SYS IOMUX CFG SAIF SYSCFG Function Selector 20"]
    #[inline(always)]
    pub const fn func_sel5(&self) -> &FuncSel5 {
        &self.func_sel5
    }
    #[doc = "0x18 - SYS IOMUX CFG SAIF SYSCFG Function Selector 24"]
    #[inline(always)]
    pub const fn func_sel6(&self) -> &FuncSel6 {
        &self.func_sel6
    }
}
#[doc = "func_sel0 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG Function Selector 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func_sel0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func_sel0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func_sel0`]
module"]
#[doc(alias = "func_sel0")]
pub type FuncSel0 = crate::Reg<func_sel0::FuncSel0Spec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG Function Selector 0"]
pub mod func_sel0;
#[doc = "func_sel1 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG Function Selector 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func_sel1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func_sel1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func_sel1`]
module"]
#[doc(alias = "func_sel1")]
pub type FuncSel1 = crate::Reg<func_sel1::FuncSel1Spec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG Function Selector 4"]
pub mod func_sel1;
#[doc = "func_sel2 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG Function Selector 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func_sel2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func_sel2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func_sel2`]
module"]
#[doc(alias = "func_sel2")]
pub type FuncSel2 = crate::Reg<func_sel2::FuncSel2Spec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG Function Selector 8"]
pub mod func_sel2;
#[doc = "func_sel3 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG Function Selector 12\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func_sel3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func_sel3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func_sel3`]
module"]
#[doc(alias = "func_sel3")]
pub type FuncSel3 = crate::Reg<func_sel3::FuncSel3Spec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG Function Selector 12"]
pub mod func_sel3;
#[doc = "func_sel4 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG Function Selector 16\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func_sel4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func_sel4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func_sel4`]
module"]
#[doc(alias = "func_sel4")]
pub type FuncSel4 = crate::Reg<func_sel4::FuncSel4Spec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG Function Selector 16"]
pub mod func_sel4;
#[doc = "func_sel5 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG Function Selector 20\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func_sel5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func_sel5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func_sel5`]
module"]
#[doc(alias = "func_sel5")]
pub type FuncSel5 = crate::Reg<func_sel5::FuncSel5Spec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG Function Selector 20"]
pub mod func_sel5;
#[doc = "func_sel6 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG Function Selector 24\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`func_sel6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`func_sel6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@func_sel6`]
module"]
#[doc(alias = "func_sel6")]
pub type FuncSel6 = crate::Reg<func_sel6::FuncSel6Spec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG Function Selector 24"]
pub mod func_sel6;
