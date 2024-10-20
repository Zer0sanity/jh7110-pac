#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    i2c_con: I2cCon,
    tar: Tar,
    sar: Sar,
    _reserved3: [u8; 0x04],
    data_cmd: DataCmd,
    ss_scl_hcnt: SsSclHcnt,
    ss_scl_lcnt: SsSclLcnt,
    fs_scl_hcnt: FsSclHcnt,
    fs_scl_lcnt: FsSclLcnt,
    hs_scl_hcnt: HsSclHcnt,
    hs_scl_lcnt: HsSclLcnt,
    intr_stat: IntrStat,
    intr_mask: IntrMask,
    raw_intr_stat: RawIntrStat,
    rx_tl: RxTl,
    tx_tl: TxTl,
    clr_intr: ClrIntr,
    clr_rx_under: ClrRxUnder,
    clr_rx_over: ClrRxOver,
    clr_tx_over: ClrTxOver,
    clr_rd_req: ClrRdReq,
    clr_tx_abrt: ClrTxAbrt,
    clr_rx_done: ClrRxDone,
    clr_activity: ClrActivity,
    clr_stop_det: ClrStopDet,
    clr_start_det: ClrStartDet,
    clr_gen_call: ClrGenCall,
    enable: Enable,
    status: Status,
    txflr: Txflr,
    rxflr: Rxflr,
    sda_hold: SdaHold,
    tx_abrt_source: TxAbrtSource,
    _reserved32: [u8; 0x18],
    enable_status: EnableStatus,
    _reserved33: [u8; 0x08],
    clr_restart_det: ClrRestartDet,
    _reserved34: [u8; 0x48],
    comp_param_1: CompParam1,
    comp_version: CompVersion,
    comp_type: CompType,
}
impl RegisterBlock {
    #[doc = "0x00 - DesignWare I2C CON"]
    #[inline(always)]
    pub const fn i2c_con(&self) -> &I2cCon {
        &self.i2c_con
    }
    #[doc = "0x04 - DesignWare I2C TAR"]
    #[inline(always)]
    pub const fn tar(&self) -> &Tar {
        &self.tar
    }
    #[doc = "0x08 - DesignWare I2C SAR"]
    #[inline(always)]
    pub const fn sar(&self) -> &Sar {
        &self.sar
    }
    #[doc = "0x10 - DesignWare I2C Data Command"]
    #[inline(always)]
    pub const fn data_cmd(&self) -> &DataCmd {
        &self.data_cmd
    }
    #[doc = "0x14 - DesignWare I2C SS SCL HCNT"]
    #[inline(always)]
    pub const fn ss_scl_hcnt(&self) -> &SsSclHcnt {
        &self.ss_scl_hcnt
    }
    #[doc = "0x18 - DesignWare I2C SS SCL LCNT"]
    #[inline(always)]
    pub const fn ss_scl_lcnt(&self) -> &SsSclLcnt {
        &self.ss_scl_lcnt
    }
    #[doc = "0x1c - DesignWare I2C FS SCL HCNT"]
    #[inline(always)]
    pub const fn fs_scl_hcnt(&self) -> &FsSclHcnt {
        &self.fs_scl_hcnt
    }
    #[doc = "0x20 - DesignWare I2C FS SCL LCNT"]
    #[inline(always)]
    pub const fn fs_scl_lcnt(&self) -> &FsSclLcnt {
        &self.fs_scl_lcnt
    }
    #[doc = "0x24 - DesignWare I2C HS SCL HCNT"]
    #[inline(always)]
    pub const fn hs_scl_hcnt(&self) -> &HsSclHcnt {
        &self.hs_scl_hcnt
    }
    #[doc = "0x28 - DesignWare I2C HS SCL LCNT"]
    #[inline(always)]
    pub const fn hs_scl_lcnt(&self) -> &HsSclLcnt {
        &self.hs_scl_lcnt
    }
    #[doc = "0x2c - DesignWare I2C Interrupt Status"]
    #[inline(always)]
    pub const fn intr_stat(&self) -> &IntrStat {
        &self.intr_stat
    }
    #[doc = "0x30 - DesignWare I2C Interrupt Mask"]
    #[inline(always)]
    pub const fn intr_mask(&self) -> &IntrMask {
        &self.intr_mask
    }
    #[doc = "0x34 - DesignWare I2C Raw Interrupt Status"]
    #[inline(always)]
    pub const fn raw_intr_stat(&self) -> &RawIntrStat {
        &self.raw_intr_stat
    }
    #[doc = "0x38 - DesignWare I2C RX TL"]
    #[inline(always)]
    pub const fn rx_tl(&self) -> &RxTl {
        &self.rx_tl
    }
    #[doc = "0x3c - DesignWare I2C TX TL"]
    #[inline(always)]
    pub const fn tx_tl(&self) -> &TxTl {
        &self.tx_tl
    }
    #[doc = "0x40 - DesignWare I2C Clear Interrrupt"]
    #[inline(always)]
    pub const fn clr_intr(&self) -> &ClrIntr {
        &self.clr_intr
    }
    #[doc = "0x44 - DesignWare I2C Clear RX Underrun"]
    #[inline(always)]
    pub const fn clr_rx_under(&self) -> &ClrRxUnder {
        &self.clr_rx_under
    }
    #[doc = "0x48 - DesignWare I2C Clear RX Overrun"]
    #[inline(always)]
    pub const fn clr_rx_over(&self) -> &ClrRxOver {
        &self.clr_rx_over
    }
    #[doc = "0x4c - DesignWare I2C Clear TX Overrun"]
    #[inline(always)]
    pub const fn clr_tx_over(&self) -> &ClrTxOver {
        &self.clr_tx_over
    }
    #[doc = "0x50 - DesignWare I2C Clear Read Request"]
    #[inline(always)]
    pub const fn clr_rd_req(&self) -> &ClrRdReq {
        &self.clr_rd_req
    }
    #[doc = "0x54 - DesignWare I2C Clear TX Abort"]
    #[inline(always)]
    pub const fn clr_tx_abrt(&self) -> &ClrTxAbrt {
        &self.clr_tx_abrt
    }
    #[doc = "0x58 - DesignWare I2C Clear RX Done"]
    #[inline(always)]
    pub const fn clr_rx_done(&self) -> &ClrRxDone {
        &self.clr_rx_done
    }
    #[doc = "0x5c - DesignWare I2C Clear Activity"]
    #[inline(always)]
    pub const fn clr_activity(&self) -> &ClrActivity {
        &self.clr_activity
    }
    #[doc = "0x60 - DesignWare I2C Clear Stop DET"]
    #[inline(always)]
    pub const fn clr_stop_det(&self) -> &ClrStopDet {
        &self.clr_stop_det
    }
    #[doc = "0x64 - DesignWare I2C Clear Start DET"]
    #[inline(always)]
    pub const fn clr_start_det(&self) -> &ClrStartDet {
        &self.clr_start_det
    }
    #[doc = "0x68 - DesignWare I2C Clear General Call"]
    #[inline(always)]
    pub const fn clr_gen_call(&self) -> &ClrGenCall {
        &self.clr_gen_call
    }
    #[doc = "0x6c - DesignWare I2C Enable"]
    #[inline(always)]
    pub const fn enable(&self) -> &Enable {
        &self.enable
    }
    #[doc = "0x70 - DesignWare I2C Status"]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x74 - DesignWare I2C TX Failure"]
    #[inline(always)]
    pub const fn txflr(&self) -> &Txflr {
        &self.txflr
    }
    #[doc = "0x78 - DesignWare I2C RX Failure"]
    #[inline(always)]
    pub const fn rxflr(&self) -> &Rxflr {
        &self.rxflr
    }
    #[doc = "0x7c - DesignWare I2C SDA Hold"]
    #[inline(always)]
    pub const fn sda_hold(&self) -> &SdaHold {
        &self.sda_hold
    }
    #[doc = "0x80 - DesignWare I2C TX Abort Source"]
    #[inline(always)]
    pub const fn tx_abrt_source(&self) -> &TxAbrtSource {
        &self.tx_abrt_source
    }
    #[doc = "0x9c - DesignWare I2C Enable Status"]
    #[inline(always)]
    pub const fn enable_status(&self) -> &EnableStatus {
        &self.enable_status
    }
    #[doc = "0xa8 - DesignWare I2C Clear Restart DET"]
    #[inline(always)]
    pub const fn clr_restart_det(&self) -> &ClrRestartDet {
        &self.clr_restart_det
    }
    #[doc = "0xf4 - DesignWare I2C Compatibility Parameter 1"]
    #[inline(always)]
    pub const fn comp_param_1(&self) -> &CompParam1 {
        &self.comp_param_1
    }
    #[doc = "0xf8 - DesignWare I2C Compatibility Version"]
    #[inline(always)]
    pub const fn comp_version(&self) -> &CompVersion {
        &self.comp_version
    }
    #[doc = "0xfc - DesignWare I2C Compatibility Type"]
    #[inline(always)]
    pub const fn comp_type(&self) -> &CompType {
        &self.comp_type
    }
}
#[doc = "i2c_con (rw) register accessor: DesignWare I2C CON\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_con::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_con::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@i2c_con`]
module"]
#[doc(alias = "i2c_con")]
pub type I2cCon = crate::Reg<i2c_con::I2cConSpec>;
#[doc = "DesignWare I2C CON"]
pub mod i2c_con;
#[doc = "tar (rw) register accessor: DesignWare I2C TAR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tar`]
module"]
#[doc(alias = "tar")]
pub type Tar = crate::Reg<tar::TarSpec>;
#[doc = "DesignWare I2C TAR"]
pub mod tar;
#[doc = "sar (rw) register accessor: DesignWare I2C SAR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar`]
module"]
#[doc(alias = "sar")]
pub type Sar = crate::Reg<sar::SarSpec>;
#[doc = "DesignWare I2C SAR"]
pub mod sar;
#[doc = "data_cmd (rw) register accessor: DesignWare I2C Data Command\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data_cmd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data_cmd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data_cmd`]
module"]
#[doc(alias = "data_cmd")]
pub type DataCmd = crate::Reg<data_cmd::DataCmdSpec>;
#[doc = "DesignWare I2C Data Command"]
pub mod data_cmd;
#[doc = "ss_scl_hcnt (rw) register accessor: DesignWare I2C SS SCL HCNT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ss_scl_hcnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ss_scl_hcnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ss_scl_hcnt`]
module"]
#[doc(alias = "ss_scl_hcnt")]
pub type SsSclHcnt = crate::Reg<ss_scl_hcnt::SsSclHcntSpec>;
#[doc = "DesignWare I2C SS SCL HCNT"]
pub mod ss_scl_hcnt;
#[doc = "ss_scl_lcnt (rw) register accessor: DesignWare I2C SS SCL LCNT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ss_scl_lcnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ss_scl_lcnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ss_scl_lcnt`]
module"]
#[doc(alias = "ss_scl_lcnt")]
pub type SsSclLcnt = crate::Reg<ss_scl_lcnt::SsSclLcntSpec>;
#[doc = "DesignWare I2C SS SCL LCNT"]
pub mod ss_scl_lcnt;
#[doc = "fs_scl_hcnt (rw) register accessor: DesignWare I2C FS SCL HCNT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fs_scl_hcnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fs_scl_hcnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fs_scl_hcnt`]
module"]
#[doc(alias = "fs_scl_hcnt")]
pub type FsSclHcnt = crate::Reg<fs_scl_hcnt::FsSclHcntSpec>;
#[doc = "DesignWare I2C FS SCL HCNT"]
pub mod fs_scl_hcnt;
#[doc = "fs_scl_lcnt (rw) register accessor: DesignWare I2C FS SCL LCNT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fs_scl_lcnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fs_scl_lcnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fs_scl_lcnt`]
module"]
#[doc(alias = "fs_scl_lcnt")]
pub type FsSclLcnt = crate::Reg<fs_scl_lcnt::FsSclLcntSpec>;
#[doc = "DesignWare I2C FS SCL LCNT"]
pub mod fs_scl_lcnt;
#[doc = "hs_scl_hcnt (rw) register accessor: DesignWare I2C HS SCL HCNT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hs_scl_hcnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hs_scl_hcnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hs_scl_hcnt`]
module"]
#[doc(alias = "hs_scl_hcnt")]
pub type HsSclHcnt = crate::Reg<hs_scl_hcnt::HsSclHcntSpec>;
#[doc = "DesignWare I2C HS SCL HCNT"]
pub mod hs_scl_hcnt;
#[doc = "hs_scl_lcnt (rw) register accessor: DesignWare I2C HS SCL LCNT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hs_scl_lcnt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hs_scl_lcnt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hs_scl_lcnt`]
module"]
#[doc(alias = "hs_scl_lcnt")]
pub type HsSclLcnt = crate::Reg<hs_scl_lcnt::HsSclLcntSpec>;
#[doc = "DesignWare I2C HS SCL LCNT"]
pub mod hs_scl_lcnt;
#[doc = "intr_stat (r) register accessor: DesignWare I2C Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_stat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_stat`]
module"]
#[doc(alias = "intr_stat")]
pub type IntrStat = crate::Reg<intr_stat::IntrStatSpec>;
#[doc = "DesignWare I2C Interrupt Status"]
pub mod intr_stat;
#[doc = "intr_mask (rw) register accessor: DesignWare I2C Interrupt Mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_mask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intr_mask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intr_mask`]
module"]
#[doc(alias = "intr_mask")]
pub type IntrMask = crate::Reg<intr_mask::IntrMaskSpec>;
#[doc = "DesignWare I2C Interrupt Mask"]
pub mod intr_mask;
#[doc = "raw_intr_stat (r) register accessor: DesignWare I2C Raw Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`raw_intr_stat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@raw_intr_stat`]
module"]
#[doc(alias = "raw_intr_stat")]
pub type RawIntrStat = crate::Reg<raw_intr_stat::RawIntrStatSpec>;
#[doc = "DesignWare I2C Raw Interrupt Status"]
pub mod raw_intr_stat;
#[doc = "rx_tl (rw) register accessor: DesignWare I2C RX TL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_tl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rx_tl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_tl`]
module"]
#[doc(alias = "rx_tl")]
pub type RxTl = crate::Reg<rx_tl::RxTlSpec>;
#[doc = "DesignWare I2C RX TL"]
pub mod rx_tl;
#[doc = "tx_tl (rw) register accessor: DesignWare I2C TX TL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_tl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_tl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_tl`]
module"]
#[doc(alias = "tx_tl")]
pub type TxTl = crate::Reg<tx_tl::TxTlSpec>;
#[doc = "DesignWare I2C TX TL"]
pub mod tx_tl;
#[doc = "clr_intr (rw) register accessor: DesignWare I2C Clear Interrrupt\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clr_intr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clr_intr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clr_intr`]
module"]
#[doc(alias = "clr_intr")]
pub type ClrIntr = crate::Reg<clr_intr::ClrIntrSpec>;
#[doc = "DesignWare I2C Clear Interrrupt"]
pub mod clr_intr;
#[doc = "clr_rx_under (rw) register accessor: DesignWare I2C Clear RX Underrun\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clr_rx_under::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clr_rx_under::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clr_rx_under`]
module"]
#[doc(alias = "clr_rx_under")]
pub type ClrRxUnder = crate::Reg<clr_rx_under::ClrRxUnderSpec>;
#[doc = "DesignWare I2C Clear RX Underrun"]
pub mod clr_rx_under;
#[doc = "clr_rx_over (rw) register accessor: DesignWare I2C Clear RX Overrun\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clr_rx_over::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clr_rx_over::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clr_rx_over`]
module"]
#[doc(alias = "clr_rx_over")]
pub type ClrRxOver = crate::Reg<clr_rx_over::ClrRxOverSpec>;
#[doc = "DesignWare I2C Clear RX Overrun"]
pub mod clr_rx_over;
#[doc = "clr_tx_over (rw) register accessor: DesignWare I2C Clear TX Overrun\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clr_tx_over::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clr_tx_over::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clr_tx_over`]
module"]
#[doc(alias = "clr_tx_over")]
pub type ClrTxOver = crate::Reg<clr_tx_over::ClrTxOverSpec>;
#[doc = "DesignWare I2C Clear TX Overrun"]
pub mod clr_tx_over;
#[doc = "clr_rd_req (rw) register accessor: DesignWare I2C Clear Read Request\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clr_rd_req::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clr_rd_req::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clr_rd_req`]
module"]
#[doc(alias = "clr_rd_req")]
pub type ClrRdReq = crate::Reg<clr_rd_req::ClrRdReqSpec>;
#[doc = "DesignWare I2C Clear Read Request"]
pub mod clr_rd_req;
#[doc = "clr_tx_abrt (rw) register accessor: DesignWare I2C Clear TX Abort\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clr_tx_abrt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clr_tx_abrt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clr_tx_abrt`]
module"]
#[doc(alias = "clr_tx_abrt")]
pub type ClrTxAbrt = crate::Reg<clr_tx_abrt::ClrTxAbrtSpec>;
#[doc = "DesignWare I2C Clear TX Abort"]
pub mod clr_tx_abrt;
#[doc = "clr_rx_done (rw) register accessor: DesignWare I2C Clear RX Done\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clr_rx_done::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clr_rx_done::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clr_rx_done`]
module"]
#[doc(alias = "clr_rx_done")]
pub type ClrRxDone = crate::Reg<clr_rx_done::ClrRxDoneSpec>;
#[doc = "DesignWare I2C Clear RX Done"]
pub mod clr_rx_done;
#[doc = "clr_activity (rw) register accessor: DesignWare I2C Clear Activity\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clr_activity::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clr_activity::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clr_activity`]
module"]
#[doc(alias = "clr_activity")]
pub type ClrActivity = crate::Reg<clr_activity::ClrActivitySpec>;
#[doc = "DesignWare I2C Clear Activity"]
pub mod clr_activity;
#[doc = "clr_stop_det (rw) register accessor: DesignWare I2C Clear Stop DET\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clr_stop_det::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clr_stop_det::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clr_stop_det`]
module"]
#[doc(alias = "clr_stop_det")]
pub type ClrStopDet = crate::Reg<clr_stop_det::ClrStopDetSpec>;
#[doc = "DesignWare I2C Clear Stop DET"]
pub mod clr_stop_det;
#[doc = "clr_start_det (rw) register accessor: DesignWare I2C Clear Start DET\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clr_start_det::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clr_start_det::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clr_start_det`]
module"]
#[doc(alias = "clr_start_det")]
pub type ClrStartDet = crate::Reg<clr_start_det::ClrStartDetSpec>;
#[doc = "DesignWare I2C Clear Start DET"]
pub mod clr_start_det;
#[doc = "clr_gen_call (rw) register accessor: DesignWare I2C Clear General Call\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clr_gen_call::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clr_gen_call::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clr_gen_call`]
module"]
#[doc(alias = "clr_gen_call")]
pub type ClrGenCall = crate::Reg<clr_gen_call::ClrGenCallSpec>;
#[doc = "DesignWare I2C Clear General Call"]
pub mod clr_gen_call;
#[doc = "enable (rw) register accessor: DesignWare I2C Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`enable::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enable::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enable`]
module"]
#[doc(alias = "enable")]
pub type Enable = crate::Reg<enable::EnableSpec>;
#[doc = "DesignWare I2C Enable"]
pub mod enable;
#[doc = "status (r) register accessor: DesignWare I2C Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
#[doc(alias = "status")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "DesignWare I2C Status"]
pub mod status;
#[doc = "txflr (rw) register accessor: DesignWare I2C TX Failure\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txflr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txflr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txflr`]
module"]
#[doc(alias = "txflr")]
pub type Txflr = crate::Reg<txflr::TxflrSpec>;
#[doc = "DesignWare I2C TX Failure"]
pub mod txflr;
#[doc = "rxflr (rw) register accessor: DesignWare I2C RX Failure\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxflr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxflr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxflr`]
module"]
#[doc(alias = "rxflr")]
pub type Rxflr = crate::Reg<rxflr::RxflrSpec>;
#[doc = "DesignWare I2C RX Failure"]
pub mod rxflr;
#[doc = "sda_hold (rw) register accessor: DesignWare I2C SDA Hold\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sda_hold::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sda_hold::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sda_hold`]
module"]
#[doc(alias = "sda_hold")]
pub type SdaHold = crate::Reg<sda_hold::SdaHoldSpec>;
#[doc = "DesignWare I2C SDA Hold"]
pub mod sda_hold;
#[doc = "tx_abrt_source (r) register accessor: DesignWare I2C TX Abort Source\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_abrt_source::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_abrt_source`]
module"]
#[doc(alias = "tx_abrt_source")]
pub type TxAbrtSource = crate::Reg<tx_abrt_source::TxAbrtSourceSpec>;
#[doc = "DesignWare I2C TX Abort Source"]
pub mod tx_abrt_source;
#[doc = "enable_status (rw) register accessor: DesignWare I2C Enable Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`enable_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enable_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enable_status`]
module"]
#[doc(alias = "enable_status")]
pub type EnableStatus = crate::Reg<enable_status::EnableStatusSpec>;
#[doc = "DesignWare I2C Enable Status"]
pub mod enable_status;
#[doc = "clr_restart_det (rw) register accessor: DesignWare I2C Clear Restart DET\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clr_restart_det::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clr_restart_det::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clr_restart_det`]
module"]
#[doc(alias = "clr_restart_det")]
pub type ClrRestartDet = crate::Reg<clr_restart_det::ClrRestartDetSpec>;
#[doc = "DesignWare I2C Clear Restart DET"]
pub mod clr_restart_det;
#[doc = "comp_param_1 (r) register accessor: DesignWare I2C Compatibility Parameter 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comp_param_1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp_param_1`]
module"]
#[doc(alias = "comp_param_1")]
pub type CompParam1 = crate::Reg<comp_param_1::CompParam1Spec>;
#[doc = "DesignWare I2C Compatibility Parameter 1"]
pub mod comp_param_1;
#[doc = "comp_version (r) register accessor: DesignWare I2C Compatibility Version\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comp_version::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp_version`]
module"]
#[doc(alias = "comp_version")]
pub type CompVersion = crate::Reg<comp_version::CompVersionSpec>;
#[doc = "DesignWare I2C Compatibility Version"]
pub mod comp_version;
#[doc = "comp_type (r) register accessor: DesignWare I2C Compatibility Type\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comp_type::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp_type`]
module"]
#[doc(alias = "comp_type")]
pub type CompType = crate::Reg<comp_type::CompTypeSpec>;
#[doc = "DesignWare I2C Compatibility Type"]
pub mod comp_type;
