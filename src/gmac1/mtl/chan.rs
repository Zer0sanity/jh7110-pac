#[repr(C)]
#[doc = "MTL Channel registers"]
#[doc(alias = "chan")]
pub struct Chan {
    tx_op_mode: TxOpMode,
    _reserved1: [u8; 0x04],
    tx_debug: TxDebug,
    _reserved2: [u8; 0x04],
    ets_ctrl: EtsCtrl,
    _reserved3: [u8; 0x04],
    tx_queue_weight: TxQueueWeight,
    send_slope_credit: SendSlopeCredit,
    credit: [Credit; 2],
    _reserved6: [u8; 0x04],
    int_ctrl: IntCtrl,
    rx_op_mode: RxOpMode,
    _reserved8: [u8; 0x04],
    rx_debug: RxDebug,
    _reserved_chan: _ReservedChan,
}
impl Chan {
    #[doc = "0x00 - MTL Channel TX OP Mode"]
    #[inline(always)]
    pub const fn tx_op_mode(&self) -> &TxOpMode {
        &self.tx_op_mode
    }
    #[doc = "0x08 - MTL TX Debug"]
    #[inline(always)]
    pub const fn tx_debug(&self) -> &TxDebug {
        &self.tx_debug
    }
    #[doc = "0x10 - MTL Channel ETS Control"]
    #[inline(always)]
    pub const fn ets_ctrl(&self) -> &EtsCtrl {
        &self.ets_ctrl
    }
    #[doc = "0x18 - MTL Channel TX Queue Weight"]
    #[inline(always)]
    pub const fn tx_queue_weight(&self) -> &TxQueueWeight {
        &self.tx_queue_weight
    }
    #[doc = "0x1c - MTL Channel Send Slope Credit"]
    #[inline(always)]
    pub const fn send_slope_credit(&self) -> &SendSlopeCredit {
        &self.send_slope_credit
    }
    #[doc = "0x20..0x28 - MTL Channel Credit - credit0: High, credit1: Low"]
    #[inline(always)]
    pub const fn credit(&self, n: usize) -> &Credit {
        &self.credit[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x20..0x28 - MTL Channel Credit - credit0: High, credit1: Low"]
    #[inline(always)]
    pub fn credit_iter(&self) -> impl Iterator<Item = &Credit> {
        self.credit.iter()
    }
    #[doc = "0x20 - MTL Channel Credit - credit0: High, credit1: Low"]
    #[inline(always)]
    pub const fn credit_high(&self) -> &Credit {
        self.credit(0)
    }
    #[doc = "0x24 - MTL Channel Credit - credit0: High, credit1: Low"]
    #[inline(always)]
    pub const fn credit_low(&self) -> &Credit {
        self.credit(1)
    }
    #[doc = "0x2c - MTL Channel Interrupt Control"]
    #[inline(always)]
    pub const fn int_ctrl(&self) -> &IntCtrl {
        &self.int_ctrl
    }
    #[doc = "0x30 - MTL Channel RX OP Mode"]
    #[inline(always)]
    pub const fn rx_op_mode(&self) -> &RxOpMode {
        &self.rx_op_mode
    }
    #[doc = "0x38 - MTL RX Debug - GMII or MII Transmit Protocol Engine Status"]
    #[inline(always)]
    pub const fn rx_debug(&self) -> &RxDebug {
        &self.rx_debug
    }
    #[doc = "0x3c - MTL Channel Reserved"]
    #[inline(always)]
    pub const fn _reserved_chan(&self) -> &_ReservedChan {
        &self._reserved_chan
    }
}
#[doc = "tx_op_mode (rw) register accessor: MTL Channel TX OP Mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_op_mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_op_mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_op_mode`]
module"]
#[doc(alias = "tx_op_mode")]
pub type TxOpMode = crate::Reg<tx_op_mode::TxOpModeSpec>;
#[doc = "MTL Channel TX OP Mode"]
pub mod tx_op_mode;
#[doc = "tx_debug (rw) register accessor: MTL TX Debug\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_debug::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_debug::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_debug`]
module"]
#[doc(alias = "tx_debug")]
pub type TxDebug = crate::Reg<tx_debug::TxDebugSpec>;
#[doc = "MTL TX Debug"]
pub mod tx_debug;
#[doc = "ets_ctrl (rw) register accessor: MTL Channel ETS Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ets_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ets_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ets_ctrl`]
module"]
#[doc(alias = "ets_ctrl")]
pub type EtsCtrl = crate::Reg<ets_ctrl::EtsCtrlSpec>;
#[doc = "MTL Channel ETS Control"]
pub mod ets_ctrl;
#[doc = "tx_queue_weight (rw) register accessor: MTL Channel TX Queue Weight\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_queue_weight::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_queue_weight::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_queue_weight`]
module"]
#[doc(alias = "tx_queue_weight")]
pub type TxQueueWeight = crate::Reg<tx_queue_weight::TxQueueWeightSpec>;
#[doc = "MTL Channel TX Queue Weight"]
pub mod tx_queue_weight;
#[doc = "send_slope_credit (rw) register accessor: MTL Channel Send Slope Credit\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`send_slope_credit::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`send_slope_credit::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@send_slope_credit`]
module"]
#[doc(alias = "send_slope_credit")]
pub type SendSlopeCredit = crate::Reg<send_slope_credit::SendSlopeCreditSpec>;
#[doc = "MTL Channel Send Slope Credit"]
pub mod send_slope_credit;
#[doc = "credit (rw) register accessor: MTL Channel Credit - credit0: High, credit1: Low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`credit::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`credit::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@credit`]
module"]
#[doc(alias = "credit")]
pub type Credit = crate::Reg<credit::CreditSpec>;
#[doc = "MTL Channel Credit - credit0: High, credit1: Low"]
pub mod credit;
#[doc = "int_ctrl (rw) register accessor: MTL Channel Interrupt Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_ctrl`]
module"]
#[doc(alias = "int_ctrl")]
pub type IntCtrl = crate::Reg<int_ctrl::IntCtrlSpec>;
#[doc = "MTL Channel Interrupt Control"]
pub mod int_ctrl;
#[doc = "rx_op_mode (rw) register accessor: MTL Channel RX OP Mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_op_mode::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rx_op_mode::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_op_mode`]
module"]
#[doc(alias = "rx_op_mode")]
pub type RxOpMode = crate::Reg<rx_op_mode::RxOpModeSpec>;
#[doc = "MTL Channel RX OP Mode"]
pub mod rx_op_mode;
#[doc = "rx_debug (rw) register accessor: MTL RX Debug - GMII or MII Transmit Protocol Engine Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_debug::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rx_debug::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_debug`]
module"]
#[doc(alias = "rx_debug")]
pub type RxDebug = crate::Reg<rx_debug::RxDebugSpec>;
#[doc = "MTL RX Debug - GMII or MII Transmit Protocol Engine Status"]
pub mod rx_debug;
#[doc = "_reserved_chan (rw) register accessor: MTL Channel Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_reserved_chan::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`_reserved_chan::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_reserved_chan`]
module"]
#[doc(alias = "_reserved_chan")]
pub type _ReservedChan = crate::Reg<_reserved_chan::_ReservedChanSpec>;
#[doc = "MTL Channel Reserved"]
pub mod _reserved_chan;
