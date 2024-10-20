#[doc = "Register `width` reader"]
pub type R = crate::R<WidthSpec>;
#[doc = "Register `width` writer"]
pub type W = crate::W<WidthSpec>;
#[doc = "Field `width` reader - PPS Width"]
pub type WidthR = crate::FieldReader<u32>;
#[doc = "Field `width` writer - PPS Width"]
pub type WidthW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - PPS Width"]
    #[inline(always)]
    pub fn width(&self) -> WidthR {
        WidthR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - PPS Width"]
    #[inline(always)]
    #[must_use]
    pub fn width(&mut self) -> WidthW<WidthSpec> {
        WidthW::new(self, 0)
    }
}
#[doc = "PPS Width\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`width::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`width::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WidthSpec;
impl crate::RegisterSpec for WidthSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`width::R`](R) reader structure"]
impl crate::Readable for WidthSpec {}
#[doc = "`write(|w| ..)` method takes [`width::W`](W) writer structure"]
impl crate::Writable for WidthSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets width to value 0"]
impl crate::Resettable for WidthSpec {
    const RESET_VALUE: u32 = 0;
}
