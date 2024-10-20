#[doc = "Register `pcmgbcr` reader"]
pub type R = crate::R<PcmgbcrSpec>;
#[doc = "Register `pcmgbcr` writer"]
pub type W = crate::W<PcmgbcrSpec>;
#[doc = "Field `enable` reader - PCM GB enable"]
pub type EnableR = crate::BitReader;
#[doc = "Field `enable` writer - PCM GB enable"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ms` reader - PCM GB master-slave mode - 0: master, 1: slave"]
pub type MsR = crate::BitReader;
#[doc = "Field `ms` writer - PCM GB master-slave mode - 0: master, 1: slave"]
pub type MsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `syncm` reader - PCM GB sync mode - 0: short, 1: long."]
pub type SyncmR = crate::BitReader;
#[doc = "Field `syncm` writer - PCM GB sync mode - 0: short, 1: long."]
pub type SyncmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `elm` reader - PCM GB early-late mode - 0: late, 1: early. Only works while syncm is 0."]
pub type ElmR = crate::BitReader;
#[doc = "Field `elm` writer - PCM GB early-late mode - 0: late, 1: early. Only works while syncm is 0."]
pub type ElmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `clkpol` reader - PCM GB clock polarity - 0: TX rising / RX falling, 1: TX falling / RX rising."]
pub type ClkpolR = crate::BitReader;
#[doc = "Field `clkpol` writer - PCM GB clock polarity - 0: TX rising / RX falling, 1: TX falling / RX rising."]
pub type ClkpolW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - PCM GB enable"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PCM GB master-slave mode - 0: master, 1: slave"]
    #[inline(always)]
    pub fn ms(&self) -> MsR {
        MsR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PCM GB sync mode - 0: short, 1: long."]
    #[inline(always)]
    pub fn syncm(&self) -> SyncmR {
        SyncmR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PCM GB early-late mode - 0: late, 1: early. Only works while syncm is 0."]
    #[inline(always)]
    pub fn elm(&self) -> ElmR {
        ElmR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - PCM GB clock polarity - 0: TX rising / RX falling, 1: TX falling / RX rising."]
    #[inline(always)]
    pub fn clkpol(&self) -> ClkpolR {
        ClkpolR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PCM GB enable"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> EnableW<PcmgbcrSpec> {
        EnableW::new(self, 0)
    }
    #[doc = "Bit 1 - PCM GB master-slave mode - 0: master, 1: slave"]
    #[inline(always)]
    #[must_use]
    pub fn ms(&mut self) -> MsW<PcmgbcrSpec> {
        MsW::new(self, 1)
    }
    #[doc = "Bit 2 - PCM GB sync mode - 0: short, 1: long."]
    #[inline(always)]
    #[must_use]
    pub fn syncm(&mut self) -> SyncmW<PcmgbcrSpec> {
        SyncmW::new(self, 2)
    }
    #[doc = "Bit 3 - PCM GB early-late mode - 0: late, 1: early. Only works while syncm is 0."]
    #[inline(always)]
    #[must_use]
    pub fn elm(&mut self) -> ElmW<PcmgbcrSpec> {
        ElmW::new(self, 3)
    }
    #[doc = "Bit 5 - PCM GB clock polarity - 0: TX rising / RX falling, 1: TX falling / RX rising."]
    #[inline(always)]
    #[must_use]
    pub fn clkpol(&mut self) -> ClkpolW<PcmgbcrSpec> {
        ClkpolW::new(self, 5)
    }
}
#[doc = "TDM PCM GB Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcmgbcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcmgbcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcmgbcrSpec;
impl crate::RegisterSpec for PcmgbcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcmgbcr::R`](R) reader structure"]
impl crate::Readable for PcmgbcrSpec {}
#[doc = "`write(|w| ..)` method takes [`pcmgbcr::W`](W) writer structure"]
impl crate::Writable for PcmgbcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets pcmgbcr to value 0"]
impl crate::Resettable for PcmgbcrSpec {
    const RESET_VALUE: u32 = 0;
}
