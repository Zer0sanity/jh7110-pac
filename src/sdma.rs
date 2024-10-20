#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    int: Int,
    raw_int: RawInt,
    enbld_chns: EnbldChns,
    soft: Soft,
    configuration: Configuration,
    sync: Sync,
    _reserved6: [u8; 0xc8],
    channel: [Channel; 8],
    _reserved7: [u8; 0x0300],
    test: Test,
    _reserved8: [u8; 0x0ad0],
    periph_id: PeriphId,
    pcell_id: PcellId,
}
impl RegisterBlock {
    #[doc = "0x00..0x14 - DMAC Interrupt registers"]
    #[inline(always)]
    pub const fn int(&self) -> &Int {
        &self.int
    }
    #[doc = "0x14..0x1c - DMAC Raw Interrupt registers"]
    #[inline(always)]
    pub const fn raw_int(&self) -> &RawInt {
        &self.raw_int
    }
    #[doc = "0x1c - DMA Enabled Channels register - indicates the DMA channels that are enabled, as indicated by the Enable bit in the DMACCxConfiguration Register. A HIGH bit indicates that a DMA channel is enabled. A bit is cleared on completion of the DMA transfer."]
    #[inline(always)]
    pub const fn enbld_chns(&self) -> &EnbldChns {
        &self.enbld_chns
    }
    #[doc = "0x20..0x30 - DMAC Software registers"]
    #[inline(always)]
    pub const fn soft(&self) -> &Soft {
        &self.soft
    }
    #[doc = "0x30 - DMA Configuration register - configures the operation of the DMAC. You can alter the endianness of the individual AHB master interfaces by writing to the M1 and M2 bits of this register. The M1 bit enables you to alter the endianness of AHB master interface 1. The M2 bit enables you to alter the endianness of AHB master interface 2. The AHB master interfaces are set to little-endian mode on reset."]
    #[inline(always)]
    pub const fn configuration(&self) -> &Configuration {
        &self.configuration
    }
    #[doc = "0x34 - DMA Synchronization register - enables or disables synchronization logic for the DMA request signals. A bit set to 0 enables the synchronization logic for a particular group of DMA requests. A bit set to 1 disables the synchronization logic for a particular group of DMA requests. This register is reset to 0, and synchronization logic enabled."]
    #[inline(always)]
    pub const fn sync(&self) -> &Sync {
        &self.sync
    }
    #[doc = "0x100..0x200 - DMAC Channel registers"]
    #[inline(always)]
    pub const fn channel(&self, n: usize) -> &Channel {
        &self.channel[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x100..0x200 - DMAC Channel registers"]
    #[inline(always)]
    pub fn channel_iter(&self) -> impl Iterator<Item = &Channel> {
        self.channel.iter()
    }
    #[doc = "0x500..0x510 - DMAC Test registers - controls and reads registers used in peripheral integration tests."]
    #[inline(always)]
    pub const fn test(&self) -> &Test {
        &self.test
    }
    #[doc = "0xfe0..0xff0 - DMAC Peripheral ID registers - You can treat the registers conceptually as a 32-bit register. These read-only registers provide the following peripheral options :: PartNumber\\[11:0\\]
This identifies the peripheral. The three digit product code 0x080 is used. :: Designer ID\\[19:12\\]
This is the identification of the designer. ARM Limited is 0x41, (ASCII A). :: Revision\\[23:20\\]
This is the revision number of the peripheral. The revision number starts from 0. :: Configuration\\[31:24\\]
This is the configuration option of the peripheral."]
    #[inline(always)]
    pub const fn periph_id(&self) -> &PeriphId {
        &self.periph_id
    }
    #[doc = "0xff0..0x1000 - DMAC PrimeCell ID registers - You can treat the registers conceptually as a 32-bit register. The register is a standard cross-peripheral identification system. The DMACPCellID Register is set to 0xB105F00D."]
    #[inline(always)]
    pub const fn pcell_id(&self) -> &PcellId {
        &self.pcell_id
    }
}
#[doc = "DMAC Interrupt registers"]
pub use self::int::Int;
#[doc = r"Cluster"]
#[doc = "DMAC Interrupt registers"]
pub mod int;
#[doc = "DMAC Raw Interrupt registers"]
pub use self::raw_int::RawInt;
#[doc = r"Cluster"]
#[doc = "DMAC Raw Interrupt registers"]
pub mod raw_int;
#[doc = "enbld_chns (r) register accessor: DMA Enabled Channels register - indicates the DMA channels that are enabled, as indicated by the Enable bit in the DMACCxConfiguration Register. A HIGH bit indicates that a DMA channel is enabled. A bit is cleared on completion of the DMA transfer.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`enbld_chns::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enbld_chns`]
module"]
#[doc(alias = "enbld_chns")]
pub type EnbldChns = crate::Reg<enbld_chns::EnbldChnsSpec>;
#[doc = "DMA Enabled Channels register - indicates the DMA channels that are enabled, as indicated by the Enable bit in the DMACCxConfiguration Register. A HIGH bit indicates that a DMA channel is enabled. A bit is cleared on completion of the DMA transfer."]
pub mod enbld_chns;
#[doc = "DMAC Software registers"]
pub use self::soft::Soft;
#[doc = r"Cluster"]
#[doc = "DMAC Software registers"]
pub mod soft;
#[doc = "configuration (rw) register accessor: DMA Configuration register - configures the operation of the DMAC. You can alter the endianness of the individual AHB master interfaces by writing to the M1 and M2 bits of this register. The M1 bit enables you to alter the endianness of AHB master interface 1. The M2 bit enables you to alter the endianness of AHB master interface 2. The AHB master interfaces are set to little-endian mode on reset.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`configuration::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`configuration::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@configuration`]
module"]
#[doc(alias = "configuration")]
pub type Configuration = crate::Reg<configuration::ConfigurationSpec>;
#[doc = "DMA Configuration register - configures the operation of the DMAC. You can alter the endianness of the individual AHB master interfaces by writing to the M1 and M2 bits of this register. The M1 bit enables you to alter the endianness of AHB master interface 1. The M2 bit enables you to alter the endianness of AHB master interface 2. The AHB master interfaces are set to little-endian mode on reset."]
pub mod configuration;
#[doc = "sync (rw) register accessor: DMA Synchronization register - enables or disables synchronization logic for the DMA request signals. A bit set to 0 enables the synchronization logic for a particular group of DMA requests. A bit set to 1 disables the synchronization logic for a particular group of DMA requests. This register is reset to 0, and synchronization logic enabled.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sync::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sync::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sync`]
module"]
#[doc(alias = "sync")]
pub type Sync = crate::Reg<sync::SyncSpec>;
#[doc = "DMA Synchronization register - enables or disables synchronization logic for the DMA request signals. A bit set to 0 enables the synchronization logic for a particular group of DMA requests. A bit set to 1 disables the synchronization logic for a particular group of DMA requests. This register is reset to 0, and synchronization logic enabled."]
pub mod sync;
#[doc = "DMAC Channel registers"]
pub use self::channel::Channel;
#[doc = r"Cluster"]
#[doc = "DMAC Channel registers"]
pub mod channel;
#[doc = "DMAC Test registers - controls and reads registers used in peripheral integration tests."]
pub use self::test::Test;
#[doc = r"Cluster"]
#[doc = "DMAC Test registers - controls and reads registers used in peripheral integration tests."]
pub mod test;
#[doc = "DMAC Peripheral ID registers - You can treat the registers conceptually as a 32-bit register. These read-only registers provide the following peripheral options :: PartNumber\\[11:0\\]
This identifies the peripheral. The three digit product code 0x080 is used. :: Designer ID\\[19:12\\]
This is the identification of the designer. ARM Limited is 0x41, (ASCII A). :: Revision\\[23:20\\]
This is the revision number of the peripheral. The revision number starts from 0. :: Configuration\\[31:24\\]
This is the configuration option of the peripheral."]
pub use self::periph_id::PeriphId;
#[doc = r"Cluster"]
#[doc = "DMAC Peripheral ID registers - You can treat the registers conceptually as a 32-bit register. These read-only registers provide the following peripheral options :: PartNumber\\[11:0\\]
This identifies the peripheral. The three digit product code 0x080 is used. :: Designer ID\\[19:12\\]
This is the identification of the designer. ARM Limited is 0x41, (ASCII A). :: Revision\\[23:20\\]
This is the revision number of the peripheral. The revision number starts from 0. :: Configuration\\[31:24\\]
This is the configuration option of the peripheral."]
pub mod periph_id;
#[doc = "DMAC PrimeCell ID registers - You can treat the registers conceptually as a 32-bit register. The register is a standard cross-peripheral identification system. The DMACPCellID Register is set to 0xB105F00D."]
pub use self::pcell_id::PcellId;
#[doc = r"Cluster"]
#[doc = "DMAC PrimeCell ID registers - You can treat the registers conceptually as a 32-bit register. The register is a standard cross-peripheral identification system. The DMACPCellID Register is set to 0xB105F00D."]
pub mod pcell_id;
