#[doc = "Register `dma` reader"]
pub type R = crate::R<DmaSpec>;
#[doc = "Register `dma` writer"]
pub type W = crate::W<DmaSpec>;
#[doc = "Field `single` reader - "]
pub type SingleR = crate::FieldReader;
#[doc = "Field `single` writer - "]
pub type SingleW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `burst` reader - "]
pub type BurstR = crate::FieldReader;
#[doc = "Field `burst` writer - "]
pub type BurstW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn single(&self) -> SingleR {
        SingleR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn burst(&self) -> BurstR {
        BurstR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn single(&mut self) -> SingleW<DmaSpec> {
        SingleW::new(self, 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    #[must_use]
    pub fn burst(&mut self) -> BurstW<DmaSpec> {
        BurstW::new(self, 8)
    }
}
#[doc = "Cadence QSPI Direct Memory Access\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaSpec;
impl crate::RegisterSpec for DmaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma::R`](R) reader structure"]
impl crate::Readable for DmaSpec {}
#[doc = "`write(|w| ..)` method takes [`dma::W`](W) writer structure"]
impl crate::Writable for DmaSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets dma to value 0"]
impl crate::Resettable for DmaSpec {
    const RESET_VALUE: u32 = 0;
}
