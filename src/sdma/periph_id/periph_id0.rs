#[doc = "Register `periph_id0` reader"]
pub type R = crate::R<PeriphId0Spec>;
#[doc = "Field `part_number0` reader - These bits read back as 0x80"]
pub type PartNumber0R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - These bits read back as 0x80"]
    #[inline(always)]
    pub fn part_number0(&self) -> PartNumber0R {
        PartNumber0R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "DMA Peripheral ID 0 register - is hard-coded and the fields in the register determine the reset value.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`periph_id0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PeriphId0Spec;
impl crate::RegisterSpec for PeriphId0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`periph_id0::R`](R) reader structure"]
impl crate::Readable for PeriphId0Spec {}
#[doc = "`reset()` method sets periph_id0 to value 0x80"]
impl crate::Resettable for PeriphId0Spec {
    const RESET_VALUE: u32 = 0x80;
}
