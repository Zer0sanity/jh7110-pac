#[doc = "Register `hcs_params1` reader"]
pub type R = crate::R<HcsParams1Spec>;
#[doc = "Field `max_intrs` reader - USB3 XHCI host controller max interrupts."]
pub type MaxIntrsR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 8:18 - USB3 XHCI host controller max interrupts."]
    #[inline(always)]
    pub fn max_intrs(&self) -> MaxIntrsR {
        MaxIntrsR::new(((self.bits >> 8) & 0x07ff) as u16)
    }
}
#[doc = "USB3 XHCI host controller structural parameters 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hcs_params1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HcsParams1Spec;
impl crate::RegisterSpec for HcsParams1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcs_params1::R`](R) reader structure"]
impl crate::Readable for HcsParams1Spec {}
#[doc = "`reset()` method sets hcs_params1 to value 0"]
impl crate::Resettable for HcsParams1Spec {
    const RESET_VALUE: u32 = 0;
}
