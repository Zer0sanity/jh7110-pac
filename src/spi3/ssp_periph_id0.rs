#[doc = "Register `ssp_periph_id0` reader"]
pub type R = crate::R<SspPeriphId0Spec>;
#[doc = "Field `part_number0` reader - These bits read back as 0x22"]
pub type PartNumber0R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - These bits read back as 0x22"]
    #[inline(always)]
    pub fn part_number0(&self) -> PartNumber0R {
        PartNumber0R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "The SSPPeriphID0 register is hard-coded and the fields within the register determine reset value. The SSPPeriphID0-3 registers are four 8-bit registers, that span address locations 0xFE0 to 0xFEC. The registers can conceptually be treated as a single 32-bit register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ssp_periph_id0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SspPeriphId0Spec;
impl crate::RegisterSpec for SspPeriphId0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ssp_periph_id0::R`](R) reader structure"]
impl crate::Readable for SspPeriphId0Spec {}
#[doc = "`reset()` method sets ssp_periph_id0 to value 0"]
impl crate::Resettable for SspPeriphId0Spec {
    const RESET_VALUE: u16 = 0;
}
