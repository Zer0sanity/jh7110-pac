#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    config: Config,
    rd_instr: RdInstr,
    wr_instr: WrInstr,
    delay: Delay,
    read_capture: ReadCapture,
    size: Size,
    sram_partition: SramPartition,
    indirect_trigger: IndirectTrigger,
    dma: Dma,
    remap: Remap,
    mode_bit: ModeBit,
    sdram_level: SdramLevel,
    _reserved12: [u8; 0x08],
    wr_completion_ctrl: WrCompletionCtrl,
    _reserved13: [u8; 0x04],
    irq_status: IrqStatus,
    irq_mask: IrqMask,
    _reserved15: [u8; 0x18],
    indirect_rd: IndirectRd,
    _reserved16: [u8; 0x0c],
    indirect_wr: IndirectWr,
    indirect_wr_watermark: IndirectWrWatermark,
    indirect_wr_start_addr: IndirectWrStartAddr,
    indirect_wr_bytes: IndirectWrBytes,
    _reserved20: [u8; 0x10],
    cmd_ctrl: CmdCtrl,
    cmd_address: CmdAddress,
    _reserved22: [u8; 0x08],
    cmd_read_at_lower: CmdReadAtLower,
    cmd_read_at_upper: CmdReadAtUpper,
    cmd_write_at_lower: CmdWriteAtLower,
    cmd_write_at_upper: CmdWriteAtUpper,
    polling_status: PollingStatus,
    _reserved27: [u8; 0x2c],
    ext_lower: ExtLower,
}
impl RegisterBlock {
    #[doc = "0x00 - Cadence QSPI Configuration"]
    #[inline(always)]
    pub const fn config(&self) -> &Config {
        &self.config
    }
    #[doc = "0x04 - Cadence QSPI Read Instruction"]
    #[inline(always)]
    pub const fn rd_instr(&self) -> &RdInstr {
        &self.rd_instr
    }
    #[doc = "0x08 - Cadence QSPI Write Instruction"]
    #[inline(always)]
    pub const fn wr_instr(&self) -> &WrInstr {
        &self.wr_instr
    }
    #[doc = "0x0c - Cadence QSPI Delay"]
    #[inline(always)]
    pub const fn delay(&self) -> &Delay {
        &self.delay
    }
    #[doc = "0x10 - Cadence QSPI Read Capture"]
    #[inline(always)]
    pub const fn read_capture(&self) -> &ReadCapture {
        &self.read_capture
    }
    #[doc = "0x14 - Cadence QSPI Size Configuration"]
    #[inline(always)]
    pub const fn size(&self) -> &Size {
        &self.size
    }
    #[doc = "0x18 - Cadence QSPI SRAM Partition Size"]
    #[inline(always)]
    pub const fn sram_partition(&self) -> &SramPartition {
        &self.sram_partition
    }
    #[doc = "0x1c - Cadence QSPI Indirect Trigger Address"]
    #[inline(always)]
    pub const fn indirect_trigger(&self) -> &IndirectTrigger {
        &self.indirect_trigger
    }
    #[doc = "0x20 - Cadence QSPI Direct Memory Access"]
    #[inline(always)]
    pub const fn dma(&self) -> &Dma {
        &self.dma
    }
    #[doc = "0x24 - Cadence QSPI Remap Address"]
    #[inline(always)]
    pub const fn remap(&self) -> &Remap {
        &self.remap
    }
    #[doc = "0x28 - Cadence QSPI Mode Bit(s)"]
    #[inline(always)]
    pub const fn mode_bit(&self) -> &ModeBit {
        &self.mode_bit
    }
    #[doc = "0x2c - Cadence QSPI SDRAM Level"]
    #[inline(always)]
    pub const fn sdram_level(&self) -> &SdramLevel {
        &self.sdram_level
    }
    #[doc = "0x38 - Cadence QSPI Write Completion Control"]
    #[inline(always)]
    pub const fn wr_completion_ctrl(&self) -> &WrCompletionCtrl {
        &self.wr_completion_ctrl
    }
    #[doc = "0x40 - Cadence QSPI IRQ Status"]
    #[inline(always)]
    pub const fn irq_status(&self) -> &IrqStatus {
        &self.irq_status
    }
    #[doc = "0x44 - Cadence QSPI IRQ Mask"]
    #[inline(always)]
    pub const fn irq_mask(&self) -> &IrqMask {
        &self.irq_mask
    }
    #[doc = "0x60 - Cadence QSPI Indirect Read"]
    #[inline(always)]
    pub const fn indirect_rd(&self) -> &IndirectRd {
        &self.indirect_rd
    }
    #[doc = "0x70 - Cadence QSPI Indirect Write"]
    #[inline(always)]
    pub const fn indirect_wr(&self) -> &IndirectWr {
        &self.indirect_wr
    }
    #[doc = "0x74 - Cadence QSPI Indirect Write Watermark"]
    #[inline(always)]
    pub const fn indirect_wr_watermark(&self) -> &IndirectWrWatermark {
        &self.indirect_wr_watermark
    }
    #[doc = "0x78 - Cadence QSPI Indirect Write Start Address"]
    #[inline(always)]
    pub const fn indirect_wr_start_addr(&self) -> &IndirectWrStartAddr {
        &self.indirect_wr_start_addr
    }
    #[doc = "0x7c - Cadence QSPI Indirect Write Bytes"]
    #[inline(always)]
    pub const fn indirect_wr_bytes(&self) -> &IndirectWrBytes {
        &self.indirect_wr_bytes
    }
    #[doc = "0x90 - Cadence QSPI Command Control"]
    #[inline(always)]
    pub const fn cmd_ctrl(&self) -> &CmdCtrl {
        &self.cmd_ctrl
    }
    #[doc = "0x94 - Cadence QSPI Command Address"]
    #[inline(always)]
    pub const fn cmd_address(&self) -> &CmdAddress {
        &self.cmd_address
    }
    #[doc = "0xa0 - Cadence QSPI Command Read at Lower"]
    #[inline(always)]
    pub const fn cmd_read_at_lower(&self) -> &CmdReadAtLower {
        &self.cmd_read_at_lower
    }
    #[doc = "0xa4 - Cadence QSPI Command Read at Upper"]
    #[inline(always)]
    pub const fn cmd_read_at_upper(&self) -> &CmdReadAtUpper {
        &self.cmd_read_at_upper
    }
    #[doc = "0xa8 - Cadence QSPI Command Write at Lower"]
    #[inline(always)]
    pub const fn cmd_write_at_lower(&self) -> &CmdWriteAtLower {
        &self.cmd_write_at_lower
    }
    #[doc = "0xac - Cadence QSPI Command Write at Upper"]
    #[inline(always)]
    pub const fn cmd_write_at_upper(&self) -> &CmdWriteAtUpper {
        &self.cmd_write_at_upper
    }
    #[doc = "0xb0 - Cadence QSPI Polling Status"]
    #[inline(always)]
    pub const fn polling_status(&self) -> &PollingStatus {
        &self.polling_status
    }
    #[doc = "0xe0 - Cadence QSPI Extension Lower"]
    #[inline(always)]
    pub const fn ext_lower(&self) -> &ExtLower {
        &self.ext_lower
    }
}
#[doc = "config (rw) register accessor: Cadence QSPI Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config`]
module"]
#[doc(alias = "config")]
pub type Config = crate::Reg<config::ConfigSpec>;
#[doc = "Cadence QSPI Configuration"]
pub mod config;
#[doc = "rd_instr (rw) register accessor: Cadence QSPI Read Instruction\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rd_instr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rd_instr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rd_instr`]
module"]
#[doc(alias = "rd_instr")]
pub type RdInstr = crate::Reg<rd_instr::RdInstrSpec>;
#[doc = "Cadence QSPI Read Instruction"]
pub mod rd_instr;
#[doc = "wr_instr (rw) register accessor: Cadence QSPI Write Instruction\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wr_instr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wr_instr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wr_instr`]
module"]
#[doc(alias = "wr_instr")]
pub type WrInstr = crate::Reg<wr_instr::WrInstrSpec>;
#[doc = "Cadence QSPI Write Instruction"]
pub mod wr_instr;
#[doc = "delay (rw) register accessor: Cadence QSPI Delay\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`delay::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`delay::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@delay`]
module"]
#[doc(alias = "delay")]
pub type Delay = crate::Reg<delay::DelaySpec>;
#[doc = "Cadence QSPI Delay"]
pub mod delay;
#[doc = "read_capture (rw) register accessor: Cadence QSPI Read Capture\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`read_capture::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`read_capture::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@read_capture`]
module"]
#[doc(alias = "read_capture")]
pub type ReadCapture = crate::Reg<read_capture::ReadCaptureSpec>;
#[doc = "Cadence QSPI Read Capture"]
pub mod read_capture;
#[doc = "size (rw) register accessor: Cadence QSPI Size Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`size::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`size::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@size`]
module"]
#[doc(alias = "size")]
pub type Size = crate::Reg<size::SizeSpec>;
#[doc = "Cadence QSPI Size Configuration"]
pub mod size;
#[doc = "sram_partition (rw) register accessor: Cadence QSPI SRAM Partition Size\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sram_partition::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sram_partition::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sram_partition`]
module"]
#[doc(alias = "sram_partition")]
pub type SramPartition = crate::Reg<sram_partition::SramPartitionSpec>;
#[doc = "Cadence QSPI SRAM Partition Size"]
pub mod sram_partition;
#[doc = "indirect_trigger (rw) register accessor: Cadence QSPI Indirect Trigger Address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`indirect_trigger::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`indirect_trigger::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@indirect_trigger`]
module"]
#[doc(alias = "indirect_trigger")]
pub type IndirectTrigger = crate::Reg<indirect_trigger::IndirectTriggerSpec>;
#[doc = "Cadence QSPI Indirect Trigger Address"]
pub mod indirect_trigger;
#[doc = "dma (rw) register accessor: Cadence QSPI Direct Memory Access\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma`]
module"]
#[doc(alias = "dma")]
pub type Dma = crate::Reg<dma::DmaSpec>;
#[doc = "Cadence QSPI Direct Memory Access"]
pub mod dma;
#[doc = "remap (rw) register accessor: Cadence QSPI Remap Address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`remap::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`remap::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@remap`]
module"]
#[doc(alias = "remap")]
pub type Remap = crate::Reg<remap::RemapSpec>;
#[doc = "Cadence QSPI Remap Address"]
pub mod remap;
#[doc = "mode_bit (rw) register accessor: Cadence QSPI Mode Bit(s)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mode_bit::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mode_bit::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mode_bit`]
module"]
#[doc(alias = "mode_bit")]
pub type ModeBit = crate::Reg<mode_bit::ModeBitSpec>;
#[doc = "Cadence QSPI Mode Bit(s)"]
pub mod mode_bit;
#[doc = "sdram_level (r) register accessor: Cadence QSPI SDRAM Level\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdram_level::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sdram_level`]
module"]
#[doc(alias = "sdram_level")]
pub type SdramLevel = crate::Reg<sdram_level::SdramLevelSpec>;
#[doc = "Cadence QSPI SDRAM Level"]
pub mod sdram_level;
#[doc = "wr_completion_ctrl (rw) register accessor: Cadence QSPI Write Completion Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wr_completion_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wr_completion_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wr_completion_ctrl`]
module"]
#[doc(alias = "wr_completion_ctrl")]
pub type WrCompletionCtrl = crate::Reg<wr_completion_ctrl::WrCompletionCtrlSpec>;
#[doc = "Cadence QSPI Write Completion Control"]
pub mod wr_completion_ctrl;
#[doc = "irq_status (rw) register accessor: Cadence QSPI IRQ Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irq_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`irq_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irq_status`]
module"]
#[doc(alias = "irq_status")]
pub type IrqStatus = crate::Reg<irq_status::IrqStatusSpec>;
#[doc = "Cadence QSPI IRQ Status"]
pub mod irq_status;
#[doc = "irq_mask (rw) register accessor: Cadence QSPI IRQ Mask\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irq_mask::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`irq_mask::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irq_mask`]
module"]
#[doc(alias = "irq_mask")]
pub type IrqMask = crate::Reg<irq_mask::IrqMaskSpec>;
#[doc = "Cadence QSPI IRQ Mask"]
pub mod irq_mask;
#[doc = "indirect_rd (rw) register accessor: Cadence QSPI Indirect Read\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`indirect_rd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`indirect_rd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@indirect_rd`]
module"]
#[doc(alias = "indirect_rd")]
pub type IndirectRd = crate::Reg<indirect_rd::IndirectRdSpec>;
#[doc = "Cadence QSPI Indirect Read"]
pub mod indirect_rd;
#[doc = "indirect_wr (rw) register accessor: Cadence QSPI Indirect Write\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`indirect_wr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`indirect_wr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@indirect_wr`]
module"]
#[doc(alias = "indirect_wr")]
pub type IndirectWr = crate::Reg<indirect_wr::IndirectWrSpec>;
#[doc = "Cadence QSPI Indirect Write"]
pub mod indirect_wr;
#[doc = "indirect_wr_watermark (rw) register accessor: Cadence QSPI Indirect Write Watermark\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`indirect_wr_watermark::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`indirect_wr_watermark::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@indirect_wr_watermark`]
module"]
#[doc(alias = "indirect_wr_watermark")]
pub type IndirectWrWatermark = crate::Reg<indirect_wr_watermark::IndirectWrWatermarkSpec>;
#[doc = "Cadence QSPI Indirect Write Watermark"]
pub mod indirect_wr_watermark;
#[doc = "indirect_wr_start_addr (rw) register accessor: Cadence QSPI Indirect Write Start Address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`indirect_wr_start_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`indirect_wr_start_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@indirect_wr_start_addr`]
module"]
#[doc(alias = "indirect_wr_start_addr")]
pub type IndirectWrStartAddr = crate::Reg<indirect_wr_start_addr::IndirectWrStartAddrSpec>;
#[doc = "Cadence QSPI Indirect Write Start Address"]
pub mod indirect_wr_start_addr;
#[doc = "indirect_wr_bytes (rw) register accessor: Cadence QSPI Indirect Write Bytes\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`indirect_wr_bytes::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`indirect_wr_bytes::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@indirect_wr_bytes`]
module"]
#[doc(alias = "indirect_wr_bytes")]
pub type IndirectWrBytes = crate::Reg<indirect_wr_bytes::IndirectWrBytesSpec>;
#[doc = "Cadence QSPI Indirect Write Bytes"]
pub mod indirect_wr_bytes;
#[doc = "cmd_ctrl (rw) register accessor: Cadence QSPI Command Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd_ctrl`]
module"]
#[doc(alias = "cmd_ctrl")]
pub type CmdCtrl = crate::Reg<cmd_ctrl::CmdCtrlSpec>;
#[doc = "Cadence QSPI Command Control"]
pub mod cmd_ctrl;
#[doc = "cmd_address (rw) register accessor: Cadence QSPI Command Address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd_address::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd_address::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd_address`]
module"]
#[doc(alias = "cmd_address")]
pub type CmdAddress = crate::Reg<cmd_address::CmdAddressSpec>;
#[doc = "Cadence QSPI Command Address"]
pub mod cmd_address;
#[doc = "cmd_read_at_lower (rw) register accessor: Cadence QSPI Command Read at Lower\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd_read_at_lower::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd_read_at_lower::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd_read_at_lower`]
module"]
#[doc(alias = "cmd_read_at_lower")]
pub type CmdReadAtLower = crate::Reg<cmd_read_at_lower::CmdReadAtLowerSpec>;
#[doc = "Cadence QSPI Command Read at Lower"]
pub mod cmd_read_at_lower;
#[doc = "cmd_read_at_upper (rw) register accessor: Cadence QSPI Command Read at Upper\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd_read_at_upper::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd_read_at_upper::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd_read_at_upper`]
module"]
#[doc(alias = "cmd_read_at_upper")]
pub type CmdReadAtUpper = crate::Reg<cmd_read_at_upper::CmdReadAtUpperSpec>;
#[doc = "Cadence QSPI Command Read at Upper"]
pub mod cmd_read_at_upper;
#[doc = "cmd_write_at_lower (rw) register accessor: Cadence QSPI Command Write at Lower\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd_write_at_lower::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd_write_at_lower::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd_write_at_lower`]
module"]
#[doc(alias = "cmd_write_at_lower")]
pub type CmdWriteAtLower = crate::Reg<cmd_write_at_lower::CmdWriteAtLowerSpec>;
#[doc = "Cadence QSPI Command Write at Lower"]
pub mod cmd_write_at_lower;
#[doc = "cmd_write_at_upper (rw) register accessor: Cadence QSPI Command Write at Upper\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd_write_at_upper::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd_write_at_upper::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd_write_at_upper`]
module"]
#[doc(alias = "cmd_write_at_upper")]
pub type CmdWriteAtUpper = crate::Reg<cmd_write_at_upper::CmdWriteAtUpperSpec>;
#[doc = "Cadence QSPI Command Write at Upper"]
pub mod cmd_write_at_upper;
#[doc = "polling_status (rw) register accessor: Cadence QSPI Polling Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`polling_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`polling_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@polling_status`]
module"]
#[doc(alias = "polling_status")]
pub type PollingStatus = crate::Reg<polling_status::PollingStatusSpec>;
#[doc = "Cadence QSPI Polling Status"]
pub mod polling_status;
#[doc = "ext_lower (rw) register accessor: Cadence QSPI Extension Lower\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ext_lower::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ext_lower::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ext_lower`]
module"]
#[doc(alias = "ext_lower")]
pub type ExtLower = crate::Reg<ext_lower::ExtLowerSpec>;
#[doc = "Cadence QSPI Extension Lower"]
pub mod ext_lower;
