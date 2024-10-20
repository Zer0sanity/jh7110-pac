#[doc = "Register `verid` reader"]
pub type R = crate::R<VeridSpec>;
#[doc = "Register `verid` writer"]
pub type W = crate::W<VeridSpec>;
#[doc = "Field `verid` reader - MMC version ID"]
pub type VeridR = crate::FieldReader<u16>;
#[doc = "Field `verid` writer - MMC version ID"]
pub type VeridW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - MMC version ID"]
    #[inline(always)]
    pub fn verid(&self) -> VeridR {
        VeridR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - MMC version ID"]
    #[inline(always)]
    #[must_use]
    pub fn verid(&mut self) -> VeridW<VeridSpec> {
        VeridW::new(self, 0)
    }
}
#[doc = "MMC version ID\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`verid::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`verid::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VeridSpec;
impl crate::RegisterSpec for VeridSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`verid::R`](R) reader structure"]
impl crate::Readable for VeridSpec {}
#[doc = "`write(|w| ..)` method takes [`verid::W`](W) writer structure"]
impl crate::Writable for VeridSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets verid to value 0"]
impl crate::Resettable for VeridSpec {
    const RESET_VALUE: u32 = 0;
}
