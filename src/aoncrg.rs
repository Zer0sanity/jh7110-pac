#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    clk_osc: ClkOsc,
    clk_aon_apb: ClkAonApb,
    clk_ahb_gmac5: ClkAhbGmac5,
    clk_axi_gmac5: ClkAxiGmac5,
    clk_gmac0_rmii_rtx: ClkGmac0RmiiRtx,
    clk_gmac5_axi64_tx: ClkGmac5Axi64Tx,
    clk_gmac5_axi64_txi: ClkGmac5Axi64Txi,
    clk_gmac5_axi64_rx: ClkGmac5Axi64Rx,
    clk_gmac5_axi64_rxi: ClkGmac5Axi64Rxi,
    clk_optc_apb: ClkOptcApb,
    clk_rtc_hms_apb: ClkRtcHmsApb,
    clk_rtc_internal: ClkRtcInternal,
    clk_rtc_hms_osc32k: ClkRtcHmsOsc32k,
    clk_rtc_hms_cal: ClkRtcHmsCal,
    soft_rst_addr_sel: SoftRstAddrSel,
    aoncrg_rst_status: AoncrgRstStatus,
}
impl RegisterBlock {
    #[doc = "0x00 - Oscillator Clock"]
    #[inline(always)]
    pub const fn clk_osc(&self) -> &ClkOsc {
        &self.clk_osc
    }
    #[doc = "0x04 - AON APB Function Clock"]
    #[inline(always)]
    pub const fn clk_aon_apb(&self) -> &ClkAonApb {
        &self.clk_aon_apb
    }
    #[doc = "0x08 - AHB GMAC5 Clock"]
    #[inline(always)]
    pub const fn clk_ahb_gmac5(&self) -> &ClkAhbGmac5 {
        &self.clk_ahb_gmac5
    }
    #[doc = "0x0c - AXI GMAC5 Clock"]
    #[inline(always)]
    pub const fn clk_axi_gmac5(&self) -> &ClkAxiGmac5 {
        &self.clk_axi_gmac5
    }
    #[doc = "0x10 - GMAC0 RMII RTX Clock"]
    #[inline(always)]
    pub const fn clk_gmac0_rmii_rtx(&self) -> &ClkGmac0RmiiRtx {
        &self.clk_gmac0_rmii_rtx
    }
    #[doc = "0x14 - GMAC5 AXI64 Clock Transmitter"]
    #[inline(always)]
    pub const fn clk_gmac5_axi64_tx(&self) -> &ClkGmac5Axi64Tx {
        &self.clk_gmac5_axi64_tx
    }
    #[doc = "0x18 - GMAC5 AXI64 Clock Transmission Inverter"]
    #[inline(always)]
    pub const fn clk_gmac5_axi64_txi(&self) -> &ClkGmac5Axi64Txi {
        &self.clk_gmac5_axi64_txi
    }
    #[doc = "0x1c - GMAC5 AXI64 Clock Receiver"]
    #[inline(always)]
    pub const fn clk_gmac5_axi64_rx(&self) -> &ClkGmac5Axi64Rx {
        &self.clk_gmac5_axi64_rx
    }
    #[doc = "0x20 - GMAC5 AXI64 Clock Receiving Inverter"]
    #[inline(always)]
    pub const fn clk_gmac5_axi64_rxi(&self) -> &ClkGmac5Axi64Rxi {
        &self.clk_gmac5_axi64_rxi
    }
    #[doc = "0x24 - OPTC APB Clock"]
    #[inline(always)]
    pub const fn clk_optc_apb(&self) -> &ClkOptcApb {
        &self.clk_optc_apb
    }
    #[doc = "0x28 - RTC HMS APB Clock"]
    #[inline(always)]
    pub const fn clk_rtc_hms_apb(&self) -> &ClkRtcHmsApb {
        &self.clk_rtc_hms_apb
    }
    #[doc = "0x2c - RTC Internal Clock"]
    #[inline(always)]
    pub const fn clk_rtc_internal(&self) -> &ClkRtcInternal {
        &self.clk_rtc_internal
    }
    #[doc = "0x30 - RTC HMS Clock Oscillator 32K"]
    #[inline(always)]
    pub const fn clk_rtc_hms_osc32k(&self) -> &ClkRtcHmsOsc32k {
        &self.clk_rtc_hms_osc32k
    }
    #[doc = "0x34 - RTC HMS Clock Calculator"]
    #[inline(always)]
    pub const fn clk_rtc_hms_cal(&self) -> &ClkRtcHmsCal {
        &self.clk_rtc_hms_cal
    }
    #[doc = "0x38 - Software RESET Address Selector"]
    #[inline(always)]
    pub const fn soft_rst_addr_sel(&self) -> &SoftRstAddrSel {
        &self.soft_rst_addr_sel
    }
    #[doc = "0x3c - AONCRG RESET Status"]
    #[inline(always)]
    pub const fn aoncrg_rst_status(&self) -> &AoncrgRstStatus {
        &self.aoncrg_rst_status
    }
}
#[doc = "clk_osc (rw) register accessor: Oscillator Clock\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_osc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_osc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_osc`]
module"]
#[doc(alias = "clk_osc")]
pub type ClkOsc = crate::Reg<clk_osc::ClkOscSpec>;
#[doc = "Oscillator Clock"]
pub mod clk_osc;
#[doc = "clk_aon_apb (rw) register accessor: AON APB Function Clock\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_aon_apb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_aon_apb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_aon_apb`]
module"]
#[doc(alias = "clk_aon_apb")]
pub type ClkAonApb = crate::Reg<clk_aon_apb::ClkAonApbSpec>;
#[doc = "AON APB Function Clock"]
pub mod clk_aon_apb;
#[doc = "clk_ahb_gmac5 (rw) register accessor: AHB GMAC5 Clock\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_ahb_gmac5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_ahb_gmac5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_ahb_gmac5`]
module"]
#[doc(alias = "clk_ahb_gmac5")]
pub type ClkAhbGmac5 = crate::Reg<clk_ahb_gmac5::ClkAhbGmac5Spec>;
#[doc = "AHB GMAC5 Clock"]
pub mod clk_ahb_gmac5;
#[doc = "clk_axi_gmac5 (rw) register accessor: AXI GMAC5 Clock\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_axi_gmac5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_axi_gmac5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_axi_gmac5`]
module"]
#[doc(alias = "clk_axi_gmac5")]
pub type ClkAxiGmac5 = crate::Reg<clk_axi_gmac5::ClkAxiGmac5Spec>;
#[doc = "AXI GMAC5 Clock"]
pub mod clk_axi_gmac5;
#[doc = "clk_gmac0_rmii_rtx (rw) register accessor: GMAC0 RMII RTX Clock\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_gmac0_rmii_rtx::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_gmac0_rmii_rtx::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_gmac0_rmii_rtx`]
module"]
#[doc(alias = "clk_gmac0_rmii_rtx")]
pub type ClkGmac0RmiiRtx = crate::Reg<clk_gmac0_rmii_rtx::ClkGmac0RmiiRtxSpec>;
#[doc = "GMAC0 RMII RTX Clock"]
pub mod clk_gmac0_rmii_rtx;
#[doc = "clk_gmac5_axi64_tx (rw) register accessor: GMAC5 AXI64 Clock Transmitter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_gmac5_axi64_tx::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_gmac5_axi64_tx::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_gmac5_axi64_tx`]
module"]
#[doc(alias = "clk_gmac5_axi64_tx")]
pub type ClkGmac5Axi64Tx = crate::Reg<clk_gmac5_axi64_tx::ClkGmac5Axi64TxSpec>;
#[doc = "GMAC5 AXI64 Clock Transmitter"]
pub mod clk_gmac5_axi64_tx;
#[doc = "clk_gmac5_axi64_txi (rw) register accessor: GMAC5 AXI64 Clock Transmission Inverter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_gmac5_axi64_txi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_gmac5_axi64_txi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_gmac5_axi64_txi`]
module"]
#[doc(alias = "clk_gmac5_axi64_txi")]
pub type ClkGmac5Axi64Txi = crate::Reg<clk_gmac5_axi64_txi::ClkGmac5Axi64TxiSpec>;
#[doc = "GMAC5 AXI64 Clock Transmission Inverter"]
pub mod clk_gmac5_axi64_txi;
#[doc = "clk_gmac5_axi64_rx (rw) register accessor: GMAC5 AXI64 Clock Receiver\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_gmac5_axi64_rx::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_gmac5_axi64_rx::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_gmac5_axi64_rx`]
module"]
#[doc(alias = "clk_gmac5_axi64_rx")]
pub type ClkGmac5Axi64Rx = crate::Reg<clk_gmac5_axi64_rx::ClkGmac5Axi64RxSpec>;
#[doc = "GMAC5 AXI64 Clock Receiver"]
pub mod clk_gmac5_axi64_rx;
#[doc = "clk_gmac5_axi64_rxi (rw) register accessor: GMAC5 AXI64 Clock Receiving Inverter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_gmac5_axi64_rxi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_gmac5_axi64_rxi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_gmac5_axi64_rxi`]
module"]
#[doc(alias = "clk_gmac5_axi64_rxi")]
pub type ClkGmac5Axi64Rxi = crate::Reg<clk_gmac5_axi64_rxi::ClkGmac5Axi64RxiSpec>;
#[doc = "GMAC5 AXI64 Clock Receiving Inverter"]
pub mod clk_gmac5_axi64_rxi;
#[doc = "clk_optc_apb (rw) register accessor: OPTC APB Clock\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_optc_apb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_optc_apb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_optc_apb`]
module"]
#[doc(alias = "clk_optc_apb")]
pub type ClkOptcApb = crate::Reg<clk_optc_apb::ClkOptcApbSpec>;
#[doc = "OPTC APB Clock"]
pub mod clk_optc_apb;
#[doc = "clk_rtc_hms_apb (rw) register accessor: RTC HMS APB Clock\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_rtc_hms_apb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_rtc_hms_apb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_rtc_hms_apb`]
module"]
#[doc(alias = "clk_rtc_hms_apb")]
pub type ClkRtcHmsApb = crate::Reg<clk_rtc_hms_apb::ClkRtcHmsApbSpec>;
#[doc = "RTC HMS APB Clock"]
pub mod clk_rtc_hms_apb;
#[doc = "clk_rtc_internal (rw) register accessor: RTC Internal Clock\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_rtc_internal::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_rtc_internal::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_rtc_internal`]
module"]
#[doc(alias = "clk_rtc_internal")]
pub type ClkRtcInternal = crate::Reg<clk_rtc_internal::ClkRtcInternalSpec>;
#[doc = "RTC Internal Clock"]
pub mod clk_rtc_internal;
#[doc = "clk_rtc_hms_osc32k (rw) register accessor: RTC HMS Clock Oscillator 32K\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_rtc_hms_osc32k::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_rtc_hms_osc32k::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_rtc_hms_osc32k`]
module"]
#[doc(alias = "clk_rtc_hms_osc32k")]
pub type ClkRtcHmsOsc32k = crate::Reg<clk_rtc_hms_osc32k::ClkRtcHmsOsc32kSpec>;
#[doc = "RTC HMS Clock Oscillator 32K"]
pub mod clk_rtc_hms_osc32k;
#[doc = "clk_rtc_hms_cal (rw) register accessor: RTC HMS Clock Calculator\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_rtc_hms_cal::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_rtc_hms_cal::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_rtc_hms_cal`]
module"]
#[doc(alias = "clk_rtc_hms_cal")]
pub type ClkRtcHmsCal = crate::Reg<clk_rtc_hms_cal::ClkRtcHmsCalSpec>;
#[doc = "RTC HMS Clock Calculator"]
pub mod clk_rtc_hms_cal;
#[doc = "soft_rst_addr_sel (rw) register accessor: Software RESET Address Selector\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`soft_rst_addr_sel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`soft_rst_addr_sel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@soft_rst_addr_sel`]
module"]
#[doc(alias = "soft_rst_addr_sel")]
pub type SoftRstAddrSel = crate::Reg<soft_rst_addr_sel::SoftRstAddrSelSpec>;
#[doc = "Software RESET Address Selector"]
pub mod soft_rst_addr_sel;
#[doc = "aoncrg_rst_status (rw) register accessor: AONCRG RESET Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aoncrg_rst_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`aoncrg_rst_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@aoncrg_rst_status`]
module"]
#[doc(alias = "aoncrg_rst_status")]
pub type AoncrgRstStatus = crate::Reg<aoncrg_rst_status::AoncrgRstStatusSpec>;
#[doc = "AONCRG RESET Status"]
pub mod aoncrg_rst_status;
