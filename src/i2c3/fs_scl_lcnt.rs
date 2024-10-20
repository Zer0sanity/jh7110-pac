#[doc = "Register `fs_scl_lcnt` reader"]
pub type R = crate::R<FsSclLcntSpec>;
#[doc = "Register `fs_scl_lcnt` writer"]
pub type W = crate::W<FsSclLcntSpec>;
#[doc = "Field `fs_scl_lcnt` reader - "]
pub type FsSclLcntR = crate::FieldReader<u32>;
#[doc = "Field `fs_scl_lcnt` writer - "]
pub type FsSclLcntW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn fs_scl_lcnt(&self) -> FsSclLcntR {
        FsSclLcntR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn fs_scl_lcnt(&mut self) -> FsSclLcntW<FsSclLcntSpec> {
        FsSclLcntW::new(self, 0)
    }
}
#[doc = "DesignWare I2C FS SCL LCNT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fs_scl_lcnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fs_scl_lcnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FsSclLcntSpec;
impl crate::RegisterSpec for FsSclLcntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fs_scl_lcnt::R`](R) reader structure"]
impl crate::Readable for FsSclLcntSpec {}
#[doc = "`write(|w| ..)` method takes [`fs_scl_lcnt::W`](W) writer structure"]
impl crate::Writable for FsSclLcntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets fs_scl_lcnt to value 0"]
impl crate::Resettable for FsSclLcntSpec {
    const RESET_VALUE: u32 = 0;
}
