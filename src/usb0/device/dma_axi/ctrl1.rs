#[doc = "Register `ctrl1` reader"]
pub type R = crate::R<Ctrl1Spec>;
#[doc = "Register `ctrl1` writer"]
pub type W = crate::W<Ctrl1Spec>;
#[doc = "Field `ctrl1` reader - DMA AXI control 1."]
pub type Ctrl1R = crate::FieldReader<u32>;
#[doc = "Field `ctrl1` writer - DMA AXI control 1."]
pub type Ctrl1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - DMA AXI control 1."]
    #[inline(always)]
    pub fn ctrl1(&self) -> Ctrl1R {
        Ctrl1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - DMA AXI control 1."]
    #[inline(always)]
    #[must_use]
    pub fn ctrl1(&mut self) -> Ctrl1W<Ctrl1Spec> {
        Ctrl1W::new(self, 0)
    }
}
#[doc = "Device DMA AXI control 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctrl1Spec;
impl crate::RegisterSpec for Ctrl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl1::R`](R) reader structure"]
impl crate::Readable for Ctrl1Spec {}
#[doc = "`write(|w| ..)` method takes [`ctrl1::W`](W) writer structure"]
impl crate::Writable for Ctrl1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ctrl1 to value 0"]
impl crate::Resettable for Ctrl1Spec {
    const RESET_VALUE: u32 = 0;
}
