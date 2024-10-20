#[doc = "Register `an_status` reader"]
pub type R = crate::R<AnStatusSpec>;
#[doc = "Register `an_status` writer"]
pub type W = crate::W<AnStatusSpec>;
#[doc = "Field `ls` reader - Link Status - 0: down, 1: up"]
pub type LsR = crate::BitReader;
#[doc = "Field `ls` writer - Link Status - 0: down, 1: up"]
pub type LsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ana` reader - Auto-Negotiation Ability"]
pub type AnaR = crate::BitReader;
#[doc = "Field `ana` writer - Auto-Negotiation Ability"]
pub type AnaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `anc` reader - Auto-Negotiation Complete"]
pub type AncR = crate::BitReader;
#[doc = "Field `anc` writer - Auto-Negotiation Complete"]
pub type AncW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `es` reader - Extended Status"]
pub type EsR = crate::BitReader;
#[doc = "Field `es` writer - Extended Status"]
pub type EsW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - Link Status - 0: down, 1: up"]
    #[inline(always)]
    pub fn ls(&self) -> LsR {
        LsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Auto-Negotiation Ability"]
    #[inline(always)]
    pub fn ana(&self) -> AnaR {
        AnaR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Auto-Negotiation Complete"]
    #[inline(always)]
    pub fn anc(&self) -> AncR {
        AncR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Extended Status"]
    #[inline(always)]
    pub fn es(&self) -> EsR {
        EsR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Link Status - 0: down, 1: up"]
    #[inline(always)]
    #[must_use]
    pub fn ls(&mut self) -> LsW<AnStatusSpec> {
        LsW::new(self, 2)
    }
    #[doc = "Bit 3 - Auto-Negotiation Ability"]
    #[inline(always)]
    #[must_use]
    pub fn ana(&mut self) -> AnaW<AnStatusSpec> {
        AnaW::new(self, 3)
    }
    #[doc = "Bit 5 - Auto-Negotiation Complete"]
    #[inline(always)]
    #[must_use]
    pub fn anc(&mut self) -> AncW<AnStatusSpec> {
        AncW::new(self, 5)
    }
    #[doc = "Bit 8 - Extended Status"]
    #[inline(always)]
    #[must_use]
    pub fn es(&mut self) -> EsW<AnStatusSpec> {
        EsW::new(self, 8)
    }
}
#[doc = "Auto-Negotiation Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`an_status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`an_status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AnStatusSpec;
impl crate::RegisterSpec for AnStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`an_status::R`](R) reader structure"]
impl crate::Readable for AnStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`an_status::W`](W) writer structure"]
impl crate::Writable for AnStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets an_status to value 0"]
impl crate::Resettable for AnStatusSpec {
    const RESET_VALUE: u32 = 0;
}
