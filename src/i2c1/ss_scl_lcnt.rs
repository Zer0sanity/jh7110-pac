#[doc = "Register `ss_scl_lcnt` reader"]
pub type R = crate::R<SsSclLcntSpec>;
#[doc = "Register `ss_scl_lcnt` writer"]
pub type W = crate::W<SsSclLcntSpec>;
#[doc = "Field `ss_scl_lcnt` reader - "]
pub type SsSclLcntR = crate::FieldReader<u32>;
#[doc = "Field `ss_scl_lcnt` writer - "]
pub type SsSclLcntW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn ss_scl_lcnt(&self) -> SsSclLcntR {
        SsSclLcntR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn ss_scl_lcnt(&mut self) -> SsSclLcntW<SsSclLcntSpec> {
        SsSclLcntW::new(self, 0)
    }
}
#[doc = "DesignWare I2C SS SCL LCNT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ss_scl_lcnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ss_scl_lcnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SsSclLcntSpec;
impl crate::RegisterSpec for SsSclLcntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ss_scl_lcnt::R`](R) reader structure"]
impl crate::Readable for SsSclLcntSpec {}
#[doc = "`write(|w| ..)` method takes [`ss_scl_lcnt::W`](W) writer structure"]
impl crate::Writable for SsSclLcntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ss_scl_lcnt to value 0"]
impl crate::Resettable for SsSclLcntSpec {
    const RESET_VALUE: u32 = 0;
}
