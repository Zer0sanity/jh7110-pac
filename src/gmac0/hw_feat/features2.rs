#[doc = "Register `features2` reader"]
pub type R = crate::R<Features2Spec>;
#[doc = "Register `features2` writer"]
pub type W = crate::W<Features2Spec>;
#[doc = "Field `rxqcnt` reader - RX Queue Count"]
pub type RxqcntR = crate::FieldReader;
#[doc = "Field `rxqcnt` writer - RX Queue Count"]
pub type RxqcntW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `txqcnt` reader - TX Queue Count"]
pub type TxqcntR = crate::FieldReader;
#[doc = "Field `txqcnt` writer - TX Queue Count"]
pub type TxqcntW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `rxchcnt` reader - RX Channel Count"]
pub type RxchcntR = crate::FieldReader;
#[doc = "Field `rxchcnt` writer - RX Channel Count"]
pub type RxchcntW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `txchcnt` reader - TX Channel Count"]
pub type TxchcntR = crate::FieldReader;
#[doc = "Field `txchcnt` writer - TX Channel Count"]
pub type TxchcntW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ppsoutnum` reader - PPS Out Number"]
pub type PpsoutnumR = crate::FieldReader;
#[doc = "Field `ppsoutnum` writer - PPS Out Number"]
pub type PpsoutnumW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `auxsnapnum` reader - AUX Snap Number"]
pub type AuxsnapnumR = crate::FieldReader;
#[doc = "Field `auxsnapnum` writer - AUX Snap Number"]
pub type AuxsnapnumW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:3 - RX Queue Count"]
    #[inline(always)]
    pub fn rxqcnt(&self) -> RxqcntR {
        RxqcntR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 6:9 - TX Queue Count"]
    #[inline(always)]
    pub fn txqcnt(&self) -> TxqcntR {
        TxqcntR::new(((self.bits >> 6) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - RX Channel Count"]
    #[inline(always)]
    pub fn rxchcnt(&self) -> RxchcntR {
        RxchcntR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 18:21 - TX Channel Count"]
    #[inline(always)]
    pub fn txchcnt(&self) -> TxchcntR {
        TxchcntR::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bits 24:26 - PPS Out Number"]
    #[inline(always)]
    pub fn ppsoutnum(&self) -> PpsoutnumR {
        PpsoutnumR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:30 - AUX Snap Number"]
    #[inline(always)]
    pub fn auxsnapnum(&self) -> AuxsnapnumR {
        AuxsnapnumR::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - RX Queue Count"]
    #[inline(always)]
    #[must_use]
    pub fn rxqcnt(&mut self) -> RxqcntW<Features2Spec> {
        RxqcntW::new(self, 0)
    }
    #[doc = "Bits 6:9 - TX Queue Count"]
    #[inline(always)]
    #[must_use]
    pub fn txqcnt(&mut self) -> TxqcntW<Features2Spec> {
        TxqcntW::new(self, 6)
    }
    #[doc = "Bits 12:15 - RX Channel Count"]
    #[inline(always)]
    #[must_use]
    pub fn rxchcnt(&mut self) -> RxchcntW<Features2Spec> {
        RxchcntW::new(self, 12)
    }
    #[doc = "Bits 18:21 - TX Channel Count"]
    #[inline(always)]
    #[must_use]
    pub fn txchcnt(&mut self) -> TxchcntW<Features2Spec> {
        TxchcntW::new(self, 18)
    }
    #[doc = "Bits 24:26 - PPS Out Number"]
    #[inline(always)]
    #[must_use]
    pub fn ppsoutnum(&mut self) -> PpsoutnumW<Features2Spec> {
        PpsoutnumW::new(self, 24)
    }
    #[doc = "Bits 28:30 - AUX Snap Number"]
    #[inline(always)]
    #[must_use]
    pub fn auxsnapnum(&mut self) -> AuxsnapnumW<Features2Spec> {
        AuxsnapnumW::new(self, 28)
    }
}
#[doc = "Hardware Features 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`features2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`features2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Features2Spec;
impl crate::RegisterSpec for Features2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`features2::R`](R) reader structure"]
impl crate::Readable for Features2Spec {}
#[doc = "`write(|w| ..)` method takes [`features2::W`](W) writer structure"]
impl crate::Writable for Features2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets features2 to value 0"]
impl crate::Resettable for Features2Spec {
    const RESET_VALUE: u32 = 0;
}
