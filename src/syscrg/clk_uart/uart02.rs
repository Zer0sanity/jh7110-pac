#[repr(C)]
#[doc = "Clock UART U0-U2"]
#[doc(alias = "uart02")]
pub struct Uart02 {
    apb: Apb,
    core: Core,
}
impl Uart02 {
    #[doc = "0x00 - Clock UART APB"]
    #[inline(always)]
    pub const fn apb(&self) -> &Apb {
        &self.apb
    }
    #[doc = "0x04 - Clock UART Core"]
    #[inline(always)]
    pub const fn core(&self) -> &Core {
        &self.core
    }
}
#[doc = "apb (rw) register accessor: Clock UART APB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb`]
module"]
#[doc(alias = "apb")]
pub type Apb = crate::Reg<apb::ApbSpec>;
#[doc = "Clock UART APB"]
pub mod apb;
#[doc = "core (rw) register accessor: Clock UART Core\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core`]
module"]
#[doc(alias = "core")]
pub type Core = crate::Reg<core::CoreSpec>;
#[doc = "Clock UART Core"]
pub mod core;
