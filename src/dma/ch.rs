#[repr(C)]
#[doc = "DesignWare DMAC Channel registers"]
#[doc(alias = "ch")]
pub struct Ch {
    sar: Sar,
    dar: Dar,
    block_ts: BlockTs,
    ctl: Ctl,
    _reserved_4_cfg: [u8; 0x08],
    llp: Llp,
    status: Status,
    swhs: [Swhs; 2],
    blk_tr_resume_req: BlkTrResumeReq,
    axi_id: AxiId,
    axi_qos: AxiQos,
    stat: [Stat; 2],
    statar: [Statar; 2],
    int: Int,
    _reserved14: [u8; 0x58],
    _reserved_channel: _ReservedChannel,
}
impl Ch {
    #[doc = "0x00..0x08 - DMAC Channel Source address of DMA transfer."]
    #[inline(always)]
    pub const fn sar(&self) -> &Sar {
        &self.sar
    }
    #[doc = "0x08..0x10 - DMAC Channel Destination address of DMA transfer."]
    #[inline(always)]
    pub const fn dar(&self) -> &Dar {
        &self.dar
    }
    #[doc = "0x10..0x18 - DMAC Block transfer size."]
    #[inline(always)]
    pub const fn block_ts(&self) -> &BlockTs {
        &self.block_ts
    }
    #[doc = "0x18..0x20 - DMAC Channel Control."]
    #[inline(always)]
    pub const fn ctl(&self) -> &Ctl {
        &self.ctl
    }
    #[doc = "0x20..0x28 - DMAC Channel Configuration 2 register (only exists for DMAX_NUM_CHANNELS > 8)."]
    #[inline(always)]
    pub const fn cfg2(&self) -> &Cfg2 {
        unsafe { &*(self as *const Self).cast::<u8>().add(32).cast() }
    }
    #[doc = "0x20..0x28 - DMAC Channel Configuration register (only exists for DMAX_NUM_CHANNELS &lt;= 8)."]
    #[inline(always)]
    pub const fn cfg(&self) -> &Cfg {
        unsafe { &*(self as *const Self).cast::<u8>().add(32).cast() }
    }
    #[doc = "0x28..0x30 - Linked list pointer register."]
    #[inline(always)]
    pub const fn llp(&self) -> &Llp {
        &self.llp
    }
    #[doc = "0x30..0x38 - DMAC Channel Status register."]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x38..0x48 - Software Handshake register."]
    #[inline(always)]
    pub const fn swhs(&self, n: usize) -> &Swhs {
        &self.swhs[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x38..0x48 - Software Handshake register."]
    #[inline(always)]
    pub fn swhs_iter(&self) -> impl Iterator<Item = &Swhs> {
        self.swhs.iter()
    }
    #[doc = "0x38..0x40 - Software Handshake register."]
    #[inline(always)]
    pub const fn swhs_src(&self) -> &Swhs {
        self.swhs(0)
    }
    #[doc = "0x40..0x48 - Software Handshake register."]
    #[inline(always)]
    pub const fn swhs_dst(&self) -> &Swhs {
        self.swhs(1)
    }
    #[doc = "0x48..0x50 - Block Transfer Resume Request register."]
    #[inline(always)]
    pub const fn blk_tr_resume_req(&self) -> &BlkTrResumeReq {
        &self.blk_tr_resume_req
    }
    #[doc = "0x50..0x58 - Channel AXI ID register."]
    #[inline(always)]
    pub const fn axi_id(&self) -> &AxiId {
        &self.axi_id
    }
    #[doc = "0x58..0x60 - Channel AXI QOS register - **NOTE** this register is only allowed to be modified when the channel is disabled."]
    #[inline(always)]
    pub const fn axi_qos(&self) -> &AxiQos {
        &self.axi_qos
    }
    #[doc = "0x60..0x70 - Channel Status register."]
    #[inline(always)]
    pub const fn stat(&self, n: usize) -> &Stat {
        &self.stat[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x60..0x70 - Channel Status register."]
    #[inline(always)]
    pub fn stat_iter(&self) -> impl Iterator<Item = &Stat> {
        self.stat.iter()
    }
    #[doc = "0x60..0x68 - Channel Status register."]
    #[inline(always)]
    pub const fn stat_src(&self) -> &Stat {
        self.stat(0)
    }
    #[doc = "0x68..0x70 - Channel Status register."]
    #[inline(always)]
    pub const fn stat_dst(&self) -> &Stat {
        self.stat(1)
    }
    #[doc = "0x70..0x80 - Channel Status Fetch Address register."]
    #[inline(always)]
    pub const fn statar(&self, n: usize) -> &Statar {
        &self.statar[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x70..0x80 - Channel Status Fetch Address register."]
    #[inline(always)]
    pub fn statar_iter(&self) -> impl Iterator<Item = &Statar> {
        self.statar.iter()
    }
    #[doc = "0x70..0x78 - Channel Status Fetch Address register."]
    #[inline(always)]
    pub const fn statar_src(&self) -> &Statar {
        self.statar(0)
    }
    #[doc = "0x78..0x80 - Channel Status Fetch Address register."]
    #[inline(always)]
    pub const fn statar_dst(&self) -> &Statar {
        self.statar(1)
    }
    #[doc = "0x80..0xa0 - Channel Interrupt registers."]
    #[inline(always)]
    pub const fn int(&self) -> &Int {
        &self.int
    }
    #[doc = "0xf8..0x100 - DMAC Reserved register."]
    #[inline(always)]
    pub const fn _reserved_channel(&self) -> &_ReservedChannel {
        &self._reserved_channel
    }
}
#[doc = "sar (rw) register accessor: DMAC Channel Source address of DMA transfer.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sar`]
module"]
#[doc(alias = "sar")]
pub type Sar = crate::Reg<sar::SarSpec>;
#[doc = "DMAC Channel Source address of DMA transfer."]
pub mod sar;
#[doc = "dar (rw) register accessor: DMAC Channel Destination address of DMA transfer.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dar::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dar::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dar`]
module"]
#[doc(alias = "dar")]
pub type Dar = crate::Reg<dar::DarSpec>;
#[doc = "DMAC Channel Destination address of DMA transfer."]
pub mod dar;
#[doc = "block_ts (rw) register accessor: DMAC Block transfer size.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`block_ts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`block_ts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@block_ts`]
module"]
#[doc(alias = "block_ts")]
pub type BlockTs = crate::Reg<block_ts::BlockTsSpec>;
#[doc = "DMAC Block transfer size."]
pub mod block_ts;
#[doc = "ctl (rw) register accessor: DMAC Channel Control.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctl`]
module"]
#[doc(alias = "ctl")]
pub type Ctl = crate::Reg<ctl::CtlSpec>;
#[doc = "DMAC Channel Control."]
pub mod ctl;
#[doc = "cfg (rw) register accessor: DMAC Channel Configuration register (only exists for DMAX_NUM_CHANNELS &lt;= 8).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
#[doc(alias = "cfg")]
pub type Cfg = crate::Reg<cfg::CfgSpec>;
#[doc = "DMAC Channel Configuration register (only exists for DMAX_NUM_CHANNELS &lt;= 8)."]
pub mod cfg;
#[doc = "cfg2 (rw) register accessor: DMAC Channel Configuration 2 register (only exists for DMAX_NUM_CHANNELS > 8).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfg2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfg2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg2`]
module"]
#[doc(alias = "cfg2")]
pub type Cfg2 = crate::Reg<cfg2::Cfg2Spec>;
#[doc = "DMAC Channel Configuration 2 register (only exists for DMAX_NUM_CHANNELS > 8)."]
pub mod cfg2;
#[doc = "llp (rw) register accessor: Linked list pointer register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`llp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`llp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@llp`]
module"]
#[doc(alias = "llp")]
pub type Llp = crate::Reg<llp::LlpSpec>;
#[doc = "Linked list pointer register."]
pub mod llp;
#[doc = "status (r) register accessor: DMAC Channel Status register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`]
module"]
#[doc(alias = "status")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "DMAC Channel Status register."]
pub mod status;
#[doc = "swhs (rw) register accessor: Software Handshake register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swhs::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swhs::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swhs`]
module"]
#[doc(alias = "swhs")]
pub type Swhs = crate::Reg<swhs::SwhsSpec>;
#[doc = "Software Handshake register."]
pub mod swhs;
#[doc = "blk_tr_resume_req (w) register accessor: Block Transfer Resume Request register.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blk_tr_resume_req::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@blk_tr_resume_req`]
module"]
#[doc(alias = "blk_tr_resume_req")]
pub type BlkTrResumeReq = crate::Reg<blk_tr_resume_req::BlkTrResumeReqSpec>;
#[doc = "Block Transfer Resume Request register."]
pub mod blk_tr_resume_req;
#[doc = "axi_id (rw) register accessor: Channel AXI ID register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`axi_id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`axi_id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@axi_id`]
module"]
#[doc(alias = "axi_id")]
pub type AxiId = crate::Reg<axi_id::AxiIdSpec>;
#[doc = "Channel AXI ID register."]
pub mod axi_id;
#[doc = "axi_qos (rw) register accessor: Channel AXI QOS register - **NOTE** this register is only allowed to be modified when the channel is disabled.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`axi_qos::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`axi_qos::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@axi_qos`]
module"]
#[doc(alias = "axi_qos")]
pub type AxiQos = crate::Reg<axi_qos::AxiQosSpec>;
#[doc = "Channel AXI QOS register - **NOTE** this register is only allowed to be modified when the channel is disabled."]
pub mod axi_qos;
#[doc = "stat (r) register accessor: Channel Status register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stat::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@stat`]
module"]
#[doc(alias = "stat")]
pub type Stat = crate::Reg<stat::StatSpec>;
#[doc = "Channel Status register."]
pub mod stat;
#[doc = "statar (r) register accessor: Channel Status Fetch Address register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`statar::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@statar`]
module"]
#[doc(alias = "statar")]
pub type Statar = crate::Reg<statar::StatarSpec>;
#[doc = "Channel Status Fetch Address register."]
pub mod statar;
#[doc = "Channel Interrupt registers."]
pub use self::int::Int;
#[doc = r"Cluster"]
#[doc = "Channel Interrupt registers."]
pub mod int;
#[doc = "_reserved_channel (rw) register accessor: DMAC Reserved register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_reserved_channel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`_reserved_channel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_reserved_channel`]
module"]
#[doc(alias = "_reserved_channel")]
pub type _ReservedChannel = crate::Reg<_reserved_channel::_ReservedChannelSpec>;
#[doc = "DMAC Reserved register."]
pub mod _reserved_channel;
