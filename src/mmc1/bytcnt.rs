#[doc = "Register `bytcnt` reader"]
pub type R = crate::R<BytcntSpec>;
#[doc = "Register `bytcnt` writer"]
pub type W = crate::W<BytcntSpec>;
#[doc = "Field `bytcnt` reader - MMC byte count"]
pub type BytcntR = crate::FieldReader<u32>;
#[doc = "Field `bytcnt` writer - MMC byte count"]
pub type BytcntW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - MMC byte count"]
    #[inline(always)]
    pub fn bytcnt(&self) -> BytcntR {
        BytcntR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - MMC byte count"]
    #[inline(always)]
    #[must_use]
    pub fn bytcnt(&mut self) -> BytcntW<BytcntSpec> {
        BytcntW::new(self, 0)
    }
}
#[doc = "MMC byte count\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bytcnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bytcnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BytcntSpec;
impl crate::RegisterSpec for BytcntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bytcnt::R`](R) reader structure"]
impl crate::Readable for BytcntSpec {}
#[doc = "`write(|w| ..)` method takes [`bytcnt::W`](W) writer structure"]
impl crate::Writable for BytcntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets bytcnt to value 0"]
impl crate::Resettable for BytcntSpec {
    const RESET_VALUE: u32 = 0;
}
