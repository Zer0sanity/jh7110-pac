#[doc = "Register `clr_start_det` reader"]
pub type R = crate::R<ClrStartDetSpec>;
#[doc = "Register `clr_start_det` writer"]
pub type W = crate::W<ClrStartDetSpec>;
#[doc = "Field `clr_start_det` reader - "]
pub type ClrStartDetR = crate::FieldReader<u32>;
#[doc = "Field `clr_start_det` writer - "]
pub type ClrStartDetW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn clr_start_det(&self) -> ClrStartDetR {
        ClrStartDetR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn clr_start_det(&mut self) -> ClrStartDetW<ClrStartDetSpec> {
        ClrStartDetW::new(self, 0)
    }
}
#[doc = "DesignWare I2C Clear Start DET\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clr_start_det::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clr_start_det::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClrStartDetSpec;
impl crate::RegisterSpec for ClrStartDetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clr_start_det::R`](R) reader structure"]
impl crate::Readable for ClrStartDetSpec {}
#[doc = "`write(|w| ..)` method takes [`clr_start_det::W`](W) writer structure"]
impl crate::Writable for ClrStartDetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets clr_start_det to value 0"]
impl crate::Resettable for ClrStartDetSpec {
    const RESET_VALUE: u32 = 0;
}
