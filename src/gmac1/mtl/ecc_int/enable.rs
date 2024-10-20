#[doc = "Register `enable` reader"]
pub type R = crate::R<EnableSpec>;
#[doc = "Register `enable` writer"]
pub type W = crate::W<EnableSpec>;
#[doc = "Field `txceie` reader - MTL ECC TXCE Interrupt Enable"]
pub type TxceieR = crate::BitReader;
#[doc = "Field `txceie` writer - MTL ECC TXCE Interrupt Enable"]
pub type TxceieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rxceie` reader - MTL ECC RXCE Interrupt Enable"]
pub type RxceieR = crate::BitReader;
#[doc = "Field `rxceie` writer - MTL ECC RXCE Interrupt Enable"]
pub type RxceieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `eceie` reader - MTL ECC ECE Interrupt Enable"]
pub type EceieR = crate::BitReader;
#[doc = "Field `eceie` writer - MTL ECC ECE Interrupt Enable"]
pub type EceieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rpceie` reader - MTL ECC RPCE Interrupt Enable"]
pub type RpceieR = crate::BitReader;
#[doc = "Field `rpceie` writer - MTL ECC RPCE Interrupt Enable"]
pub type RpceieW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - MTL ECC TXCE Interrupt Enable"]
    #[inline(always)]
    pub fn txceie(&self) -> TxceieR {
        TxceieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - MTL ECC RXCE Interrupt Enable"]
    #[inline(always)]
    pub fn rxceie(&self) -> RxceieR {
        RxceieR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - MTL ECC ECE Interrupt Enable"]
    #[inline(always)]
    pub fn eceie(&self) -> EceieR {
        EceieR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - MTL ECC RPCE Interrupt Enable"]
    #[inline(always)]
    pub fn rpceie(&self) -> RpceieR {
        RpceieR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MTL ECC TXCE Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn txceie(&mut self) -> TxceieW<EnableSpec> {
        TxceieW::new(self, 0)
    }
    #[doc = "Bit 4 - MTL ECC RXCE Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxceie(&mut self) -> RxceieW<EnableSpec> {
        RxceieW::new(self, 4)
    }
    #[doc = "Bit 8 - MTL ECC ECE Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn eceie(&mut self) -> EceieW<EnableSpec> {
        EceieW::new(self, 8)
    }
    #[doc = "Bit 12 - MTL ECC RPCE Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rpceie(&mut self) -> RpceieW<EnableSpec> {
        RpceieW::new(self, 12)
    }
}
#[doc = "MTL ECC Interrupt Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`enable::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enable::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EnableSpec;
impl crate::RegisterSpec for EnableSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`enable::R`](R) reader structure"]
impl crate::Readable for EnableSpec {}
#[doc = "`write(|w| ..)` method takes [`enable::W`](W) writer structure"]
impl crate::Writable for EnableSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets enable to value 0"]
impl crate::Resettable for EnableSpec {
    const RESET_VALUE: u32 = 0;
}
