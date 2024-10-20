#[doc = "Register `itop2` reader"]
pub type R = crate::R<Itop2Spec>;
#[doc = "Register `itop2` writer"]
pub type W = crate::W<Itop2Spec>;
#[doc = "Field `tc` reader - Controls and reads the DMACTC\\[15:0\\]
output lines in test mode."]
pub type TcR = crate::FieldReader<u16>;
#[doc = "Field `tc` writer - Controls and reads the DMACTC\\[15:0\\]
output lines in test mode."]
pub type TcW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Controls and reads the DMACTC\\[15:0\\]
output lines in test mode."]
    #[inline(always)]
    pub fn tc(&self) -> TcR {
        TcR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Controls and reads the DMACTC\\[15:0\\]
output lines in test mode."]
    #[inline(always)]
    #[must_use]
    pub fn tc(&mut self) -> TcW<Itop2Spec> {
        TcW::new(self, 0)
    }
}
#[doc = "DMA Integration Test Output 2 register - controls and reads the DMACTC\\[15:0\\]
output lines in test mode.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`itop2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`itop2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Itop2Spec;
impl crate::RegisterSpec for Itop2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`itop2::R`](R) reader structure"]
impl crate::Readable for Itop2Spec {}
#[doc = "`write(|w| ..)` method takes [`itop2::W`](W) writer structure"]
impl crate::Writable for Itop2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets itop2 to value 0"]
impl crate::Resettable for Itop2Spec {
    const RESET_VALUE: u32 = 0;
}
