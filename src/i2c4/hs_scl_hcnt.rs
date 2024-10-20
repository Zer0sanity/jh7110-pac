#[doc = "Register `hs_scl_hcnt` reader"]
pub type R = crate::R<HsSclHcntSpec>;
#[doc = "Register `hs_scl_hcnt` writer"]
pub type W = crate::W<HsSclHcntSpec>;
#[doc = "Field `hs_scl_hcnt` reader - "]
pub type HsSclHcntR = crate::FieldReader<u32>;
#[doc = "Field `hs_scl_hcnt` writer - "]
pub type HsSclHcntW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn hs_scl_hcnt(&self) -> HsSclHcntR {
        HsSclHcntR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn hs_scl_hcnt(&mut self) -> HsSclHcntW<HsSclHcntSpec> {
        HsSclHcntW::new(self, 0)
    }
}
#[doc = "DesignWare I2C HS SCL HCNT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hs_scl_hcnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hs_scl_hcnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HsSclHcntSpec;
impl crate::RegisterSpec for HsSclHcntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hs_scl_hcnt::R`](R) reader structure"]
impl crate::Readable for HsSclHcntSpec {}
#[doc = "`write(|w| ..)` method takes [`hs_scl_hcnt::W`](W) writer structure"]
impl crate::Writable for HsSclHcntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets hs_scl_hcnt to value 0"]
impl crate::Resettable for HsSclHcntSpec {
    const RESET_VALUE: u32 = 0;
}
