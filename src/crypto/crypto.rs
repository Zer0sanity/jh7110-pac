#[repr(C)]
#[doc = "JH7110 Crypto CRYPTO registers"]
#[doc(alias = "crypto")]
pub struct Crypto {
    cacr: Cacr,
    casr: Casr,
    caar: Caar,
    _reserved3: [u8; 0xfc],
    caer: Caer,
    _reserved4: [u8; 0xfc],
    canr: Canr,
    _reserved5: [u8; 0xfc],
    caafr: Caafr,
    caefr: Caefr,
    canfr: Canfr,
    fifo_counter: FifoCounter,
}
impl Crypto {
    #[doc = "0x00 - JH7110 Crypto CA Control Register"]
    #[inline(always)]
    pub const fn cacr(&self) -> &Cacr {
        &self.cacr
    }
    #[doc = "0x04 - JH7110 Crypto CA Status Register"]
    #[inline(always)]
    pub const fn casr(&self) -> &Casr {
        &self.casr
    }
    #[doc = "0x08 - JH7110 Crypto CAAR"]
    #[inline(always)]
    pub const fn caar(&self) -> &Caar {
        &self.caar
    }
    #[doc = "0x108 - JH7110 Crypto CAER"]
    #[inline(always)]
    pub const fn caer(&self) -> &Caer {
        &self.caer
    }
    #[doc = "0x208 - JH7110 Crypto CANR"]
    #[inline(always)]
    pub const fn canr(&self) -> &Canr {
        &self.canr
    }
    #[doc = "0x308 - JH7110 Crypto CAAFR"]
    #[inline(always)]
    pub const fn caafr(&self) -> &Caafr {
        &self.caafr
    }
    #[doc = "0x30c - JH7110 Crypto CAEFR"]
    #[inline(always)]
    pub const fn caefr(&self) -> &Caefr {
        &self.caefr
    }
    #[doc = "0x310 - JH7110 Crypto CANFR"]
    #[inline(always)]
    pub const fn canfr(&self) -> &Canfr {
        &self.canfr
    }
    #[doc = "0x314 - JH7110 Crypto FIFO Counter"]
    #[inline(always)]
    pub const fn fifo_counter(&self) -> &FifoCounter {
        &self.fifo_counter
    }
}
#[doc = "cacr (rw) register accessor: JH7110 Crypto CA Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cacr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cacr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cacr`]
module"]
#[doc(alias = "cacr")]
pub type Cacr = crate::Reg<cacr::CacrSpec>;
#[doc = "JH7110 Crypto CA Control Register"]
pub mod cacr;
#[doc = "casr (r) register accessor: JH7110 Crypto CA Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`casr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@casr`]
module"]
#[doc(alias = "casr")]
pub type Casr = crate::Reg<casr::CasrSpec>;
#[doc = "JH7110 Crypto CA Status Register"]
pub mod casr;
#[doc = "caar (rw) register accessor: JH7110 Crypto CAAR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`caar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`caar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@caar`]
module"]
#[doc(alias = "caar")]
pub type Caar = crate::Reg<caar::CaarSpec>;
#[doc = "JH7110 Crypto CAAR"]
pub mod caar;
#[doc = "caer (rw) register accessor: JH7110 Crypto CAER\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`caer::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`caer::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@caer`]
module"]
#[doc(alias = "caer")]
pub type Caer = crate::Reg<caer::CaerSpec>;
#[doc = "JH7110 Crypto CAER"]
pub mod caer;
#[doc = "canr (rw) register accessor: JH7110 Crypto CANR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`canr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`canr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@canr`]
module"]
#[doc(alias = "canr")]
pub type Canr = crate::Reg<canr::CanrSpec>;
#[doc = "JH7110 Crypto CANR"]
pub mod canr;
#[doc = "caafr (rw) register accessor: JH7110 Crypto CAAFR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`caafr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`caafr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@caafr`]
module"]
#[doc(alias = "caafr")]
pub type Caafr = crate::Reg<caafr::CaafrSpec>;
#[doc = "JH7110 Crypto CAAFR"]
pub mod caafr;
#[doc = "caefr (rw) register accessor: JH7110 Crypto CAEFR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`caefr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`caefr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@caefr`]
module"]
#[doc(alias = "caefr")]
pub type Caefr = crate::Reg<caefr::CaefrSpec>;
#[doc = "JH7110 Crypto CAEFR"]
pub mod caefr;
#[doc = "canfr (rw) register accessor: JH7110 Crypto CANFR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`canfr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`canfr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@canfr`]
module"]
#[doc(alias = "canfr")]
pub type Canfr = crate::Reg<canfr::CanfrSpec>;
#[doc = "JH7110 Crypto CANFR"]
pub mod canfr;
#[doc = "fifo_counter (rw) register accessor: JH7110 Crypto FIFO Counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fifo_counter::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fifo_counter::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo_counter`]
module"]
#[doc(alias = "fifo_counter")]
pub type FifoCounter = crate::Reg<fifo_counter::FifoCounterSpec>;
#[doc = "JH7110 Crypto FIFO Counter"]
pub mod fifo_counter;
