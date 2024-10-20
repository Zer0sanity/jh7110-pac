#[doc = "Register `int_ctrl` reader"]
pub type R = crate::R<IntCtrlSpec>;
#[doc = "Register `int_ctrl` writer"]
pub type W = crate::W<IntCtrlSpec>;
#[doc = "Field `rx_overflow_int` reader - MTL RX Overflow Interrupt"]
pub type RxOverflowIntR = crate::BitReader;
#[doc = "Field `rx_overflow_int` writer - MTL RX Overflow Interrupt"]
pub type RxOverflowIntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rx_overflow_int_en` reader - MTL RX Overflow Interrupt Enable"]
pub type RxOverflowIntEnR = crate::BitReader;
#[doc = "Field `rx_overflow_int_en` writer - MTL RX Overflow Interrupt Enable"]
pub type RxOverflowIntEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 16 - MTL RX Overflow Interrupt"]
    #[inline(always)]
    pub fn rx_overflow_int(&self) -> RxOverflowIntR {
        RxOverflowIntR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - MTL RX Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn rx_overflow_int_en(&self) -> RxOverflowIntEnR {
        RxOverflowIntEnR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - MTL RX Overflow Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn rx_overflow_int(&mut self) -> RxOverflowIntW<IntCtrlSpec> {
        RxOverflowIntW::new(self, 16)
    }
    #[doc = "Bit 24 - MTL RX Overflow Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rx_overflow_int_en(&mut self) -> RxOverflowIntEnW<IntCtrlSpec> {
        RxOverflowIntEnW::new(self, 24)
    }
}
#[doc = "MTL Channel Interrupt Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntCtrlSpec;
impl crate::RegisterSpec for IntCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_ctrl::R`](R) reader structure"]
impl crate::Readable for IntCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`int_ctrl::W`](W) writer structure"]
impl crate::Writable for IntCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets int_ctrl to value 0"]
impl crate::Resettable for IntCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
