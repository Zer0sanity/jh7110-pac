#[doc = "Register `reset_vector_35_32` reader"]
pub type R = crate::R<ResetVector35_32Spec>;
#[doc = "Register `reset_vector_35_32` writer"]
pub type W = crate::W<ResetVector35_32Spec>;
#[doc = "Field `vectors` reader - Reset vector bits"]
pub type VectorsR = crate::FieldReader;
#[doc = "Field `vectors` writer - Reset vector bits"]
pub type VectorsW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Reset vector bits"]
    #[inline(always)]
    pub fn vectors(&self) -> VectorsR {
        VectorsR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Reset vector bits"]
    #[inline(always)]
    #[must_use]
    pub fn vectors(&mut self) -> VectorsW<ResetVector35_32Spec> {
        VectorsW::new(self, 0)
    }
}
#[doc = "Reset vector register with 4 vector fields\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reset_vector_35_32::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reset_vector_35_32::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ResetVector35_32Spec;
impl crate::RegisterSpec for ResetVector35_32Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reset_vector_35_32::R`](R) reader structure"]
impl crate::Readable for ResetVector35_32Spec {}
#[doc = "`write(|w| ..)` method takes [`reset_vector_35_32::W`](W) writer structure"]
impl crate::Writable for ResetVector35_32Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets reset_vector_35_32 to value 0"]
impl crate::Resettable for ResetVector35_32Spec {
    const RESET_VALUE: u32 = 0;
}
