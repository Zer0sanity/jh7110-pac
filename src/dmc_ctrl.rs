#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    csr: [Csr; 1024],
    sec: [Sec; 2048],
}
impl RegisterBlock {
    #[doc = "0x00..0x1000 - DDR Memory Control CSR register"]
    #[inline(always)]
    pub const fn csr(&self, n: usize) -> &Csr {
        &self.csr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x1000 - DDR Memory Control CSR register"]
    #[inline(always)]
    pub fn csr_iter(&self) -> impl Iterator<Item = &Csr> {
        self.csr.iter()
    }
    #[doc = "0x1000..0x3000 - DDR Memory Control SEC register"]
    #[inline(always)]
    pub const fn sec(&self, n: usize) -> &Sec {
        &self.sec[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1000..0x3000 - DDR Memory Control SEC register"]
    #[inline(always)]
    pub fn sec_iter(&self) -> impl Iterator<Item = &Sec> {
        self.sec.iter()
    }
}
#[doc = "csr (rw) register accessor: DDR Memory Control CSR register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csr`]
module"]
#[doc(alias = "csr")]
pub type Csr = crate::Reg<csr::CsrSpec>;
#[doc = "DDR Memory Control CSR register"]
pub mod csr;
#[doc = "sec (rw) register accessor: DDR Memory Control SEC register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sec::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sec::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sec`]
module"]
#[doc(alias = "sec")]
pub type Sec = crate::Reg<sec::SecSpec>;
#[doc = "DDR Memory Control SEC register"]
pub mod sec;
