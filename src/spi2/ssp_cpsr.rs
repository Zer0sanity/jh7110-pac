#[doc = "Register `ssp_cpsr` reader"]
pub type R = crate::R<SspCpsrSpec>;
#[doc = "Register `ssp_cpsr` writer"]
pub type W = crate::W<SspCpsrSpec>;
#[doc = "Field `cpsdvsr` reader - Clock prescale divisor. Must be an even number from 2-254, depending on the frequency of SSPCLK. The least significant bit always returns zero on reads."]
pub type CpsdvsrR = crate::FieldReader;
#[doc = "Field `cpsdvsr` writer - Clock prescale divisor. Must be an even number from 2-254, depending on the frequency of SSPCLK. The least significant bit always returns zero on reads."]
pub type CpsdvsrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Clock prescale divisor. Must be an even number from 2-254, depending on the frequency of SSPCLK. The least significant bit always returns zero on reads."]
    #[inline(always)]
    pub fn cpsdvsr(&self) -> CpsdvsrR {
        CpsdvsrR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Clock prescale divisor. Must be an even number from 2-254, depending on the frequency of SSPCLK. The least significant bit always returns zero on reads."]
    #[inline(always)]
    #[must_use]
    pub fn cpsdvsr(&mut self) -> CpsdvsrW<SspCpsrSpec> {
        CpsdvsrW::new(self, 0)
    }
}
#[doc = "SSPCPSR is the clock prescale register and specifies the division factor by which the input SSPCLK must be internally divided before further use. The value programmed into this register must be an even number between \\[2:254\\]. The least significant bit of the programmed number is hard-coded to zero. If an odd number is written to this register, data read back from this register has the least significant bit as zero.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ssp_cpsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ssp_cpsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SspCpsrSpec;
impl crate::RegisterSpec for SspCpsrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ssp_cpsr::R`](R) reader structure"]
impl crate::Readable for SspCpsrSpec {}
#[doc = "`write(|w| ..)` method takes [`ssp_cpsr::W`](W) writer structure"]
impl crate::Writable for SspCpsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets ssp_cpsr to value 0"]
impl crate::Resettable for SspCpsrSpec {
    const RESET_VALUE: u16 = 0;
}
