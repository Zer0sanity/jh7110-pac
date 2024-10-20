#[doc = "Register `tx_tl` reader"]
pub type R = crate::R<TxTlSpec>;
#[doc = "Register `tx_tl` writer"]
pub type W = crate::W<TxTlSpec>;
#[doc = "Field `tx_tl` reader - "]
pub type TxTlR = crate::FieldReader<u32>;
#[doc = "Field `tx_tl` writer - "]
pub type TxTlW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn tx_tl(&self) -> TxTlR {
        TxTlR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn tx_tl(&mut self) -> TxTlW<TxTlSpec> {
        TxTlW::new(self, 0)
    }
}
#[doc = "DesignWare I2C TX TL\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_tl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_tl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxTlSpec;
impl crate::RegisterSpec for TxTlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_tl::R`](R) reader structure"]
impl crate::Readable for TxTlSpec {}
#[doc = "`write(|w| ..)` method takes [`tx_tl::W`](W) writer structure"]
impl crate::Writable for TxTlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets tx_tl to value 0"]
impl crate::Resettable for TxTlSpec {
    const RESET_VALUE: u32 = 0;
}
