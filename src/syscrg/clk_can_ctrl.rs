#[repr(C)]
#[doc = "Clock CAN Controller"]
#[doc(alias = "clk_can_ctrl")]
pub struct ClkCanCtrl {
    apb: Apb,
    tim: Tim,
    can: Can,
}
impl ClkCanCtrl {
    #[doc = "0x00 - Clock Internal Controller APB"]
    #[inline(always)]
    pub const fn apb(&self) -> &Apb {
        &self.apb
    }
    #[doc = "0x04 - Clock Internal Controller Timer"]
    #[inline(always)]
    pub const fn tim(&self) -> &Tim {
        &self.tim
    }
    #[doc = "0x08 - Clock Internal Controller CAN"]
    #[inline(always)]
    pub const fn can(&self) -> &Can {
        &self.can
    }
}
#[doc = "apb (rw) register accessor: Clock Internal Controller APB\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb`]
module"]
#[doc(alias = "apb")]
pub type Apb = crate::Reg<apb::ApbSpec>;
#[doc = "Clock Internal Controller APB"]
pub mod apb;
#[doc = "tim (rw) register accessor: Clock Internal Controller Timer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tim::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tim::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tim`]
module"]
#[doc(alias = "tim")]
pub type Tim = crate::Reg<tim::TimSpec>;
#[doc = "Clock Internal Controller Timer"]
pub mod tim;
#[doc = "can (rw) register accessor: Clock Internal Controller CAN\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`can::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`can::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@can`]
module"]
#[doc(alias = "can")]
pub type Can = crate::Reg<can::CanSpec>;
#[doc = "Clock Internal Controller CAN"]
pub mod can;
