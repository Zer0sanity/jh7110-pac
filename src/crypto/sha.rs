#[repr(C)]
#[doc = "JH7110 Crypto SHA registers"]
#[doc(alias = "sha")]
pub struct Sha {
    csr: Csr,
    wdr: Wdr,
    rdr: Rdr,
    wsr: Wsr,
    wlen: [Wlen; 4],
    wkr: Wkr,
    klen: Klen,
}
impl Sha {
    #[doc = "0x00 - JH7110 Crypto SHA CSR"]
    #[inline(always)]
    pub const fn csr(&self) -> &Csr {
        &self.csr
    }
    #[doc = "0x04 - JH7110 Crypto SHA WDR"]
    #[inline(always)]
    pub const fn wdr(&self) -> &Wdr {
        &self.wdr
    }
    #[doc = "0x08 - JH7110 Crypto SHA RDR"]
    #[inline(always)]
    pub const fn rdr(&self) -> &Rdr {
        &self.rdr
    }
    #[doc = "0x0c - JH7110 Crypto SHA WSR"]
    #[inline(always)]
    pub const fn wsr(&self) -> &Wsr {
        &self.wsr
    }
    #[doc = "0x10..0x20 - JH7110 Crypto SHA WLEN"]
    #[inline(always)]
    pub const fn wlen(&self, n: usize) -> &Wlen {
        &self.wlen[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x10..0x20 - JH7110 Crypto SHA WLEN"]
    #[inline(always)]
    pub fn wlen_iter(&self) -> impl Iterator<Item = &Wlen> {
        self.wlen.iter()
    }
    #[doc = "0x10 - JH7110 Crypto SHA WLEN"]
    #[inline(always)]
    pub const fn wlen3(&self) -> &Wlen {
        self.wlen(0)
    }
    #[doc = "0x14 - JH7110 Crypto SHA WLEN"]
    #[inline(always)]
    pub const fn wlen2(&self) -> &Wlen {
        self.wlen(1)
    }
    #[doc = "0x18 - JH7110 Crypto SHA WLEN"]
    #[inline(always)]
    pub const fn wlen1(&self) -> &Wlen {
        self.wlen(2)
    }
    #[doc = "0x1c - JH7110 Crypto SHA WLEN"]
    #[inline(always)]
    pub const fn wlen0(&self) -> &Wlen {
        self.wlen(3)
    }
    #[doc = "0x20 - JH7110 Crypto SHA WKR"]
    #[inline(always)]
    pub const fn wkr(&self) -> &Wkr {
        &self.wkr
    }
    #[doc = "0x24 - JH7110 Crypto SHA KLEN"]
    #[inline(always)]
    pub const fn klen(&self) -> &Klen {
        &self.klen
    }
}
#[doc = "csr (rw) register accessor: JH7110 Crypto SHA CSR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csr`]
module"]
#[doc(alias = "csr")]
pub type Csr = crate::Reg<csr::CsrSpec>;
#[doc = "JH7110 Crypto SHA CSR"]
pub mod csr;
#[doc = "wdr (rw) register accessor: JH7110 Crypto SHA WDR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wdr`]
module"]
#[doc(alias = "wdr")]
pub type Wdr = crate::Reg<wdr::WdrSpec>;
#[doc = "JH7110 Crypto SHA WDR"]
pub mod wdr;
#[doc = "rdr (rw) register accessor: JH7110 Crypto SHA RDR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rdr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rdr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rdr`]
module"]
#[doc(alias = "rdr")]
pub type Rdr = crate::Reg<rdr::RdrSpec>;
#[doc = "JH7110 Crypto SHA RDR"]
pub mod rdr;
#[doc = "wsr (rw) register accessor: JH7110 Crypto SHA WSR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wsr`]
module"]
#[doc(alias = "wsr")]
pub type Wsr = crate::Reg<wsr::WsrSpec>;
#[doc = "JH7110 Crypto SHA WSR"]
pub mod wsr;
#[doc = "wlen (rw) register accessor: JH7110 Crypto SHA WLEN\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wlen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wlen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wlen`]
module"]
#[doc(alias = "wlen")]
pub type Wlen = crate::Reg<wlen::WlenSpec>;
#[doc = "JH7110 Crypto SHA WLEN"]
pub mod wlen;
#[doc = "wkr (rw) register accessor: JH7110 Crypto SHA WKR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wkr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wkr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wkr`]
module"]
#[doc(alias = "wkr")]
pub type Wkr = crate::Reg<wkr::WkrSpec>;
#[doc = "JH7110 Crypto SHA WKR"]
pub mod wkr;
#[doc = "klen (rw) register accessor: JH7110 Crypto SHA KLEN\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`klen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`klen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@klen`]
module"]
#[doc(alias = "klen")]
pub type Klen = crate::Reg<klen::KlenSpec>;
#[doc = "JH7110 Crypto SHA KLEN"]
pub mod klen;
