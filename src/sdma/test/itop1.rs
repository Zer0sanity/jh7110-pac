#[doc = "Register `itop1` reader"]
pub type R = crate::R<Itop1Spec>;
#[doc = "Register `itop1` writer"]
pub type W = crate::W<Itop1Spec>;
#[doc = "Field `clr` reader - Controls and reads the DMACCLR\\[15:0\\]
output lines in test mode."]
pub type ClrR = crate::FieldReader<u16>;
#[doc = "Field `clr` writer - Controls and reads the DMACCLR\\[15:0\\]
output lines in test mode."]
pub type ClrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Controls and reads the DMACCLR\\[15:0\\]
output lines in test mode."]
    #[inline(always)]
    pub fn clr(&self) -> ClrR {
        ClrR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Controls and reads the DMACCLR\\[15:0\\]
output lines in test mode."]
    #[inline(always)]
    #[must_use]
    pub fn clr(&mut self) -> ClrW<Itop1Spec> {
        ClrW::new(self, 0)
    }
}
#[doc = "DMA Integration Test Output 1 register - controls and reads the DMACCLR\\[15:0\\]
output lines in test mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`itop1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`itop1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Itop1Spec;
impl crate::RegisterSpec for Itop1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`itop1::R`](R) reader structure"]
impl crate::Readable for Itop1Spec {}
#[doc = "`write(|w| ..)` method takes [`itop1::W`](W) writer structure"]
impl crate::Writable for Itop1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets itop1 to value 0"]
impl crate::Resettable for Itop1Spec {
    const RESET_VALUE: u32 = 0;
}
