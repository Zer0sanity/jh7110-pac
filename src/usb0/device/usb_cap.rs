#[repr(C)]
#[doc = "USB3 Global Capability registers."]
#[doc(alias = "usb_cap")]
pub struct UsbCap {
    cap1: Cap1,
    cap2: Cap2,
    cap3: Cap3,
    cap4: Cap4,
    cap5: Cap5,
    cap6: Cap6,
}
impl UsbCap {
    #[doc = "0x00 - USB3 Global capability 1."]
    #[inline(always)]
    pub const fn cap1(&self) -> &Cap1 {
        &self.cap1
    }
    #[doc = "0x04 - USB3 Global capability 2."]
    #[inline(always)]
    pub const fn cap2(&self) -> &Cap2 {
        &self.cap2
    }
    #[doc = "0x08 - USB3 Global capability 3."]
    #[inline(always)]
    pub const fn cap3(&self) -> &Cap3 {
        &self.cap3
    }
    #[doc = "0x0c - USB3 Global capability 4."]
    #[inline(always)]
    pub const fn cap4(&self) -> &Cap4 {
        &self.cap4
    }
    #[doc = "0x10 - USB3 Global capability 5."]
    #[inline(always)]
    pub const fn cap5(&self) -> &Cap5 {
        &self.cap5
    }
    #[doc = "0x14 - USB3 Global capability 6."]
    #[inline(always)]
    pub const fn cap6(&self) -> &Cap6 {
        &self.cap6
    }
}
#[doc = "cap1 (rw) register accessor: USB3 Global capability 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cap1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cap1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cap1`]
module"]
#[doc(alias = "cap1")]
pub type Cap1 = crate::Reg<cap1::Cap1Spec>;
#[doc = "USB3 Global capability 1."]
pub mod cap1;
#[doc = "cap2 (rw) register accessor: USB3 Global capability 2.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cap2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cap2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cap2`]
module"]
#[doc(alias = "cap2")]
pub type Cap2 = crate::Reg<cap2::Cap2Spec>;
#[doc = "USB3 Global capability 2."]
pub mod cap2;
#[doc = "cap3 (rw) register accessor: USB3 Global capability 3.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cap3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cap3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cap3`]
module"]
#[doc(alias = "cap3")]
pub type Cap3 = crate::Reg<cap3::Cap3Spec>;
#[doc = "USB3 Global capability 3."]
pub mod cap3;
#[doc = "cap4 (rw) register accessor: USB3 Global capability 4.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cap4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cap4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cap4`]
module"]
#[doc(alias = "cap4")]
pub type Cap4 = crate::Reg<cap4::Cap4Spec>;
#[doc = "USB3 Global capability 4."]
pub mod cap4;
#[doc = "cap5 (rw) register accessor: USB3 Global capability 5.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cap5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cap5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cap5`]
module"]
#[doc(alias = "cap5")]
pub type Cap5 = crate::Reg<cap5::Cap5Spec>;
#[doc = "USB3 Global capability 5."]
pub mod cap5;
#[doc = "cap6 (r) register accessor: USB3 Global capability 6.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cap6::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cap6`]
module"]
#[doc(alias = "cap6")]
pub type Cap6 = crate::Reg<cap6::Cap6Spec>;
#[doc = "USB3 Global capability 6."]
pub mod cap6;
