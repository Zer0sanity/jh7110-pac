#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved_0_dll: [u8; 0x04],
    _reserved_1_dlh: [u8; 0x04],
    _reserved_2_fcr: [u8; 0x04],
    lcr: Lcr,
    mcr: Mcr,
    lsr: Lsr,
    msr: Msr,
    scr: Scr,
    lpdll: Lpdll,
    lpdlh: Lpdlh,
    _reserved10: [u8; 0x08],
    shadow: [Shadow; 16],
    far: Far,
    tfr: Tfr,
    rfw: Rfw,
    usr: Usr,
    tfl: Tfl,
    rfl: Rfl,
    srr: Srr,
    srts: Srts,
    sbcr: Sbcr,
    sdmam: Sdmam,
    sfe: Sfe,
    srt: Srt,
    stet: Stet,
    htx: Htx,
    dmasa: Dmasa,
    _reserved26: [u8; 0x48],
    cpr: Cpr,
    _reserved_27_ctr: [u8; 0x04],
}
impl RegisterBlock {
    #[doc = "0x00 - Divisor Latch Low"]
    #[inline(always)]
    pub const fn dll(&self) -> &Dll {
        unsafe { &*(self as *const Self).cast::<u8>().add(0).cast() }
    }
    #[doc = "0x00 - Transmit Holding Register"]
    #[inline(always)]
    pub const fn thr(&self) -> &Thr {
        unsafe { &*(self as *const Self).cast::<u8>().add(0).cast() }
    }
    #[doc = "0x00 - Receive Buffer Register"]
    #[inline(always)]
    pub const fn rbr(&self) -> &Rbr {
        unsafe { &*(self as *const Self).cast::<u8>().add(0).cast() }
    }
    #[doc = "0x04 - Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ier(&self) -> &Ier {
        unsafe { &*(self as *const Self).cast::<u8>().add(4).cast() }
    }
    #[doc = "0x04 - Divisor Latch High"]
    #[inline(always)]
    pub const fn dlh(&self) -> &Dlh {
        unsafe { &*(self as *const Self).cast::<u8>().add(4).cast() }
    }
    #[doc = "0x08 - FIFO Control Register"]
    #[inline(always)]
    pub const fn fcr(&self) -> &Fcr {
        unsafe { &*(self as *const Self).cast::<u8>().add(8).cast() }
    }
    #[doc = "0x08 - Interrupt Identity Register"]
    #[inline(always)]
    pub const fn iir(&self) -> &Iir {
        unsafe { &*(self as *const Self).cast::<u8>().add(8).cast() }
    }
    #[doc = "0x0c - Line Control Register"]
    #[inline(always)]
    pub const fn lcr(&self) -> &Lcr {
        &self.lcr
    }
    #[doc = "0x10 - Modem Control Register"]
    #[inline(always)]
    pub const fn mcr(&self) -> &Mcr {
        &self.mcr
    }
    #[doc = "0x14 - Line Status Register"]
    #[inline(always)]
    pub const fn lsr(&self) -> &Lsr {
        &self.lsr
    }
    #[doc = "0x18 - Modem Status Register"]
    #[inline(always)]
    pub const fn msr(&self) -> &Msr {
        &self.msr
    }
    #[doc = "0x1c - Scratch Pad Register"]
    #[inline(always)]
    pub const fn scr(&self) -> &Scr {
        &self.scr
    }
    #[doc = "0x20 - Low Power Divisor Latch Low Register: This register is only valid when the DW_apb_uart is configured to have SIR low-power reception capabilities implemented (SIR_LP_RX = Yes). If SIR low-power reception capabilities are not implemented, this register does not exist and reading from thsi register address returns zero."]
    #[inline(always)]
    pub const fn lpdll(&self) -> &Lpdll {
        &self.lpdll
    }
    #[doc = "0x24 - Low Power Divisor Latch High Register: This register is only valid when the DW_apb_uart is configured to have SIR low-power reception capabilities implemented (SIR_LP_RX = Yes). If SIR low-power reception capabilities are not implemented, this register does not exist and reading from thsi register address returns zero."]
    #[inline(always)]
    pub const fn lpdlh(&self) -> &Lpdlh {
        &self.lpdlh
    }
    #[doc = "0x30..0x70 - This register is only valid when the DW_apb_uart is configured to have additional shadow registers implemented (SHADOW == YES)."]
    #[inline(always)]
    pub const fn shadow(&self, n: usize) -> &Shadow {
        &self.shadow[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x30..0x70 - This register is only valid when the DW_apb_uart is configured to have additional shadow registers implemented (SHADOW == YES)."]
    #[inline(always)]
    pub fn shadow_iter(&self) -> impl Iterator<Item = &Shadow> {
        self.shadow.iter()
    }
    #[doc = "0x70 - FIFO Access Register"]
    #[inline(always)]
    pub const fn far(&self) -> &Far {
        &self.far
    }
    #[doc = "0x74 - Transmit FIFO Read"]
    #[inline(always)]
    pub const fn tfr(&self) -> &Tfr {
        &self.tfr
    }
    #[doc = "0x78 - Receive FIFO Write"]
    #[inline(always)]
    pub const fn rfw(&self) -> &Rfw {
        &self.rfw
    }
    #[doc = "0x7c - UART Status Register"]
    #[inline(always)]
    pub const fn usr(&self) -> &Usr {
        &self.usr
    }
    #[doc = "0x80 - Transmit FIFO Level: This register is only valid when the DW_apb_uart is configured to have additional FIFO status registers implemented (FIFO_STAT == YES). If status registers are not implemented, this register does not exist and reading from this register address returns zero."]
    #[inline(always)]
    pub const fn tfl(&self) -> &Tfl {
        &self.tfl
    }
    #[doc = "0x84 - Receive FIFO Level: This register is only valid when the DW_apb_uart is configured to have additional FIFO status registers implemented (FIFO_STAT == YES). If status registers are not implemented, this register does not exist and reading from this register address returns zero."]
    #[inline(always)]
    pub const fn rfl(&self) -> &Rfl {
        &self.rfl
    }
    #[doc = "0x88 - Software Reset Register: This register is only valid when the DW_apb_uart is configured to have additional shadow registers implemented (SHADOW == YES). If shadow registers are not implemented, this register does not exist and reading from this register address returns zero."]
    #[inline(always)]
    pub const fn srr(&self) -> &Srr {
        &self.srr
    }
    #[doc = "0x8c - Shadow Request to Send: This register is only valid when the DW_apb_uart is configured to have additional shadow registers implemented (SHADOW == YES). If shadow registers are not implemented, this register does not exist and reading from this register address returns zero."]
    #[inline(always)]
    pub const fn srts(&self) -> &Srts {
        &self.srts
    }
    #[doc = "0x90 - Shadow Break Control Register: This register is only valid when the DW_apb_uart is configured to have additional shadow registers implemented (SHADOW == YES). If shadow registers are not implemented, this register does not exist and reading from this register address returns zero."]
    #[inline(always)]
    pub const fn sbcr(&self) -> &Sbcr {
        &self.sbcr
    }
    #[doc = "0x94 - Shadow DMA Mode: This register is only valid when the DW_apb_uart is configured to have additional FIFO registers implemented (FIFO_MODE != None) and additional shadow registers implemented (SHADOW == YES). If these registers are not implemented, this register does not exist and reading from this register address returns zero."]
    #[inline(always)]
    pub const fn sdmam(&self) -> &Sdmam {
        &self.sdmam
    }
    #[doc = "0x98 - Shadow FIFO Enable: This register is only valid when the DW_apb_uart is configured to have additional FIFO registers implemented (FIFO_MODE != None) and additional shadow registers implemented (SHADOW == YES). If these registers are not implemented, this register does not exist and reading from this register address returns zero."]
    #[inline(always)]
    pub const fn sfe(&self) -> &Sfe {
        &self.sfe
    }
    #[doc = "0x9c - Shadow RCVR Trigger: This register is only valid when the DW_apb_uart is configured to have additional FIFO registers implemented (FIFO_MODE != None) and additional shadow registers implemented (SHADOW == YES). If these registers are not implemented, this register does not exist and reading from this register address returns zero."]
    #[inline(always)]
    pub const fn srt(&self) -> &Srt {
        &self.srt
    }
    #[doc = "0xa0 - Shadow TX Empty Trigger: This register is only valid when the DW_apb_uart is configured to have FIFOs implemented (FIFO_MODE != NONE) and THRE interrupt support implemented (THRE_MODE_USER == Enabled) and additional shadow registers implemented (SHADOW == YES). If FIFOs are not implemented or THRE interrupt support is not implemented or shadow registers are not implemented, this register does not exist and reading from this register address returns zero."]
    #[inline(always)]
    pub const fn stet(&self) -> &Stet {
        &self.stet
    }
    #[doc = "0xa4 - Halt TX"]
    #[inline(always)]
    pub const fn htx(&self) -> &Htx {
        &self.htx
    }
    #[doc = "0xa8 - DMA Software Acknowledge"]
    #[inline(always)]
    pub const fn dmasa(&self) -> &Dmasa {
        &self.dmasa
    }
    #[doc = "0xf4 - Component Parameter Register: This register is only valid when the DW_apb_uart is configured to have the Component Parameter register implemented (UART_ADD_ENCODED_PARAMS == YES). If the Component Parameter register is not implemented, this register does not exist and reading from this register address returns zero."]
    #[inline(always)]
    pub const fn cpr(&self) -> &Cpr {
        &self.cpr
    }
    #[doc = "0xf8 - Component Type Register: This register is only valid when the DW_apb_uart is configured to have additional features implemented (ADDITIONAL_FEATURES == YES). If additional features are not implemented, this register does not exist and reading from this register address returns zero."]
    #[inline(always)]
    pub const fn ctr(&self) -> &Ctr {
        unsafe { &*(self as *const Self).cast::<u8>().add(248).cast() }
    }
    #[doc = "0xf8 - UART Component Version: This register is only valid when the DW_apb_uart is configured to have additional features implemented (ADDITIONAL_FEATURES == YES). If additional features are not implemented, this register does not exist and reading from this register address returns zero."]
    #[inline(always)]
    pub const fn ucv(&self) -> &Ucv {
        unsafe { &*(self as *const Self).cast::<u8>().add(248).cast() }
    }
}
#[doc = "rbr (r) register accessor: Receive Buffer Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rbr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rbr`]
module"]
#[doc(alias = "rbr")]
pub type Rbr = crate::Reg<rbr::RbrSpec>;
#[doc = "Receive Buffer Register"]
pub mod rbr;
#[doc = "thr (w) register accessor: Transmit Holding Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`thr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@thr`]
module"]
#[doc(alias = "thr")]
pub type Thr = crate::Reg<thr::ThrSpec>;
#[doc = "Transmit Holding Register"]
pub mod thr;
#[doc = "dll (rw) register accessor: Divisor Latch Low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dll::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dll::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dll`]
module"]
#[doc(alias = "dll")]
pub type Dll = crate::Reg<dll::DllSpec>;
#[doc = "Divisor Latch Low"]
pub mod dll;
#[doc = "dlh (rw) register accessor: Divisor Latch High\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dlh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dlh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dlh`]
module"]
#[doc(alias = "dlh")]
pub type Dlh = crate::Reg<dlh::DlhSpec>;
#[doc = "Divisor Latch High"]
pub mod dlh;
#[doc = "ier (rw) register accessor: Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ier::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ier::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier`]
module"]
#[doc(alias = "ier")]
pub type Ier = crate::Reg<ier::IerSpec>;
#[doc = "Interrupt Enable Register"]
pub mod ier;
#[doc = "iir (r) register accessor: Interrupt Identity Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`iir::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iir`]
module"]
#[doc(alias = "iir")]
pub type Iir = crate::Reg<iir::IirSpec>;
#[doc = "Interrupt Identity Register"]
pub mod iir;
#[doc = "fcr (w) register accessor: FIFO Control Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcr`]
module"]
#[doc(alias = "fcr")]
pub type Fcr = crate::Reg<fcr::FcrSpec>;
#[doc = "FIFO Control Register"]
pub mod fcr;
#[doc = "lcr (rw) register accessor: Line Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcr`]
module"]
#[doc(alias = "lcr")]
pub type Lcr = crate::Reg<lcr::LcrSpec>;
#[doc = "Line Control Register"]
pub mod lcr;
#[doc = "mcr (rw) register accessor: Modem Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcr`]
module"]
#[doc(alias = "mcr")]
pub type Mcr = crate::Reg<mcr::McrSpec>;
#[doc = "Modem Control Register"]
pub mod mcr;
#[doc = "lsr (r) register accessor: Line Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lsr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lsr`]
module"]
#[doc(alias = "lsr")]
pub type Lsr = crate::Reg<lsr::LsrSpec>;
#[doc = "Line Status Register"]
pub mod lsr;
#[doc = "msr (r) register accessor: Modem Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msr`]
module"]
#[doc(alias = "msr")]
pub type Msr = crate::Reg<msr::MsrSpec>;
#[doc = "Modem Status Register"]
pub mod msr;
#[doc = "scr (rw) register accessor: Scratch Pad Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scr`]
module"]
#[doc(alias = "scr")]
pub type Scr = crate::Reg<scr::ScrSpec>;
#[doc = "Scratch Pad Register"]
pub mod scr;
#[doc = "lpdll (rw) register accessor: Low Power Divisor Latch Low Register: This register is only valid when the DW_apb_uart is configured to have SIR low-power reception capabilities implemented (SIR_LP_RX = Yes). If SIR low-power reception capabilities are not implemented, this register does not exist and reading from thsi register address returns zero.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpdll::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpdll::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpdll`]
module"]
#[doc(alias = "lpdll")]
pub type Lpdll = crate::Reg<lpdll::LpdllSpec>;
#[doc = "Low Power Divisor Latch Low Register: This register is only valid when the DW_apb_uart is configured to have SIR low-power reception capabilities implemented (SIR_LP_RX = Yes). If SIR low-power reception capabilities are not implemented, this register does not exist and reading from thsi register address returns zero."]
pub mod lpdll;
#[doc = "lpdlh (rw) register accessor: Low Power Divisor Latch High Register: This register is only valid when the DW_apb_uart is configured to have SIR low-power reception capabilities implemented (SIR_LP_RX = Yes). If SIR low-power reception capabilities are not implemented, this register does not exist and reading from thsi register address returns zero.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpdlh::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpdlh::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpdlh`]
module"]
#[doc(alias = "lpdlh")]
pub type Lpdlh = crate::Reg<lpdlh::LpdlhSpec>;
#[doc = "Low Power Divisor Latch High Register: This register is only valid when the DW_apb_uart is configured to have SIR low-power reception capabilities implemented (SIR_LP_RX = Yes). If SIR low-power reception capabilities are not implemented, this register does not exist and reading from thsi register address returns zero."]
pub mod lpdlh;
#[doc = "shadow (rw) register accessor: This register is only valid when the DW_apb_uart is configured to have additional shadow registers implemented (SHADOW == YES).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`shadow::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`shadow::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shadow`]
module"]
#[doc(alias = "shadow")]
pub type Shadow = crate::Reg<shadow::ShadowSpec>;
#[doc = "This register is only valid when the DW_apb_uart is configured to have additional shadow registers implemented (SHADOW == YES)."]
pub mod shadow;
#[doc = "far (rw) register accessor: FIFO Access Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`far::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`far::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@far`]
module"]
#[doc(alias = "far")]
pub type Far = crate::Reg<far::FarSpec>;
#[doc = "FIFO Access Register"]
pub mod far;
#[doc = "tfr (r) register accessor: Transmit FIFO Read\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tfr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tfr`]
module"]
#[doc(alias = "tfr")]
pub type Tfr = crate::Reg<tfr::TfrSpec>;
#[doc = "Transmit FIFO Read"]
pub mod tfr;
#[doc = "rfw (w) register accessor: Receive FIFO Write\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rfw::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rfw`]
module"]
#[doc(alias = "rfw")]
pub type Rfw = crate::Reg<rfw::RfwSpec>;
#[doc = "Receive FIFO Write"]
pub mod rfw;
#[doc = "usr (r) register accessor: UART Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usr`]
module"]
#[doc(alias = "usr")]
pub type Usr = crate::Reg<usr::UsrSpec>;
#[doc = "UART Status Register"]
pub mod usr;
#[doc = "tfl (r) register accessor: Transmit FIFO Level: This register is only valid when the DW_apb_uart is configured to have additional FIFO status registers implemented (FIFO_STAT == YES). If status registers are not implemented, this register does not exist and reading from this register address returns zero.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tfl::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tfl`]
module"]
#[doc(alias = "tfl")]
pub type Tfl = crate::Reg<tfl::TflSpec>;
#[doc = "Transmit FIFO Level: This register is only valid when the DW_apb_uart is configured to have additional FIFO status registers implemented (FIFO_STAT == YES). If status registers are not implemented, this register does not exist and reading from this register address returns zero."]
pub mod tfl;
#[doc = "rfl (r) register accessor: Receive FIFO Level: This register is only valid when the DW_apb_uart is configured to have additional FIFO status registers implemented (FIFO_STAT == YES). If status registers are not implemented, this register does not exist and reading from this register address returns zero.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rfl::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rfl`]
module"]
#[doc(alias = "rfl")]
pub type Rfl = crate::Reg<rfl::RflSpec>;
#[doc = "Receive FIFO Level: This register is only valid when the DW_apb_uart is configured to have additional FIFO status registers implemented (FIFO_STAT == YES). If status registers are not implemented, this register does not exist and reading from this register address returns zero."]
pub mod rfl;
#[doc = "srr (w) register accessor: Software Reset Register: This register is only valid when the DW_apb_uart is configured to have additional shadow registers implemented (SHADOW == YES). If shadow registers are not implemented, this register does not exist and reading from this register address returns zero.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srr`]
module"]
#[doc(alias = "srr")]
pub type Srr = crate::Reg<srr::SrrSpec>;
#[doc = "Software Reset Register: This register is only valid when the DW_apb_uart is configured to have additional shadow registers implemented (SHADOW == YES). If shadow registers are not implemented, this register does not exist and reading from this register address returns zero."]
pub mod srr;
#[doc = "srts (rw) register accessor: Shadow Request to Send: This register is only valid when the DW_apb_uart is configured to have additional shadow registers implemented (SHADOW == YES). If shadow registers are not implemented, this register does not exist and reading from this register address returns zero.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srts`]
module"]
#[doc(alias = "srts")]
pub type Srts = crate::Reg<srts::SrtsSpec>;
#[doc = "Shadow Request to Send: This register is only valid when the DW_apb_uart is configured to have additional shadow registers implemented (SHADOW == YES). If shadow registers are not implemented, this register does not exist and reading from this register address returns zero."]
pub mod srts;
#[doc = "sbcr (rw) register accessor: Shadow Break Control Register: This register is only valid when the DW_apb_uart is configured to have additional shadow registers implemented (SHADOW == YES). If shadow registers are not implemented, this register does not exist and reading from this register address returns zero.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sbcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sbcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sbcr`]
module"]
#[doc(alias = "sbcr")]
pub type Sbcr = crate::Reg<sbcr::SbcrSpec>;
#[doc = "Shadow Break Control Register: This register is only valid when the DW_apb_uart is configured to have additional shadow registers implemented (SHADOW == YES). If shadow registers are not implemented, this register does not exist and reading from this register address returns zero."]
pub mod sbcr;
#[doc = "sdmam (rw) register accessor: Shadow DMA Mode: This register is only valid when the DW_apb_uart is configured to have additional FIFO registers implemented (FIFO_MODE != None) and additional shadow registers implemented (SHADOW == YES). If these registers are not implemented, this register does not exist and reading from this register address returns zero.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdmam::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdmam::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdmam`]
module"]
#[doc(alias = "sdmam")]
pub type Sdmam = crate::Reg<sdmam::SdmamSpec>;
#[doc = "Shadow DMA Mode: This register is only valid when the DW_apb_uart is configured to have additional FIFO registers implemented (FIFO_MODE != None) and additional shadow registers implemented (SHADOW == YES). If these registers are not implemented, this register does not exist and reading from this register address returns zero."]
pub mod sdmam;
#[doc = "sfe (rw) register accessor: Shadow FIFO Enable: This register is only valid when the DW_apb_uart is configured to have additional FIFO registers implemented (FIFO_MODE != None) and additional shadow registers implemented (SHADOW == YES). If these registers are not implemented, this register does not exist and reading from this register address returns zero.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sfe::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sfe::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sfe`]
module"]
#[doc(alias = "sfe")]
pub type Sfe = crate::Reg<sfe::SfeSpec>;
#[doc = "Shadow FIFO Enable: This register is only valid when the DW_apb_uart is configured to have additional FIFO registers implemented (FIFO_MODE != None) and additional shadow registers implemented (SHADOW == YES). If these registers are not implemented, this register does not exist and reading from this register address returns zero."]
pub mod sfe;
#[doc = "srt (rw) register accessor: Shadow RCVR Trigger: This register is only valid when the DW_apb_uart is configured to have additional FIFO registers implemented (FIFO_MODE != None) and additional shadow registers implemented (SHADOW == YES). If these registers are not implemented, this register does not exist and reading from this register address returns zero.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`srt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`srt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srt`]
module"]
#[doc(alias = "srt")]
pub type Srt = crate::Reg<srt::SrtSpec>;
#[doc = "Shadow RCVR Trigger: This register is only valid when the DW_apb_uart is configured to have additional FIFO registers implemented (FIFO_MODE != None) and additional shadow registers implemented (SHADOW == YES). If these registers are not implemented, this register does not exist and reading from this register address returns zero."]
pub mod srt;
#[doc = "stet (rw) register accessor: Shadow TX Empty Trigger: This register is only valid when the DW_apb_uart is configured to have FIFOs implemented (FIFO_MODE != NONE) and THRE interrupt support implemented (THRE_MODE_USER == Enabled) and additional shadow registers implemented (SHADOW == YES). If FIFOs are not implemented or THRE interrupt support is not implemented or shadow registers are not implemented, this register does not exist and reading from this register address returns zero.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stet::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stet::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stet`]
module"]
#[doc(alias = "stet")]
pub type Stet = crate::Reg<stet::StetSpec>;
#[doc = "Shadow TX Empty Trigger: This register is only valid when the DW_apb_uart is configured to have FIFOs implemented (FIFO_MODE != NONE) and THRE interrupt support implemented (THRE_MODE_USER == Enabled) and additional shadow registers implemented (SHADOW == YES). If FIFOs are not implemented or THRE interrupt support is not implemented or shadow registers are not implemented, this register does not exist and reading from this register address returns zero."]
pub mod stet;
#[doc = "htx (rw) register accessor: Halt TX\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`htx::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`htx::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@htx`]
module"]
#[doc(alias = "htx")]
pub type Htx = crate::Reg<htx::HtxSpec>;
#[doc = "Halt TX"]
pub mod htx;
#[doc = "dmasa (w) register accessor: DMA Software Acknowledge\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmasa::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmasa`]
module"]
#[doc(alias = "dmasa")]
pub type Dmasa = crate::Reg<dmasa::DmasaSpec>;
#[doc = "DMA Software Acknowledge"]
pub mod dmasa;
#[doc = "cpr (r) register accessor: Component Parameter Register: This register is only valid when the DW_apb_uart is configured to have the Component Parameter register implemented (UART_ADD_ENCODED_PARAMS == YES). If the Component Parameter register is not implemented, this register does not exist and reading from this register address returns zero.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpr`]
module"]
#[doc(alias = "cpr")]
pub type Cpr = crate::Reg<cpr::CprSpec>;
#[doc = "Component Parameter Register: This register is only valid when the DW_apb_uart is configured to have the Component Parameter register implemented (UART_ADD_ENCODED_PARAMS == YES). If the Component Parameter register is not implemented, this register does not exist and reading from this register address returns zero."]
pub mod cpr;
#[doc = "ucv (r) register accessor: UART Component Version: This register is only valid when the DW_apb_uart is configured to have additional features implemented (ADDITIONAL_FEATURES == YES). If additional features are not implemented, this register does not exist and reading from this register address returns zero.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ucv::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ucv`]
module"]
#[doc(alias = "ucv")]
pub type Ucv = crate::Reg<ucv::UcvSpec>;
#[doc = "UART Component Version: This register is only valid when the DW_apb_uart is configured to have additional features implemented (ADDITIONAL_FEATURES == YES). If additional features are not implemented, this register does not exist and reading from this register address returns zero."]
pub mod ucv;
#[doc = "ctr (r) register accessor: Component Type Register: This register is only valid when the DW_apb_uart is configured to have additional features implemented (ADDITIONAL_FEATURES == YES). If additional features are not implemented, this register does not exist and reading from this register address returns zero.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctr`]
module"]
#[doc(alias = "ctr")]
pub type Ctr = crate::Reg<ctr::CtrSpec>;
#[doc = "Component Type Register: This register is only valid when the DW_apb_uart is configured to have additional features implemented (ADDITIONAL_FEATURES == YES). If additional features are not implemented, this register does not exist and reading from this register address returns zero."]
pub mod ctr;
