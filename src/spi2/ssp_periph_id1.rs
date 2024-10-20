#[doc = "Register `ssp_periph_id1` reader"]
pub type R = crate::R<SspPeriphId1Spec>;
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
#[doc = "The SSPPeriphID1 register is hard-coded and the fields within the register determine reset value. The SSPPeriphID0-3 registers are four 8-bit registers, that span address locations 0xFE0 to 0xFEC. The registers can conceptually be treated as a single 32-bit register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ssp_periph_id1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SspPeriphId1Spec;
impl crate::RegisterSpec for SspPeriphId1Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ssp_periph_id1::R`](R) reader structure"]
impl crate::Readable for SspPeriphId1Spec {}
#[doc = "`reset()` method sets ssp_periph_id1 to value 0"]
impl crate::Resettable for SspPeriphId1Spec {
    const RESET_VALUE: u16 = 0;
}
