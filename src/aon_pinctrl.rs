#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    fmux_gpo_doen: FmuxGpoDoen,
    fmux_gpo_dout: FmuxGpoDout,
    fmux_gpi: FmuxGpi,
    fmux_gpen: FmuxGpen,
    ioirq: [Ioirq; 5],
    ioirq_status: [IoirqStatus; 3],
    testen: Testen,
    rgpio: [Rgpio; 4],
    rstn: Rstn,
    _reserved9: [u8; 0x04],
    rtc: Rtc,
    _reserved10: [u8; 0x04],
    osc: Osc,
    gmac0: [Gmac0; 15],
}
impl RegisterBlock {
    #[doc = "0x00 - The register can be used to configure the selected (Output Enable) OEN signal for GPIO0 - GPIO3."]
    #[inline(always)]
    pub const fn fmux_gpo_doen(&self) -> &FmuxGpoDoen {
        &self.fmux_gpo_doen
    }
    #[doc = "0x04 - The register can be used to configure the selected (Digital Output) DOUT signal for GPIO0 - GPIO3."]
    #[inline(always)]
    pub const fn fmux_gpo_dout(&self) -> &FmuxGpoDout {
        &self.fmux_gpo_dout
    }
    #[doc = "0x08 - The register can be used to configure the selected GPIO connector number for input signals."]
    #[inline(always)]
    pub const fn fmux_gpi(&self) -> &FmuxGpi {
        &self.fmux_gpi
    }
    #[doc = "0x0c - Enable always-on GPIO IRQ function."]
    #[inline(always)]
    pub const fn fmux_gpen(&self) -> &FmuxGpen {
        &self.fmux_gpen
    }
    #[doc = "0x10..0x24 - Always-on GPIO IO IRQ configuration."]
    #[inline(always)]
    pub const fn ioirq(&self, n: usize) -> &Ioirq {
        &self.ioirq[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x10..0x24 - Always-on GPIO IO IRQ configuration."]
    #[inline(always)]
    pub fn ioirq_iter(&self) -> impl Iterator<Item = &Ioirq> {
        self.ioirq.iter()
    }
    #[doc = "0x10 - Always-on GPIO IO IRQ configuration."]
    #[inline(always)]
    pub const fn ioirq_is(&self) -> &Ioirq {
        self.ioirq(0)
    }
    #[doc = "0x14 - Always-on GPIO IO IRQ configuration."]
    #[inline(always)]
    pub const fn ioirq_ic(&self) -> &Ioirq {
        self.ioirq(1)
    }
    #[doc = "0x18 - Always-on GPIO IO IRQ configuration."]
    #[inline(always)]
    pub const fn ioirq_ibe(&self) -> &Ioirq {
        self.ioirq(2)
    }
    #[doc = "0x1c - Always-on GPIO IO IRQ configuration."]
    #[inline(always)]
    pub const fn ioirq_iev(&self) -> &Ioirq {
        self.ioirq(3)
    }
    #[doc = "0x20 - Always-on GPIO IO IRQ configuration."]
    #[inline(always)]
    pub const fn ioirq_ie(&self) -> &Ioirq {
        self.ioirq(4)
    }
    #[doc = "0x24..0x30 - Always-on GPIO IO IRQ configuration."]
    #[inline(always)]
    pub const fn ioirq_status(&self, n: usize) -> &IoirqStatus {
        &self.ioirq_status[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x24..0x30 - Always-on GPIO IO IRQ configuration."]
    #[inline(always)]
    pub fn ioirq_status_iter(&self) -> impl Iterator<Item = &IoirqStatus> {
        self.ioirq_status.iter()
    }
    #[doc = "0x24 - Always-on GPIO IO IRQ configuration."]
    #[inline(always)]
    pub const fn ioirq_status_ris(&self) -> &IoirqStatus {
        self.ioirq_status(0)
    }
    #[doc = "0x28 - Always-on GPIO IO IRQ configuration."]
    #[inline(always)]
    pub const fn ioirq_status_mis(&self) -> &IoirqStatus {
        self.ioirq_status(1)
    }
    #[doc = "0x2c - Always-on GPIO IO IRQ configuration."]
    #[inline(always)]
    pub const fn ioirq_status_in_sync2(&self) -> &IoirqStatus {
        self.ioirq_status(2)
    }
    #[doc = "0x30 - AON IOMUX CFG SAIF SYSCFG 48 - Enable test Power-on-Start (POS)"]
    #[inline(always)]
    pub const fn testen(&self) -> &Testen {
        &self.testen
    }
    #[doc = "0x34..0x44 - AON IOMUX CFG SAIF SYSCFG - RGPIO"]
    #[inline(always)]
    pub const fn rgpio(&self, n: usize) -> &Rgpio {
        &self.rgpio[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x34..0x44 - AON IOMUX CFG SAIF SYSCFG - RGPIO"]
    #[inline(always)]
    pub fn rgpio_iter(&self) -> impl Iterator<Item = &Rgpio> {
        self.rgpio.iter()
    }
    #[doc = "0x44 - AON IOMUX CFG SAIF SYSCFG 68 - RSTN"]
    #[inline(always)]
    pub const fn rstn(&self) -> &Rstn {
        &self.rstn
    }
    #[doc = "0x4c - AON IOMUX CFG SAIF SYSCFG 76 - RTC"]
    #[inline(always)]
    pub const fn rtc(&self) -> &Rtc {
        &self.rtc
    }
    #[doc = "0x54 - AON IOMUX CFG SAIF SYSCFG 76 - OSC"]
    #[inline(always)]
    pub const fn osc(&self) -> &Osc {
        &self.osc
    }
    #[doc = "0x58..0x94 - AON IOMUX CFG SAIF SYSCFG - GMAC0"]
    #[inline(always)]
    pub const fn gmac0(&self, n: usize) -> &Gmac0 {
        &self.gmac0[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x58..0x94 - AON IOMUX CFG SAIF SYSCFG - GMAC0"]
    #[inline(always)]
    pub fn gmac0_iter(&self) -> impl Iterator<Item = &Gmac0> {
        self.gmac0.iter()
    }
    #[doc = "0x58 - AON IOMUX CFG SAIF SYSCFG - GMAC0"]
    #[inline(always)]
    pub const fn gmac0_mdc(&self) -> &Gmac0 {
        self.gmac0(0)
    }
    #[doc = "0x5c - AON IOMUX CFG SAIF SYSCFG - GMAC0"]
    #[inline(always)]
    pub const fn gmac0_mdio(&self) -> &Gmac0 {
        self.gmac0(1)
    }
    #[doc = "0x60 - AON IOMUX CFG SAIF SYSCFG - GMAC0"]
    #[inline(always)]
    pub const fn gmac0_rxd0(&self) -> &Gmac0 {
        self.gmac0(2)
    }
    #[doc = "0x64 - AON IOMUX CFG SAIF SYSCFG - GMAC0"]
    #[inline(always)]
    pub const fn gmac0_rxd1(&self) -> &Gmac0 {
        self.gmac0(3)
    }
    #[doc = "0x68 - AON IOMUX CFG SAIF SYSCFG - GMAC0"]
    #[inline(always)]
    pub const fn gmac0_rxd2(&self) -> &Gmac0 {
        self.gmac0(4)
    }
    #[doc = "0x6c - AON IOMUX CFG SAIF SYSCFG - GMAC0"]
    #[inline(always)]
    pub const fn gmac0_rxd3(&self) -> &Gmac0 {
        self.gmac0(5)
    }
    #[doc = "0x70 - AON IOMUX CFG SAIF SYSCFG - GMAC0"]
    #[inline(always)]
    pub const fn gmac0_rxdv(&self) -> &Gmac0 {
        self.gmac0(6)
    }
    #[doc = "0x74 - AON IOMUX CFG SAIF SYSCFG - GMAC0"]
    #[inline(always)]
    pub const fn gmac0_rxc(&self) -> &Gmac0 {
        self.gmac0(7)
    }
    #[doc = "0x78 - AON IOMUX CFG SAIF SYSCFG - GMAC0"]
    #[inline(always)]
    pub const fn gmac0_txd0(&self) -> &Gmac0 {
        self.gmac0(8)
    }
    #[doc = "0x7c - AON IOMUX CFG SAIF SYSCFG - GMAC0"]
    #[inline(always)]
    pub const fn gmac0_txd1(&self) -> &Gmac0 {
        self.gmac0(9)
    }
    #[doc = "0x80 - AON IOMUX CFG SAIF SYSCFG - GMAC0"]
    #[inline(always)]
    pub const fn gmac0_txd2(&self) -> &Gmac0 {
        self.gmac0(10)
    }
    #[doc = "0x84 - AON IOMUX CFG SAIF SYSCFG - GMAC0"]
    #[inline(always)]
    pub const fn gmac0_txd3(&self) -> &Gmac0 {
        self.gmac0(11)
    }
    #[doc = "0x88 - AON IOMUX CFG SAIF SYSCFG - GMAC0"]
    #[inline(always)]
    pub const fn gmac0_txen(&self) -> &Gmac0 {
        self.gmac0(12)
    }
    #[doc = "0x8c - AON IOMUX CFG SAIF SYSCFG - GMAC0"]
    #[inline(always)]
    pub const fn gmac0_txc(&self) -> &Gmac0 {
        self.gmac0(13)
    }
    #[doc = "0x90 - AON IOMUX CFG SAIF SYSCFG - GMAC0"]
    #[inline(always)]
    pub const fn gmac0_rxc_func_sel(&self) -> &Gmac0 {
        self.gmac0(14)
    }
}
#[doc = "fmux_gpo_doen (rw) register accessor: The register can be used to configure the selected (Output Enable) OEN signal for GPIO0 - GPIO3.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmux_gpo_doen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fmux_gpo_doen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmux_gpo_doen`]
module"]
#[doc(alias = "fmux_gpo_doen")]
pub type FmuxGpoDoen = crate::Reg<fmux_gpo_doen::FmuxGpoDoenSpec>;
#[doc = "The register can be used to configure the selected (Output Enable) OEN signal for GPIO0 - GPIO3."]
pub mod fmux_gpo_doen;
#[doc = "fmux_gpo_dout (rw) register accessor: The register can be used to configure the selected (Digital Output) DOUT signal for GPIO0 - GPIO3.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmux_gpo_dout::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fmux_gpo_dout::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmux_gpo_dout`]
module"]
#[doc(alias = "fmux_gpo_dout")]
pub type FmuxGpoDout = crate::Reg<fmux_gpo_dout::FmuxGpoDoutSpec>;
#[doc = "The register can be used to configure the selected (Digital Output) DOUT signal for GPIO0 - GPIO3."]
pub mod fmux_gpo_dout;
#[doc = "fmux_gpi (rw) register accessor: The register can be used to configure the selected GPIO connector number for input signals.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmux_gpi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fmux_gpi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmux_gpi`]
module"]
#[doc(alias = "fmux_gpi")]
pub type FmuxGpi = crate::Reg<fmux_gpi::FmuxGpiSpec>;
#[doc = "The register can be used to configure the selected GPIO connector number for input signals."]
pub mod fmux_gpi;
#[doc = "fmux_gpen (rw) register accessor: Enable always-on GPIO IRQ function.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmux_gpen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fmux_gpen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmux_gpen`]
module"]
#[doc(alias = "fmux_gpen")]
pub type FmuxGpen = crate::Reg<fmux_gpen::FmuxGpenSpec>;
#[doc = "Enable always-on GPIO IRQ function."]
pub mod fmux_gpen;
#[doc = "ioirq (rw) register accessor: Always-on GPIO IO IRQ configuration.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioirq::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ioirq::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ioirq`]
module"]
#[doc(alias = "ioirq")]
pub type Ioirq = crate::Reg<ioirq::IoirqSpec>;
#[doc = "Always-on GPIO IO IRQ configuration."]
pub mod ioirq;
#[doc = "ioirq_status (r) register accessor: Always-on GPIO IO IRQ configuration.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioirq_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ioirq_status`]
module"]
#[doc(alias = "ioirq_status")]
pub type IoirqStatus = crate::Reg<ioirq_status::IoirqStatusSpec>;
#[doc = "Always-on GPIO IO IRQ configuration."]
pub mod ioirq_status;
#[doc = "testen (rw) register accessor: AON IOMUX CFG SAIF SYSCFG 48 - Enable test Power-on-Start (POS)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`testen::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`testen::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@testen`]
module"]
#[doc(alias = "testen")]
pub type Testen = crate::Reg<testen::TestenSpec>;
#[doc = "AON IOMUX CFG SAIF SYSCFG 48 - Enable test Power-on-Start (POS)"]
pub mod testen;
#[doc = "rgpio (rw) register accessor: AON IOMUX CFG SAIF SYSCFG - RGPIO\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rgpio::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rgpio::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rgpio`]
module"]
#[doc(alias = "rgpio")]
pub type Rgpio = crate::Reg<rgpio::RgpioSpec>;
#[doc = "AON IOMUX CFG SAIF SYSCFG - RGPIO"]
pub mod rgpio;
#[doc = "rstn (rw) register accessor: AON IOMUX CFG SAIF SYSCFG 68 - RSTN\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rstn::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rstn::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rstn`]
module"]
#[doc(alias = "rstn")]
pub type Rstn = crate::Reg<rstn::RstnSpec>;
#[doc = "AON IOMUX CFG SAIF SYSCFG 68 - RSTN"]
pub mod rstn;
#[doc = "rtc (rw) register accessor: AON IOMUX CFG SAIF SYSCFG 76 - RTC\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc`]
module"]
#[doc(alias = "rtc")]
pub type Rtc = crate::Reg<rtc::RtcSpec>;
#[doc = "AON IOMUX CFG SAIF SYSCFG 76 - RTC"]
pub mod rtc;
#[doc = "osc (rw) register accessor: AON IOMUX CFG SAIF SYSCFG 76 - OSC\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`osc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`osc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@osc`]
module"]
#[doc(alias = "osc")]
pub type Osc = crate::Reg<osc::OscSpec>;
#[doc = "AON IOMUX CFG SAIF SYSCFG 76 - OSC"]
pub mod osc;
#[doc = "gmac0 (rw) register accessor: AON IOMUX CFG SAIF SYSCFG - GMAC0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gmac0`]
module"]
#[doc(alias = "gmac0")]
pub type Gmac0 = crate::Reg<gmac0::Gmac0Spec>;
#[doc = "AON IOMUX CFG SAIF SYSCFG - GMAC0"]
pub mod gmac0;
