#[doc = "Register `hcc_params` reader"]
pub type R = crate::R<HccParamsSpec>;
#[doc = "Field `hcc_params` reader - USB3 XHCI host controller capability parameters."]
pub type HccParamsR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - USB3 XHCI host controller capability parameters."]
    #[inline(always)]
    pub fn hcc_params(&self) -> HccParamsR {
        HccParamsR::new(self.bits)
    }
}
#[doc = "USB3 XHCI host controller capability parameters.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcc_params::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HccParamsSpec;
impl crate::RegisterSpec for HccParamsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcc_params::R`](R) reader structure"]
impl crate::Readable for HccParamsSpec {}
#[doc = "`reset()` method sets hcc_params to value 0"]
impl crate::Resettable for HccParamsSpec {
    const RESET_VALUE: u32 = 0;
}
