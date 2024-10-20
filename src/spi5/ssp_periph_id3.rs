#[doc = "Register `ssp_periph_id3` reader"]
pub type R = crate::R<SspPeriphId3Spec>;
#[doc = "Field `configuration` reader - These bits read back as 0x80"]
pub type ConfigurationR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - These bits read back as 0x80"]
    #[inline(always)]
    pub fn configuration(&self) -> ConfigurationR {
        ConfigurationR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "The SSPPeriphID3 register is hard-coded and the fields within the register determine reset value. The SSPPeriphID0-3 registers are four 8-bit registers, that span address locations 0xFE0 to 0xFEC. The registers can conceptually be treated as a single 32-bit register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ssp_periph_id3::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SspPeriphId3Spec;
impl crate::RegisterSpec for SspPeriphId3Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ssp_periph_id3::R`](R) reader structure"]
impl crate::Readable for SspPeriphId3Spec {}
#[doc = "`reset()` method sets ssp_periph_id3 to value 0"]
impl crate::Resettable for SspPeriphId3Spec {
    const RESET_VALUE: u16 = 0;
}
