#[doc = "Register `enable` reader"]
pub type R = crate::R<EnableSpec>;
#[doc = "Register `enable` writer"]
pub type W = crate::W<EnableSpec>;
#[doc = "Field `abort` reader - "]
pub type AbortR = crate::BitReader;
#[doc = "Field `abort` writer - "]
pub type AbortW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn abort(&self) -> AbortR {
        AbortR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn abort(&mut self) -> AbortW<EnableSpec> {
        AbortW::new(self, 1)
    }
}
#[doc = "DesignWare I2C Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`enable::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enable::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EnableSpec;
impl crate::RegisterSpec for EnableSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`enable::R`](R) reader structure"]
impl crate::Readable for EnableSpec {}
#[doc = "`write(|w| ..)` method takes [`enable::W`](W) writer structure"]
impl crate::Writable for EnableSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets enable to value 0"]
impl crate::Resettable for EnableSpec {
    const RESET_VALUE: u32 = 0;
}
