#[repr(C)]
#[doc = "L2 Cache Control registers."]
#[doc(alias = "cache_control")]
pub struct CacheControl {
    config: Config,
    _reserved1: [u8; 0x04],
    way_enable: WayEnable,
    _reserved2: [u8; 0x34],
    ecc_inject_error: EccInjectError,
    _reserved3: [u8; 0xbc],
    ecc: [Ecc; 4],
    _reserved4: [u8; 0x80],
    flush: Flush,
    _reserved5: [u8; 0x05bc],
    way_mask: [WayMask; 27],
}
impl CacheControl {
    #[doc = "0x00 - L2 Cache Control configuration."]
    #[inline(always)]
    pub const fn config(&self) -> &Config {
        &self.config
    }
    #[doc = "0x08 - L2 Cache Control Way Enable register. Determines which ways of the Level 2 Cache Controller are enabled as cache."]
    #[inline(always)]
    pub const fn way_enable(&self) -> &WayEnable {
        &self.way_enable
    }
    #[doc = "0x40 - L2 Cache Control ECC Error Injection register. Can be used to insert an ECC error into either the backing data or metadata SRAM."]
    #[inline(always)]
    pub const fn ecc_inject_error(&self) -> &EccInjectError {
        &self.ecc_inject_error
    }
    #[doc = "0x100..0x180 - L2 Cache Control Directory ECC registers. Types: `dir_fix`: ECC directory fixes, `dir_fail`: ECC directory failures, `data_fix`: ECC data fixes, `data_fail`: ECC data failures."]
    #[inline(always)]
    pub const fn ecc(&self, n: usize) -> &Ecc {
        &self.ecc[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x100..0x180 - L2 Cache Control Directory ECC registers. Types: `dir_fix`: ECC directory fixes, `dir_fail`: ECC directory failures, `data_fix`: ECC data fixes, `data_fail`: ECC data failures."]
    #[inline(always)]
    pub fn ecc_iter(&self) -> impl Iterator<Item = &Ecc> {
        self.ecc.iter()
    }
    #[doc = "0x100..0x120 - L2 Cache Control Directory ECC registers. Types: `dir_fix`: ECC directory fixes, `dir_fail`: ECC directory failures, `data_fix`: ECC data fixes, `data_fail`: ECC data failures."]
    #[inline(always)]
    pub const fn ecc_dir_fix(&self) -> &Ecc {
        self.ecc(0)
    }
    #[doc = "0x120..0x140 - L2 Cache Control Directory ECC registers. Types: `dir_fix`: ECC directory fixes, `dir_fail`: ECC directory failures, `data_fix`: ECC data fixes, `data_fail`: ECC data failures."]
    #[inline(always)]
    pub const fn ecc_dir_fail(&self) -> &Ecc {
        self.ecc(1)
    }
    #[doc = "0x140..0x160 - L2 Cache Control Directory ECC registers. Types: `dir_fix`: ECC directory fixes, `dir_fail`: ECC directory failures, `data_fix`: ECC data fixes, `data_fail`: ECC data failures."]
    #[inline(always)]
    pub const fn ecc_data_fix(&self) -> &Ecc {
        self.ecc(2)
    }
    #[doc = "0x160..0x180 - L2 Cache Control Directory ECC registers. Types: `dir_fix`: ECC directory fixes, `dir_fail`: ECC directory failures, `data_fix`: ECC data fixes, `data_fail`: ECC data failures."]
    #[inline(always)]
    pub const fn ecc_data_fail(&self) -> &Ecc {
        self.ecc(3)
    }
    #[doc = "0x200..0x244 - L2 Cache Control Directory Flush registers. Can be used for flushing specific cache blocks."]
    #[inline(always)]
    pub const fn flush(&self) -> &Flush {
        &self.flush
    }
    #[doc = "0x800..0x8d8 - L2 Cache Control Way Mask registers. Configures the masks to enable cache bank ways."]
    #[inline(always)]
    pub const fn way_mask(&self, n: usize) -> &WayMask {
        &self.way_mask[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x800..0x8d8 - L2 Cache Control Way Mask registers. Configures the masks to enable cache bank ways."]
    #[inline(always)]
    pub fn way_mask_iter(&self) -> impl Iterator<Item = &WayMask> {
        self.way_mask.iter()
    }
}
#[doc = "config (r) register accessor: L2 Cache Control configuration.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`config::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config`]
module"]
#[doc(alias = "config")]
pub type Config = crate::Reg<config::ConfigSpec>;
#[doc = "L2 Cache Control configuration."]
pub mod config;
#[doc = "way_enable (rw) register accessor: L2 Cache Control Way Enable register. Determines which ways of the Level 2 Cache Controller are enabled as cache.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`way_enable::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`way_enable::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@way_enable`]
module"]
#[doc(alias = "way_enable")]
pub type WayEnable = crate::Reg<way_enable::WayEnableSpec>;
#[doc = "L2 Cache Control Way Enable register. Determines which ways of the Level 2 Cache Controller are enabled as cache."]
pub mod way_enable;
#[doc = "ecc_inject_error (rw) register accessor: L2 Cache Control ECC Error Injection register. Can be used to insert an ECC error into either the backing data or metadata SRAM.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc_inject_error::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc_inject_error::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_inject_error`]
module"]
#[doc(alias = "ecc_inject_error")]
pub type EccInjectError = crate::Reg<ecc_inject_error::EccInjectErrorSpec>;
#[doc = "L2 Cache Control ECC Error Injection register. Can be used to insert an ECC error into either the backing data or metadata SRAM."]
pub mod ecc_inject_error;
#[doc = "L2 Cache Control Directory ECC registers. Types: `dir_fix`: ECC directory fixes, `dir_fail`: ECC directory failures, `data_fix`: ECC data fixes, `data_fail`: ECC data failures."]
pub use self::ecc::Ecc;
#[doc = r"Cluster"]
#[doc = "L2 Cache Control Directory ECC registers. Types: `dir_fix`: ECC directory fixes, `dir_fail`: ECC directory failures, `data_fix`: ECC data fixes, `data_fail`: ECC data failures."]
pub mod ecc;
#[doc = "L2 Cache Control Directory Flush registers. Can be used for flushing specific cache blocks."]
pub use self::flush::Flush;
#[doc = r"Cluster"]
#[doc = "L2 Cache Control Directory Flush registers. Can be used for flushing specific cache blocks."]
pub mod flush;
#[doc = "L2 Cache Control Way Mask registers. Configures the masks to enable cache bank ways."]
pub use self::way_mask::WayMask;
#[doc = r"Cluster"]
#[doc = "L2 Cache Control Way Mask registers. Configures the masks to enable cache bank ways."]
pub mod way_mask;
