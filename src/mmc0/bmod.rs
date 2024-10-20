#[doc = "Register `bmod` reader"]
pub type R = crate::R<BmodSpec>;
#[doc = "Register `bmod` writer"]
pub type W = crate::W<BmodSpec>;
#[doc = "Field `swreset` reader - MMC internal DMAC software reset"]
pub type SwresetR = crate::BitReader;
#[doc = "Field `swreset` writer - MMC internal DMAC software reset"]
pub type SwresetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `fb` reader - MMC internal DMAC FB"]
pub type FbR = crate::BitReader;
#[doc = "Field `fb` writer - MMC internal DMAC FB"]
pub type FbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `enable` reader - MMC internal DMAC enable"]
pub type EnableR = crate::BitReader;
#[doc = "Field `enable` writer - MMC internal DMAC enable"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - MMC internal DMAC software reset"]
    #[inline(always)]
    pub fn swreset(&self) -> SwresetR {
        SwresetR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MMC internal DMAC FB"]
    #[inline(always)]
    pub fn fb(&self) -> FbR {
        FbR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 7 - MMC internal DMAC enable"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MMC internal DMAC software reset"]
    #[inline(always)]
    #[must_use]
    pub fn swreset(&mut self) -> SwresetW<BmodSpec> {
        SwresetW::new(self, 0)
    }
    #[doc = "Bit 1 - MMC internal DMAC FB"]
    #[inline(always)]
    #[must_use]
    pub fn fb(&mut self) -> FbW<BmodSpec> {
        FbW::new(self, 1)
    }
    #[doc = "Bit 7 - MMC internal DMAC enable"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> EnableW<BmodSpec> {
        EnableW::new(self, 7)
    }
}
#[doc = "MMC DMAC bus mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bmod::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bmod::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BmodSpec;
impl crate::RegisterSpec for BmodSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bmod::R`](R) reader structure"]
impl crate::Readable for BmodSpec {}
#[doc = "`write(|w| ..)` method takes [`bmod::W`](W) writer structure"]
impl crate::Writable for BmodSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets bmod to value 0"]
impl crate::Resettable for BmodSpec {
    const RESET_VALUE: u32 = 0;
}
