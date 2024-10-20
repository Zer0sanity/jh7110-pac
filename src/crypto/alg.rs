#[repr(C)]
#[doc = "JH7110 Crypto Algorithm registers"]
#[doc(alias = "alg")]
pub struct Alg {
    cr: Cr,
    fifo: Fifo,
}
impl Alg {
    #[doc = "0x00 - JH7110 Crypto Control"]
    #[inline(always)]
    pub const fn cr(&self) -> &Cr {
        &self.cr
    }
    #[doc = "0x04 - JH7110 Crypto Algorithm FIFO"]
    #[inline(always)]
    pub const fn fifo(&self) -> &Fifo {
        &self.fifo
    }
}
#[doc = "cr (rw) register accessor: JH7110 Crypto Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
#[doc(alias = "cr")]
pub type Cr = crate::Reg<cr::CrSpec>;
#[doc = "JH7110 Crypto Control"]
pub mod cr;
#[doc = "fifo (rw) register accessor: JH7110 Crypto Algorithm FIFO\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo`]
module"]
#[doc(alias = "fifo")]
pub type Fifo = crate::Reg<fifo::FifoSpec>;
#[doc = "JH7110 Crypto Algorithm FIFO"]
pub mod fifo;
