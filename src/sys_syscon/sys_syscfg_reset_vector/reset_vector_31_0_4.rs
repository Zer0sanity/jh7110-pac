#[doc = "Register `reset_vector_31_0_4` reader"]
pub type R = crate::R<ResetVector31_0_4Spec>;
#[doc = "Register `reset_vector_31_0_4` writer"]
pub type W = crate::W<ResetVector31_0_4Spec>;
#[doc = "Field `vectors` reader - Reset vector bits"]
pub type VectorsR = crate::FieldReader<u32>;
#[doc = "Field `vectors` writer - Reset vector bits"]
pub type VectorsW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Reset vector bits"]
    #[inline(always)]
    pub fn vectors(&self) -> VectorsR {
        VectorsR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Reset vector bits"]
    #[inline(always)]
    #[must_use]
    pub fn vectors(&mut self) -> VectorsW<ResetVector31_0_4Spec> {
        VectorsW::new(self, 0)
    }
}
#[doc = "Reset vector register with 32 vector fields\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reset_vector_31_0_4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reset_vector_31_0_4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ResetVector31_0_4Spec;
impl crate::RegisterSpec for ResetVector31_0_4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reset_vector_31_0_4::R`](R) reader structure"]
impl crate::Readable for ResetVector31_0_4Spec {}
#[doc = "`write(|w| ..)` method takes [`reset_vector_31_0_4::W`](W) writer structure"]
impl crate::Writable for ResetVector31_0_4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets reset_vector_31_0_4 to value 0x2a00_0000"]
impl crate::Resettable for ResetVector31_0_4Spec {
    const RESET_VALUE: u32 = 0x2a00_0000;
}
