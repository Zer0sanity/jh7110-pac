#[doc = "Register `pwren` reader"]
pub type R = crate::R<PwrenSpec>;
#[doc = "Register `pwren` writer"]
pub type W = crate::W<PwrenSpec>;
#[doc = "Field `pwren` reader - MMC Power Enable"]
pub type PwrenR = crate::FieldReader<u32>;
#[doc = "Field `pwren` writer - MMC Power Enable"]
pub type PwrenW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - MMC Power Enable"]
    #[inline(always)]
    pub fn pwren(&self) -> PwrenR {
        PwrenR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - MMC Power Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pwren(&mut self) -> PwrenW<PwrenSpec> {
        PwrenW::new(self, 0)
    }
}
#[doc = "MMC Power Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pwren::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pwren::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwrenSpec;
impl crate::RegisterSpec for PwrenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwren::R`](R) reader structure"]
impl crate::Readable for PwrenSpec {}
#[doc = "`write(|w| ..)` method takes [`pwren::W`](W) writer structure"]
impl crate::Writable for PwrenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets pwren to value 0"]
impl crate::Resettable for PwrenSpec {
    const RESET_VALUE: u32 = 0;
}