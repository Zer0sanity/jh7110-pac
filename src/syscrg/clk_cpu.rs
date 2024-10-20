#[repr(C)]
#[doc = "Clock CPU registers"]
#[doc(alias = "clk_cpu")]
pub struct ClkCpu {
    root: Root,
    core: Core,
    bus: Bus,
}
impl ClkCpu {
    #[doc = "0x00 - Clock CPU Root"]
    #[inline(always)]
    pub const fn root(&self) -> &Root {
        &self.root
    }
    #[doc = "0x04 - Clock CPU Core"]
    #[inline(always)]
    pub const fn core(&self) -> &Core {
        &self.core
    }
    #[doc = "0x08 - Clock CPU Bus"]
    #[inline(always)]
    pub const fn bus(&self) -> &Bus {
        &self.bus
    }
}
#[doc = "root (rw) register accessor: Clock CPU Root\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`root::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`root::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@root`]
module"]
#[doc(alias = "root")]
pub type Root = crate::Reg<root::RootSpec>;
#[doc = "Clock CPU Root"]
pub mod root;
#[doc = "core (rw) register accessor: Clock CPU Core\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core`]
module"]
#[doc(alias = "core")]
pub type Core = crate::Reg<core::CoreSpec>;
#[doc = "Clock CPU Core"]
pub mod core;
#[doc = "bus (rw) register accessor: Clock CPU Bus\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bus::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bus::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bus`]
module"]
#[doc(alias = "bus")]
pub type Bus = crate::Reg<bus::BusSpec>;
#[doc = "Clock CPU Bus"]
pub mod bus;
