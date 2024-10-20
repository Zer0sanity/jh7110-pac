#[doc = "Register `hs_scl_lcnt` reader"]
pub type R = crate::R<HsSclLcntSpec>;
#[doc = "Register `hs_scl_lcnt` writer"]
pub type W = crate::W<HsSclLcntSpec>;
#[doc = "Field `hs_scl_lcnt` reader - "]
pub type HsSclLcntR = crate::FieldReader<u32>;
#[doc = "Field `hs_scl_lcnt` writer - "]
pub type HsSclLcntW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn hs_scl_lcnt(&self) -> HsSclLcntR {
        HsSclLcntR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn hs_scl_lcnt(&mut self) -> HsSclLcntW<HsSclLcntSpec> {
        HsSclLcntW::new(self, 0)
    }
}
#[doc = "DesignWare I2C HS SCL LCNT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hs_scl_lcnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hs_scl_lcnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HsSclLcntSpec;
impl crate::RegisterSpec for HsSclLcntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hs_scl_lcnt::R`](R) reader structure"]
impl crate::Readable for HsSclLcntSpec {}
#[doc = "`write(|w| ..)` method takes [`hs_scl_lcnt::W`](W) writer structure"]
impl crate::Writable for HsSclLcntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets hs_scl_lcnt to value 0"]
impl crate::Resettable for HsSclLcntSpec {
    const RESET_VALUE: u32 = 0;
}
