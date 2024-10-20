#[doc = "Register `dpp_ctrl` reader"]
pub type R = crate::R<DppCtrlSpec>;
#[doc = "Register `dpp_ctrl` writer"]
pub type W = crate::W<DppCtrlSpec>;
#[doc = "Field `edpp` reader - MTL DPP EDPP"]
pub type EdppR = crate::BitReader;
#[doc = "Field `edpp` writer - MTL DPP EDPP"]
pub type EdppW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ope` reader - MTL DPP OPE"]
pub type OpeR = crate::BitReader;
#[doc = "Field `ope` writer - MTL DPP OPE"]
pub type OpeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `epsi` reader - MTL DPP EPSI"]
pub type EpsiR = crate::BitReader;
#[doc = "Field `epsi` writer - MTL DPP EPSI"]
pub type EpsiW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - MTL DPP EDPP"]
    #[inline(always)]
    pub fn edpp(&self) -> EdppR {
        EdppR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MTL DPP OPE"]
    #[inline(always)]
    pub fn ope(&self) -> OpeR {
        OpeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - MTL DPP EPSI"]
    #[inline(always)]
    pub fn epsi(&self) -> EpsiR {
        EpsiR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MTL DPP EDPP"]
    #[inline(always)]
    #[must_use]
    pub fn edpp(&mut self) -> EdppW<DppCtrlSpec> {
        EdppW::new(self, 0)
    }
    #[doc = "Bit 1 - MTL DPP OPE"]
    #[inline(always)]
    #[must_use]
    pub fn ope(&mut self) -> OpeW<DppCtrlSpec> {
        OpeW::new(self, 1)
    }
    #[doc = "Bit 2 - MTL DPP EPSI"]
    #[inline(always)]
    #[must_use]
    pub fn epsi(&mut self) -> EpsiW<DppCtrlSpec> {
        EpsiW::new(self, 2)
    }
}
#[doc = "MTL DPP Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dpp_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dpp_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DppCtrlSpec;
impl crate::RegisterSpec for DppCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dpp_ctrl::R`](R) reader structure"]
impl crate::Readable for DppCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`dpp_ctrl::W`](W) writer structure"]
impl crate::Writable for DppCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets dpp_ctrl to value 0"]
impl crate::Resettable for DppCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
