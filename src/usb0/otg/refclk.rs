#[doc = "Register `refclk` reader"]
pub type R = crate::R<RefclkSpec>;
#[doc = "Register `refclk` writer"]
pub type W = crate::W<RefclkSpec>;
#[doc = "Field `refclk` reader - USB3 OTG reference clock."]
pub type RefclkR = crate::FieldReader<u32>;
#[doc = "Field `refclk` writer - USB3 OTG reference clock."]
pub type RefclkW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - USB3 OTG reference clock."]
    #[inline(always)]
    pub fn refclk(&self) -> RefclkR {
        RefclkR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - USB3 OTG reference clock."]
    #[inline(always)]
    #[must_use]
    pub fn refclk(&mut self) -> RefclkW<RefclkSpec> {
        RefclkW::new(self, 0)
    }
}
#[doc = "USB3 OTG reference clock.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`refclk::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`refclk::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RefclkSpec;
impl crate::RegisterSpec for RefclkSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`refclk::R`](R) reader structure"]
impl crate::Readable for RefclkSpec {}
#[doc = "`write(|w| ..)` method takes [`refclk::W`](W) writer structure"]
impl crate::Writable for RefclkSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets refclk to value 0"]
impl crate::Resettable for RefclkSpec {
    const RESET_VALUE: u32 = 0;
}
