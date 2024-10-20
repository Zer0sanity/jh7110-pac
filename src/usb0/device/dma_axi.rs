#[repr(C)]
#[doc = "Device DMA registers."]
#[doc(alias = "dma_axi")]
pub struct DmaAxi {
    ctrl: Ctrl,
    id: Id,
    cap: Cap,
    ctrl0: Ctrl0,
    ctrl1: Ctrl1,
}
impl DmaAxi {
    #[doc = "0x00 - Device DMA AXI control."]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x04 - Device DMA AXI ID."]
    #[inline(always)]
    pub const fn id(&self) -> &Id {
        &self.id
    }
    #[doc = "0x08 - Device DMA AXI capability."]
    #[inline(always)]
    pub const fn cap(&self) -> &Cap {
        &self.cap
    }
    #[doc = "0x0c - Device DMA AXI control 0: **WARNING** DMA AXI max burst length - In versions preceding DEV_VER_V2, for example, iMX8QM, there exist the bugs in the DMA. These bugs occur when the trb_burst_size exceeds 16 and the address is not aligned to 128 Bytes (which is a product of the 64-bit AXI and AXI maximum burst length of 16 or 0xF+1, dma_axi_ctrl0\\[3:0\\]). This results in data corruption when it crosses the 4K border. The corruption specifically occurs from the position (4K - (address &amp; 0x7F)) to 4K. So force trb_burst_size to 16 at such platform."]
    #[inline(always)]
    pub const fn ctrl0(&self) -> &Ctrl0 {
        &self.ctrl0
    }
    #[doc = "0x10 - Device DMA AXI control 1."]
    #[inline(always)]
    pub const fn ctrl1(&self) -> &Ctrl1 {
        &self.ctrl1
    }
}
#[doc = "ctrl (rw) register accessor: Device DMA AXI control.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
#[doc(alias = "ctrl")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "Device DMA AXI control."]
pub mod ctrl;
#[doc = "id (rw) register accessor: Device DMA AXI ID.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`id::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`id::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@id`]
module"]
#[doc(alias = "id")]
pub type Id = crate::Reg<id::IdSpec>;
#[doc = "Device DMA AXI ID."]
pub mod id;
#[doc = "cap (rw) register accessor: Device DMA AXI capability.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cap::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cap::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cap`]
module"]
#[doc(alias = "cap")]
pub type Cap = crate::Reg<cap::CapSpec>;
#[doc = "Device DMA AXI capability."]
pub mod cap;
#[doc = "ctrl0 (rw) register accessor: Device DMA AXI control 0: **WARNING** DMA AXI max burst length - In versions preceding DEV_VER_V2, for example, iMX8QM, there exist the bugs in the DMA. These bugs occur when the trb_burst_size exceeds 16 and the address is not aligned to 128 Bytes (which is a product of the 64-bit AXI and AXI maximum burst length of 16 or 0xF+1, dma_axi_ctrl0\\[3:0\\]). This results in data corruption when it crosses the 4K border. The corruption specifically occurs from the position (4K - (address &amp; 0x7F)) to 4K. So force trb_burst_size to 16 at such platform.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl0`]
module"]
#[doc(alias = "ctrl0")]
pub type Ctrl0 = crate::Reg<ctrl0::Ctrl0Spec>;
#[doc = "Device DMA AXI control 0: **WARNING** DMA AXI max burst length - In versions preceding DEV_VER_V2, for example, iMX8QM, there exist the bugs in the DMA. These bugs occur when the trb_burst_size exceeds 16 and the address is not aligned to 128 Bytes (which is a product of the 64-bit AXI and AXI maximum burst length of 16 or 0xF+1, dma_axi_ctrl0\\[3:0\\]). This results in data corruption when it crosses the 4K border. The corruption specifically occurs from the position (4K - (address &amp; 0x7F)) to 4K. So force trb_burst_size to 16 at such platform."]
pub mod ctrl0;
#[doc = "ctrl1 (rw) register accessor: Device DMA AXI control 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl1`]
module"]
#[doc(alias = "ctrl1")]
pub type Ctrl1 = crate::Reg<ctrl1::Ctrl1Spec>;
#[doc = "Device DMA AXI control 1."]
pub mod ctrl1;
