#[doc = "Register `hcs_params3` reader"]
pub type R = crate::R<HcsParams3Spec>;
#[doc = "Field `hcs_params3` reader - USB3 XHCI host controller structural parameters 3."]
pub type HcsParams3R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - USB3 XHCI host controller structural parameters 3."]
    #[inline(always)]
    pub fn hcs_params3(&self) -> HcsParams3R {
        HcsParams3R::new(self.bits)
    }
}
#[doc = "USB3 XHCI host controller structural parameters 3.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcs_params3::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HcsParams3Spec;
impl crate::RegisterSpec for HcsParams3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcs_params3::R`](R) reader structure"]
impl crate::Readable for HcsParams3Spec {}
#[doc = "`reset()` method sets hcs_params3 to value 0"]
impl crate::Resettable for HcsParams3Spec {
    const RESET_VALUE: u32 = 0;
}
