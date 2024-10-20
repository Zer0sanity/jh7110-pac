#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    alg: Alg,
    ie: Ie,
    dma: [Dma; 2],
    _reserved3: [u8; 0xe8],
    aes: Aes,
    _reserved4: [u8; 0x01a0],
    sha: Sha,
    _reserved5: [u8; 0xd8],
    crypto: Crypto,
}
impl RegisterBlock {
    #[doc = "0x00..0x08 - JH7110 Crypto Algorithm registers"]
    #[inline(always)]
    pub const fn alg(&self) -> &Alg {
        &self.alg
    }
    #[doc = "0x08..0x10 - JH7110 Crypto Interrupt Enable registers"]
    #[inline(always)]
    pub const fn ie(&self) -> &Ie {
        &self.ie
    }
    #[doc = "0x10..0x18 - JH7110 Crypto DMA registers"]
    #[inline(always)]
    pub const fn dma(&self, n: usize) -> &Dma {
        &self.dma[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x10..0x18 - JH7110 Crypto DMA registers"]
    #[inline(always)]
    pub fn dma_iter(&self) -> impl Iterator<Item = &Dma> {
        self.dma.iter()
    }
    #[doc = "0x10 - JH7110 Crypto DMA registers"]
    #[inline(always)]
    pub const fn dma_in_len(&self) -> &Dma {
        self.dma(0)
    }
    #[doc = "0x14 - JH7110 Crypto DMA registers"]
    #[inline(always)]
    pub const fn dma_out_len(&self) -> &Dma {
        self.dma(1)
    }
    #[doc = "0x100..0x160 - JH7110 Crypto AES registers"]
    #[inline(always)]
    pub const fn aes(&self) -> &Aes {
        &self.aes
    }
    #[doc = "0x300..0x328 - JH7110 Crypto SHA registers"]
    #[inline(always)]
    pub const fn sha(&self) -> &Sha {
        &self.sha
    }
    #[doc = "0x400..0x718 - JH7110 Crypto CRYPTO registers"]
    #[inline(always)]
    pub const fn crypto(&self) -> &Crypto {
        &self.crypto
    }
}
#[doc = "JH7110 Crypto Algorithm registers"]
pub use self::alg::Alg;
#[doc = r"Cluster"]
#[doc = "JH7110 Crypto Algorithm registers"]
pub mod alg;
#[doc = "JH7110 Crypto Interrupt Enable registers"]
pub use self::ie::Ie;
#[doc = r"Cluster"]
#[doc = "JH7110 Crypto Interrupt Enable registers"]
pub mod ie;
#[doc = "dma (rw) register accessor: JH7110 Crypto DMA registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma`]
module"]
#[doc(alias = "dma")]
pub type Dma = crate::Reg<dma::DmaSpec>;
#[doc = "JH7110 Crypto DMA registers"]
pub mod dma;
#[doc = "JH7110 Crypto AES registers"]
pub use self::aes::Aes;
#[doc = r"Cluster"]
#[doc = "JH7110 Crypto AES registers"]
pub mod aes;
#[doc = "JH7110 Crypto SHA registers"]
pub use self::sha::Sha;
#[doc = r"Cluster"]
#[doc = "JH7110 Crypto SHA registers"]
pub mod sha;
#[doc = "JH7110 Crypto CRYPTO registers"]
pub use self::crypto::Crypto;
#[doc = r"Cluster"]
#[doc = "JH7110 Crypto CRYPTO registers"]
pub mod crypto;
