#[repr(C)]
#[doc = "Clock GMAC registers"]
#[doc(alias = "clk_gmac")]
pub struct ClkGmac {
    gmac5_axi64: [Gmac5Axi64; 2],
    gmac_src: GmacSrc,
    gmac1_gtx: Gmac1Gtx,
    gmac1_rmii_rtx: Gmac1RmiiRtx,
    gmac5_axi64_ptp: Gmac5Axi64Ptp,
    gmac5_axi64_rx: Gmac5Axi64Rx,
    gmac5_axi64_rxi: Gmac5Axi64Rxi,
    gmac5_axi64_tx: Gmac5Axi64Tx,
    gmac5_axi64_txi: Gmac5Axi64Txi,
    gmac1_gtxc: Gmac1Gtxc,
    gmac0_gtx: Gmac0Gtx,
    gmac0_ptp: Gmac0Ptp,
    gmac_phy: GmacPhy,
    gmac0_gtxc: Gmac0Gtxc,
}
impl ClkGmac {
    #[doc = "0x00..0x08 - Clock GMAC5 AXI64"]
    #[inline(always)]
    pub const fn gmac5_axi64(&self, n: usize) -> &Gmac5Axi64 {
        &self.gmac5_axi64[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x08 - Clock GMAC5 AXI64"]
    #[inline(always)]
    pub fn gmac5_axi64_iter(&self) -> impl Iterator<Item = &Gmac5Axi64> {
        self.gmac5_axi64.iter()
    }
    #[doc = "0x00 - Clock GMAC5 AXI64"]
    #[inline(always)]
    pub const fn gmac5_axi64_ahb(&self) -> &Gmac5Axi64 {
        self.gmac5_axi64(0)
    }
    #[doc = "0x04 - Clock GMAC5 AXI64"]
    #[inline(always)]
    pub const fn gmac5_axi64_axi(&self) -> &Gmac5Axi64 {
        self.gmac5_axi64(1)
    }
    #[doc = "0x08 - Clock GMAC Source"]
    #[inline(always)]
    pub const fn gmac_src(&self) -> &GmacSrc {
        &self.gmac_src
    }
    #[doc = "0x0c - Clock GMAC1 GTX"]
    #[inline(always)]
    pub const fn gmac1_gtx(&self) -> &Gmac1Gtx {
        &self.gmac1_gtx
    }
    #[doc = "0x10 - Clock GMAC RMII RTX"]
    #[inline(always)]
    pub const fn gmac1_rmii_rtx(&self) -> &Gmac1RmiiRtx {
        &self.gmac1_rmii_rtx
    }
    #[doc = "0x14 - Clock GMAC AXI64 PTP"]
    #[inline(always)]
    pub const fn gmac5_axi64_ptp(&self) -> &Gmac5Axi64Ptp {
        &self.gmac5_axi64_ptp
    }
    #[doc = "0x18 - Clock GMAC5 AXI64 RX"]
    #[inline(always)]
    pub const fn gmac5_axi64_rx(&self) -> &Gmac5Axi64Rx {
        &self.gmac5_axi64_rx
    }
    #[doc = "0x1c - Clock GMAC5 AXI64 RX Inverter"]
    #[inline(always)]
    pub const fn gmac5_axi64_rxi(&self) -> &Gmac5Axi64Rxi {
        &self.gmac5_axi64_rxi
    }
    #[doc = "0x20 - Clock GMAC5 AXI64 TX"]
    #[inline(always)]
    pub const fn gmac5_axi64_tx(&self) -> &Gmac5Axi64Tx {
        &self.gmac5_axi64_tx
    }
    #[doc = "0x24 - Clock GMAC5 AXI64 TX Inverter"]
    #[inline(always)]
    pub const fn gmac5_axi64_txi(&self) -> &Gmac5Axi64Txi {
        &self.gmac5_axi64_txi
    }
    #[doc = "0x28 - Clock GMAC1 GTXC"]
    #[inline(always)]
    pub const fn gmac1_gtxc(&self) -> &Gmac1Gtxc {
        &self.gmac1_gtxc
    }
    #[doc = "0x2c - Clock GMAC0 GTX"]
    #[inline(always)]
    pub const fn gmac0_gtx(&self) -> &Gmac0Gtx {
        &self.gmac0_gtx
    }
    #[doc = "0x30 - Clock GMAC0 PTP"]
    #[inline(always)]
    pub const fn gmac0_ptp(&self) -> &Gmac0Ptp {
        &self.gmac0_ptp
    }
    #[doc = "0x34 - Clock GMAC PHY"]
    #[inline(always)]
    pub const fn gmac_phy(&self) -> &GmacPhy {
        &self.gmac_phy
    }
    #[doc = "0x38 - Clock GMAC0 GTXC"]
    #[inline(always)]
    pub const fn gmac0_gtxc(&self) -> &Gmac0Gtxc {
        &self.gmac0_gtxc
    }
}
#[doc = "gmac5_axi64 (rw) register accessor: Clock GMAC5 AXI64\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac5_axi64::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac5_axi64::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac5_axi64`]
module"]
#[doc(alias = "gmac5_axi64")]
pub type Gmac5Axi64 = crate::Reg<gmac5_axi64::Gmac5Axi64Spec>;
#[doc = "Clock GMAC5 AXI64"]
pub mod gmac5_axi64;
#[doc = "gmac_src (rw) register accessor: Clock GMAC Source\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac_src::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac_src::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac_src`]
module"]
#[doc(alias = "gmac_src")]
pub type GmacSrc = crate::Reg<gmac_src::GmacSrcSpec>;
#[doc = "Clock GMAC Source"]
pub mod gmac_src;
#[doc = "gmac1_gtx (rw) register accessor: Clock GMAC1 GTX\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac1_gtx::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac1_gtx::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac1_gtx`]
module"]
#[doc(alias = "gmac1_gtx")]
pub type Gmac1Gtx = crate::Reg<gmac1_gtx::Gmac1GtxSpec>;
#[doc = "Clock GMAC1 GTX"]
pub mod gmac1_gtx;
#[doc = "gmac1_rmii_rtx (rw) register accessor: Clock GMAC RMII RTX\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac1_rmii_rtx::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac1_rmii_rtx::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac1_rmii_rtx`]
module"]
#[doc(alias = "gmac1_rmii_rtx")]
pub type Gmac1RmiiRtx = crate::Reg<gmac1_rmii_rtx::Gmac1RmiiRtxSpec>;
#[doc = "Clock GMAC RMII RTX"]
pub mod gmac1_rmii_rtx;
#[doc = "gmac5_axi64_ptp (rw) register accessor: Clock GMAC AXI64 PTP\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac5_axi64_ptp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac5_axi64_ptp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac5_axi64_ptp`]
module"]
#[doc(alias = "gmac5_axi64_ptp")]
pub type Gmac5Axi64Ptp = crate::Reg<gmac5_axi64_ptp::Gmac5Axi64PtpSpec>;
#[doc = "Clock GMAC AXI64 PTP"]
pub mod gmac5_axi64_ptp;
#[doc = "gmac5_axi64_rx (rw) register accessor: Clock GMAC5 AXI64 RX\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac5_axi64_rx::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac5_axi64_rx::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac5_axi64_rx`]
module"]
#[doc(alias = "gmac5_axi64_rx")]
pub type Gmac5Axi64Rx = crate::Reg<gmac5_axi64_rx::Gmac5Axi64RxSpec>;
#[doc = "Clock GMAC5 AXI64 RX"]
pub mod gmac5_axi64_rx;
#[doc = "gmac5_axi64_rxi (rw) register accessor: Clock GMAC5 AXI64 RX Inverter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac5_axi64_rxi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac5_axi64_rxi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac5_axi64_rxi`]
module"]
#[doc(alias = "gmac5_axi64_rxi")]
pub type Gmac5Axi64Rxi = crate::Reg<gmac5_axi64_rxi::Gmac5Axi64RxiSpec>;
#[doc = "Clock GMAC5 AXI64 RX Inverter"]
pub mod gmac5_axi64_rxi;
#[doc = "gmac5_axi64_tx (rw) register accessor: Clock GMAC5 AXI64 TX\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac5_axi64_tx::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac5_axi64_tx::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac5_axi64_tx`]
module"]
#[doc(alias = "gmac5_axi64_tx")]
pub type Gmac5Axi64Tx = crate::Reg<gmac5_axi64_tx::Gmac5Axi64TxSpec>;
#[doc = "Clock GMAC5 AXI64 TX"]
pub mod gmac5_axi64_tx;
#[doc = "gmac5_axi64_txi (rw) register accessor: Clock GMAC5 AXI64 TX Inverter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac5_axi64_txi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac5_axi64_txi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac5_axi64_txi`]
module"]
#[doc(alias = "gmac5_axi64_txi")]
pub type Gmac5Axi64Txi = crate::Reg<gmac5_axi64_txi::Gmac5Axi64TxiSpec>;
#[doc = "Clock GMAC5 AXI64 TX Inverter"]
pub mod gmac5_axi64_txi;
#[doc = "gmac1_gtxc (rw) register accessor: Clock GMAC1 GTXC\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac1_gtxc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac1_gtxc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac1_gtxc`]
module"]
#[doc(alias = "gmac1_gtxc")]
pub type Gmac1Gtxc = crate::Reg<gmac1_gtxc::Gmac1GtxcSpec>;
#[doc = "Clock GMAC1 GTXC"]
pub mod gmac1_gtxc;
#[doc = "gmac0_gtx (rw) register accessor: Clock GMAC0 GTX\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac0_gtx::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac0_gtx::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac0_gtx`]
module"]
#[doc(alias = "gmac0_gtx")]
pub type Gmac0Gtx = crate::Reg<gmac0_gtx::Gmac0GtxSpec>;
#[doc = "Clock GMAC0 GTX"]
pub mod gmac0_gtx;
#[doc = "gmac0_ptp (rw) register accessor: Clock GMAC0 PTP\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac0_ptp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac0_ptp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac0_ptp`]
module"]
#[doc(alias = "gmac0_ptp")]
pub type Gmac0Ptp = crate::Reg<gmac0_ptp::Gmac0PtpSpec>;
#[doc = "Clock GMAC0 PTP"]
pub mod gmac0_ptp;
#[doc = "gmac_phy (rw) register accessor: Clock GMAC PHY\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac_phy::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac_phy::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac_phy`]
module"]
#[doc(alias = "gmac_phy")]
pub type GmacPhy = crate::Reg<gmac_phy::GmacPhySpec>;
#[doc = "Clock GMAC PHY"]
pub mod gmac_phy;
#[doc = "gmac0_gtxc (rw) register accessor: Clock GMAC0 GTXC\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac0_gtxc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac0_gtxc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac0_gtxc`]
module"]
#[doc(alias = "gmac0_gtxc")]
pub type Gmac0Gtxc = crate::Reg<gmac0_gtxc::Gmac0GtxcSpec>;
#[doc = "Clock GMAC0 GTXC"]
pub mod gmac0_gtxc;
