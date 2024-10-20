#[doc = "Register `lock` reader"]
pub type R = crate::R<LockSpec>;
#[doc = "Register `lock` writer"]
pub type W = crate::W<LockSpec>;
#[doc = "Field `lock` reader - Write magic values to enable/disable writes to other watchdog registers: 0x1acce551: enable writes / unlock, 0xe5331aae: disable writes / lock. Defaults to the 'lock' value."]
pub type LockR = crate::FieldReader<u32>;
#[doc = "Field `lock` writer - Write magic values to enable/disable writes to other watchdog registers: 0x1acce551: enable writes / unlock, 0xe5331aae: disable writes / lock. Defaults to the 'lock' value."]
pub type LockW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Write magic values to enable/disable writes to other watchdog registers: 0x1acce551: enable writes / unlock, 0xe5331aae: disable writes / lock. Defaults to the 'lock' value."]
    #[inline(always)]
    pub fn lock(&self) -> LockR {
        LockR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Write magic values to enable/disable writes to other watchdog registers: 0x1acce551: enable writes / unlock, 0xe5331aae: disable writes / lock. Defaults to the 'lock' value."]
    #[inline(always)]
    #[must_use]
    pub fn lock(&mut self) -> LockW<LockSpec> {
        LockW::new(self, 0)
    }
}
#[doc = "StarFive JH7110 Watchdog Lock register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lock::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lock::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LockSpec;
impl crate::RegisterSpec for LockSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lock::R`](R) reader structure"]
impl crate::Readable for LockSpec {}
#[doc = "`write(|w| ..)` method takes [`lock::W`](W) writer structure"]
impl crate::Writable for LockSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets lock to value 0xe533_1aae"]
impl crate::Resettable for LockSpec {
    const RESET_VALUE: u32 = 0xe533_1aae;
}
