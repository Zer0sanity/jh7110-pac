#[doc = "Register `ctrl0` reader"]
pub type R = crate::R<Ctrl0Spec>;
#[doc = "Register `ctrl0` writer"]
pub type W = crate::W<Ctrl0Spec>;
#[doc = "Field `max_burst` reader - Device DMA AXI max burst length - length = value + 1?"]
pub type MaxBurstR = crate::FieldReader;
#[doc = "Field `max_burst` writer - Device DMA AXI max burst length - length = value + 1?"]
pub type MaxBurstW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Device DMA AXI max burst length - length = value + 1?"]
    #[inline(always)]
    pub fn max_burst(&self) -> MaxBurstR {
        MaxBurstR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Device DMA AXI max burst length - length = value + 1?"]
    #[inline(always)]
    #[must_use]
    pub fn max_burst(&mut self) -> MaxBurstW<Ctrl0Spec> {
        MaxBurstW::new(self, 0)
    }
}
#[doc = "Device DMA AXI control 0: **WARNING** DMA AXI max burst length - In versions preceding DEV_VER_V2, for example, iMX8QM, there exist the bugs in the DMA. These bugs occur when the trb_burst_size exceeds 16 and the address is not aligned to 128 Bytes (which is a product of the 64-bit AXI and AXI maximum burst length of 16 or 0xF+1, dma_axi_ctrl0\\[3:0\\]). This results in data corruption when it crosses the 4K border. The corruption specifically occurs from the position (4K - (address &amp; 0x7F)) to 4K. So force trb_burst_size to 16 at such platform.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctrl0Spec;
impl crate::RegisterSpec for Ctrl0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl0::R`](R) reader structure"]
impl crate::Readable for Ctrl0Spec {}
#[doc = "`write(|w| ..)` method takes [`ctrl0::W`](W) writer structure"]
impl crate::Writable for Ctrl0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ctrl0 to value 0"]
impl crate::Resettable for Ctrl0Spec {
    const RESET_VALUE: u32 = 0;
}
