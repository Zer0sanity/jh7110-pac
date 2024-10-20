#[repr(C)]
#[doc = "MTL registers"]
#[doc(alias = "mtl")]
pub struct Mtl {
    operation_mode: OperationMode,
    _reserved1: [u8; 0x1c],
    int_status: IntStatus,
    _reserved2: [u8; 0x0c],
    rx_queue_dma: [RxQueueDma; 2],
    _reserved3: [u8; 0x68],
    rxp: Rxp,
    _reserved4: [u8; 0x08],
    ecc_ctrl: EccCtrl,
    safety_int_status: SafetyIntStatus,
    ecc_int: EccInt,
    _reserved7: [u8; 0x10],
    dpp_ctrl: DppCtrl,
    _reserved8: [u8; 0x1c],
    chan: [Chan; 8],
}
impl Mtl {
    #[doc = "0x00 - MTL Operation Mode"]
    #[inline(always)]
    pub const fn operation_mode(&self) -> &OperationMode {
        &self.operation_mode
    }
    #[doc = "0x20 - MTL Interrupt Status"]
    #[inline(always)]
    pub const fn int_status(&self) -> &IntStatus {
        &self.int_status
    }
    #[doc = "0x30..0x38 - MTL RX Queue DMA - rx_queue_dma0: channel 0-3, rx_queue_dma1: channel 4-7"]
    #[inline(always)]
    pub const fn rx_queue_dma(&self, n: usize) -> &RxQueueDma {
        &self.rx_queue_dma[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x30..0x38 - MTL RX Queue DMA - rx_queue_dma0: channel 0-3, rx_queue_dma1: channel 4-7"]
    #[inline(always)]
    pub fn rx_queue_dma_iter(&self) -> impl Iterator<Item = &RxQueueDma> {
        self.rx_queue_dma.iter()
    }
    #[doc = "0xa0..0xb8 - MTL RXP registers"]
    #[inline(always)]
    pub const fn rxp(&self) -> &Rxp {
        &self.rxp
    }
    #[doc = "0xc0 - MTL ECC Control"]
    #[inline(always)]
    pub const fn ecc_ctrl(&self) -> &EccCtrl {
        &self.ecc_ctrl
    }
    #[doc = "0xc4 - MTL Safety Interrupt Status"]
    #[inline(always)]
    pub const fn safety_int_status(&self) -> &SafetyIntStatus {
        &self.safety_int_status
    }
    #[doc = "0xc8..0xd0 - MTL ECC Interrupt registers"]
    #[inline(always)]
    pub const fn ecc_int(&self) -> &EccInt {
        &self.ecc_int
    }
    #[doc = "0xe0 - MTL DPP Control"]
    #[inline(always)]
    pub const fn dpp_ctrl(&self) -> &DppCtrl {
        &self.dpp_ctrl
    }
    #[doc = "0x100..0x300 - MTL Channel registers"]
    #[inline(always)]
    pub const fn chan(&self, n: usize) -> &Chan {
        &self.chan[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x100..0x300 - MTL Channel registers"]
    #[inline(always)]
    pub fn chan_iter(&self) -> impl Iterator<Item = &Chan> {
        self.chan.iter()
    }
}
#[doc = "operation_mode (rw) register accessor: MTL Operation Mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`operation_mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`operation_mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@operation_mode`]
module"]
#[doc(alias = "operation_mode")]
pub type OperationMode = crate::Reg<operation_mode::OperationModeSpec>;
#[doc = "MTL Operation Mode"]
pub mod operation_mode;
#[doc = "int_status (rw) register accessor: MTL Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_status`]
module"]
#[doc(alias = "int_status")]
pub type IntStatus = crate::Reg<int_status::IntStatusSpec>;
#[doc = "MTL Interrupt Status"]
pub mod int_status;
#[doc = "rx_queue_dma (rw) register accessor: MTL RX Queue DMA - rx_queue_dma0: channel 0-3, rx_queue_dma1: channel 4-7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_queue_dma::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rx_queue_dma::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_queue_dma`]
module"]
#[doc(alias = "rx_queue_dma")]
pub type RxQueueDma = crate::Reg<rx_queue_dma::RxQueueDmaSpec>;
#[doc = "MTL RX Queue DMA - rx_queue_dma0: channel 0-3, rx_queue_dma1: channel 4-7"]
pub mod rx_queue_dma;
#[doc = "MTL RXP registers"]
pub use self::rxp::Rxp;
#[doc = r"Cluster"]
#[doc = "MTL RXP registers"]
pub mod rxp;
#[doc = "ecc_ctrl (rw) register accessor: MTL ECC Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_ctrl`]
module"]
#[doc(alias = "ecc_ctrl")]
pub type EccCtrl = crate::Reg<ecc_ctrl::EccCtrlSpec>;
#[doc = "MTL ECC Control"]
pub mod ecc_ctrl;
#[doc = "safety_int_status (rw) register accessor: MTL Safety Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`safety_int_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`safety_int_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@safety_int_status`]
module"]
#[doc(alias = "safety_int_status")]
pub type SafetyIntStatus = crate::Reg<safety_int_status::SafetyIntStatusSpec>;
#[doc = "MTL Safety Interrupt Status"]
pub mod safety_int_status;
#[doc = "MTL ECC Interrupt registers"]
pub use self::ecc_int::EccInt;
#[doc = r"Cluster"]
#[doc = "MTL ECC Interrupt registers"]
pub mod ecc_int;
#[doc = "dpp_ctrl (rw) register accessor: MTL DPP Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dpp_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dpp_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dpp_ctrl`]
module"]
#[doc(alias = "dpp_ctrl")]
pub type DppCtrl = crate::Reg<dpp_ctrl::DppCtrlSpec>;
#[doc = "MTL DPP Control"]
pub mod dpp_ctrl;
#[doc = "MTL Channel registers"]
pub use self::chan::Chan;
#[doc = r"Cluster"]
#[doc = "MTL Channel registers"]
pub mod chan;
