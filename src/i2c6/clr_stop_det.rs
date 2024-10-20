#[doc = "Register `clr_stop_det` reader"]
pub type R = crate::R<ClrStopDetSpec>;
#[doc = "Register `clr_stop_det` writer"]
pub type W = crate::W<ClrStopDetSpec>;
#[doc = "Field `clr_stop_det` reader - "]
pub type ClrStopDetR = crate::FieldReader<u32>;
#[doc = "Field `clr_stop_det` writer - "]
pub type ClrStopDetW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn clr_stop_det(&self) -> ClrStopDetR {
        ClrStopDetR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn clr_stop_det(&mut self) -> ClrStopDetW<ClrStopDetSpec> {
        ClrStopDetW::new(self, 0)
    }
}
#[doc = "DesignWare I2C Clear Stop DET\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clr_stop_det::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clr_stop_det::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClrStopDetSpec;
impl crate::RegisterSpec for ClrStopDetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clr_stop_det::R`](R) reader structure"]
impl crate::Readable for ClrStopDetSpec {}
#[doc = "`write(|w| ..)` method takes [`clr_stop_det::W`](W) writer structure"]
impl crate::Writable for ClrStopDetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets clr_stop_det to value 0"]
impl crate::Resettable for ClrStopDetSpec {
    const RESET_VALUE: u32 = 0;
}
