#[doc = "Register `ssp_periph_id2` reader"]
pub type R = crate::R<SspPeriphId2Spec>;
#[doc = "Field `designer1` reader - These bits read back as 0x4"]
pub type Designer1R = crate::FieldReader;
#[doc = "Field `revision` reader - These bits return the peripheral revision"]
pub type RevisionR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - These bits read back as 0x4"]
    #[inline(always)]
    pub fn designer1(&self) -> Designer1R {
        Designer1R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - These bits return the peripheral revision"]
    #[inline(always)]
    pub fn revision(&self) -> RevisionR {
        RevisionR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "The SSPPeriphID2 register is hard-coded and the fields within the register determine reset value. The SSPPeriphID0-3 registers are four 8-bit registers, that span address locations 0xFE0 to 0xFEC. The registers can conceptually be treated as a single 32-bit register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ssp_periph_id2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SspPeriphId2Spec;
impl crate::RegisterSpec for SspPeriphId2Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ssp_periph_id2::R`](R) reader structure"]
impl crate::Readable for SspPeriphId2Spec {}
#[doc = "`reset()` method sets ssp_periph_id2 to value 0"]
impl crate::Resettable for SspPeriphId2Spec {
    const RESET_VALUE: u16 = 0;
}
