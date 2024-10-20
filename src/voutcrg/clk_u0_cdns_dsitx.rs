#[repr(C)]
#[doc = "Clock U0 Cadence DSI Transmit registers"]
#[doc(alias = "clk_u0_cdns_dsitx")]
pub struct ClkU0CdnsDsitx {
    apb: Apb,
    sys: Sys,
    dpi: Dpi,
    txesc: Txesc,
}
impl ClkU0CdnsDsitx {
    #[doc = "0x00 - Clock U0 Cadence DSI Transmit APB"]
    #[inline(always)]
    pub const fn apb(&self) -> &Apb {
        &self.apb
    }
    #[doc = "0x04 - Clock U0 Cadence DSI Transmit SYS"]
    #[inline(always)]
    pub const fn sys(&self) -> &Sys {
        &self.sys
    }
    #[doc = "0x08 - Clock U0 Cadence DSI Transmit DPI"]
    #[inline(always)]
    pub const fn dpi(&self) -> &Dpi {
        &self.dpi
    }
    #[doc = "0x0c - Clock U0 Cadence DSI Transmit TXESC"]
    #[inline(always)]
    pub const fn txesc(&self) -> &Txesc {
        &self.txesc
    }
}
#[doc = "apb (rw) register accessor: Clock U0 Cadence DSI Transmit APB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb`]
module"]
#[doc(alias = "apb")]
pub type Apb = crate::Reg<apb::ApbSpec>;
#[doc = "Clock U0 Cadence DSI Transmit APB"]
pub mod apb;
#[doc = "sys (rw) register accessor: Clock U0 Cadence DSI Transmit SYS\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sys`]
module"]
#[doc(alias = "sys")]
pub type Sys = crate::Reg<sys::SysSpec>;
#[doc = "Clock U0 Cadence DSI Transmit SYS"]
pub mod sys;
#[doc = "dpi (rw) register accessor: Clock U0 Cadence DSI Transmit DPI\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dpi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dpi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dpi`]
module"]
#[doc(alias = "dpi")]
pub type Dpi = crate::Reg<dpi::DpiSpec>;
#[doc = "Clock U0 Cadence DSI Transmit DPI"]
pub mod dpi;
#[doc = "txesc (rw) register accessor: Clock U0 Cadence DSI Transmit TXESC\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txesc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txesc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txesc`]
module"]
#[doc(alias = "txesc")]
pub type Txesc = crate::Reg<txesc::TxescSpec>;
#[doc = "Clock U0 Cadence DSI Transmit TXESC"]
pub mod txesc;
