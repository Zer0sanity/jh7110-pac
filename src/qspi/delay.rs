#[doc = "Register `delay` reader"]
pub type R = crate::R<DelaySpec>;
#[doc = "Register `delay` writer"]
pub type W = crate::W<DelaySpec>;
#[doc = "Field `tslch` reader - TSLCH Delay Value"]
pub type TslchR = crate::FieldReader;
#[doc = "Field `tslch` writer - TSLCH Delay Value"]
pub type TslchW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `tchsh` reader - TCHSH Delay Value"]
pub type TchshR = crate::FieldReader;
#[doc = "Field `tchsh` writer - TCHSH Delay Value"]
pub type TchshW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `tsd2d` reader - TSD2D Delay Value"]
pub type Tsd2dR = crate::FieldReader;
#[doc = "Field `tsd2d` writer - TSD2D Delay Value"]
pub type Tsd2dW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `tshsl` reader - TSHSL Delay Value"]
pub type TshslR = crate::FieldReader;
#[doc = "Field `tshsl` writer - TSHSL Delay Value"]
pub type TshslW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - TSLCH Delay Value"]
    #[inline(always)]
    pub fn tslch(&self) -> TslchR {
        TslchR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - TCHSH Delay Value"]
    #[inline(always)]
    pub fn tchsh(&self) -> TchshR {
        TchshR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - TSD2D Delay Value"]
    #[inline(always)]
    pub fn tsd2d(&self) -> Tsd2dR {
        Tsd2dR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - TSHSL Delay Value"]
    #[inline(always)]
    pub fn tshsl(&self) -> TshslR {
        TshslR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - TSLCH Delay Value"]
    #[inline(always)]
    #[must_use]
    pub fn tslch(&mut self) -> TslchW<DelaySpec> {
        TslchW::new(self, 0)
    }
    #[doc = "Bits 8:15 - TCHSH Delay Value"]
    #[inline(always)]
    #[must_use]
    pub fn tchsh(&mut self) -> TchshW<DelaySpec> {
        TchshW::new(self, 8)
    }
    #[doc = "Bits 16:23 - TSD2D Delay Value"]
    #[inline(always)]
    #[must_use]
    pub fn tsd2d(&mut self) -> Tsd2dW<DelaySpec> {
        Tsd2dW::new(self, 16)
    }
    #[doc = "Bits 24:31 - TSHSL Delay Value"]
    #[inline(always)]
    #[must_use]
    pub fn tshsl(&mut self) -> TshslW<DelaySpec> {
        TshslW::new(self, 24)
    }
}
#[doc = "Cadence QSPI Delay\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`delay::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`delay::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DelaySpec;
impl crate::RegisterSpec for DelaySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`delay::R`](R) reader structure"]
impl crate::Readable for DelaySpec {}
#[doc = "`write(|w| ..)` method takes [`delay::W`](W) writer structure"]
impl crate::Writable for DelaySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets delay to value 0"]
impl crate::Resettable for DelaySpec {
    const RESET_VALUE: u32 = 0;
}
