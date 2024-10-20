#[doc = "Register `periph_id1` reader"]
pub type R = crate::R<PeriphId1Spec>;
#[doc = "Field `part_number1` reader - These bits read back as 0x0"]
pub type PartNumber1R = crate::FieldReader;
#[doc = "Field `designer0` reader - These bits read back as 0x1"]
pub type Designer0R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - These bits read back as 0x0"]
    #[inline(always)]
    pub fn part_number1(&self) -> PartNumber1R {
        PartNumber1R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - These bits read back as 0x1"]
    #[inline(always)]
    pub fn designer0(&self) -> Designer0R {
        Designer0R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[doc = "DMA Peripheral ID 1 register - is hard-coded and the fields in the register determine the reset value.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`periph_id1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PeriphId1Spec;
impl crate::RegisterSpec for PeriphId1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`periph_id1::R`](R) reader structure"]
impl crate::Readable for PeriphId1Spec {}
#[doc = "`reset()` method sets periph_id1 to value 0x10"]
impl crate::Resettable for PeriphId1Spec {
    const RESET_VALUE: u32 = 0x10;
}
