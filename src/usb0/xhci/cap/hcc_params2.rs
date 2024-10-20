#[doc = "Register `hcc_params2` reader"]
pub type R = crate::R<HccParams2Spec>;
#[doc = "Field `hcc_params2` reader - USB3 XHCI host controller capability parameters."]
pub type HccParams2R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - USB3 XHCI host controller capability parameters."]
    #[inline(always)]
    pub fn hcc_params2(&self) -> HccParams2R {
        HccParams2R::new(self.bits)
    }
}
#[doc = "USB3 XHCI host controller capabilities parameters - XHCI v1.1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcc_params2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HccParams2Spec;
impl crate::RegisterSpec for HccParams2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcc_params2::R`](R) reader structure"]
impl crate::Readable for HccParams2Spec {}
#[doc = "`reset()` method sets hcc_params2 to value 0"]
impl crate::Resettable for HccParams2Spec {
    const RESET_VALUE: u32 = 0;
}
