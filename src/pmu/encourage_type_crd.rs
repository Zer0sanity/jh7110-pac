#[doc = "Register `encourage_type_crd` reader"]
pub type R = crate::R<EncourageTypeCrdSpec>;
#[doc = "Field `hw_event_crd` reader - Hardware/Software encouragement type record. 0: Software, 1: Hardware."]
pub type HwEventCrdR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Hardware/Software encouragement type record. 0: Software, 1: Hardware."]
    #[inline(always)]
    pub fn hw_event_crd(&self) -> HwEventCrdR {
        HwEventCrdR::new((self.bits & 1) != 0)
    }
}
#[doc = "Hardware Event Type Record register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`encourage_type_crd::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EncourageTypeCrdSpec;
impl crate::RegisterSpec for EncourageTypeCrdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`encourage_type_crd::R`](R) reader structure"]
impl crate::Readable for EncourageTypeCrdSpec {}
#[doc = "`reset()` method sets encourage_type_crd to value 0"]
impl crate::Resettable for EncourageTypeCrdSpec {
    const RESET_VALUE: u32 = 0;
}
