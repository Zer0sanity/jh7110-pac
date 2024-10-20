#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    csr: [Csr; 2048],
    base: [Base; 2048],
    ac_base: [AcBase; 2048],
}
impl RegisterBlock {
    #[doc = "0x00..0x2000 - DDR Memory Control PHY CSR register"]
    #[inline(always)]
    pub const fn csr(&self, n: usize) -> &Csr {
        &self.csr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x2000 - DDR Memory Control PHY CSR register"]
    #[inline(always)]
    pub fn csr_iter(&self) -> impl Iterator<Item = &Csr> {
        self.csr.iter()
    }
    #[doc = "0x2000..0x4000 - DDR Memory Control PHY Base register"]
    #[inline(always)]
    pub const fn base(&self, n: usize) -> &Base {
        &self.base[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x2000..0x4000 - DDR Memory Control PHY Base register"]
    #[inline(always)]
    pub fn base_iter(&self) -> impl Iterator<Item = &Base> {
        self.base.iter()
    }
    #[doc = "0x4000..0x6000 - DDR Memory Control PHY AC Base register"]
    #[inline(always)]
    pub const fn ac_base(&self, n: usize) -> &AcBase {
        &self.ac_base[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x4000..0x6000 - DDR Memory Control PHY AC Base register"]
    #[inline(always)]
    pub fn ac_base_iter(&self) -> impl Iterator<Item = &AcBase> {
        self.ac_base.iter()
    }
}
#[doc = "csr (rw) register accessor: DDR Memory Control PHY CSR register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csr`]
module"]
#[doc(alias = "csr")]
pub type Csr = crate::Reg<csr::CsrSpec>;
#[doc = "DDR Memory Control PHY CSR register"]
pub mod csr;
#[doc = "base (rw) register accessor: DDR Memory Control PHY Base register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`base::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`base::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@base`]
module"]
#[doc(alias = "base")]
pub type Base = crate::Reg<base::BaseSpec>;
#[doc = "DDR Memory Control PHY Base register"]
pub mod base;
#[doc = "ac_base (rw) register accessor: DDR Memory Control PHY AC Base register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ac_base::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ac_base::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ac_base`]
module"]
#[doc(alias = "ac_base")]
pub type AcBase = crate::Reg<ac_base::AcBaseSpec>;
#[doc = "DDR Memory Control PHY AC Base register"]
pub mod ac_base;
