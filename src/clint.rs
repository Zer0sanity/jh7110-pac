#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    msip: [Msip; 5],
    _reserved1: [u8; 0x3fec],
    mtimecmp: [Mtimecmp; 5],
    _reserved2: [u8; 0x7fd0],
    mtime: Mtime,
}
impl RegisterBlock {
    #[doc = "0x00..0x14 - CLINT MSIP (Machine Software Interrupt) register"]
    #[inline(always)]
    pub const fn msip(&self, n: usize) -> &Msip {
        &self.msip[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x14 - CLINT MSIP (Machine Software Interrupt) register"]
    #[inline(always)]
    pub fn msip_iter(&self) -> impl Iterator<Item = &Msip> {
        self.msip.iter()
    }
    #[doc = "0x4000..0x4028 - CLINT MTIMECMP (Machine Time Compare) register"]
    #[inline(always)]
    pub const fn mtimecmp(&self, n: usize) -> &Mtimecmp {
        &self.mtimecmp[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x4000..0x4028 - CLINT MTIMECMP (Machine Time Compare) register"]
    #[inline(always)]
    pub fn mtimecmp_iter(&self) -> impl Iterator<Item = &Mtimecmp> {
        self.mtimecmp.iter()
    }
    #[doc = "0xbff8..0xc000 - CLINT MTIME (Machine Time) register"]
    #[inline(always)]
    pub const fn mtime(&self) -> &Mtime {
        &self.mtime
    }
}
#[doc = "msip (rw) register accessor: CLINT MSIP (Machine Software Interrupt) register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msip::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`msip::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msip`]
module"]
#[doc(alias = "msip")]
pub type Msip = crate::Reg<msip::MsipSpec>;
#[doc = "CLINT MSIP (Machine Software Interrupt) register"]
pub mod msip;
#[doc = "mtimecmp (rw) register accessor: CLINT MTIMECMP (Machine Time Compare) register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mtimecmp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mtimecmp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mtimecmp`]
module"]
#[doc(alias = "mtimecmp")]
pub type Mtimecmp = crate::Reg<mtimecmp::MtimecmpSpec>;
#[doc = "CLINT MTIMECMP (Machine Time Compare) register"]
pub mod mtimecmp;
#[doc = "mtime (rw) register accessor: CLINT MTIME (Machine Time) register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mtime::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mtime::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mtime`]
module"]
#[doc(alias = "mtime")]
pub type Mtime = crate::Reg<mtime::MtimeSpec>;
#[doc = "CLINT MTIME (Machine Time) register"]
pub mod mtime;
