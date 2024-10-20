#[doc = "Register `hw_event_crd` reader"]
pub type R = crate::R<HwEventCrdSpec>;
#[doc = "Field `hw_event_crd` reader - "]
pub type HwEventCrdR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn hw_event_crd(&self) -> HwEventCrdR {
        HwEventCrdR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "Hardware Event Record register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hw_event_crd::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HwEventCrdSpec;
impl crate::RegisterSpec for HwEventCrdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hw_event_crd::R`](R) reader structure"]
impl crate::Readable for HwEventCrdSpec {}
#[doc = "`reset()` method sets hw_event_crd to value 0"]
impl crate::Resettable for HwEventCrdSpec {
    const RESET_VALUE: u32 = 0;
}
