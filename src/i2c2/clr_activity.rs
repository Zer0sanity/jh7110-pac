#[doc = "Register `clr_activity` reader"]
pub type R = crate::R<ClrActivitySpec>;
#[doc = "Register `clr_activity` writer"]
pub type W = crate::W<ClrActivitySpec>;
#[doc = "Field `clr_activity` reader - "]
pub type ClrActivityR = crate::FieldReader<u32>;
#[doc = "Field `clr_activity` writer - "]
pub type ClrActivityW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn clr_activity(&self) -> ClrActivityR {
        ClrActivityR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn clr_activity(&mut self) -> ClrActivityW<ClrActivitySpec> {
        ClrActivityW::new(self, 0)
    }
}
#[doc = "DesignWare I2C Clear Activity\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clr_activity::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clr_activity::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClrActivitySpec;
impl crate::RegisterSpec for ClrActivitySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clr_activity::R`](R) reader structure"]
impl crate::Readable for ClrActivitySpec {}
#[doc = "`write(|w| ..)` method takes [`clr_activity::W`](W) writer structure"]
impl crate::Writable for ClrActivitySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets clr_activity to value 0"]
impl crate::Resettable for ClrActivitySpec {
    const RESET_VALUE: u32 = 0;
}
