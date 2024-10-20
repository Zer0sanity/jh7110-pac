#[doc = "Register `ane_exp` reader"]
pub type R = crate::R<AneExpSpec>;
#[doc = "Register `ane_exp` writer"]
pub type W = crate::W<AneExpSpec>;
#[doc = "Field `ane_exp` reader - Auto-Negotiation Extend Expansion"]
pub type AneExpR = crate::FieldReader<u32>;
#[doc = "Field `ane_exp` writer - Auto-Negotiation Extend Expansion"]
pub type AneExpW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Auto-Negotiation Extend Expansion"]
    #[inline(always)]
    pub fn ane_exp(&self) -> AneExpR {
        AneExpR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Auto-Negotiation Extend Expansion"]
    #[inline(always)]
    #[must_use]
    pub fn ane_exp(&mut self) -> AneExpW<AneExpSpec> {
        AneExpW::new(self, 0)
    }
}
#[doc = "Auto-Negotiation Extend Expansion\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ane_exp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ane_exp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AneExpSpec;
impl crate::RegisterSpec for AneExpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ane_exp::R`](R) reader structure"]
impl crate::Readable for AneExpSpec {}
#[doc = "`write(|w| ..)` method takes [`ane_exp::W`](W) writer structure"]
impl crate::Writable for AneExpSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ane_exp to value 0"]
impl crate::Resettable for AneExpSpec {
    const RESET_VALUE: u32 = 0;
}
