#[repr(C)]
#[doc = "L2 Cache Control Directory ECC registers. Types: `dir_fix`: ECC directory fixes, `dir_fail`: ECC directory failures, `data_fix`: ECC data fixes, `data_fail`: ECC data failures."]
#[doc(alias = "ecc")]
pub struct Ecc {
    addr: [Addr; 2],
    count: Count,
    _reserved2: [u8; 0x10],
    _ecc_reserved: _EccReserved,
}
impl Ecc {
    #[doc = "0x00..0x08 - L2 Cache Control ECC Type Address registers. Contains the low- and high-address bits of the most recent failure."]
    #[inline(always)]
    pub const fn addr(&self, n: usize) -> &Addr {
        &self.addr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x08 - L2 Cache Control ECC Type Address registers. Contains the low- and high-address bits of the most recent failure."]
    #[inline(always)]
    pub fn addr_iter(&self) -> impl Iterator<Item = &Addr> {
        self.addr.iter()
    }
    #[doc = "0x00 - L2 Cache Control ECC Type Address registers. Contains the low- and high-address bits of the most recent failure."]
    #[inline(always)]
    pub const fn addr_low(&self) -> &Addr {
        self.addr(0)
    }
    #[doc = "0x04 - L2 Cache Control ECC Type Address registers. Contains the low- and high-address bits of the most recent failure."]
    #[inline(always)]
    pub const fn addr_high(&self) -> &Addr {
        self.addr(1)
    }
    #[doc = "0x08 - L2 Cache Control ECC Type Count register. Reports the number of times an ECC error occured."]
    #[inline(always)]
    pub const fn count(&self) -> &Count {
        &self.count
    }
    #[doc = "0x1c - L2 Cache Control ECC Type (`directory`, `data`) reserved register."]
    #[inline(always)]
    pub const fn _ecc_reserved(&self) -> &_EccReserved {
        &self._ecc_reserved
    }
}
#[doc = "addr (r) register accessor: L2 Cache Control ECC Type Address registers. Contains the low- and high-address bits of the most recent failure.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`addr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@addr`]
module"]
#[doc(alias = "addr")]
pub type Addr = crate::Reg<addr::AddrSpec>;
#[doc = "L2 Cache Control ECC Type Address registers. Contains the low- and high-address bits of the most recent failure."]
pub mod addr;
#[doc = "count (r) register accessor: L2 Cache Control ECC Type Count register. Reports the number of times an ECC error occured.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`count::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@count`]
module"]
#[doc(alias = "count")]
pub type Count = crate::Reg<count::CountSpec>;
#[doc = "L2 Cache Control ECC Type Count register. Reports the number of times an ECC error occured."]
pub mod count;
#[doc = "_ecc_reserved (rw) register accessor: L2 Cache Control ECC Type (`directory`, `data`) reserved register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_ecc_reserved::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`_ecc_reserved::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_ecc_reserved`]
module"]
#[doc(alias = "_ecc_reserved")]
pub type _EccReserved = crate::Reg<_ecc_reserved::_EccReservedSpec>;
#[doc = "L2 Cache Control ECC Type (`directory`, `data`) reserved register."]
pub mod _ecc_reserved;
