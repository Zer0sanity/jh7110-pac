#[doc = "Register `fs_scl_hcnt` reader"]
pub type R = crate::R<FsSclHcntSpec>;
#[doc = "Register `fs_scl_hcnt` writer"]
pub type W = crate::W<FsSclHcntSpec>;
#[doc = "Field `fs_scl_hcnt` reader - "]
pub type FsSclHcntR = crate::FieldReader<u32>;
#[doc = "Field `fs_scl_hcnt` writer - "]
pub type FsSclHcntW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn fs_scl_hcnt(&self) -> FsSclHcntR {
        FsSclHcntR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn fs_scl_hcnt(&mut self) -> FsSclHcntW<FsSclHcntSpec> {
        FsSclHcntW::new(self, 0)
    }
}
#[doc = "DesignWare I2C FS SCL HCNT\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fs_scl_hcnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fs_scl_hcnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FsSclHcntSpec;
impl crate::RegisterSpec for FsSclHcntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fs_scl_hcnt::R`](R) reader structure"]
impl crate::Readable for FsSclHcntSpec {}
#[doc = "`write(|w| ..)` method takes [`fs_scl_hcnt::W`](W) writer structure"]
impl crate::Writable for FsSclHcntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets fs_scl_hcnt to value 0"]
impl crate::Resettable for FsSclHcntSpec {
    const RESET_VALUE: u32 = 0;
}
