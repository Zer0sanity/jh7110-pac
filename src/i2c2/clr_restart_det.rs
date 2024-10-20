#[doc = "Register `clr_restart_det` reader"]
pub type R = crate::R<ClrRestartDetSpec>;
#[doc = "Register `clr_restart_det` writer"]
pub type W = crate::W<ClrRestartDetSpec>;
#[doc = "Field `clr_restart_det` reader - "]
pub type ClrRestartDetR = crate::FieldReader<u32>;
#[doc = "Field `clr_restart_det` writer - "]
pub type ClrRestartDetW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn clr_restart_det(&self) -> ClrRestartDetR {
        ClrRestartDetR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn clr_restart_det(&mut self) -> ClrRestartDetW<ClrRestartDetSpec> {
        ClrRestartDetW::new(self, 0)
    }
}
#[doc = "DesignWare I2C Clear Restart DET\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clr_restart_det::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clr_restart_det::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClrRestartDetSpec;
impl crate::RegisterSpec for ClrRestartDetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clr_restart_det::R`](R) reader structure"]
impl crate::Readable for ClrRestartDetSpec {}
#[doc = "`write(|w| ..)` method takes [`clr_restart_det::W`](W) writer structure"]
impl crate::Writable for ClrRestartDetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets clr_restart_det to value 0"]
impl crate::Resettable for ClrRestartDetSpec {
    const RESET_VALUE: u32 = 0;
}
