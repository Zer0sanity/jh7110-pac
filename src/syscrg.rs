#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    clk_cpu: ClkCpu,
    clk_gpu: ClkGpu,
    clk_peripheral: ClkPeripheral,
    clk_bus: ClkBus,
    clk_nocstg_bus: ClkNocstgBus,
    clk_axi_cfg0: ClkAxiCfg0,
    clk_stg_axiahb: ClkStgAxiahb,
    clk_ahb: [ClkAhb; 2],
    clk_apb_bus: ClkApbBus,
    clk_apb0: ClkApb0,
    clk_pll: [ClkPll; 3],
    clk_audio: ClkAudio,
    clk_mclk: ClkMclk,
    clk_isp: ClkIsp,
    clk_gclk: ClkGclk,
    clk_u7mc: ClkU7mc,
    clk_noc_bus: ClkNocBus,
    clk_osc_div2: ClkOscDiv2,
    clk_pll1_div4: ClkPll1Div4,
    clk_pll1_div8: ClkPll1Div8,
    clk_ddr: ClkDdr,
    clk_img_gpu: ClkImgGpu,
    clk_isp_dom: ClkIspDom,
    clk_hifi4: ClkHifi4,
    clk_axi_cfg1_dec: ClkAxiCfg1Dec,
    clk_vout: ClkVout,
    clk_jpeg: ClkJpeg,
    clk_vdec: ClkVdec,
    clk_venc: ClkVenc,
    clk_axi_cfg0_dec: ClkAxiCfg0Dec,
    clk_aximem_128b_axi: ClkAximem128bAxi,
    clk_qspi: ClkQspi,
    clk_sd: ClkSd,
    clk_usb_125m: ClkUsb125m,
    clk_noc_stg_axi: ClkNocStgAxi,
    clk_gmac: ClkGmac,
    clk_pclk: ClkPclk,
    clk_mbox_apb: ClkMboxApb,
    clk_internal_ctrl_apb: ClkInternalCtrlApb,
    clk_can_ctrl: [ClkCanCtrl; 2],
    clk_pwm_apb: ClkPwmApb,
    clk_wdt: ClkWdt,
    clk_tim: ClkTim,
    clk_temp_sensor: ClkTempSensor,
    clk_spi: ClkSpi,
    clk_i2c: ClkI2c,
    clk_uart: ClkUart,
    clk_pwmdac: ClkPwmdac,
    clk_spdif: ClkSpdif,
    clk_i2s: [ClkI2s; 3],
    clk_pdm: ClkPdm,
    clk_tdm: ClkTdm,
    clk_jtag_trng: ClkJtagTrng,
    rst: Rst,
}
impl RegisterBlock {
    #[doc = "0x00..0x0c - Clock CPU registers"]
    #[inline(always)]
    pub const fn clk_cpu(&self) -> &ClkCpu {
        &self.clk_cpu
    }
    #[doc = "0x0c - Clock GPU registers"]
    #[inline(always)]
    pub const fn clk_gpu(&self) -> &ClkGpu {
        &self.clk_gpu
    }
    #[doc = "0x10 - Clock Peripheral registers"]
    #[inline(always)]
    pub const fn clk_peripheral(&self) -> &ClkPeripheral {
        &self.clk_peripheral
    }
    #[doc = "0x14 - Clock Bus registers"]
    #[inline(always)]
    pub const fn clk_bus(&self) -> &ClkBus {
        &self.clk_bus
    }
    #[doc = "0x18 - Clock NOC STG Bus"]
    #[inline(always)]
    pub const fn clk_nocstg_bus(&self) -> &ClkNocstgBus {
        &self.clk_nocstg_bus
    }
    #[doc = "0x1c - Clock AXI Configuration"]
    #[inline(always)]
    pub const fn clk_axi_cfg0(&self) -> &ClkAxiCfg0 {
        &self.clk_axi_cfg0
    }
    #[doc = "0x20 - Clock STG AXI AHB"]
    #[inline(always)]
    pub const fn clk_stg_axiahb(&self) -> &ClkStgAxiahb {
        &self.clk_stg_axiahb
    }
    #[doc = "0x24..0x2c - Clock AHB"]
    #[inline(always)]
    pub const fn clk_ahb(&self, n: usize) -> &ClkAhb {
        &self.clk_ahb[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x24..0x2c - Clock AHB"]
    #[inline(always)]
    pub fn clk_ahb_iter(&self) -> impl Iterator<Item = &ClkAhb> {
        self.clk_ahb.iter()
    }
    #[doc = "0x2c - Clock APB Bus"]
    #[inline(always)]
    pub const fn clk_apb_bus(&self) -> &ClkApbBus {
        &self.clk_apb_bus
    }
    #[doc = "0x30 - Clock APB"]
    #[inline(always)]
    pub const fn clk_apb0(&self) -> &ClkApb0 {
        &self.clk_apb0
    }
    #[doc = "0x34..0x40 - Clock PLL Divider 2"]
    #[inline(always)]
    pub const fn clk_pll(&self, n: usize) -> &ClkPll {
        &self.clk_pll[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x34..0x40 - Clock PLL Divider 2"]
    #[inline(always)]
    pub fn clk_pll_iter(&self) -> impl Iterator<Item = &ClkPll> {
        self.clk_pll.iter()
    }
    #[doc = "0x34 - Clock PLL Divider 2"]
    #[inline(always)]
    pub const fn clk_pll0_div2(&self) -> &ClkPll {
        self.clk_pll(0)
    }
    #[doc = "0x38 - Clock PLL Divider 2"]
    #[inline(always)]
    pub const fn clk_pll1_div2(&self) -> &ClkPll {
        self.clk_pll(1)
    }
    #[doc = "0x3c - Clock PLL Divider 2"]
    #[inline(always)]
    pub const fn clk_pll2_div2(&self) -> &ClkPll {
        self.clk_pll(2)
    }
    #[doc = "0x40 - Clock Audio registers"]
    #[inline(always)]
    pub const fn clk_audio(&self) -> &ClkAudio {
        &self.clk_audio
    }
    #[doc = "0x44..0x50 - Clock MCLK registers"]
    #[inline(always)]
    pub const fn clk_mclk(&self) -> &ClkMclk {
        &self.clk_mclk
    }
    #[doc = "0x50..0x58 - Clock ISP registers"]
    #[inline(always)]
    pub const fn clk_isp(&self) -> &ClkIsp {
        &self.clk_isp
    }
    #[doc = "0x58..0x64 - Clock GCLK registers"]
    #[inline(always)]
    pub const fn clk_gclk(&self) -> &ClkGclk {
        &self.clk_gclk
    }
    #[doc = "0x64..0x98 - Clock U7MC registers"]
    #[inline(always)]
    pub const fn clk_u7mc(&self) -> &ClkU7mc {
        &self.clk_u7mc
    }
    #[doc = "0x98..0xa0 - Clock NOC Bus registers"]
    #[inline(always)]
    pub const fn clk_noc_bus(&self) -> &ClkNocBus {
        &self.clk_noc_bus
    }
    #[doc = "0xa0 - clk_osc_div2"]
    #[inline(always)]
    pub const fn clk_osc_div2(&self) -> &ClkOscDiv2 {
        &self.clk_osc_div2
    }
    #[doc = "0xa4 - clk_pll1_div4"]
    #[inline(always)]
    pub const fn clk_pll1_div4(&self) -> &ClkPll1Div4 {
        &self.clk_pll1_div4
    }
    #[doc = "0xa8 - clk_pll1_div8"]
    #[inline(always)]
    pub const fn clk_pll1_div8(&self) -> &ClkPll1Div8 {
        &self.clk_pll1_div8
    }
    #[doc = "0xac..0xb4 - Clock DDR registers"]
    #[inline(always)]
    pub const fn clk_ddr(&self) -> &ClkDdr {
        &self.clk_ddr
    }
    #[doc = "0xb4..0xcc - Clock IMG GPU registers"]
    #[inline(always)]
    pub const fn clk_img_gpu(&self) -> &ClkImgGpu {
        &self.clk_img_gpu
    }
    #[doc = "0xcc..0xd8 - Clock ISP DOM registers"]
    #[inline(always)]
    pub const fn clk_isp_dom(&self) -> &ClkIspDom {
        &self.clk_isp_dom
    }
    #[doc = "0xd8..0xe0 - Clock HIFI4 registers"]
    #[inline(always)]
    pub const fn clk_hifi4(&self) -> &ClkHifi4 {
        &self.clk_hifi4
    }
    #[doc = "0xe0..0xe8 - Clock AXI CFG 1 DEC registers"]
    #[inline(always)]
    pub const fn clk_axi_cfg1_dec(&self) -> &ClkAxiCfg1Dec {
        &self.clk_axi_cfg1_dec
    }
    #[doc = "0xe8..0x104 - Clock Video Output registers"]
    #[inline(always)]
    pub const fn clk_vout(&self) -> &ClkVout {
        &self.clk_vout
    }
    #[doc = "0x104..0x114 - Clock CODAJ12 registers"]
    #[inline(always)]
    pub const fn clk_jpeg(&self) -> &ClkJpeg {
        &self.clk_jpeg
    }
    #[doc = "0x114..0x134 - Clock Video Decoder registers"]
    #[inline(always)]
    pub const fn clk_vdec(&self) -> &ClkVdec {
        &self.clk_vdec
    }
    #[doc = "0x134..0x14c - Clock Video Encoder registers"]
    #[inline(always)]
    pub const fn clk_venc(&self) -> &ClkVenc {
        &self.clk_venc
    }
    #[doc = "0x14c..0x158 - Clock AXI CFG 0 DEC registers"]
    #[inline(always)]
    pub const fn clk_axi_cfg0_dec(&self) -> &ClkAxiCfg0Dec {
        &self.clk_axi_cfg0_dec
    }
    #[doc = "0x158 - Clock AXIMEM 128B AXI"]
    #[inline(always)]
    pub const fn clk_aximem_128b_axi(&self) -> &ClkAximem128bAxi {
        &self.clk_aximem_128b_axi
    }
    #[doc = "0x15c..0x16c - Clock QSPI registers"]
    #[inline(always)]
    pub const fn clk_qspi(&self) -> &ClkQspi {
        &self.clk_qspi
    }
    #[doc = "0x16c..0x17c - Clock SD registers"]
    #[inline(always)]
    pub const fn clk_sd(&self) -> &ClkSd {
        &self.clk_sd
    }
    #[doc = "0x17c - Clock USB 125M"]
    #[inline(always)]
    pub const fn clk_usb_125m(&self) -> &ClkUsb125m {
        &self.clk_usb_125m
    }
    #[doc = "0x180 - Clock NOC STG AXI"]
    #[inline(always)]
    pub const fn clk_noc_stg_axi(&self) -> &ClkNocStgAxi {
        &self.clk_noc_stg_axi
    }
    #[doc = "0x184..0x1c0 - Clock GMAC registers"]
    #[inline(always)]
    pub const fn clk_gmac(&self) -> &ClkGmac {
        &self.clk_gmac
    }
    #[doc = "0x1c0 - Clock PCLK"]
    #[inline(always)]
    pub const fn clk_pclk(&self) -> &ClkPclk {
        &self.clk_pclk
    }
    #[doc = "0x1c4 - Clock Mailbox APB"]
    #[inline(always)]
    pub const fn clk_mbox_apb(&self) -> &ClkMboxApb {
        &self.clk_mbox_apb
    }
    #[doc = "0x1c8 - Clock Internal Controller APB"]
    #[inline(always)]
    pub const fn clk_internal_ctrl_apb(&self) -> &ClkInternalCtrlApb {
        &self.clk_internal_ctrl_apb
    }
    #[doc = "0x1cc..0x1e4 - Clock CAN Controller"]
    #[inline(always)]
    pub const fn clk_can_ctrl(&self, n: usize) -> &ClkCanCtrl {
        &self.clk_can_ctrl[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1cc..0x1e4 - Clock CAN Controller"]
    #[inline(always)]
    pub fn clk_can_ctrl_iter(&self) -> impl Iterator<Item = &ClkCanCtrl> {
        self.clk_can_ctrl.iter()
    }
    #[doc = "0x1cc..0x1d8 - Clock CAN Controller"]
    #[inline(always)]
    pub const fn clk_can_ctrl_u0(&self) -> &ClkCanCtrl {
        self.clk_can_ctrl(0)
    }
    #[doc = "0x1d8..0x1e4 - Clock CAN Controller"]
    #[inline(always)]
    pub const fn clk_can_ctrl_u1(&self) -> &ClkCanCtrl {
        self.clk_can_ctrl(1)
    }
    #[doc = "0x1e4 - Clock PWM APB"]
    #[inline(always)]
    pub const fn clk_pwm_apb(&self) -> &ClkPwmApb {
        &self.clk_pwm_apb
    }
    #[doc = "0x1e8..0x1f0 - Clock WDT registers"]
    #[inline(always)]
    pub const fn clk_wdt(&self) -> &ClkWdt {
        &self.clk_wdt
    }
    #[doc = "0x1f0..0x204 - Clock Timer"]
    #[inline(always)]
    pub const fn clk_tim(&self) -> &ClkTim {
        &self.clk_tim
    }
    #[doc = "0x204..0x20c - Clock Temperature registers"]
    #[inline(always)]
    pub const fn clk_temp_sensor(&self) -> &ClkTempSensor {
        &self.clk_temp_sensor
    }
    #[doc = "0x20c..0x228 - Clock SPI registers"]
    #[inline(always)]
    pub const fn clk_spi(&self) -> &ClkSpi {
        &self.clk_spi
    }
    #[doc = "0x228..0x244 - Clock I2C registers"]
    #[inline(always)]
    pub const fn clk_i2c(&self) -> &ClkI2c {
        &self.clk_i2c
    }
    #[doc = "0x244..0x274 - Clock UART"]
    #[inline(always)]
    pub const fn clk_uart(&self) -> &ClkUart {
        &self.clk_uart
    }
    #[doc = "0x274..0x27c - Clock PWMDAC registers"]
    #[inline(always)]
    pub const fn clk_pwmdac(&self) -> &ClkPwmdac {
        &self.clk_pwmdac
    }
    #[doc = "0x27c..0x284 - Clock SPDIF registers"]
    #[inline(always)]
    pub const fn clk_spdif(&self) -> &ClkSpdif {
        &self.clk_spdif
    }
    #[doc = "0x284..0x2d8 - Clock I2S"]
    #[inline(always)]
    pub const fn clk_i2s(&self, n: usize) -> &ClkI2s {
        &self.clk_i2s[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x284..0x2d8 - Clock I2S"]
    #[inline(always)]
    pub fn clk_i2s_iter(&self) -> impl Iterator<Item = &ClkI2s> {
        self.clk_i2s.iter()
    }
    #[doc = "0x284..0x2a0 - Clock I2S"]
    #[inline(always)]
    pub const fn clk_i2stx_u0(&self) -> &ClkI2s {
        self.clk_i2s(0)
    }
    #[doc = "0x2a0..0x2bc - Clock I2S"]
    #[inline(always)]
    pub const fn clk_i2stx_u1(&self) -> &ClkI2s {
        self.clk_i2s(1)
    }
    #[doc = "0x2bc..0x2d8 - Clock I2S"]
    #[inline(always)]
    pub const fn clk_i2srx(&self) -> &ClkI2s {
        self.clk_i2s(2)
    }
    #[doc = "0x2d8..0x2e0 - Clock PDM"]
    #[inline(always)]
    pub const fn clk_pdm(&self) -> &ClkPdm {
        &self.clk_pdm
    }
    #[doc = "0x2e0..0x2f4 - Clock TDM"]
    #[inline(always)]
    pub const fn clk_tdm(&self) -> &ClkTdm {
        &self.clk_tdm
    }
    #[doc = "0x2f4 - Clock JTAG TRNG"]
    #[inline(always)]
    pub const fn clk_jtag_trng(&self) -> &ClkJtagTrng {
        &self.clk_jtag_trng
    }
    #[doc = "0x2f8..0x318 - SYSCRG RESET registers"]
    #[inline(always)]
    pub const fn rst(&self) -> &Rst {
        &self.rst
    }
}
#[doc = "Clock CPU registers"]
pub use self::clk_cpu::ClkCpu;
#[doc = r"Cluster"]
#[doc = "Clock CPU registers"]
pub mod clk_cpu;
#[doc = "Clock GPU registers"]
pub use self::clk_gpu::ClkGpu;
#[doc = r"Cluster"]
#[doc = "Clock GPU registers"]
pub mod clk_gpu;
#[doc = "Clock Peripheral registers"]
pub use self::clk_peripheral::ClkPeripheral;
#[doc = r"Cluster"]
#[doc = "Clock Peripheral registers"]
pub mod clk_peripheral;
#[doc = "Clock Bus registers"]
pub use self::clk_bus::ClkBus;
#[doc = r"Cluster"]
#[doc = "Clock Bus registers"]
pub mod clk_bus;
#[doc = "clk_nocstg_bus (rw) register accessor: Clock NOC STG Bus\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_nocstg_bus::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_nocstg_bus::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_nocstg_bus`]
module"]
#[doc(alias = "clk_nocstg_bus")]
pub type ClkNocstgBus = crate::Reg<clk_nocstg_bus::ClkNocstgBusSpec>;
#[doc = "Clock NOC STG Bus"]
pub mod clk_nocstg_bus;
#[doc = "clk_axi_cfg0 (rw) register accessor: Clock AXI Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_axi_cfg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_axi_cfg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_axi_cfg0`]
module"]
#[doc(alias = "clk_axi_cfg0")]
pub type ClkAxiCfg0 = crate::Reg<clk_axi_cfg0::ClkAxiCfg0Spec>;
#[doc = "Clock AXI Configuration"]
pub mod clk_axi_cfg0;
#[doc = "clk_stg_axiahb (rw) register accessor: Clock STG AXI AHB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_stg_axiahb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_stg_axiahb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_stg_axiahb`]
module"]
#[doc(alias = "clk_stg_axiahb")]
pub type ClkStgAxiahb = crate::Reg<clk_stg_axiahb::ClkStgAxiahbSpec>;
#[doc = "Clock STG AXI AHB"]
pub mod clk_stg_axiahb;
#[doc = "clk_ahb (rw) register accessor: Clock AHB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_ahb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_ahb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_ahb`]
module"]
#[doc(alias = "clk_ahb")]
pub type ClkAhb = crate::Reg<clk_ahb::ClkAhbSpec>;
#[doc = "Clock AHB"]
pub mod clk_ahb;
#[doc = "clk_apb_bus (rw) register accessor: Clock APB Bus\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_apb_bus::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_apb_bus::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_apb_bus`]
module"]
#[doc(alias = "clk_apb_bus")]
pub type ClkApbBus = crate::Reg<clk_apb_bus::ClkApbBusSpec>;
#[doc = "Clock APB Bus"]
pub mod clk_apb_bus;
#[doc = "clk_apb0 (rw) register accessor: Clock APB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_apb0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_apb0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_apb0`]
module"]
#[doc(alias = "clk_apb0")]
pub type ClkApb0 = crate::Reg<clk_apb0::ClkApb0Spec>;
#[doc = "Clock APB"]
pub mod clk_apb0;
#[doc = "clk_pll (rw) register accessor: Clock PLL Divider 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_pll::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_pll::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_pll`]
module"]
#[doc(alias = "clk_pll")]
pub type ClkPll = crate::Reg<clk_pll::ClkPllSpec>;
#[doc = "Clock PLL Divider 2"]
pub mod clk_pll;
#[doc = "Clock Audio registers"]
pub use self::clk_audio::ClkAudio;
#[doc = r"Cluster"]
#[doc = "Clock Audio registers"]
pub mod clk_audio;
#[doc = "Clock MCLK registers"]
pub use self::clk_mclk::ClkMclk;
#[doc = r"Cluster"]
#[doc = "Clock MCLK registers"]
pub mod clk_mclk;
#[doc = "Clock ISP registers"]
pub use self::clk_isp::ClkIsp;
#[doc = r"Cluster"]
#[doc = "Clock ISP registers"]
pub mod clk_isp;
#[doc = "Clock GCLK registers"]
pub use self::clk_gclk::ClkGclk;
#[doc = r"Cluster"]
#[doc = "Clock GCLK registers"]
pub mod clk_gclk;
#[doc = "Clock U7MC registers"]
pub use self::clk_u7mc::ClkU7mc;
#[doc = r"Cluster"]
#[doc = "Clock U7MC registers"]
pub mod clk_u7mc;
#[doc = "Clock NOC Bus registers"]
pub use self::clk_noc_bus::ClkNocBus;
#[doc = r"Cluster"]
#[doc = "Clock NOC Bus registers"]
pub mod clk_noc_bus;
#[doc = "clk_osc_div2 (rw) register accessor: clk_osc_div2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_osc_div2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_osc_div2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_osc_div2`]
module"]
#[doc(alias = "clk_osc_div2")]
pub type ClkOscDiv2 = crate::Reg<clk_osc_div2::ClkOscDiv2Spec>;
#[doc = "clk_osc_div2"]
pub mod clk_osc_div2;
#[doc = "clk_pll1_div4 (rw) register accessor: clk_pll1_div4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_pll1_div4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_pll1_div4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_pll1_div4`]
module"]
#[doc(alias = "clk_pll1_div4")]
pub type ClkPll1Div4 = crate::Reg<clk_pll1_div4::ClkPll1Div4Spec>;
#[doc = "clk_pll1_div4"]
pub mod clk_pll1_div4;
#[doc = "clk_pll1_div8 (rw) register accessor: clk_pll1_div8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_pll1_div8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_pll1_div8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_pll1_div8`]
module"]
#[doc(alias = "clk_pll1_div8")]
pub type ClkPll1Div8 = crate::Reg<clk_pll1_div8::ClkPll1Div8Spec>;
#[doc = "clk_pll1_div8"]
pub mod clk_pll1_div8;
#[doc = "Clock DDR registers"]
pub use self::clk_ddr::ClkDdr;
#[doc = r"Cluster"]
#[doc = "Clock DDR registers"]
pub mod clk_ddr;
#[doc = "Clock IMG GPU registers"]
pub use self::clk_img_gpu::ClkImgGpu;
#[doc = r"Cluster"]
#[doc = "Clock IMG GPU registers"]
pub mod clk_img_gpu;
#[doc = "Clock ISP DOM registers"]
pub use self::clk_isp_dom::ClkIspDom;
#[doc = r"Cluster"]
#[doc = "Clock ISP DOM registers"]
pub mod clk_isp_dom;
#[doc = "Clock HIFI4 registers"]
pub use self::clk_hifi4::ClkHifi4;
#[doc = r"Cluster"]
#[doc = "Clock HIFI4 registers"]
pub mod clk_hifi4;
#[doc = "Clock AXI CFG 1 DEC registers"]
pub use self::clk_axi_cfg1_dec::ClkAxiCfg1Dec;
#[doc = r"Cluster"]
#[doc = "Clock AXI CFG 1 DEC registers"]
pub mod clk_axi_cfg1_dec;
#[doc = "Clock Video Output registers"]
pub use self::clk_vout::ClkVout;
#[doc = r"Cluster"]
#[doc = "Clock Video Output registers"]
pub mod clk_vout;
#[doc = "Clock CODAJ12 registers"]
pub use self::clk_jpeg::ClkJpeg;
#[doc = r"Cluster"]
#[doc = "Clock CODAJ12 registers"]
pub mod clk_jpeg;
#[doc = "Clock Video Decoder registers"]
pub use self::clk_vdec::ClkVdec;
#[doc = r"Cluster"]
#[doc = "Clock Video Decoder registers"]
pub mod clk_vdec;
#[doc = "Clock Video Encoder registers"]
pub use self::clk_venc::ClkVenc;
#[doc = r"Cluster"]
#[doc = "Clock Video Encoder registers"]
pub mod clk_venc;
#[doc = "Clock AXI CFG 0 DEC registers"]
pub use self::clk_axi_cfg0_dec::ClkAxiCfg0Dec;
#[doc = r"Cluster"]
#[doc = "Clock AXI CFG 0 DEC registers"]
pub mod clk_axi_cfg0_dec;
#[doc = "clk_aximem_128b_axi (rw) register accessor: Clock AXIMEM 128B AXI\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_aximem_128b_axi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_aximem_128b_axi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_aximem_128b_axi`]
module"]
#[doc(alias = "clk_aximem_128b_axi")]
pub type ClkAximem128bAxi = crate::Reg<clk_aximem_128b_axi::ClkAximem128bAxiSpec>;
#[doc = "Clock AXIMEM 128B AXI"]
pub mod clk_aximem_128b_axi;
#[doc = "Clock QSPI registers"]
pub use self::clk_qspi::ClkQspi;
#[doc = r"Cluster"]
#[doc = "Clock QSPI registers"]
pub mod clk_qspi;
#[doc = "Clock SD registers"]
pub use self::clk_sd::ClkSd;
#[doc = r"Cluster"]
#[doc = "Clock SD registers"]
pub mod clk_sd;
#[doc = "clk_usb_125m (rw) register accessor: Clock USB 125M\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_usb_125m::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_usb_125m::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_usb_125m`]
module"]
#[doc(alias = "clk_usb_125m")]
pub type ClkUsb125m = crate::Reg<clk_usb_125m::ClkUsb125mSpec>;
#[doc = "Clock USB 125M"]
pub mod clk_usb_125m;
#[doc = "clk_noc_stg_axi (rw) register accessor: Clock NOC STG AXI\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_noc_stg_axi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_noc_stg_axi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_noc_stg_axi`]
module"]
#[doc(alias = "clk_noc_stg_axi")]
pub type ClkNocStgAxi = crate::Reg<clk_noc_stg_axi::ClkNocStgAxiSpec>;
#[doc = "Clock NOC STG AXI"]
pub mod clk_noc_stg_axi;
#[doc = "Clock GMAC registers"]
pub use self::clk_gmac::ClkGmac;
#[doc = r"Cluster"]
#[doc = "Clock GMAC registers"]
pub mod clk_gmac;
#[doc = "clk_pclk (rw) register accessor: Clock PCLK\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_pclk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_pclk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_pclk`]
module"]
#[doc(alias = "clk_pclk")]
pub type ClkPclk = crate::Reg<clk_pclk::ClkPclkSpec>;
#[doc = "Clock PCLK"]
pub mod clk_pclk;
#[doc = "clk_mbox_apb (rw) register accessor: Clock Mailbox APB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_mbox_apb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_mbox_apb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_mbox_apb`]
module"]
#[doc(alias = "clk_mbox_apb")]
pub type ClkMboxApb = crate::Reg<clk_mbox_apb::ClkMboxApbSpec>;
#[doc = "Clock Mailbox APB"]
pub mod clk_mbox_apb;
#[doc = "clk_internal_ctrl_apb (rw) register accessor: Clock Internal Controller APB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_internal_ctrl_apb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_internal_ctrl_apb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_internal_ctrl_apb`]
module"]
#[doc(alias = "clk_internal_ctrl_apb")]
pub type ClkInternalCtrlApb = crate::Reg<clk_internal_ctrl_apb::ClkInternalCtrlApbSpec>;
#[doc = "Clock Internal Controller APB"]
pub mod clk_internal_ctrl_apb;
#[doc = "Clock CAN Controller"]
pub use self::clk_can_ctrl::ClkCanCtrl;
#[doc = r"Cluster"]
#[doc = "Clock CAN Controller"]
pub mod clk_can_ctrl;
#[doc = "clk_pwm_apb (rw) register accessor: Clock PWM APB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_pwm_apb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_pwm_apb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_pwm_apb`]
module"]
#[doc(alias = "clk_pwm_apb")]
pub type ClkPwmApb = crate::Reg<clk_pwm_apb::ClkPwmApbSpec>;
#[doc = "Clock PWM APB"]
pub mod clk_pwm_apb;
#[doc = "Clock WDT registers"]
pub use self::clk_wdt::ClkWdt;
#[doc = r"Cluster"]
#[doc = "Clock WDT registers"]
pub mod clk_wdt;
#[doc = "Clock Timer"]
pub use self::clk_tim::ClkTim;
#[doc = r"Cluster"]
#[doc = "Clock Timer"]
pub mod clk_tim;
#[doc = "Clock Temperature registers"]
pub use self::clk_temp_sensor::ClkTempSensor;
#[doc = r"Cluster"]
#[doc = "Clock Temperature registers"]
pub mod clk_temp_sensor;
#[doc = "Clock SPI registers"]
pub use self::clk_spi::ClkSpi;
#[doc = r"Cluster"]
#[doc = "Clock SPI registers"]
pub mod clk_spi;
#[doc = "Clock I2C registers"]
pub use self::clk_i2c::ClkI2c;
#[doc = r"Cluster"]
#[doc = "Clock I2C registers"]
pub mod clk_i2c;
#[doc = "Clock UART"]
pub use self::clk_uart::ClkUart;
#[doc = r"Cluster"]
#[doc = "Clock UART"]
pub mod clk_uart;
#[doc = "Clock PWMDAC registers"]
pub use self::clk_pwmdac::ClkPwmdac;
#[doc = r"Cluster"]
#[doc = "Clock PWMDAC registers"]
pub mod clk_pwmdac;
#[doc = "Clock SPDIF registers"]
pub use self::clk_spdif::ClkSpdif;
#[doc = r"Cluster"]
#[doc = "Clock SPDIF registers"]
pub mod clk_spdif;
#[doc = "Clock I2S"]
pub use self::clk_i2s::ClkI2s;
#[doc = r"Cluster"]
#[doc = "Clock I2S"]
pub mod clk_i2s;
#[doc = "Clock PDM"]
pub use self::clk_pdm::ClkPdm;
#[doc = r"Cluster"]
#[doc = "Clock PDM"]
pub mod clk_pdm;
#[doc = "Clock TDM"]
pub use self::clk_tdm::ClkTdm;
#[doc = r"Cluster"]
#[doc = "Clock TDM"]
pub mod clk_tdm;
#[doc = "clk_jtag_trng (rw) register accessor: Clock JTAG TRNG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_jtag_trng::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_jtag_trng::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_jtag_trng`]
module"]
#[doc(alias = "clk_jtag_trng")]
pub type ClkJtagTrng = crate::Reg<clk_jtag_trng::ClkJtagTrngSpec>;
#[doc = "Clock JTAG TRNG"]
pub mod clk_jtag_trng;
#[doc = "SYSCRG RESET registers"]
pub use self::rst::Rst;
#[doc = r"Cluster"]
#[doc = "SYSCRG RESET registers"]
pub mod rst;
