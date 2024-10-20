#[doc = "Register `rx_tl` reader"]
pub type R = crate::R<RxTlSpec>;
#[doc = "Register `rx_tl` writer"]
pub type W = crate::W<RxTlSpec>;
#[doc = "Field `rx_tl` reader - "]
pub type RxTlR = crate::FieldReader<u32>;
#[doc = "Field `rx_tl` writer - "]
pub type RxTlW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn rx_tl(&self) -> RxTlR {
        RxTlR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn rx_tl(&mut self) -> RxTlW<RxTlSpec> {
        RxTlW::new(self, 0)
    }
}
#[doc = "DesignWare I2C RX TL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_tl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rx_tl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxTlSpec;
impl crate::RegisterSpec for RxTlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_tl::R`](R) reader structure"]
impl crate::Readable for RxTlSpec {}
#[doc = "`write(|w| ..)` method takes [`rx_tl::W`](W) writer structure"]
impl crate::Writable for RxTlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets rx_tl to value 0"]
impl crate::Resettable for RxTlSpec {
    const RESET_VALUE: u32 = 0;
}
