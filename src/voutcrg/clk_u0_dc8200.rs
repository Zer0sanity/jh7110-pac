#[repr(C)]
#[doc = "Clock U0 DC8200 registers"]
#[doc(alias = "clk_u0_dc8200")]
pub struct ClkU0Dc8200 {
    axi: Axi,
    core: Core,
    ahb: Ahb,
    pix: [Pix; 2],
}
impl ClkU0Dc8200 {
    #[doc = "0x00 - Clock U0 DC8200 AXI"]
    #[inline(always)]
    pub const fn axi(&self) -> &Axi {
        &self.axi
    }
    #[doc = "0x04 - Clock U0 DC8200 Core"]
    #[inline(always)]
    pub const fn core(&self) -> &Core {
        &self.core
    }
    #[doc = "0x08 - Clock U0 DC8200 AHB"]
    #[inline(always)]
    pub const fn ahb(&self) -> &Ahb {
        &self.ahb
    }
    #[doc = "0x0c..0x14 - Clock U0 DC8200 Pixel Clock selector"]
    #[inline(always)]
    pub const fn pix(&self, n: usize) -> &Pix {
        &self.pix[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x0c..0x14 - Clock U0 DC8200 Pixel Clock selector"]
    #[inline(always)]
    pub fn pix_iter(&self) -> impl Iterator<Item = &Pix> {
        self.pix.iter()
    }
}
#[doc = "axi (rw) register accessor: Clock U0 DC8200 AXI\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`axi::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`axi::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@axi`]
module"]
#[doc(alias = "axi")]
pub type Axi = crate::Reg<axi::AxiSpec>;
#[doc = "Clock U0 DC8200 AXI"]
pub mod axi;
#[doc = "core (rw) register accessor: Clock U0 DC8200 Core\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@core`]
module"]
#[doc(alias = "core")]
pub type Core = crate::Reg<core::CoreSpec>;
#[doc = "Clock U0 DC8200 Core"]
pub mod core;
#[doc = "ahb (rw) register accessor: Clock U0 DC8200 AHB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb`]
module"]
#[doc(alias = "ahb")]
pub type Ahb = crate::Reg<ahb::AhbSpec>;
#[doc = "Clock U0 DC8200 AHB"]
pub mod ahb;
#[doc = "pix (rw) register accessor: Clock U0 DC8200 Pixel Clock selector\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pix::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pix::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pix`]
module"]
#[doc(alias = "pix")]
pub type Pix = crate::Reg<pix::PixSpec>;
#[doc = "Clock U0 DC8200 Pixel Clock selector"]
pub mod pix;
