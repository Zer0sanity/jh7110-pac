#[repr(C)]
#[doc = "The register can be used to configure the selected GPIO connector number for input signals. The signal name is indicated in the \"Name\" column of the following table per StarFive naming conventions. For example, name \"u0_WAVE511_i_uart_rxsin_cfg\" indicates the corresponding input signal is \"u0_WAVE511.i_uart_rxsin\". See GPIO Input Signals (on page 107) for a complete list of the input GPIO signals."]
#[doc(alias = "gpi")]
pub struct Gpi {
    gpi0: Gpi0,
    gpi1: Gpi1,
    gpi2: Gpi2,
    gpi3: Gpi3,
    gpi4: Gpi4,
    gpi5: Gpi5,
    gpi6: Gpi6,
    gpi7: Gpi7,
    gpi8: Gpi8,
    gpi9: Gpi9,
    gpi10: Gpi10,
    gpi11: Gpi11,
    gpi12: Gpi12,
    gpi13: Gpi13,
    gpi14: Gpi14,
    gpi15: Gpi15,
    gpi16: Gpi16,
    gpi17: Gpi17,
    gpi18: Gpi18,
    gpi19: Gpi19,
    gpi20: Gpi20,
    gpi21: Gpi21,
    gpi22: Gpi22,
}
impl Gpi {
    #[doc = "0x00 - SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI: \\[wave511_uart_rxsin, can_rxd_0, usb_over_current, spdif_spdi_fi\\]"]
    #[inline(always)]
    pub const fn gpi0(&self) -> &Gpi0 {
        &self.gpi0
    }
    #[doc = "0x04 - SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI: \\[jtag_trstn, hdmi_cec_sda, hdmi_ddc_scl, hdmi_ddc_sda\\]"]
    #[inline(always)]
    pub const fn gpi1(&self) -> &Gpi1 {
        &self.gpi1
    }
    #[doc = "0x08 - SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI: \\[hdmi_hpd, i2c_clk_0, i2c_data_0, sdio_detect_0\\]"]
    #[inline(always)]
    pub const fn gpi2(&self) -> &Gpi2 {
        &self.gpi2
    }
    #[doc = "0x0c - SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI: \\[sdio_int_0, sdio_write_prt_0, uart_sin_0, hifi4_jtck_0\\]"]
    #[inline(always)]
    pub const fn gpi3(&self) -> &Gpi3 {
        &self.gpi3
    }
    #[doc = "0x10 - SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI: \\[hifi4_jtdi, hifi4_jtms, hifi4_jtrstn, jtag_tdi\\]"]
    #[inline(always)]
    pub const fn gpi4(&self) -> &Gpi4 {
        &self.gpi4
    }
    #[doc = "0x14 - SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI: \\[jtag_tms, pdm_dmic_0, pdm_dmic_1, audio_i2srx_0\\]"]
    #[inline(always)]
    pub const fn gpi5(&self) -> &Gpi5 {
        &self.gpi5
    }
    #[doc = "0x18 - SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI: \\[audio_i2srx_1, audio_i2srx_2, spi_clkin_0, spi_fssin_0\\]"]
    #[inline(always)]
    pub const fn gpi6(&self) -> &Gpi6 {
        &self.gpi6
    }
    #[doc = "0x1c - SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI: \\[spi_rxd_0, jtag_tck, mclk, i2srx_bclk_slv_0\\]"]
    #[inline(always)]
    pub const fn gpi7(&self) -> &Gpi7 {
        &self.gpi7
    }
    #[doc = "0x20 - SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI: \\[i2srx_lrck_slv_0, i2stx_bclk_slv_0, i2stx_lrck_slv_0, tdm_clk_slv_0\\]"]
    #[inline(always)]
    pub const fn gpi8(&self) -> &Gpi8 {
        &self.gpi8
    }
    #[doc = "0x24 - SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI: \\[pcm_rxd_0, pcm_synon_0, can_rxd_1, i2c_clk_1\\]"]
    #[inline(always)]
    pub const fn gpi9(&self) -> &Gpi9 {
        &self.gpi9
    }
    #[doc = "0x28 - SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI: \\[i2c_data_1, sdio_detect_1, sdio_int_1, sdio_write_prt_1\\]"]
    #[inline(always)]
    pub const fn gpi10(&self) -> &Gpi10 {
        &self.gpi10
    }
    #[doc = "0x2c - SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI: \\[sdio_ccmd_1, sdio_cdata_0, sdio_cdata_1, sdio_cdata_2\\]"]
    #[inline(always)]
    pub const fn gpi11(&self) -> &Gpi11 {
        &self.gpi11
    }
    #[doc = "0x30 - SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI: \\[sdio_cdata_3, sdio_cdata_4, sdio_cdata_5, sdio_cdata_6\\]"]
    #[inline(always)]
    pub const fn gpi12(&self) -> &Gpi12 {
        &self.gpi12
    }
    #[doc = "0x34 - SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI: \\[sdio_cdata_7, sdio_data_strobe, uart_cts_1, uart_sin_1\\]"]
    #[inline(always)]
    pub const fn gpi13(&self) -> &Gpi13 {
        &self.gpi13
    }
    #[doc = "0x38 - SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI: \\[spi_clkin_1, spi_fssin_1, spi_rxd_1, i2c_clk_2\\]"]
    #[inline(always)]
    pub const fn gpi14(&self) -> &Gpi14 {
        &self.gpi14
    }
    #[doc = "0x3c - SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI: \\[i2c_data_2, uart_cts_2, uart_sin_2, spi_clkin_2\\]"]
    #[inline(always)]
    pub const fn gpi15(&self) -> &Gpi15 {
        &self.gpi15
    }
    #[doc = "0x40 - SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI: \\[spi_fssin_2, spi_rxd_2, i2c_clk_3, i2c_data_3\\]"]
    #[inline(always)]
    pub const fn gpi16(&self) -> &Gpi16 {
        &self.gpi16
    }
    #[doc = "0x44 - SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI: \\[uart_sin_3, spi_clkin_3, spi_fssin_3, spi_rxd_3\\]"]
    #[inline(always)]
    pub const fn gpi17(&self) -> &Gpi17 {
        &self.gpi17
    }
    #[doc = "0x48 - SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI: \\[i2c_clk_4, i2c_data_4, uart_cts_4, uart_sin_4\\]"]
    #[inline(always)]
    pub const fn gpi18(&self) -> &Gpi18 {
        &self.gpi18
    }
    #[doc = "0x4c - SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI: \\[spi_clkin_4, spi_fssin_4, spi_rxd_4, i2c_clk_5\\]"]
    #[inline(always)]
    pub const fn gpi19(&self) -> &Gpi19 {
        &self.gpi19
    }
    #[doc = "0x50 - SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI: \\[i2c_data_5, uart_cts_5, uart_sin_5, spi_clkin_5\\]"]
    #[inline(always)]
    pub const fn gpi20(&self) -> &Gpi20 {
        &self.gpi20
    }
    #[doc = "0x54 - SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI: \\[spi_fssin_5, spi_rxd_5, i2c_clk_6, i2c_data_6\\]"]
    #[inline(always)]
    pub const fn gpi21(&self) -> &Gpi21 {
        &self.gpi21
    }
    #[doc = "0x58 - SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI: \\[spi_clkin_6, spi_fssin_6, spi_rxd_6\\]"]
    #[inline(always)]
    pub const fn gpi22(&self) -> &Gpi22 {
        &self.gpi22
    }
}
#[doc = "gpi0 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI: \\[wave511_uart_rxsin, can_rxd_0, usb_over_current, spdif_spdi_fi\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpi0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpi0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpi0`]
module"]
#[doc(alias = "gpi0")]
pub type Gpi0 = crate::Reg<gpi0::Gpi0Spec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI: \\[wave511_uart_rxsin, can_rxd_0, usb_over_current, spdif_spdi_fi\\]"]
pub mod gpi0;
#[doc = "gpi1 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI: \\[jtag_trstn, hdmi_cec_sda, hdmi_ddc_scl, hdmi_ddc_sda\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpi1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpi1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpi1`]
module"]
#[doc(alias = "gpi1")]
pub type Gpi1 = crate::Reg<gpi1::Gpi1Spec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI: \\[jtag_trstn, hdmi_cec_sda, hdmi_ddc_scl, hdmi_ddc_sda\\]"]
pub mod gpi1;
#[doc = "gpi2 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI: \\[hdmi_hpd, i2c_clk_0, i2c_data_0, sdio_detect_0\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpi2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpi2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpi2`]
module"]
#[doc(alias = "gpi2")]
pub type Gpi2 = crate::Reg<gpi2::Gpi2Spec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI: \\[hdmi_hpd, i2c_clk_0, i2c_data_0, sdio_detect_0\\]"]
pub mod gpi2;
#[doc = "gpi3 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI: \\[sdio_int_0, sdio_write_prt_0, uart_sin_0, hifi4_jtck_0\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpi3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpi3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpi3`]
module"]
#[doc(alias = "gpi3")]
pub type Gpi3 = crate::Reg<gpi3::Gpi3Spec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI: \\[sdio_int_0, sdio_write_prt_0, uart_sin_0, hifi4_jtck_0\\]"]
pub mod gpi3;
#[doc = "gpi4 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI: \\[hifi4_jtdi, hifi4_jtms, hifi4_jtrstn, jtag_tdi\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpi4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpi4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpi4`]
module"]
#[doc(alias = "gpi4")]
pub type Gpi4 = crate::Reg<gpi4::Gpi4Spec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI: \\[hifi4_jtdi, hifi4_jtms, hifi4_jtrstn, jtag_tdi\\]"]
pub mod gpi4;
#[doc = "gpi5 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI: \\[jtag_tms, pdm_dmic_0, pdm_dmic_1, audio_i2srx_0\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpi5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpi5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpi5`]
module"]
#[doc(alias = "gpi5")]
pub type Gpi5 = crate::Reg<gpi5::Gpi5Spec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI: \\[jtag_tms, pdm_dmic_0, pdm_dmic_1, audio_i2srx_0\\]"]
pub mod gpi5;
#[doc = "gpi6 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI: \\[audio_i2srx_1, audio_i2srx_2, spi_clkin_0, spi_fssin_0\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpi6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpi6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpi6`]
module"]
#[doc(alias = "gpi6")]
pub type Gpi6 = crate::Reg<gpi6::Gpi6Spec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI: \\[audio_i2srx_1, audio_i2srx_2, spi_clkin_0, spi_fssin_0\\]"]
pub mod gpi6;
#[doc = "gpi7 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI: \\[spi_rxd_0, jtag_tck, mclk, i2srx_bclk_slv_0\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpi7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpi7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpi7`]
module"]
#[doc(alias = "gpi7")]
pub type Gpi7 = crate::Reg<gpi7::Gpi7Spec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI: \\[spi_rxd_0, jtag_tck, mclk, i2srx_bclk_slv_0\\]"]
pub mod gpi7;
#[doc = "gpi8 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI: \\[i2srx_lrck_slv_0, i2stx_bclk_slv_0, i2stx_lrck_slv_0, tdm_clk_slv_0\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpi8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpi8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpi8`]
module"]
#[doc(alias = "gpi8")]
pub type Gpi8 = crate::Reg<gpi8::Gpi8Spec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI: \\[i2srx_lrck_slv_0, i2stx_bclk_slv_0, i2stx_lrck_slv_0, tdm_clk_slv_0\\]"]
pub mod gpi8;
#[doc = "gpi9 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI: \\[pcm_rxd_0, pcm_synon_0, can_rxd_1, i2c_clk_1\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpi9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpi9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpi9`]
module"]
#[doc(alias = "gpi9")]
pub type Gpi9 = crate::Reg<gpi9::Gpi9Spec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI: \\[pcm_rxd_0, pcm_synon_0, can_rxd_1, i2c_clk_1\\]"]
pub mod gpi9;
#[doc = "gpi10 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI: \\[i2c_data_1, sdio_detect_1, sdio_int_1, sdio_write_prt_1\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpi10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpi10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpi10`]
module"]
#[doc(alias = "gpi10")]
pub type Gpi10 = crate::Reg<gpi10::Gpi10Spec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI: \\[i2c_data_1, sdio_detect_1, sdio_int_1, sdio_write_prt_1\\]"]
pub mod gpi10;
#[doc = "gpi11 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI: \\[sdio_ccmd_1, sdio_cdata_0, sdio_cdata_1, sdio_cdata_2\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpi11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpi11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpi11`]
module"]
#[doc(alias = "gpi11")]
pub type Gpi11 = crate::Reg<gpi11::Gpi11Spec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI: \\[sdio_ccmd_1, sdio_cdata_0, sdio_cdata_1, sdio_cdata_2\\]"]
pub mod gpi11;
#[doc = "gpi12 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI: \\[sdio_cdata_3, sdio_cdata_4, sdio_cdata_5, sdio_cdata_6\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpi12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpi12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpi12`]
module"]
#[doc(alias = "gpi12")]
pub type Gpi12 = crate::Reg<gpi12::Gpi12Spec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI: \\[sdio_cdata_3, sdio_cdata_4, sdio_cdata_5, sdio_cdata_6\\]"]
pub mod gpi12;
#[doc = "gpi13 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI: \\[sdio_cdata_7, sdio_data_strobe, uart_cts_1, uart_sin_1\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpi13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpi13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpi13`]
module"]
#[doc(alias = "gpi13")]
pub type Gpi13 = crate::Reg<gpi13::Gpi13Spec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI: \\[sdio_cdata_7, sdio_data_strobe, uart_cts_1, uart_sin_1\\]"]
pub mod gpi13;
#[doc = "gpi14 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI: \\[spi_clkin_1, spi_fssin_1, spi_rxd_1, i2c_clk_2\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpi14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpi14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpi14`]
module"]
#[doc(alias = "gpi14")]
pub type Gpi14 = crate::Reg<gpi14::Gpi14Spec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI: \\[spi_clkin_1, spi_fssin_1, spi_rxd_1, i2c_clk_2\\]"]
pub mod gpi14;
#[doc = "gpi15 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI: \\[i2c_data_2, uart_cts_2, uart_sin_2, spi_clkin_2\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpi15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpi15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpi15`]
module"]
#[doc(alias = "gpi15")]
pub type Gpi15 = crate::Reg<gpi15::Gpi15Spec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI: \\[i2c_data_2, uart_cts_2, uart_sin_2, spi_clkin_2\\]"]
pub mod gpi15;
#[doc = "gpi16 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI: \\[spi_fssin_2, spi_rxd_2, i2c_clk_3, i2c_data_3\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpi16::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpi16::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpi16`]
module"]
#[doc(alias = "gpi16")]
pub type Gpi16 = crate::Reg<gpi16::Gpi16Spec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI: \\[spi_fssin_2, spi_rxd_2, i2c_clk_3, i2c_data_3\\]"]
pub mod gpi16;
#[doc = "gpi17 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI: \\[uart_sin_3, spi_clkin_3, spi_fssin_3, spi_rxd_3\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpi17::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpi17::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpi17`]
module"]
#[doc(alias = "gpi17")]
pub type Gpi17 = crate::Reg<gpi17::Gpi17Spec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI: \\[uart_sin_3, spi_clkin_3, spi_fssin_3, spi_rxd_3\\]"]
pub mod gpi17;
#[doc = "gpi18 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI: \\[i2c_clk_4, i2c_data_4, uart_cts_4, uart_sin_4\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpi18::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpi18::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpi18`]
module"]
#[doc(alias = "gpi18")]
pub type Gpi18 = crate::Reg<gpi18::Gpi18Spec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI: \\[i2c_clk_4, i2c_data_4, uart_cts_4, uart_sin_4\\]"]
pub mod gpi18;
#[doc = "gpi19 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI: \\[spi_clkin_4, spi_fssin_4, spi_rxd_4, i2c_clk_5\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpi19::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpi19::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpi19`]
module"]
#[doc(alias = "gpi19")]
pub type Gpi19 = crate::Reg<gpi19::Gpi19Spec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI: \\[spi_clkin_4, spi_fssin_4, spi_rxd_4, i2c_clk_5\\]"]
pub mod gpi19;
#[doc = "gpi20 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI: \\[i2c_data_5, uart_cts_5, uart_sin_5, spi_clkin_5\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpi20::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpi20::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpi20`]
module"]
#[doc(alias = "gpi20")]
pub type Gpi20 = crate::Reg<gpi20::Gpi20Spec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI: \\[i2c_data_5, uart_cts_5, uart_sin_5, spi_clkin_5\\]"]
pub mod gpi20;
#[doc = "gpi21 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI: \\[spi_fssin_5, spi_rxd_5, i2c_clk_6, i2c_data_6\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpi21::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpi21::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpi21`]
module"]
#[doc(alias = "gpi21")]
pub type Gpi21 = crate::Reg<gpi21::Gpi21Spec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI: \\[spi_fssin_5, spi_rxd_5, i2c_clk_6, i2c_data_6\\]"]
pub mod gpi21;
#[doc = "gpi22 (rw) register accessor: SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI: \\[spi_clkin_6, spi_fssin_6, spi_rxd_6\\]\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpi22::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpi22::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpi22`]
module"]
#[doc(alias = "gpi22")]
pub type Gpi22 = crate::Reg<gpi22::Gpi22Spec>;
#[doc = "SYS IOMUX CFG SAIF SYSCFG FMUX GPIO GPI: \\[spi_clkin_6, spi_fssin_6, spi_rxd_6\\]"]
pub mod gpi22;
