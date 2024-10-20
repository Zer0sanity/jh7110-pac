#[doc = "Register `features3` reader"]
pub type R = crate::R<Features3Spec>;
#[doc = "Register `features3` writer"]
pub type W = crate::W<Features3Spec>;
#[doc = "Field `nrvf` reader - NRVF"]
pub type NrvfR = crate::FieldReader;
#[doc = "Field `nrvf` writer - NRVF"]
pub type NrvfW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `dvlan` reader - DVLAN"]
pub type DvlanR = crate::BitReader;
#[doc = "Field `dvlan` writer - DVLAN"]
pub type DvlanW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `frpsel` reader - FRP Select"]
pub type FrpselR = crate::BitReader;
#[doc = "Field `frpsel` writer - FRP Select"]
pub type FrpselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `frpbs` reader - FRP BS"]
pub type FrpbsR = crate::FieldReader;
#[doc = "Field `frpbs` writer - FRP BS"]
pub type FrpbsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `frpes` reader - FRP ES"]
pub type FrpesR = crate::FieldReader;
#[doc = "Field `frpes` writer - FRP ES"]
pub type FrpesW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `estsel` reader - EST Select"]
pub type EstselR = crate::BitReader;
#[doc = "Field `estsel` writer - EST Select"]
pub type EstselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `estdep` reader - EST DEP"]
pub type EstdepR = crate::FieldReader;
#[doc = "Field `estdep` writer - EST DEP"]
pub type EstdepW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `estwid` reader - EST WID"]
pub type EstwidR = crate::FieldReader;
#[doc = "Field `estwid` writer - EST WID"]
pub type EstwidW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `fpesel` reader - FPE Select"]
pub type FpeselR = crate::BitReader;
#[doc = "Field `fpesel` writer - FPE Select"]
pub type FpeselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tbssel` reader - TBS Select"]
pub type TbsselR = crate::BitReader;
#[doc = "Field `tbssel` writer - TBS Select"]
pub type TbsselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `asp` reader - ASP"]
pub type AspR = crate::FieldReader;
#[doc = "Field `asp` writer - ASP"]
pub type AspW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:2 - NRVF"]
    #[inline(always)]
    pub fn nrvf(&self) -> NrvfR {
        NrvfR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 5 - DVLAN"]
    #[inline(always)]
    pub fn dvlan(&self) -> DvlanR {
        DvlanR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 10 - FRP Select"]
    #[inline(always)]
    pub fn frpsel(&self) -> FrpselR {
        FrpselR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:12 - FRP BS"]
    #[inline(always)]
    pub fn frpbs(&self) -> FrpbsR {
        FrpbsR::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bits 13:14 - FRP ES"]
    #[inline(always)]
    pub fn frpes(&self) -> FrpesR {
        FrpesR::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 16 - EST Select"]
    #[inline(always)]
    pub fn estsel(&self) -> EstselR {
        EstselR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:19 - EST DEP"]
    #[inline(always)]
    pub fn estdep(&self) -> EstdepR {
        EstdepR::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bits 20:21 - EST WID"]
    #[inline(always)]
    pub fn estwid(&self) -> EstwidR {
        EstwidR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 26 - FPE Select"]
    #[inline(always)]
    pub fn fpesel(&self) -> FpeselR {
        FpeselR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - TBS Select"]
    #[inline(always)]
    pub fn tbssel(&self) -> TbsselR {
        TbsselR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:29 - ASP"]
    #[inline(always)]
    pub fn asp(&self) -> AspR {
        AspR::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - NRVF"]
    #[inline(always)]
    #[must_use]
    pub fn nrvf(&mut self) -> NrvfW<Features3Spec> {
        NrvfW::new(self, 0)
    }
    #[doc = "Bit 5 - DVLAN"]
    #[inline(always)]
    #[must_use]
    pub fn dvlan(&mut self) -> DvlanW<Features3Spec> {
        DvlanW::new(self, 5)
    }
    #[doc = "Bit 10 - FRP Select"]
    #[inline(always)]
    #[must_use]
    pub fn frpsel(&mut self) -> FrpselW<Features3Spec> {
        FrpselW::new(self, 10)
    }
    #[doc = "Bits 11:12 - FRP BS"]
    #[inline(always)]
    #[must_use]
    pub fn frpbs(&mut self) -> FrpbsW<Features3Spec> {
        FrpbsW::new(self, 11)
    }
    #[doc = "Bits 13:14 - FRP ES"]
    #[inline(always)]
    #[must_use]
    pub fn frpes(&mut self) -> FrpesW<Features3Spec> {
        FrpesW::new(self, 13)
    }
    #[doc = "Bit 16 - EST Select"]
    #[inline(always)]
    #[must_use]
    pub fn estsel(&mut self) -> EstselW<Features3Spec> {
        EstselW::new(self, 16)
    }
    #[doc = "Bits 17:19 - EST DEP"]
    #[inline(always)]
    #[must_use]
    pub fn estdep(&mut self) -> EstdepW<Features3Spec> {
        EstdepW::new(self, 17)
    }
    #[doc = "Bits 20:21 - EST WID"]
    #[inline(always)]
    #[must_use]
    pub fn estwid(&mut self) -> EstwidW<Features3Spec> {
        EstwidW::new(self, 20)
    }
    #[doc = "Bit 26 - FPE Select"]
    #[inline(always)]
    #[must_use]
    pub fn fpesel(&mut self) -> FpeselW<Features3Spec> {
        FpeselW::new(self, 26)
    }
    #[doc = "Bit 27 - TBS Select"]
    #[inline(always)]
    #[must_use]
    pub fn tbssel(&mut self) -> TbsselW<Features3Spec> {
        TbsselW::new(self, 27)
    }
    #[doc = "Bits 28:29 - ASP"]
    #[inline(always)]
    #[must_use]
    pub fn asp(&mut self) -> AspW<Features3Spec> {
        AspW::new(self, 28)
    }
}
#[doc = "Hardware Features 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`features3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`features3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Features3Spec;
impl crate::RegisterSpec for Features3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`features3::R`](R) reader structure"]
impl crate::Readable for Features3Spec {}
#[doc = "`write(|w| ..)` method takes [`features3::W`](W) writer structure"]
impl crate::Writable for Features3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets features3 to value 0"]
impl crate::Resettable for Features3Spec {
    const RESET_VALUE: u32 = 0;
}
