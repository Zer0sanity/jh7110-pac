#[repr(C)]
#[doc = "USB Interrupt registers."]
#[doc(alias = "usb_int")]
pub struct UsbInt {
    en: En,
    sts: Sts,
}
impl UsbInt {
    #[doc = "0x00 - Global Interrupt Enable."]
    #[inline(always)]
    pub const fn en(&self) -> &En {
        &self.en
    }
    #[doc = "0x04 - Global Interrupt Status."]
    #[inline(always)]
    pub const fn sts(&self) -> &Sts {
        &self.sts
    }
}
#[doc = "en (rw) register accessor: Global Interrupt Enable.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`en::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`en::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@en`]
module"]
#[doc(alias = "en")]
pub type En = crate::Reg<en::EnSpec>;
#[doc = "Global Interrupt Enable."]
pub mod en;
#[doc = "sts (rw) register accessor: Global Interrupt Status.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sts::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sts::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sts`]
module"]
#[doc(alias = "sts")]
pub type Sts = crate::Reg<sts::StsSpec>;
#[doc = "Global Interrupt Status."]
pub mod sts;
