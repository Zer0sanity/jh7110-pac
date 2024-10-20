#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    clk_hifi4_core: ClkHifi4Core,
    clk_usb_apb: ClkUsbApb,
    clk_usb_utmi_apb: ClkUsbUtmiApb,
    clk_usb_axi: ClkUsbAxi,
    clk_usb_ipm: ClkUsbIpm,
    clk_usb_stb: ClkUsbStb,
    clk_usb_app125: ClkUsbApp125,
    clk_usb_refclk: ClkUsbRefclk,
    clk_pcie: [ClkPcie; 2],
    clk_pcie_u1_slv_dec_main: ClkPcieU1SlvDecMain,
    clk_sec_hclk: ClkSecHclk,
    clk_sec_ahb: ClkSecAhb,
    clk_stg_mtrx: [ClkStgMtrx; 2],
    clk_stg_mtrx_group1_hifi: ClkStgMtrxGroup1Hifi,
    clk_e2_rtc: ClkE2Rtc,
    clk_e2_core: ClkE2Core,
    clk_e2_dbg: ClkE2Dbg,
    clk_dma_axi: ClkDmaAxi,
    clk_dma_ahb: ClkDmaAhb,
    rst: [Rst; 2],
}
impl RegisterBlock {
    #[doc = "0x00 - Clock HIFI4 Core"]
    #[inline(always)]
    pub const fn clk_hifi4_core(&self) -> &ClkHifi4Core {
        &self.clk_hifi4_core
    }
    #[doc = "0x04 - Clock USB APB"]
    #[inline(always)]
    pub const fn clk_usb_apb(&self) -> &ClkUsbApb {
        &self.clk_usb_apb
    }
    #[doc = "0x08 - Clock USB UTMI APB"]
    #[inline(always)]
    pub const fn clk_usb_utmi_apb(&self) -> &ClkUsbUtmiApb {
        &self.clk_usb_utmi_apb
    }
    #[doc = "0x0c - Clock USB AXI"]
    #[inline(always)]
    pub const fn clk_usb_axi(&self) -> &ClkUsbAxi {
        &self.clk_usb_axi
    }
    #[doc = "0x10 - Clock USB IPM"]
    #[inline(always)]
    pub const fn clk_usb_ipm(&self) -> &ClkUsbIpm {
        &self.clk_usb_ipm
    }
    #[doc = "0x14 - Clock USB STB"]
    #[inline(always)]
    pub const fn clk_usb_stb(&self) -> &ClkUsbStb {
        &self.clk_usb_stb
    }
    #[doc = "0x18 - Clock USB APP 125"]
    #[inline(always)]
    pub const fn clk_usb_app125(&self) -> &ClkUsbApp125 {
        &self.clk_usb_app125
    }
    #[doc = "0x1c - Clock USB REFCLK"]
    #[inline(always)]
    pub const fn clk_usb_refclk(&self) -> &ClkUsbRefclk {
        &self.clk_usb_refclk
    }
    #[doc = "0x20..0x38 - Clock PCIe configuration"]
    #[inline(always)]
    pub const fn clk_pcie(&self, n: usize) -> &ClkPcie {
        &self.clk_pcie[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x20..0x38 - Clock PCIe configuration"]
    #[inline(always)]
    pub fn clk_pcie_iter(&self) -> impl Iterator<Item = &ClkPcie> {
        self.clk_pcie.iter()
    }
    #[doc = "0x20..0x2c - Clock PCIe configuration"]
    #[inline(always)]
    pub const fn clk_pcie_u0(&self) -> &ClkPcie {
        self.clk_pcie(0)
    }
    #[doc = "0x2c..0x38 - Clock PCIe configuration"]
    #[inline(always)]
    pub const fn clk_pcie_u1(&self) -> &ClkPcie {
        self.clk_pcie(1)
    }
    #[doc = "0x38 - Clock PCIe01 SLV DEC Main"]
    #[inline(always)]
    pub const fn clk_pcie_u1_slv_dec_main(&self) -> &ClkPcieU1SlvDecMain {
        &self.clk_pcie_u1_slv_dec_main
    }
    #[doc = "0x3c - Clock SEC HCLK"]
    #[inline(always)]
    pub const fn clk_sec_hclk(&self) -> &ClkSecHclk {
        &self.clk_sec_hclk
    }
    #[doc = "0x40 - Clock SEC AHB"]
    #[inline(always)]
    pub const fn clk_sec_ahb(&self) -> &ClkSecAhb {
        &self.clk_sec_ahb
    }
    #[doc = "0x44..0x5c - Clock STG Matrix Group configuration"]
    #[inline(always)]
    pub const fn clk_stg_mtrx(&self, n: usize) -> &ClkStgMtrx {
        &self.clk_stg_mtrx[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x44..0x5c - Clock STG Matrix Group configuration"]
    #[inline(always)]
    pub fn clk_stg_mtrx_iter(&self) -> impl Iterator<Item = &ClkStgMtrx> {
        self.clk_stg_mtrx.iter()
    }
    #[doc = "0x44..0x50 - Clock STG Matrix Group configuration"]
    #[inline(always)]
    pub const fn clk_stg_mtrx_group0(&self) -> &ClkStgMtrx {
        self.clk_stg_mtrx(0)
    }
    #[doc = "0x50..0x5c - Clock STG Matrix Group configuration"]
    #[inline(always)]
    pub const fn clk_stg_mtrx_group1(&self) -> &ClkStgMtrx {
        self.clk_stg_mtrx(1)
    }
    #[doc = "0x5c - Clock STG Matrix Group 1 HIFI"]
    #[inline(always)]
    pub const fn clk_stg_mtrx_group1_hifi(&self) -> &ClkStgMtrxGroup1Hifi {
        &self.clk_stg_mtrx_group1_hifi
    }
    #[doc = "0x60 - Clock E2 RTC"]
    #[inline(always)]
    pub const fn clk_e2_rtc(&self) -> &ClkE2Rtc {
        &self.clk_e2_rtc
    }
    #[doc = "0x64 - Clock E2 Core"]
    #[inline(always)]
    pub const fn clk_e2_core(&self) -> &ClkE2Core {
        &self.clk_e2_core
    }
    #[doc = "0x68 - Clock E2 Debug"]
    #[inline(always)]
    pub const fn clk_e2_dbg(&self) -> &ClkE2Dbg {
        &self.clk_e2_dbg
    }
    #[doc = "0x6c - Clock DMA AXI"]
    #[inline(always)]
    pub const fn clk_dma_axi(&self) -> &ClkDmaAxi {
        &self.clk_dma_axi
    }
    #[doc = "0x70 - Clock DMA AHB"]
    #[inline(always)]
    pub const fn clk_dma_ahb(&self) -> &ClkDmaAhb {
        &self.clk_dma_ahb
    }
    #[doc = "0x74..0x7c - STGCRG RESET"]
    #[inline(always)]
    pub const fn rst(&self, n: usize) -> &Rst {
        &self.rst[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x74..0x7c - STGCRG RESET"]
    #[inline(always)]
    pub fn rst_iter(&self) -> impl Iterator<Item = &Rst> {
        self.rst.iter()
    }
    #[doc = "0x74 - STGCRG RESET"]
    #[inline(always)]
    pub const fn rst_soft_addr_sel(&self) -> &Rst {
        self.rst(0)
    }
    #[doc = "0x78 - STGCRG RESET"]
    #[inline(always)]
    pub const fn rst_stgcrg_stat(&self) -> &Rst {
        self.rst(1)
    }
}
#[doc = "clk_hifi4_core (rw) register accessor: Clock HIFI4 Core\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_hifi4_core::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_hifi4_core::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_hifi4_core`]
module"]
#[doc(alias = "clk_hifi4_core")]
pub type ClkHifi4Core = crate::Reg<clk_hifi4_core::ClkHifi4CoreSpec>;
#[doc = "Clock HIFI4 Core"]
pub mod clk_hifi4_core;
#[doc = "clk_usb_apb (rw) register accessor: Clock USB APB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_usb_apb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_usb_apb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_usb_apb`]
module"]
#[doc(alias = "clk_usb_apb")]
pub type ClkUsbApb = crate::Reg<clk_usb_apb::ClkUsbApbSpec>;
#[doc = "Clock USB APB"]
pub mod clk_usb_apb;
#[doc = "clk_usb_utmi_apb (rw) register accessor: Clock USB UTMI APB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_usb_utmi_apb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_usb_utmi_apb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_usb_utmi_apb`]
module"]
#[doc(alias = "clk_usb_utmi_apb")]
pub type ClkUsbUtmiApb = crate::Reg<clk_usb_utmi_apb::ClkUsbUtmiApbSpec>;
#[doc = "Clock USB UTMI APB"]
pub mod clk_usb_utmi_apb;
#[doc = "clk_usb_axi (rw) register accessor: Clock USB AXI\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_usb_axi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_usb_axi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_usb_axi`]
module"]
#[doc(alias = "clk_usb_axi")]
pub type ClkUsbAxi = crate::Reg<clk_usb_axi::ClkUsbAxiSpec>;
#[doc = "Clock USB AXI"]
pub mod clk_usb_axi;
#[doc = "clk_usb_ipm (rw) register accessor: Clock USB IPM\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_usb_ipm::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_usb_ipm::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_usb_ipm`]
module"]
#[doc(alias = "clk_usb_ipm")]
pub type ClkUsbIpm = crate::Reg<clk_usb_ipm::ClkUsbIpmSpec>;
#[doc = "Clock USB IPM"]
pub mod clk_usb_ipm;
#[doc = "clk_usb_stb (rw) register accessor: Clock USB STB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_usb_stb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_usb_stb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_usb_stb`]
module"]
#[doc(alias = "clk_usb_stb")]
pub type ClkUsbStb = crate::Reg<clk_usb_stb::ClkUsbStbSpec>;
#[doc = "Clock USB STB"]
pub mod clk_usb_stb;
#[doc = "clk_usb_app125 (rw) register accessor: Clock USB APP 125\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_usb_app125::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_usb_app125::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_usb_app125`]
module"]
#[doc(alias = "clk_usb_app125")]
pub type ClkUsbApp125 = crate::Reg<clk_usb_app125::ClkUsbApp125Spec>;
#[doc = "Clock USB APP 125"]
pub mod clk_usb_app125;
#[doc = "clk_usb_refclk (rw) register accessor: Clock USB REFCLK\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_usb_refclk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_usb_refclk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_usb_refclk`]
module"]
#[doc(alias = "clk_usb_refclk")]
pub type ClkUsbRefclk = crate::Reg<clk_usb_refclk::ClkUsbRefclkSpec>;
#[doc = "Clock USB REFCLK"]
pub mod clk_usb_refclk;
#[doc = "Clock PCIe configuration"]
pub use self::clk_pcie::ClkPcie;
#[doc = r"Cluster"]
#[doc = "Clock PCIe configuration"]
pub mod clk_pcie;
#[doc = "clk_pcie_u1_slv_dec_main (rw) register accessor: Clock PCIe01 SLV DEC Main\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_pcie_u1_slv_dec_main::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_pcie_u1_slv_dec_main::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_pcie_u1_slv_dec_main`]
module"]
#[doc(alias = "clk_pcie_u1_slv_dec_main")]
pub type ClkPcieU1SlvDecMain = crate::Reg<clk_pcie_u1_slv_dec_main::ClkPcieU1SlvDecMainSpec>;
#[doc = "Clock PCIe01 SLV DEC Main"]
pub mod clk_pcie_u1_slv_dec_main;
#[doc = "clk_sec_hclk (rw) register accessor: Clock SEC HCLK\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_sec_hclk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_sec_hclk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_sec_hclk`]
module"]
#[doc(alias = "clk_sec_hclk")]
pub type ClkSecHclk = crate::Reg<clk_sec_hclk::ClkSecHclkSpec>;
#[doc = "Clock SEC HCLK"]
pub mod clk_sec_hclk;
#[doc = "clk_sec_ahb (rw) register accessor: Clock SEC AHB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_sec_ahb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_sec_ahb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_sec_ahb`]
module"]
#[doc(alias = "clk_sec_ahb")]
pub type ClkSecAhb = crate::Reg<clk_sec_ahb::ClkSecAhbSpec>;
#[doc = "Clock SEC AHB"]
pub mod clk_sec_ahb;
#[doc = "Clock STG Matrix Group configuration"]
pub use self::clk_stg_mtrx::ClkStgMtrx;
#[doc = r"Cluster"]
#[doc = "Clock STG Matrix Group configuration"]
pub mod clk_stg_mtrx;
#[doc = "clk_stg_mtrx_group1_hifi (rw) register accessor: Clock STG Matrix Group 1 HIFI\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_stg_mtrx_group1_hifi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_stg_mtrx_group1_hifi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_stg_mtrx_group1_hifi`]
module"]
#[doc(alias = "clk_stg_mtrx_group1_hifi")]
pub type ClkStgMtrxGroup1Hifi = crate::Reg<clk_stg_mtrx_group1_hifi::ClkStgMtrxGroup1HifiSpec>;
#[doc = "Clock STG Matrix Group 1 HIFI"]
pub mod clk_stg_mtrx_group1_hifi;
#[doc = "clk_e2_rtc (rw) register accessor: Clock E2 RTC\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_e2_rtc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_e2_rtc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_e2_rtc`]
module"]
#[doc(alias = "clk_e2_rtc")]
pub type ClkE2Rtc = crate::Reg<clk_e2_rtc::ClkE2RtcSpec>;
#[doc = "Clock E2 RTC"]
pub mod clk_e2_rtc;
#[doc = "clk_e2_core (rw) register accessor: Clock E2 Core\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_e2_core::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_e2_core::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_e2_core`]
module"]
#[doc(alias = "clk_e2_core")]
pub type ClkE2Core = crate::Reg<clk_e2_core::ClkE2CoreSpec>;
#[doc = "Clock E2 Core"]
pub mod clk_e2_core;
#[doc = "clk_e2_dbg (rw) register accessor: Clock E2 Debug\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_e2_dbg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_e2_dbg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_e2_dbg`]
module"]
#[doc(alias = "clk_e2_dbg")]
pub type ClkE2Dbg = crate::Reg<clk_e2_dbg::ClkE2DbgSpec>;
#[doc = "Clock E2 Debug"]
pub mod clk_e2_dbg;
#[doc = "clk_dma_axi (rw) register accessor: Clock DMA AXI\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_dma_axi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_dma_axi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_dma_axi`]
module"]
#[doc(alias = "clk_dma_axi")]
pub type ClkDmaAxi = crate::Reg<clk_dma_axi::ClkDmaAxiSpec>;
#[doc = "Clock DMA AXI"]
pub mod clk_dma_axi;
#[doc = "clk_dma_ahb (rw) register accessor: Clock DMA AHB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_dma_ahb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_dma_ahb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_dma_ahb`]
module"]
#[doc(alias = "clk_dma_ahb")]
pub type ClkDmaAhb = crate::Reg<clk_dma_ahb::ClkDmaAhbSpec>;
#[doc = "Clock DMA AHB"]
pub mod clk_dma_ahb;
#[doc = "rst (rw) register accessor: STGCRG RESET\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rst`]
module"]
#[doc(alias = "rst")]
pub type Rst = crate::Reg<rst::RstSpec>;
#[doc = "STGCRG RESET"]
pub mod rst;
