#[doc = "Register `ie` reader"]
pub type R = crate::R<IeSpec>;
#[doc = "Register `ie` writer"]
pub type W = crate::W<IeSpec>;
#[doc = "Field `rand_rdy_en` reader - RAND Ready Enable"]
pub type RandRdyEnR = crate::BitReader;
#[doc = "Field `rand_rdy_en` writer - RAND Ready Enable"]
pub type RandRdyEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `seed_done_en` reader - Seed Done Enable"]
pub type SeedDoneEnR = crate::BitReader;
#[doc = "Field `seed_done_en` writer - Seed Done Enable"]
pub type SeedDoneEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `lfsr_lockup_en` reader - LFSR Lockup Enable"]
pub type LfsrLockupEnR = crate::BitReader;
#[doc = "Field `lfsr_lockup_en` writer - LFSR Lockup Enable"]
pub type LfsrLockupEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `glbl_en` reader - Global Enable"]
pub type GlblEnR = crate::BitReader;
#[doc = "Field `glbl_en` writer - Global Enable"]
pub type GlblEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - RAND Ready Enable"]
    #[inline(always)]
    pub fn rand_rdy_en(&self) -> RandRdyEnR {
        RandRdyEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Seed Done Enable"]
    #[inline(always)]
    pub fn seed_done_en(&self) -> SeedDoneEnR {
        SeedDoneEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - LFSR Lockup Enable"]
    #[inline(always)]
    pub fn lfsr_lockup_en(&self) -> LfsrLockupEnR {
        LfsrLockupEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 31 - Global Enable"]
    #[inline(always)]
    pub fn glbl_en(&self) -> GlblEnR {
        GlblEnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RAND Ready Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rand_rdy_en(&mut self) -> RandRdyEnW<IeSpec> {
        RandRdyEnW::new(self, 0)
    }
    #[doc = "Bit 1 - Seed Done Enable"]
    #[inline(always)]
    #[must_use]
    pub fn seed_done_en(&mut self) -> SeedDoneEnW<IeSpec> {
        SeedDoneEnW::new(self, 1)
    }
    #[doc = "Bit 4 - LFSR Lockup Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lfsr_lockup_en(&mut self) -> LfsrLockupEnW<IeSpec> {
        LfsrLockupEnW::new(self, 4)
    }
    #[doc = "Bit 31 - Global Enable"]
    #[inline(always)]
    #[must_use]
    pub fn glbl_en(&mut self) -> GlblEnW<IeSpec> {
        GlblEnW::new(self, 31)
    }
}
#[doc = "TRNG Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ie::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ie::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IeSpec;
impl crate::RegisterSpec for IeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ie::R`](R) reader structure"]
impl crate::Readable for IeSpec {}
#[doc = "`write(|w| ..)` method takes [`ie::W`](W) writer structure"]
impl crate::Writable for IeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ie to value 0"]
impl crate::Resettable for IeSpec {
    const RESET_VALUE: u32 = 0;
}
