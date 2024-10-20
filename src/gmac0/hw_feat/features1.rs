#[doc = "Register `features1` reader"]
pub type R = crate::R<Features1Spec>;
#[doc = "Register `features1` writer"]
pub type W = crate::W<Features1Spec>;
#[doc = "Field `rxfifosize` reader - RX FIFO Size"]
pub type RxfifosizeR = crate::FieldReader;
#[doc = "Field `rxfifosize` writer - RX FIFO Size"]
pub type RxfifosizeW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `txfifosize` reader - TX FIFO Size"]
pub type TxfifosizeR = crate::FieldReader;
#[doc = "Field `txfifosize` writer - TX FIFO Size"]
pub type TxfifosizeW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `addr64` reader - Address 64"]
pub type Addr64R = crate::FieldReader;
#[doc = "Field `addr64` writer - Address 64"]
pub type Addr64W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `sphen` reader - SPH Enable"]
pub type SphenR = crate::BitReader;
#[doc = "Field `sphen` writer - SPH Enable"]
pub type SphenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tsoen` reader - TSO Enable"]
pub type TsoenR = crate::BitReader;
#[doc = "Field `tsoen` writer - TSO Enable"]
pub type TsoenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `avsel` reader - AV Select"]
pub type AvselR = crate::BitReader;
#[doc = "Field `avsel` writer - AV Select"]
pub type AvselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `hash_tb_sz` reader - Hash Table Size"]
pub type HashTbSzR = crate::FieldReader;
#[doc = "Field `hash_tb_sz` writer - Hash Table Size"]
pub type HashTbSzW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `l3l4fnum` reader - L3 L4 FNUM"]
pub type L3l4fnumR = crate::FieldReader;
#[doc = "Field `l3l4fnum` writer - L3 L4 FNUM"]
pub type L3l4fnumW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:4 - RX FIFO Size"]
    #[inline(always)]
    pub fn rxfifosize(&self) -> RxfifosizeR {
        RxfifosizeR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 6:10 - TX FIFO Size"]
    #[inline(always)]
    pub fn txfifosize(&self) -> TxfifosizeR {
        TxfifosizeR::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bits 14:15 - Address 64"]
    #[inline(always)]
    pub fn addr64(&self) -> Addr64R {
        Addr64R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 17 - SPH Enable"]
    #[inline(always)]
    pub fn sphen(&self) -> SphenR {
        SphenR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TSO Enable"]
    #[inline(always)]
    pub fn tsoen(&self) -> TsoenR {
        TsoenR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - AV Select"]
    #[inline(always)]
    pub fn avsel(&self) -> AvselR {
        AvselR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 24:25 - Hash Table Size"]
    #[inline(always)]
    pub fn hash_tb_sz(&self) -> HashTbSzR {
        HashTbSzR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 27:30 - L3 L4 FNUM"]
    #[inline(always)]
    pub fn l3l4fnum(&self) -> L3l4fnumR {
        L3l4fnumR::new(((self.bits >> 27) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - RX FIFO Size"]
    #[inline(always)]
    #[must_use]
    pub fn rxfifosize(&mut self) -> RxfifosizeW<Features1Spec> {
        RxfifosizeW::new(self, 0)
    }
    #[doc = "Bits 6:10 - TX FIFO Size"]
    #[inline(always)]
    #[must_use]
    pub fn txfifosize(&mut self) -> TxfifosizeW<Features1Spec> {
        TxfifosizeW::new(self, 6)
    }
    #[doc = "Bits 14:15 - Address 64"]
    #[inline(always)]
    #[must_use]
    pub fn addr64(&mut self) -> Addr64W<Features1Spec> {
        Addr64W::new(self, 14)
    }
    #[doc = "Bit 17 - SPH Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sphen(&mut self) -> SphenW<Features1Spec> {
        SphenW::new(self, 17)
    }
    #[doc = "Bit 18 - TSO Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tsoen(&mut self) -> TsoenW<Features1Spec> {
        TsoenW::new(self, 18)
    }
    #[doc = "Bit 20 - AV Select"]
    #[inline(always)]
    #[must_use]
    pub fn avsel(&mut self) -> AvselW<Features1Spec> {
        AvselW::new(self, 20)
    }
    #[doc = "Bits 24:25 - Hash Table Size"]
    #[inline(always)]
    #[must_use]
    pub fn hash_tb_sz(&mut self) -> HashTbSzW<Features1Spec> {
        HashTbSzW::new(self, 24)
    }
    #[doc = "Bits 27:30 - L3 L4 FNUM"]
    #[inline(always)]
    #[must_use]
    pub fn l3l4fnum(&mut self) -> L3l4fnumW<Features1Spec> {
        L3l4fnumW::new(self, 27)
    }
}
#[doc = "Hardware Features 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`features1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`features1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Features1Spec;
impl crate::RegisterSpec for Features1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`features1::R`](R) reader structure"]
impl crate::Readable for Features1Spec {}
#[doc = "`write(|w| ..)` method takes [`features1::W`](W) writer structure"]
impl crate::Writable for Features1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets features1 to value 0"]
impl crate::Resettable for Features1Spec {
    const RESET_VALUE: u32 = 0;
}
