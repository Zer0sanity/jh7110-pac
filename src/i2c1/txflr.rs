#[doc = "Register `txflr` reader"]
pub type R = crate::R<TxflrSpec>;
#[doc = "Register `txflr` writer"]
pub type W = crate::W<TxflrSpec>;
#[doc = "Field `txflr` reader - "]
pub type TxflrR = crate::FieldReader<u32>;
#[doc = "Field `txflr` writer - "]
pub type TxflrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn txflr(&self) -> TxflrR {
        TxflrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn txflr(&mut self) -> TxflrW<TxflrSpec> {
        TxflrW::new(self, 0)
    }
}
#[doc = "DesignWare I2C TX Failure\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txflr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txflr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxflrSpec;
impl crate::RegisterSpec for TxflrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txflr::R`](R) reader structure"]
impl crate::Readable for TxflrSpec {}
#[doc = "`write(|w| ..)` method takes [`txflr::W`](W) writer structure"]
impl crate::Writable for TxflrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets txflr to value 0"]
impl crate::Resettable for TxflrSpec {
    const RESET_VALUE: u32 = 0;
}
