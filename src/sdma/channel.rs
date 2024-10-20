#[repr(C)]
#[doc = "DMAC Channel registers"]
#[doc(alias = "channel")]
pub struct Channel {
    src_addr: SrcAddr,
    dst_addr: DstAddr,
    lli: Lli,
    control: Control,
    configuration: Configuration,
    _reserved5: [u8; 0x08],
    _reserved_channel: _ReservedChannel,
}
impl Channel {
    #[doc = "0x00 - DMAC Source Address register - contain the current source address, byte-aligned, of the data to be transferred. Software programs each register directly before the appropriate channel is enabled."]
    #[inline(always)]
    pub const fn src_addr(&self) -> &SrcAddr {
        &self.src_addr
    }
    #[doc = "0x04 - DMA Destination Address register - contain the current destination address, byte-aligned, of the data to be transferred. Software programs each register directly before the channel is enabled. When the DMA channel is enabled, the register is updated as the destination address is incremented and by following the linked list when a complete packet of data has been transferred. Reading the register when the channel is active does not provide useful information. This is because by the time the software has processed the value read, the channel might have progressed. It is intended to be read-only when a channel has stopped. In this case, it shows the destination address of the last item read."]
    #[inline(always)]
    pub const fn dst_addr(&self) -> &DstAddr {
        &self.dst_addr
    }
    #[doc = "0x08 - DMA Linked List Item register"]
    #[inline(always)]
    pub const fn lli(&self) -> &Lli {
        &self.lli
    }
    #[doc = "0x0c - DMA Channel Control register"]
    #[inline(always)]
    pub const fn control(&self) -> &Control {
        &self.control
    }
    #[doc = "0x10 - DMA Channel Configuration register"]
    #[inline(always)]
    pub const fn configuration(&self) -> &Configuration {
        &self.configuration
    }
    #[doc = "0x1c - Reserved Channel - not meant for actual use."]
    #[inline(always)]
    pub const fn _reserved_channel(&self) -> &_ReservedChannel {
        &self._reserved_channel
    }
}
#[doc = "src_addr (rw) register accessor: DMAC Source Address register - contain the current source address, byte-aligned, of the data to be transferred. Software programs each register directly before the appropriate channel is enabled.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`src_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`src_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@src_addr`]
module"]
#[doc(alias = "src_addr")]
pub type SrcAddr = crate::Reg<src_addr::SrcAddrSpec>;
#[doc = "DMAC Source Address register - contain the current source address, byte-aligned, of the data to be transferred. Software programs each register directly before the appropriate channel is enabled."]
pub mod src_addr;
#[doc = "dst_addr (rw) register accessor: DMA Destination Address register - contain the current destination address, byte-aligned, of the data to be transferred. Software programs each register directly before the channel is enabled. When the DMA channel is enabled, the register is updated as the destination address is incremented and by following the linked list when a complete packet of data has been transferred. Reading the register when the channel is active does not provide useful information. This is because by the time the software has processed the value read, the channel might have progressed. It is intended to be read-only when a channel has stopped. In this case, it shows the destination address of the last item read.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dst_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dst_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dst_addr`]
module"]
#[doc(alias = "dst_addr")]
pub type DstAddr = crate::Reg<dst_addr::DstAddrSpec>;
#[doc = "DMA Destination Address register - contain the current destination address, byte-aligned, of the data to be transferred. Software programs each register directly before the channel is enabled. When the DMA channel is enabled, the register is updated as the destination address is incremented and by following the linked list when a complete packet of data has been transferred. Reading the register when the channel is active does not provide useful information. This is because by the time the software has processed the value read, the channel might have progressed. It is intended to be read-only when a channel has stopped. In this case, it shows the destination address of the last item read."]
pub mod dst_addr;
#[doc = "lli (rw) register accessor: DMA Linked List Item register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lli::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lli::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lli`]
module"]
#[doc(alias = "lli")]
pub type Lli = crate::Reg<lli::LliSpec>;
#[doc = "DMA Linked List Item register"]
pub mod lli;
#[doc = "control (rw) register accessor: DMA Channel Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`control::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`control::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@control`]
module"]
#[doc(alias = "control")]
pub type Control = crate::Reg<control::ControlSpec>;
#[doc = "DMA Channel Control register"]
pub mod control;
#[doc = "configuration (rw) register accessor: DMA Channel Configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`configuration::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`configuration::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@configuration`]
module"]
#[doc(alias = "configuration")]
pub type Configuration = crate::Reg<configuration::ConfigurationSpec>;
#[doc = "DMA Channel Configuration register"]
pub mod configuration;
#[doc = "_reserved_channel (rw) register accessor: Reserved Channel - not meant for actual use.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`_reserved_channel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`_reserved_channel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@_reserved_channel`]
module"]
#[doc(alias = "_reserved_channel")]
pub type _ReservedChannel = crate::Reg<_reserved_channel::_ReservedChannelSpec>;
#[doc = "Reserved Channel - not meant for actual use."]
pub mod _reserved_channel;
