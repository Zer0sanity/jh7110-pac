#[doc = "Register `periph_id2` reader"]
pub type R = crate::R<PeriphId2Spec>;
#[doc = "Field `designer1` reader - These bits read back as 0x4"]
pub type Designer1R = crate::FieldReader;
#[doc = "Field `revision` reader - These bits read back as 0x1"]
pub type RevisionR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - These bits read back as 0x4"]
    #[inline(always)]
    pub fn designer1(&self) -> Designer1R {
        Designer1R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - These bits read back as 0x1"]
    #[inline(always)]
    pub fn revision(&self) -> RevisionR {
        RevisionR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "DMA Peripheral ID 2 register - is hard-coded and the fields in the register determine the reset value.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`periph_id2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PeriphId2Spec;
impl crate::RegisterSpec for PeriphId2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`periph_id2::R`](R) reader structure"]
impl crate::Readable for PeriphId2Spec {}
#[doc = "`reset()` method sets periph_id2 to value 0x14"]
impl crate::Resettable for PeriphId2Spec {
    const RESET_VALUE: u32 = 0x14;
}
