#[doc = "Register `cap` reader"]
pub type R = crate::R<CapSpec>;
#[doc = "Register `cap` writer"]
pub type W = crate::W<CapSpec>;
#[doc = "Field `cap` reader - DMA AXI capability."]
pub type CapR = crate::FieldReader<u32>;
#[doc = "Field `cap` writer - DMA AXI capability."]
pub type CapW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - DMA AXI capability."]
    #[inline(always)]
    pub fn cap(&self) -> CapR {
        CapR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - DMA AXI capability."]
    #[inline(always)]
    #[must_use]
    pub fn cap(&mut self) -> CapW<CapSpec> {
        CapW::new(self, 0)
    }
}
#[doc = "Device DMA AXI capability.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cap::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cap::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CapSpec;
impl crate::RegisterSpec for CapSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cap::R`](R) reader structure"]
impl crate::Readable for CapSpec {}
#[doc = "`write(|w| ..)` method takes [`cap::W`](W) writer structure"]
impl crate::Writable for CapSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets cap to value 0"]
impl crate::Resettable for CapSpec {
    const RESET_VALUE: u32 = 0;
}
