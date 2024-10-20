#[doc = "Register `rx_queue_dma[%s]` reader"]
pub type R = crate::R<RxQueueDmaSpec>;
#[doc = "Register `rx_queue_dma[%s]` writer"]
pub type W = crate::W<RxQueueDmaSpec>;
#[doc = "Field `channel(0-3)` reader - RX DMA Channel"]
pub type ChannelR = crate::FieldReader;
#[doc = "Field `channel(0-3)` writer - RX DMA Channel"]
pub type ChannelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "RX DMA Channel"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `channel0` field"]
    #[inline(always)]
    pub fn channel(&self, n: u8) -> ChannelR {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        ChannelR::new(((self.bits >> (n * 8)) & 0x0f) as u8)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "RX DMA Channel"]
    #[inline(always)]
    pub fn channel_iter(&self) -> impl Iterator<Item = ChannelR> + '_ {
        (0..4).map(move |n| ChannelR::new(((self.bits >> (n * 8)) & 0x0f) as u8))
    }
    #[doc = "Bits 0:3 - RX DMA Channel"]
    #[inline(always)]
    pub fn channel0(&self) -> ChannelR {
        ChannelR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - RX DMA Channel"]
    #[inline(always)]
    pub fn channel1(&self) -> ChannelR {
        ChannelR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - RX DMA Channel"]
    #[inline(always)]
    pub fn channel2(&self) -> ChannelR {
        ChannelR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - RX DMA Channel"]
    #[inline(always)]
    pub fn channel3(&self) -> ChannelR {
        ChannelR::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "RX DMA Channel"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `channel0` field"]
    #[inline(always)]
    #[must_use]
    pub fn channel(&mut self, n: u8) -> ChannelW<RxQueueDmaSpec> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        ChannelW::new(self, n * 8)
    }
    #[doc = "Bits 0:3 - RX DMA Channel"]
    #[inline(always)]
    #[must_use]
    pub fn channel0(&mut self) -> ChannelW<RxQueueDmaSpec> {
        ChannelW::new(self, 0)
    }
    #[doc = "Bits 8:11 - RX DMA Channel"]
    #[inline(always)]
    #[must_use]
    pub fn channel1(&mut self) -> ChannelW<RxQueueDmaSpec> {
        ChannelW::new(self, 8)
    }
    #[doc = "Bits 16:19 - RX DMA Channel"]
    #[inline(always)]
    #[must_use]
    pub fn channel2(&mut self) -> ChannelW<RxQueueDmaSpec> {
        ChannelW::new(self, 16)
    }
    #[doc = "Bits 24:27 - RX DMA Channel"]
    #[inline(always)]
    #[must_use]
    pub fn channel3(&mut self) -> ChannelW<RxQueueDmaSpec> {
        ChannelW::new(self, 24)
    }
}
#[doc = "MTL RX Queue DMA - rx_queue_dma0: channel 0-3, rx_queue_dma1: channel 4-7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_queue_dma::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rx_queue_dma::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxQueueDmaSpec;
impl crate::RegisterSpec for RxQueueDmaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_queue_dma::R`](R) reader structure"]
impl crate::Readable for RxQueueDmaSpec {}
#[doc = "`write(|w| ..)` method takes [`rx_queue_dma::W`](W) writer structure"]
impl crate::Writable for RxQueueDmaSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets rx_queue_dma[%s]
to value 0"]
impl crate::Resettable for RxQueueDmaSpec {
    const RESET_VALUE: u32 = 0;
}
