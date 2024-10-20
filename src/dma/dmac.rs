#[repr(C)]
#[doc = "DesignWare DMAC registers"]
#[doc(alias = "dmac")]
pub struct Dmac {
    id: Id,
    compver: Compver,
    cfg: Cfg,
    _reserved_3_chen: [u8; 0x08],
    chsusp: Chsusp,
    chabort: Chabort,
    _reserved_6_intstatus: [u8; 0x08],
    common: Common,
    reset: Reset,
    lowpower_cfg: LowpowerCfg,
    _reserved10: [u8; 0x90],
    _reserved_dmac: _ReservedDmac,
}
impl Dmac {
    #[doc = "0x00..0x08 - DMAC ID register contains the 64-bit identification value."]
    #[inline(always)]
    pub const fn id(&self) -> &Id {
        &self.id
    }
    #[doc = "0x08..0x10 - DMAC Component Version register contains the 32-bit component version."]
    #[inline(always)]
    pub const fn compver(&self) -> &Compver {
        &self.compver
    }
    #[doc = "0x10..0x18 - DMAC Configuration register contains the DMAC config settings."]
    #[inline(always)]
    pub const fn cfg(&self) -> &Cfg {
        &self.cfg
    }
    #[doc = "0x18..0x20 - DMAC Channel Enable register contains the DMAC channel enable settings. Only exists when DMAX_NUM_CHANNELS > 8"]
    #[inline(always)]
    pub const fn chen2(&self) -> &Chen2 {
        unsafe { &*(self as *const Self).cast::<u8>().add(24).cast() }
    }
    #[doc = "0x18..0x20 - DMAC Channel Enable register contains the DMAC channel enable settings. Only exists when DMAX_NUM_CHANNELS &lt;= 8."]
    #[inline(always)]
    pub const fn chen(&self) -> &Chen {
        unsafe { &*(self as *const Self).cast::<u8>().add(24).cast() }
    }
    #[doc = "0x20..0x28 - DMAC Channel Suspend register contains the DMAC channel suspend settings. Only exists when DMAX_NUM_CHANNELS > 8"]
    #[inline(always)]
    pub const fn chsusp(&self) -> &Chsusp {
        &self.chsusp
    }
    #[doc = "0x28..0x30 - DMAC Channel Abort register contains the DMAC channel abort settings. Only exists when DMAX_NUM_CHANNELS > 8"]
    #[inline(always)]
    pub const fn chabort(&self) -> &Chabort {
        &self.chabort
    }
    #[doc = "0x30..0x38 - DMAC Channel Interrupt Status register contains the DMAC channel interrupt status. Only exists when DMAX_NUM_CHANNELS > 8"]
    #[inline(always)]
    pub const fn intstatus2(&self) -> &Intstatus2 {
        unsafe { &*(self as *const Self).cast::<u8>().add(48).cast() }
    }
    #[doc = "0x30..0x38 - DMAC Interrupt Status register contains the DMAC interrupt status. Only exists when DMAX_NUM_CHANNELS &lt;= 8"]
    #[inline(always)]
    pub const fn intstatus(&self) -> &Intstatus {
        unsafe { &*(self as *const Self).cast::<u8>().add(48).cast() }
    }
    #[doc = "0x38..0x58 - DesignWare DMAC Common registers"]
    #[inline(always)]
    pub const fn common(&self) -> &Common {
        &self.common
    }
    #[doc = "0x58..0x60 - DMAC Channel Interrupt Status register contains the DMAC channel interrupt status. Only exists when DMAX_NUM_CHANNELS > 8"]
    #[inline(always)]
    pub const fn reset(&self) -> &Reset {
        &self.reset
    }
    #[doc = "0x60..0x68 - DMAC Low Power Configuration register."]
    #[inline(always)]
    pub const fn lowpower_cfg(&self) -> &LowpowerCfg {
        &self.lowpower_cfg
    }
    #[doc = "0xf8..0x100 - DMAC Reserved register."]
    #[inline(always)]
    pub const fn _reserved_dmac(&self) -> &_ReservedDmac {
        &self._reserved_dmac
    }
}
#[doc = "id (r) register accessor: DMAC ID register contains the 64-bit identification value.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`id::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@id`]
module"]
#[doc(alias = "id")]
pub type Id = crate::Reg<id::IdSpec>;
#[doc = "DMAC ID register contains the 64-bit identification value."]
pub mod id;
#[doc = "compver (r) register accessor: DMAC Component Version register contains the 32-bit component version.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`compver::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@compver`]
module"]
#[doc(alias = "compver")]
pub type Compver = crate::Reg<compver::CompverSpec>;
#[doc = "DMAC Component Version register contains the 32-bit component version."]
pub mod compver;
#[doc = "cfg (rw) register accessor: DMAC Configuration register contains the DMAC config settings.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
#[doc(alias = "cfg")]
pub type Cfg = crate::Reg<cfg::CfgSpec>;
#[doc = "DMAC Configuration register contains the DMAC config settings."]
pub mod cfg;
#[doc = "chen (rw) register accessor: DMAC Channel Enable register contains the DMAC channel enable settings. Only exists when DMAX_NUM_CHANNELS &lt;= 8.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chen`]
module"]
#[doc(alias = "chen")]
pub type Chen = crate::Reg<chen::ChenSpec>;
#[doc = "DMAC Channel Enable register contains the DMAC channel enable settings. Only exists when DMAX_NUM_CHANNELS &lt;= 8."]
pub mod chen;
#[doc = "chen2 (rw) register accessor: DMAC Channel Enable register contains the DMAC channel enable settings. Only exists when DMAX_NUM_CHANNELS > 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chen2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chen2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chen2`]
module"]
#[doc(alias = "chen2")]
pub type Chen2 = crate::Reg<chen2::Chen2Spec>;
#[doc = "DMAC Channel Enable register contains the DMAC channel enable settings. Only exists when DMAX_NUM_CHANNELS > 8"]
pub mod chen2;
#[doc = "chsusp (rw) register accessor: DMAC Channel Suspend register contains the DMAC channel suspend settings. Only exists when DMAX_NUM_CHANNELS > 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chsusp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chsusp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chsusp`]
module"]
#[doc(alias = "chsusp")]
pub type Chsusp = crate::Reg<chsusp::ChsuspSpec>;
#[doc = "DMAC Channel Suspend register contains the DMAC channel suspend settings. Only exists when DMAX_NUM_CHANNELS > 8"]
pub mod chsusp;
#[doc = "chabort (rw) register accessor: DMAC Channel Abort register contains the DMAC channel abort settings. Only exists when DMAX_NUM_CHANNELS > 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chabort::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chabort::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chabort`]
module"]
#[doc(alias = "chabort")]
pub type Chabort = crate::Reg<chabort::ChabortSpec>;
#[doc = "DMAC Channel Abort register contains the DMAC channel abort settings. Only exists when DMAX_NUM_CHANNELS > 8"]
pub mod chabort;
#[doc = "intstatus (r) register accessor: DMAC Interrupt Status register contains the DMAC interrupt status. Only exists when DMAX_NUM_CHANNELS &lt;= 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intstatus::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intstatus`]
module"]
#[doc(alias = "intstatus")]
pub type Intstatus = crate::Reg<intstatus::IntstatusSpec>;
#[doc = "DMAC Interrupt Status register contains the DMAC interrupt status. Only exists when DMAX_NUM_CHANNELS &lt;= 8"]
pub mod intstatus;
#[doc = "intstatus2 (r) register accessor: DMAC Channel Interrupt Status register contains the DMAC channel interrupt status. Only exists when DMAX_NUM_CHANNELS > 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intstatus2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intstatus2`]
module"]
#[doc(alias = "intstatus2")]
pub type Intstatus2 = crate::Reg<intstatus2::Intstatus2Spec>;
#[doc = "DMAC Channel Interrupt Status register contains the DMAC channel interrupt status. Only exists when DMAX_NUM_CHANNELS > 8"]
pub mod intstatus2;
#[doc = "DesignWare DMAC Common registers"]
pub use self::common::Common;
#[doc = r"Cluster"]
#[doc = "DesignWare DMAC Common registers"]
pub mod common;
#[doc = "reset (rw) register accessor: DMAC Channel Interrupt Status register contains the DMAC channel interrupt status. Only exists when DMAX_NUM_CHANNELS > 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reset::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reset::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reset`]
module"]
#[doc(alias = "reset")]
pub type Reset = crate::Reg<reset::ResetSpec>;
#[doc = "DMAC Channel Interrupt Status register contains the DMAC channel interrupt status. Only exists when DMAX_NUM_CHANNELS > 8"]
pub mod reset;
#[doc = "lowpower_cfg (rw) register accessor: DMAC Low Power Configuration register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lowpower_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lowpower_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lowpower_cfg`]
module"]
#[doc(alias = "lowpower_cfg")]
pub type LowpowerCfg = crate::Reg<lowpower_cfg::LowpowerCfgSpec>;
#[doc = "DMAC Low Power Configuration register."]
pub mod lowpower_cfg;
#[doc = "_reserved_dmac (rw) register accessor: DMAC Reserved register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_reserved_dmac::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`_reserved_dmac::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_reserved_dmac`]
module"]
#[doc(alias = "_reserved_dmac")]
pub type _ReservedDmac = crate::Reg<_reserved_dmac::_ReservedDmacSpec>;
#[doc = "DMAC Reserved register."]
pub mod _reserved_dmac;
