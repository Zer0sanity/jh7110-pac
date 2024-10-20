#[repr(C)]
#[doc = "JH7110 Crypto AES registers"]
#[doc(alias = "aes")]
pub struct Aes {
    aesdio0r: Aesdio0r,
    key: [Key; 8],
    csr: Csr,
    iv: [Iv; 4],
    _reserved4: [u8; 0x04],
    nonce: [Nonce; 4],
    alen: [Alen; 2],
    mlen: [Mlen; 2],
    ivlen: Ivlen,
}
impl Aes {
    #[doc = "0x00 - JH7110 Crypto AES AESDIO0R"]
    #[inline(always)]
    pub const fn aesdio0r(&self) -> &Aesdio0r {
        &self.aesdio0r
    }
    #[doc = "0x04..0x24 - JH7110 Crypto AES Key"]
    #[inline(always)]
    pub const fn key(&self, n: usize) -> &Key {
        &self.key[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x04..0x24 - JH7110 Crypto AES Key"]
    #[inline(always)]
    pub fn key_iter(&self) -> impl Iterator<Item = &Key> {
        self.key.iter()
    }
    #[doc = "0x24 - JH7110 Crypto AES Control Status Register"]
    #[inline(always)]
    pub const fn csr(&self) -> &Csr {
        &self.csr
    }
    #[doc = "0x28..0x38 - JH7110 Crypto AES IV"]
    #[inline(always)]
    pub const fn iv(&self, n: usize) -> &Iv {
        &self.iv[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x28..0x38 - JH7110 Crypto AES IV"]
    #[inline(always)]
    pub fn iv_iter(&self) -> impl Iterator<Item = &Iv> {
        self.iv.iter()
    }
    #[doc = "0x3c..0x4c - JH7110 Crypto AES Nonce"]
    #[inline(always)]
    pub const fn nonce(&self, n: usize) -> &Nonce {
        &self.nonce[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x3c..0x4c - JH7110 Crypto AES Nonce"]
    #[inline(always)]
    pub fn nonce_iter(&self) -> impl Iterator<Item = &Nonce> {
        self.nonce.iter()
    }
    #[doc = "0x4c..0x54 - JH7110 Crypto AES ALEN"]
    #[inline(always)]
    pub const fn alen(&self, n: usize) -> &Alen {
        &self.alen[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x4c..0x54 - JH7110 Crypto AES ALEN"]
    #[inline(always)]
    pub fn alen_iter(&self) -> impl Iterator<Item = &Alen> {
        self.alen.iter()
    }
    #[doc = "0x54..0x5c - JH7110 Crypto AES MLEN"]
    #[inline(always)]
    pub const fn mlen(&self, n: usize) -> &Mlen {
        &self.mlen[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x54..0x5c - JH7110 Crypto AES MLEN"]
    #[inline(always)]
    pub fn mlen_iter(&self) -> impl Iterator<Item = &Mlen> {
        self.mlen.iter()
    }
    #[doc = "0x5c - JH7110 Crypto AES IVLEN"]
    #[inline(always)]
    pub const fn ivlen(&self) -> &Ivlen {
        &self.ivlen
    }
}
#[doc = "aesdio0r (rw) register accessor: JH7110 Crypto AES AESDIO0R\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aesdio0r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aesdio0r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aesdio0r`]
module"]
#[doc(alias = "aesdio0r")]
pub type Aesdio0r = crate::Reg<aesdio0r::Aesdio0rSpec>;
#[doc = "JH7110 Crypto AES AESDIO0R"]
pub mod aesdio0r;
#[doc = "key (rw) register accessor: JH7110 Crypto AES Key\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`key::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`key::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@key`]
module"]
#[doc(alias = "key")]
pub type Key = crate::Reg<key::KeySpec>;
#[doc = "JH7110 Crypto AES Key"]
pub mod key;
#[doc = "csr (rw) register accessor: JH7110 Crypto AES Control Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`csr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`csr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csr`]
module"]
#[doc(alias = "csr")]
pub type Csr = crate::Reg<csr::CsrSpec>;
#[doc = "JH7110 Crypto AES Control Status Register"]
pub mod csr;
#[doc = "iv (rw) register accessor: JH7110 Crypto AES IV\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`iv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iv`]
module"]
#[doc(alias = "iv")]
pub type Iv = crate::Reg<iv::IvSpec>;
#[doc = "JH7110 Crypto AES IV"]
pub mod iv;
#[doc = "nonce (rw) register accessor: JH7110 Crypto AES Nonce\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`nonce::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`nonce::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nonce`]
module"]
#[doc(alias = "nonce")]
pub type Nonce = crate::Reg<nonce::NonceSpec>;
#[doc = "JH7110 Crypto AES Nonce"]
pub mod nonce;
#[doc = "alen (rw) register accessor: JH7110 Crypto AES ALEN\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`alen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`alen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@alen`]
module"]
#[doc(alias = "alen")]
pub type Alen = crate::Reg<alen::AlenSpec>;
#[doc = "JH7110 Crypto AES ALEN"]
pub mod alen;
#[doc = "mlen (rw) register accessor: JH7110 Crypto AES MLEN\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mlen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mlen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mlen`]
module"]
#[doc(alias = "mlen")]
pub type Mlen = crate::Reg<mlen::MlenSpec>;
#[doc = "JH7110 Crypto AES MLEN"]
pub mod mlen;
#[doc = "ivlen (rw) register accessor: JH7110 Crypto AES IVLEN\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ivlen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ivlen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ivlen`]
module"]
#[doc(alias = "ivlen")]
pub type Ivlen = crate::Reg<ivlen::IvlenSpec>;
#[doc = "JH7110 Crypto AES IVLEN"]
pub mod ivlen;
