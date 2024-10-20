#[doc = "Register `wsr` reader"]
pub type R = crate::R<WsrSpec>;
#[doc = "Register `wsr` writer"]
pub type W = crate::W<WsrSpec>;
#[doc = "Field `wsr` reader - SHA WSR"]
pub type WsrR = crate::FieldReader<u32>;
#[doc = "Field `wsr` writer - SHA WSR"]
pub type WsrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SHA WSR"]
    #[inline(always)]
    pub fn wsr(&self) -> WsrR {
        WsrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SHA WSR"]
    #[inline(always)]
    #[must_use]
    pub fn wsr(&mut self) -> WsrW<WsrSpec> {
        WsrW::new(self, 0)
    }
}
#[doc = "JH7110 Crypto SHA WSR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WsrSpec;
impl crate::RegisterSpec for WsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wsr::R`](R) reader structure"]
impl crate::Readable for WsrSpec {}
#[doc = "`write(|w| ..)` method takes [`wsr::W`](W) writer structure"]
impl crate::Writable for WsrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets wsr to value 0"]
impl crate::Resettable for WsrSpec {
    const RESET_VALUE: u32 = 0;
}
