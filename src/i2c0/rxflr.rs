#[doc = "Register `rxflr` reader"]
pub type R = crate::R<RxflrSpec>;
#[doc = "Register `rxflr` writer"]
pub type W = crate::W<RxflrSpec>;
#[doc = "Field `rxflr` reader - "]
pub type RxflrR = crate::FieldReader<u32>;
#[doc = "Field `rxflr` writer - "]
pub type RxflrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn rxflr(&self) -> RxflrR {
        RxflrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn rxflr(&mut self) -> RxflrW<RxflrSpec> {
        RxflrW::new(self, 0)
    }
}
#[doc = "DesignWare I2C RX Failure\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxflr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxflr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxflrSpec;
impl crate::RegisterSpec for RxflrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxflr::R`](R) reader structure"]
impl crate::Readable for RxflrSpec {}
#[doc = "`write(|w| ..)` method takes [`rxflr::W`](W) writer structure"]
impl crate::Writable for RxflrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets rxflr to value 0"]
impl crate::Resettable for RxflrSpec {
    const RESET_VALUE: u32 = 0;
}
