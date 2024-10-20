#[doc = "Register `ecc_ctrl` reader"]
pub type R = crate::R<EccCtrlSpec>;
#[doc = "Register `ecc_ctrl` writer"]
pub type W = crate::W<EccCtrlSpec>;
#[doc = "Field `mtxee` reader - MTL ECC MTX EE"]
pub type MtxeeR = crate::BitReader;
#[doc = "Field `mtxee` writer - MTL ECC MTX EE"]
pub type MtxeeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `mrxee` reader - MTL ECC MRX EE"]
pub type MrxeeR = crate::BitReader;
#[doc = "Field `mrxee` writer - MTL ECC MRX EE"]
pub type MrxeeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `mestee` reader - MTL ECC MEST EE"]
pub type MesteeR = crate::BitReader;
#[doc = "Field `mestee` writer - MTL ECC MEST EE"]
pub type MesteeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `mrxpee` reader - MTL ECC MRXP EE"]
pub type MrxpeeR = crate::BitReader;
#[doc = "Field `mrxpee` writer - MTL ECC MRXP EE"]
pub type MrxpeeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tsoee` reader - MTL ECC TSO EE"]
pub type TsoeeR = crate::BitReader;
#[doc = "Field `tsoee` writer - MTL ECC TSO EE"]
pub type TsoeeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `meeao` reader - MTL ECC MEE AO"]
pub type MeeaoR = crate::BitReader;
#[doc = "Field `meeao` writer - MTL ECC MEE AO"]
pub type MeeaoW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - MTL ECC MTX EE"]
    #[inline(always)]
    pub fn mtxee(&self) -> MtxeeR {
        MtxeeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MTL ECC MRX EE"]
    #[inline(always)]
    pub fn mrxee(&self) -> MrxeeR {
        MrxeeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - MTL ECC MEST EE"]
    #[inline(always)]
    pub fn mestee(&self) -> MesteeR {
        MesteeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - MTL ECC MRXP EE"]
    #[inline(always)]
    pub fn mrxpee(&self) -> MrxpeeR {
        MrxpeeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - MTL ECC TSO EE"]
    #[inline(always)]
    pub fn tsoee(&self) -> TsoeeR {
        TsoeeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - MTL ECC MEE AO"]
    #[inline(always)]
    pub fn meeao(&self) -> MeeaoR {
        MeeaoR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MTL ECC MTX EE"]
    #[inline(always)]
    #[must_use]
    pub fn mtxee(&mut self) -> MtxeeW<EccCtrlSpec> {
        MtxeeW::new(self, 0)
    }
    #[doc = "Bit 1 - MTL ECC MRX EE"]
    #[inline(always)]
    #[must_use]
    pub fn mrxee(&mut self) -> MrxeeW<EccCtrlSpec> {
        MrxeeW::new(self, 1)
    }
    #[doc = "Bit 2 - MTL ECC MEST EE"]
    #[inline(always)]
    #[must_use]
    pub fn mestee(&mut self) -> MesteeW<EccCtrlSpec> {
        MesteeW::new(self, 2)
    }
    #[doc = "Bit 3 - MTL ECC MRXP EE"]
    #[inline(always)]
    #[must_use]
    pub fn mrxpee(&mut self) -> MrxpeeW<EccCtrlSpec> {
        MrxpeeW::new(self, 3)
    }
    #[doc = "Bit 4 - MTL ECC TSO EE"]
    #[inline(always)]
    #[must_use]
    pub fn tsoee(&mut self) -> TsoeeW<EccCtrlSpec> {
        TsoeeW::new(self, 4)
    }
    #[doc = "Bit 8 - MTL ECC MEE AO"]
    #[inline(always)]
    #[must_use]
    pub fn meeao(&mut self) -> MeeaoW<EccCtrlSpec> {
        MeeaoW::new(self, 8)
    }
}
#[doc = "MTL ECC Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ecc_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ecc_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EccCtrlSpec;
impl crate::RegisterSpec for EccCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ecc_ctrl::R`](R) reader structure"]
impl crate::Readable for EccCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ecc_ctrl::W`](W) writer structure"]
impl crate::Writable for EccCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ecc_ctrl to value 0"]
impl crate::Resettable for EccCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
