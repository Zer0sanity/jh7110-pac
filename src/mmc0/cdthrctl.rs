#[doc = "Register `cdthrctl` reader"]
pub type R = crate::R<CdthrctlSpec>;
#[doc = "Register `cdthrctl` writer"]
pub type W = crate::W<CdthrctlSpec>;
#[doc = "Field `rd_thr_en` reader - MMC card detect read threshold enable"]
pub type RdThrEnR = crate::BitReader;
#[doc = "Field `rd_thr_en` writer - MMC card detect read threshold enable"]
pub type RdThrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wr_thr_en` reader - MMC card detect write threshold enable"]
pub type WrThrEnR = crate::BitReader;
#[doc = "Field `wr_thr_en` writer - MMC card detect write threshold enable"]
pub type WrThrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `thld` reader - MMC card detect threshold"]
pub type ThldR = crate::FieldReader<u16>;
#[doc = "Field `thld` writer - MMC card detect threshold"]
pub type ThldW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bit 0 - MMC card detect read threshold enable"]
    #[inline(always)]
    pub fn rd_thr_en(&self) -> RdThrEnR {
        RdThrEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - MMC card detect write threshold enable"]
    #[inline(always)]
    pub fn wr_thr_en(&self) -> WrThrEnR {
        WrThrEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 16:27 - MMC card detect threshold"]
    #[inline(always)]
    pub fn thld(&self) -> ThldR {
        ThldR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - MMC card detect read threshold enable"]
    #[inline(always)]
    #[must_use]
    pub fn rd_thr_en(&mut self) -> RdThrEnW<CdthrctlSpec> {
        RdThrEnW::new(self, 0)
    }
    #[doc = "Bit 2 - MMC card detect write threshold enable"]
    #[inline(always)]
    #[must_use]
    pub fn wr_thr_en(&mut self) -> WrThrEnW<CdthrctlSpec> {
        WrThrEnW::new(self, 2)
    }
    #[doc = "Bits 16:27 - MMC card detect threshold"]
    #[inline(always)]
    #[must_use]
    pub fn thld(&mut self) -> ThldW<CdthrctlSpec> {
        ThldW::new(self, 16)
    }
}
#[doc = "MMC card detect threshold control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cdthrctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cdthrctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CdthrctlSpec;
impl crate::RegisterSpec for CdthrctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cdthrctl::R`](R) reader structure"]
impl crate::Readable for CdthrctlSpec {}
#[doc = "`write(|w| ..)` method takes [`cdthrctl::W`](W) writer structure"]
impl crate::Writable for CdthrctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets cdthrctl to value 0"]
impl crate::Resettable for CdthrctlSpec {
    const RESET_VALUE: u32 = 0;
}
