#[doc = "Register `clken` reader"]
pub type R = crate::R<ClkenSpec>;
#[doc = "Register `clken` writer"]
pub type W = crate::W<ClkenSpec>;
#[doc = "Field `enable` reader - MMC Clock Enable"]
pub type EnableR = crate::BitReader;
#[doc = "Field `enable` writer - MMC Clock Enable"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `low_pwr` reader - MMC Clock Enable Low Power"]
pub type LowPwrR = crate::BitReader;
#[doc = "Field `low_pwr` writer - MMC Clock Enable Low Power"]
pub type LowPwrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - MMC Clock Enable"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 16 - MMC Clock Enable Low Power"]
    #[inline(always)]
    pub fn low_pwr(&self) -> LowPwrR {
        LowPwrR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MMC Clock Enable"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> EnableW<ClkenSpec> {
        EnableW::new(self, 0)
    }
    #[doc = "Bit 16 - MMC Clock Enable Low Power"]
    #[inline(always)]
    #[must_use]
    pub fn low_pwr(&mut self) -> LowPwrW<ClkenSpec> {
        LowPwrW::new(self, 16)
    }
}
#[doc = "MMC Clock Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clken::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clken::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkenSpec;
impl crate::RegisterSpec for ClkenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clken::R`](R) reader structure"]
impl crate::Readable for ClkenSpec {}
#[doc = "`write(|w| ..)` method takes [`clken::W`](W) writer structure"]
impl crate::Writable for ClkenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets clken to value 0"]
impl crate::Resettable for ClkenSpec {
    const RESET_VALUE: u32 = 0;
}
