#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    config: Config,
    ext_config: ExtConfig,
    packet_filter: PacketFilter,
    _reserved3: [u8; 0x04],
    hash_table: [HashTable; 8],
    _reserved4: [u8; 0x20],
    vlan_tag: VlanTag,
    vlan_tag_data: VlanTagData,
    vlan_hash_table: VlanHashTable,
    _reserved7: [u8; 0x04],
    vlan: Vlan,
    _reserved8: [u8; 0x0c],
    tx_queue_flow_ctrl: [TxQueueFlowCtrl; 8],
    rx_flow_ctrl: RxFlowCtrl,
    vff_queue_ctrl: VffQueueCtrl,
    tx_queue_priority: [TxQueuePriority; 2],
    rx_queue_ctrl: [RxQueueCtrl; 4],
    int: Int,
    _reserved14: [u8; 0x08],
    pmt: Pmt,
    _reserved15: [u8; 0x0c],
    lpi: Lpi,
    tic_counter_us: TicCounterUs,
    pcs: Pcs,
    phyif_ctrl_status: PhyifCtrlStatus,
    _reserved19: [u8; 0x18],
    debug: Debug,
    _reserved20: [u8; 0x04],
    hw_feat: HwFeat,
    _reserved21: [u8; 0xd4],
    mdio: Mdio,
    _reserved22: [u8; 0x04],
    gpio_status: GpioStatus,
    arp_addr: ArpAddr,
    _reserved24: [u8; 0xec],
    addr: [Addr; 128],
    _reserved25: [u8; 0x0200],
    l3l4: [L3l4; 8],
    _reserved26: [u8; 0xa0],
    timestamp: Timestamp,
    _reserved27: [u8; 0x4c],
    pps_ctrl: [PpsCtrl; 2],
    _reserved28: [u8; 0x08],
    pps: [Pps; 8],
    mtl: Mtl,
    _reserved30: [u8; 0x0180],
    dma: Dma,
}
impl RegisterBlock {
    #[doc = "0x00 - MAC Configuration"]
    #[inline(always)]
    pub const fn config(&self) -> &Config {
        &self.config
    }
    #[doc = "0x04 - MAC Extended Configuration"]
    #[inline(always)]
    pub const fn ext_config(&self) -> &ExtConfig {
        &self.ext_config
    }
    #[doc = "0x08 - MAC Packet Filter"]
    #[inline(always)]
    pub const fn packet_filter(&self) -> &PacketFilter {
        &self.packet_filter
    }
    #[doc = "0x10..0x30 - MAC Hash Table"]
    #[inline(always)]
    pub const fn hash_table(&self, n: usize) -> &HashTable {
        &self.hash_table[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x10..0x30 - MAC Hash Table"]
    #[inline(always)]
    pub fn hash_table_iter(&self) -> impl Iterator<Item = &HashTable> {
        self.hash_table.iter()
    }
    #[doc = "0x50 - MAC VLAN Tag"]
    #[inline(always)]
    pub const fn vlan_tag(&self) -> &VlanTag {
        &self.vlan_tag
    }
    #[doc = "0x54 - MAC VLAN Tag Data"]
    #[inline(always)]
    pub const fn vlan_tag_data(&self) -> &VlanTagData {
        &self.vlan_tag_data
    }
    #[doc = "0x58 - MAC VLAN Hash Table"]
    #[inline(always)]
    pub const fn vlan_hash_table(&self) -> &VlanHashTable {
        &self.vlan_hash_table
    }
    #[doc = "0x60 - MAC VLAN"]
    #[inline(always)]
    pub const fn vlan(&self) -> &Vlan {
        &self.vlan
    }
    #[doc = "0x70..0x90 - MAC TX Queue Flow Control"]
    #[inline(always)]
    pub const fn tx_queue_flow_ctrl(&self, n: usize) -> &TxQueueFlowCtrl {
        &self.tx_queue_flow_ctrl[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x70..0x90 - MAC TX Queue Flow Control"]
    #[inline(always)]
    pub fn tx_queue_flow_ctrl_iter(&self) -> impl Iterator<Item = &TxQueueFlowCtrl> {
        self.tx_queue_flow_ctrl.iter()
    }
    #[doc = "0x90 - MAC RX Flow Control"]
    #[inline(always)]
    pub const fn rx_flow_ctrl(&self) -> &RxFlowCtrl {
        &self.rx_flow_ctrl
    }
    #[doc = "0x94 - MAC EQoS VLAN Tag Filter Fail Packets Queuing"]
    #[inline(always)]
    pub const fn vff_queue_ctrl(&self) -> &VffQueueCtrl {
        &self.vff_queue_ctrl
    }
    #[doc = "0x98..0xa0 - MAC TX Queue Priority - tx_queue_priority0: queue 0-3, tx_queue_priority1: queue 4-7"]
    #[inline(always)]
    pub const fn tx_queue_priority(&self, n: usize) -> &TxQueuePriority {
        &self.tx_queue_priority[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x98..0xa0 - MAC TX Queue Priority - tx_queue_priority0: queue 0-3, tx_queue_priority1: queue 4-7"]
    #[inline(always)]
    pub fn tx_queue_priority_iter(&self) -> impl Iterator<Item = &TxQueuePriority> {
        self.tx_queue_priority.iter()
    }
    #[doc = "0xa0..0xb0 - MAC RX Queue Control"]
    #[inline(always)]
    pub const fn rx_queue_ctrl(&self, n: usize) -> &RxQueueCtrl {
        &self.rx_queue_ctrl[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xa0..0xb0 - MAC RX Queue Control"]
    #[inline(always)]
    pub fn rx_queue_ctrl_iter(&self) -> impl Iterator<Item = &RxQueueCtrl> {
        self.rx_queue_ctrl.iter()
    }
    #[doc = "0xb0..0xb8 - MAC Interrupt registers"]
    #[inline(always)]
    pub const fn int(&self) -> &Int {
        &self.int
    }
    #[doc = "0xc0 - PMT Control and Status"]
    #[inline(always)]
    pub const fn pmt(&self) -> &Pmt {
        &self.pmt
    }
    #[doc = "0xd0..0xdc - MAC LPI Energy Efficient Ethernet (EEE) registers"]
    #[inline(always)]
    pub const fn lpi(&self) -> &Lpi {
        &self.lpi
    }
    #[doc = "0xdc - MAC TIC Counter 1 microsecond"]
    #[inline(always)]
    pub const fn tic_counter_us(&self) -> &TicCounterUs {
        &self.tic_counter_us
    }
    #[doc = "0xe0..0xf8 - PCS (AN/TBI/SGMII/RGMII) registers"]
    #[inline(always)]
    pub const fn pcs(&self) -> &Pcs {
        &self.pcs
    }
    #[doc = "0xf8 - PHY Interface Control and Status"]
    #[inline(always)]
    pub const fn phyif_ctrl_status(&self) -> &PhyifCtrlStatus {
        &self.phyif_ctrl_status
    }
    #[doc = "0x114 - MAC Debug"]
    #[inline(always)]
    pub const fn debug(&self) -> &Debug {
        &self.debug
    }
    #[doc = "0x11c..0x12c - Hardware Feature registers"]
    #[inline(always)]
    pub const fn hw_feat(&self) -> &HwFeat {
        &self.hw_feat
    }
    #[doc = "0x200..0x208 - MDIO registers"]
    #[inline(always)]
    pub const fn mdio(&self) -> &Mdio {
        &self.mdio
    }
    #[doc = "0x20c - MAC GPIO Status"]
    #[inline(always)]
    pub const fn gpio_status(&self) -> &GpioStatus {
        &self.gpio_status
    }
    #[doc = "0x210 - MAC ARP Address"]
    #[inline(always)]
    pub const fn arp_addr(&self) -> &ArpAddr {
        &self.arp_addr
    }
    #[doc = "0x300..0x700 - Hardware Address registers"]
    #[inline(always)]
    pub const fn addr(&self, n: usize) -> &Addr {
        &self.addr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x300..0x700 - Hardware Address registers"]
    #[inline(always)]
    pub fn addr_iter(&self) -> impl Iterator<Item = &Addr> {
        self.addr.iter()
    }
    #[doc = "0x900..0xa80 - MAC L3/L4 Filter registers"]
    #[inline(always)]
    pub const fn l3l4(&self, n: usize) -> &L3l4 {
        &self.l3l4[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x900..0xa80 - MAC L3/L4 Filter registers"]
    #[inline(always)]
    pub fn l3l4_iter(&self) -> impl Iterator<Item = &L3l4> {
        self.l3l4.iter()
    }
    #[doc = "0xb20 - MAC Timestamp"]
    #[inline(always)]
    pub const fn timestamp(&self) -> &Timestamp {
        &self.timestamp
    }
    #[doc = "0xb70..0xb78 - MTL PPS Control and Status - pps_ctrl0: channel 0-3 control, pps_ctrl1: channel 4-7 control"]
    #[inline(always)]
    pub const fn pps_ctrl(&self, n: usize) -> &PpsCtrl {
        &self.pps_ctrl[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xb70..0xb78 - MTL PPS Control and Status - pps_ctrl0: channel 0-3 control, pps_ctrl1: channel 4-7 control"]
    #[inline(always)]
    pub fn pps_ctrl_iter(&self) -> impl Iterator<Item = &PpsCtrl> {
        self.pps_ctrl.iter()
    }
    #[doc = "0xb80..0xc00 - PPS registers"]
    #[inline(always)]
    pub const fn pps(&self, n: usize) -> &Pps {
        &self.pps[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0xb80..0xc00 - PPS registers"]
    #[inline(always)]
    pub fn pps_iter(&self) -> impl Iterator<Item = &Pps> {
        self.pps.iter()
    }
    #[doc = "0xc00..0xf00 - MTL registers"]
    #[inline(always)]
    pub const fn mtl(&self) -> &Mtl {
        &self.mtl
    }
    #[doc = "0x1080..0x108c - DMA registers"]
    #[inline(always)]
    pub const fn dma(&self) -> &Dma {
        &self.dma
    }
}
#[doc = "config (rw) register accessor: MAC Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config`]
module"]
#[doc(alias = "config")]
pub type Config = crate::Reg<config::ConfigSpec>;
#[doc = "MAC Configuration"]
pub mod config;
#[doc = "ext_config (rw) register accessor: MAC Extended Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ext_config::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ext_config::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ext_config`]
module"]
#[doc(alias = "ext_config")]
pub type ExtConfig = crate::Reg<ext_config::ExtConfigSpec>;
#[doc = "MAC Extended Configuration"]
pub mod ext_config;
#[doc = "packet_filter (rw) register accessor: MAC Packet Filter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`packet_filter::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`packet_filter::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@packet_filter`]
module"]
#[doc(alias = "packet_filter")]
pub type PacketFilter = crate::Reg<packet_filter::PacketFilterSpec>;
#[doc = "MAC Packet Filter"]
pub mod packet_filter;
#[doc = "hash_table (rw) register accessor: MAC Hash Table\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hash_table::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hash_table::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hash_table`]
module"]
#[doc(alias = "hash_table")]
pub type HashTable = crate::Reg<hash_table::HashTableSpec>;
#[doc = "MAC Hash Table"]
pub mod hash_table;
#[doc = "vlan_tag (rw) register accessor: MAC VLAN Tag\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vlan_tag::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vlan_tag::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vlan_tag`]
module"]
#[doc(alias = "vlan_tag")]
pub type VlanTag = crate::Reg<vlan_tag::VlanTagSpec>;
#[doc = "MAC VLAN Tag"]
pub mod vlan_tag;
#[doc = "vlan_tag_data (rw) register accessor: MAC VLAN Tag Data\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vlan_tag_data::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vlan_tag_data::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vlan_tag_data`]
module"]
#[doc(alias = "vlan_tag_data")]
pub type VlanTagData = crate::Reg<vlan_tag_data::VlanTagDataSpec>;
#[doc = "MAC VLAN Tag Data"]
pub mod vlan_tag_data;
#[doc = "vlan_hash_table (rw) register accessor: MAC VLAN Hash Table\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vlan_hash_table::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vlan_hash_table::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vlan_hash_table`]
module"]
#[doc(alias = "vlan_hash_table")]
pub type VlanHashTable = crate::Reg<vlan_hash_table::VlanHashTableSpec>;
#[doc = "MAC VLAN Hash Table"]
pub mod vlan_hash_table;
#[doc = "vlan (rw) register accessor: MAC VLAN\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vlan::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vlan::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vlan`]
module"]
#[doc(alias = "vlan")]
pub type Vlan = crate::Reg<vlan::VlanSpec>;
#[doc = "MAC VLAN"]
pub mod vlan;
#[doc = "tx_queue_flow_ctrl (rw) register accessor: MAC TX Queue Flow Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_queue_flow_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_queue_flow_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_queue_flow_ctrl`]
module"]
#[doc(alias = "tx_queue_flow_ctrl")]
pub type TxQueueFlowCtrl = crate::Reg<tx_queue_flow_ctrl::TxQueueFlowCtrlSpec>;
#[doc = "MAC TX Queue Flow Control"]
pub mod tx_queue_flow_ctrl;
#[doc = "rx_flow_ctrl (rw) register accessor: MAC RX Flow Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_flow_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rx_flow_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_flow_ctrl`]
module"]
#[doc(alias = "rx_flow_ctrl")]
pub type RxFlowCtrl = crate::Reg<rx_flow_ctrl::RxFlowCtrlSpec>;
#[doc = "MAC RX Flow Control"]
pub mod rx_flow_ctrl;
#[doc = "vff_queue_ctrl (rw) register accessor: MAC EQoS VLAN Tag Filter Fail Packets Queuing\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vff_queue_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vff_queue_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vff_queue_ctrl`]
module"]
#[doc(alias = "vff_queue_ctrl")]
pub type VffQueueCtrl = crate::Reg<vff_queue_ctrl::VffQueueCtrlSpec>;
#[doc = "MAC EQoS VLAN Tag Filter Fail Packets Queuing"]
pub mod vff_queue_ctrl;
#[doc = "tx_queue_priority (rw) register accessor: MAC TX Queue Priority - tx_queue_priority0: queue 0-3, tx_queue_priority1: queue 4-7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_queue_priority::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_queue_priority::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tx_queue_priority`]
module"]
#[doc(alias = "tx_queue_priority")]
pub type TxQueuePriority = crate::Reg<tx_queue_priority::TxQueuePrioritySpec>;
#[doc = "MAC TX Queue Priority - tx_queue_priority0: queue 0-3, tx_queue_priority1: queue 4-7"]
pub mod tx_queue_priority;
#[doc = "rx_queue_ctrl (rw) register accessor: MAC RX Queue Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_queue_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rx_queue_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rx_queue_ctrl`]
module"]
#[doc(alias = "rx_queue_ctrl")]
pub type RxQueueCtrl = crate::Reg<rx_queue_ctrl::RxQueueCtrlSpec>;
#[doc = "MAC RX Queue Control"]
pub mod rx_queue_ctrl;
#[doc = "MAC Interrupt registers"]
pub use self::int::Int;
#[doc = r"Cluster"]
#[doc = "MAC Interrupt registers"]
pub mod int;
#[doc = "pmt (rw) register accessor: PMT Control and Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmt::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmt::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmt`]
module"]
#[doc(alias = "pmt")]
pub type Pmt = crate::Reg<pmt::PmtSpec>;
#[doc = "PMT Control and Status"]
pub mod pmt;
#[doc = "MAC LPI Energy Efficient Ethernet (EEE) registers"]
pub use self::lpi::Lpi;
#[doc = r"Cluster"]
#[doc = "MAC LPI Energy Efficient Ethernet (EEE) registers"]
pub mod lpi;
#[doc = "tic_counter_us (rw) register accessor: MAC TIC Counter 1 microsecond\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tic_counter_us::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tic_counter_us::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tic_counter_us`]
module"]
#[doc(alias = "tic_counter_us")]
pub type TicCounterUs = crate::Reg<tic_counter_us::TicCounterUsSpec>;
#[doc = "MAC TIC Counter 1 microsecond"]
pub mod tic_counter_us;
#[doc = "PCS (AN/TBI/SGMII/RGMII) registers"]
pub use self::pcs::Pcs;
#[doc = r"Cluster"]
#[doc = "PCS (AN/TBI/SGMII/RGMII) registers"]
pub mod pcs;
#[doc = "phyif_ctrl_status (rw) register accessor: PHY Interface Control and Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phyif_ctrl_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`phyif_ctrl_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@phyif_ctrl_status`]
module"]
#[doc(alias = "phyif_ctrl_status")]
pub type PhyifCtrlStatus = crate::Reg<phyif_ctrl_status::PhyifCtrlStatusSpec>;
#[doc = "PHY Interface Control and Status"]
pub mod phyif_ctrl_status;
#[doc = "debug (rw) register accessor: MAC Debug\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`debug::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`debug::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@debug`]
module"]
#[doc(alias = "debug")]
pub type Debug = crate::Reg<debug::DebugSpec>;
#[doc = "MAC Debug"]
pub mod debug;
#[doc = "Hardware Feature registers"]
pub use self::hw_feat::HwFeat;
#[doc = r"Cluster"]
#[doc = "Hardware Feature registers"]
pub mod hw_feat;
#[doc = "MDIO registers"]
pub use self::mdio::Mdio;
#[doc = r"Cluster"]
#[doc = "MDIO registers"]
pub mod mdio;
#[doc = "gpio_status (rw) register accessor: MAC GPIO Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_status::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_status::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gpio_status`]
module"]
#[doc(alias = "gpio_status")]
pub type GpioStatus = crate::Reg<gpio_status::GpioStatusSpec>;
#[doc = "MAC GPIO Status"]
pub mod gpio_status;
#[doc = "arp_addr (rw) register accessor: MAC ARP Address\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`arp_addr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`arp_addr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@arp_addr`]
module"]
#[doc(alias = "arp_addr")]
pub type ArpAddr = crate::Reg<arp_addr::ArpAddrSpec>;
#[doc = "MAC ARP Address"]
pub mod arp_addr;
#[doc = "Hardware Address registers"]
pub use self::addr::Addr;
#[doc = r"Cluster"]
#[doc = "Hardware Address registers"]
pub mod addr;
#[doc = "MAC L3/L4 Filter registers"]
pub use self::l3l4::L3l4;
#[doc = r"Cluster"]
#[doc = "MAC L3/L4 Filter registers"]
pub mod l3l4;
#[doc = "timestamp (rw) register accessor: MAC Timestamp\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timestamp::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timestamp::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timestamp`]
module"]
#[doc(alias = "timestamp")]
pub type Timestamp = crate::Reg<timestamp::TimestampSpec>;
#[doc = "MAC Timestamp"]
pub mod timestamp;
#[doc = "pps_ctrl (rw) register accessor: MTL PPS Control and Status - pps_ctrl0: channel 0-3 control, pps_ctrl1: channel 4-7 control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pps_ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pps_ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pps_ctrl`]
module"]
#[doc(alias = "pps_ctrl")]
pub type PpsCtrl = crate::Reg<pps_ctrl::PpsCtrlSpec>;
#[doc = "MTL PPS Control and Status - pps_ctrl0: channel 0-3 control, pps_ctrl1: channel 4-7 control"]
pub mod pps_ctrl;
#[doc = "PPS registers"]
pub use self::pps::Pps;
#[doc = r"Cluster"]
#[doc = "PPS registers"]
pub mod pps;
#[doc = "MTL registers"]
pub use self::mtl::Mtl;
#[doc = r"Cluster"]
#[doc = "MTL registers"]
pub mod mtl;
#[doc = "DMA registers"]
pub use self::dma::Dma;
#[doc = r"Cluster"]
#[doc = "DMA registers"]
pub mod dma;
