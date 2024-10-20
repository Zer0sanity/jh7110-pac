#[doc = "Register `ss_scl_hcnt` reader"]
pub type R = crate::R<SsSclHcntSpec>;
#[doc = "Register `ss_scl_hcnt` writer"]
pub type W = crate::W<SsSclHcntSpec>;
#[doc = "Field `ss_scl_hcnt` reader - "]
pub type SsSclHcntR = crate::FieldReader<u32>;
#[doc = "Field `ss_scl_hcnt` writer - "]
pub type SsSclHcntW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn ss_scl_hcnt(&self) -> SsSclHcntR {
        SsSclHcntR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn ss_scl_hcnt(&mut self) -> SsSclHcntW<SsSclHcntSpec> {
        SsSclHcntW::new(self, 0)
    }
}
#[doc = "DesignWare I2C SS SCL HCNT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ss_scl_hcnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ss_scl_hcnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SsSclHcntSpec;
impl crate::RegisterSpec for SsSclHcntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ss_scl_hcnt::R`](R) reader structure"]
impl crate::Readable for SsSclHcntSpec {}
#[doc = "`write(|w| ..)` method takes [`ss_scl_hcnt::W`](W) writer structure"]
impl crate::Writable for SsSclHcntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ss_scl_hcnt to value 0"]
impl crate::Resettable for SsSclHcntSpec {
    const RESET_VALUE: u32 = 0;
}
