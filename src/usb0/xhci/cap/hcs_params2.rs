#[doc = "Register `hcs_params2` reader"]
pub type R = crate::R<HcsParams2Spec>;
#[doc = "Field `hcs_params2` reader - USB3 XHCI host controller structural parameters 2."]
pub type HcsParams2R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - USB3 XHCI host controller structural parameters 2."]
    #[inline(always)]
    pub fn hcs_params2(&self) -> HcsParams2R {
        HcsParams2R::new(self.bits)
    }
}
#[doc = "USB3 XHCI host controller structural parameters 2.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcs_params2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HcsParams2Spec;
impl crate::RegisterSpec for HcsParams2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcs_params2::R`](R) reader structure"]
impl crate::Readable for HcsParams2Spec {}
#[doc = "`reset()` method sets hcs_params2 to value 0"]
impl crate::Resettable for HcsParams2Spec {
    const RESET_VALUE: u32 = 0;
}
