#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pcmgbcr: Pcmgbcr,
    pcmtxcr: Pcmtxcr,
    pcmrxcr: Pcmrxcr,
    pcmdiv: Pcmdiv,
    _reserved4: [u8; 0x0702_fff0],
    tdm_fifo: [TdmFifo; 32],
}
impl RegisterBlock {
    #[doc = "0x00 - TDM PCM GB Control Register"]
    #[inline(always)]
    pub const fn pcmgbcr(&self) -> &Pcmgbcr {
        &self.pcmgbcr
    }
    #[doc = "0x04 - TDM PCM TX Control Register"]
    #[inline(always)]
    pub const fn pcmtxcr(&self) -> &Pcmtxcr {
        &self.pcmtxcr
    }
    #[doc = "0x08 - TDM PCM RX Control Register"]
    #[inline(always)]
    pub const fn pcmrxcr(&self) -> &Pcmrxcr {
        &self.pcmrxcr
    }
    #[doc = "0x0c - TDM PCM Divisor register"]
    #[inline(always)]
    pub const fn pcmdiv(&self) -> &Pcmdiv {
        &self.pcmdiv
    }
    #[doc = "0x7030000..0x7030080 - TDM FIFO registers"]
    #[inline(always)]
    pub const fn tdm_fifo(&self, n: usize) -> &TdmFifo {
        &self.tdm_fifo[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x7030000..0x7030080 - TDM FIFO registers"]
    #[inline(always)]
    pub fn tdm_fifo_iter(&self) -> impl Iterator<Item = &TdmFifo> {
        self.tdm_fifo.iter()
    }
}
#[doc = "pcmgbcr (rw) register accessor: TDM PCM GB Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcmgbcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcmgbcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcmgbcr`]
module"]
#[doc(alias = "pcmgbcr")]
pub type Pcmgbcr = crate::Reg<pcmgbcr::PcmgbcrSpec>;
#[doc = "TDM PCM GB Control Register"]
pub mod pcmgbcr;
#[doc = "pcmtxcr (rw) register accessor: TDM PCM TX Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcmtxcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcmtxcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcmtxcr`]
module"]
#[doc(alias = "pcmtxcr")]
pub type Pcmtxcr = crate::Reg<pcmtxcr::PcmtxcrSpec>;
#[doc = "TDM PCM TX Control Register"]
pub mod pcmtxcr;
#[doc = "pcmrxcr (rw) register accessor: TDM PCM RX Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcmrxcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcmrxcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcmrxcr`]
module"]
#[doc(alias = "pcmrxcr")]
pub type Pcmrxcr = crate::Reg<pcmrxcr::PcmrxcrSpec>;
#[doc = "TDM PCM RX Control Register"]
pub mod pcmrxcr;
#[doc = "pcmdiv (rw) register accessor: TDM PCM Divisor register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcmdiv::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcmdiv::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcmdiv`]
module"]
#[doc(alias = "pcmdiv")]
pub type Pcmdiv = crate::Reg<pcmdiv::PcmdivSpec>;
#[doc = "TDM PCM Divisor register"]
pub mod pcmdiv;
#[doc = "tdm_fifo (rw) register accessor: TDM FIFO registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tdm_fifo::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tdm_fifo::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tdm_fifo`]
module"]
#[doc(alias = "tdm_fifo")]
pub type TdmFifo = crate::Reg<tdm_fifo::TdmFifoSpec>;
#[doc = "TDM FIFO registers"]
pub mod tdm_fifo;
