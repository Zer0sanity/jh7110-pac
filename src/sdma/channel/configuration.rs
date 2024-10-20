#[doc = "Register `configuration` reader"]
pub type R = crate::R<ConfigurationSpec>;
#[doc = "Register `configuration` writer"]
pub type W = crate::W<ConfigurationSpec>;
#[doc = "Field `enable` reader - Channel enable - 0: channel disabled, 1: channel enabled."]
pub type EnableR = crate::BitReader;
#[doc = "Field `enable` writer - Channel enable - 0: channel disabled, 1: channel enabled."]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `src_peripheral` reader - Source peripheral. This value selects the DMA source request peripheral. This field is ignored if the source of the transfer is from memory."]
pub type SrcPeripheralR = crate::FieldReader;
#[doc = "Field `src_peripheral` writer - Source peripheral. This value selects the DMA source request peripheral. This field is ignored if the source of the transfer is from memory."]
pub type SrcPeripheralW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `dst_peripheral` reader - Destination peripheral. This value selects the DMA destination request peripheral. This field is ignored if the destination of the transfer is to memory."]
pub type DstPeripheralR = crate::FieldReader;
#[doc = "Field `dst_peripheral` writer - Destination peripheral. This value selects the DMA destination request peripheral. This field is ignored if the destination of the transfer is to memory."]
pub type DstPeripheralW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `flow_cntrl` reader - Flow control and transfer type. This value indicates the flow controller and transfer type. The flow controller can be the DMAC, the source peripheral, or the destination peripheral. The transfer type can be memory-to-memory, memory-to-peripheral, peripheral-to-memory, or peripheral-to-peripheral."]
pub type FlowCntrlR = crate::FieldReader;
#[doc = "Field `flow_cntrl` writer - Flow control and transfer type. This value indicates the flow controller and transfer type. The flow controller can be the DMAC, the source peripheral, or the destination peripheral. The transfer type can be memory-to-memory, memory-to-peripheral, peripheral-to-memory, or peripheral-to-peripheral."]
pub type FlowCntrlW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `iem` reader - Interrupt error mask. When cleared, this bit masks out the error interrupt of the relevant channel."]
pub type IemR = crate::BitReader;
#[doc = "Field `iem` writer - Interrupt error mask. When cleared, this bit masks out the error interrupt of the relevant channel."]
pub type IemW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `itc` reader - Terminal count interrupt mask. When cleared, this bit masks out the terminal count interrupt of the relevant channel."]
pub type ItcR = crate::BitReader;
#[doc = "Field `itc` writer - Terminal count interrupt mask. When cleared, this bit masks out the terminal count interrupt of the relevant channel."]
pub type ItcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `lock` reader - Lock. When set, this bit enables locked transfers. For details of how lock control works."]
pub type LockR = crate::BitReader;
#[doc = "Field `lock` writer - Lock. When set, this bit enables locked transfers. For details of how lock control works."]
pub type LockW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `active` reader - Active - 0: there channel FIFO has no data, 1: the channel FIFO has data."]
pub type ActiveR = crate::BitReader;
#[doc = "Field `halt` reader - Halt - 0: enable DMA requests, 1: ignore extra source DMA requests. The contents of the channels FIFO are drained. You can use this value with the Active and Channel Enable bits to cleanly disable a DMA channel."]
pub type HaltR = crate::BitReader;
#[doc = "Field `halt` writer - Halt - 0: enable DMA requests, 1: ignore extra source DMA requests. The contents of the channels FIFO are drained. You can use this value with the Active and Channel Enable bits to cleanly disable a DMA channel."]
pub type HaltW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Channel enable - 0: channel disabled, 1: channel enabled."]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:4 - Source peripheral. This value selects the DMA source request peripheral. This field is ignored if the source of the transfer is from memory."]
    #[inline(always)]
    pub fn src_peripheral(&self) -> SrcPeripheralR {
        SrcPeripheralR::new(((self.bits >> 1) & 0x0f) as u8)
    }
    #[doc = "Bits 6:9 - Destination peripheral. This value selects the DMA destination request peripheral. This field is ignored if the destination of the transfer is to memory."]
    #[inline(always)]
    pub fn dst_peripheral(&self) -> DstPeripheralR {
        DstPeripheralR::new(((self.bits >> 6) & 0x0f) as u8)
    }
    #[doc = "Bits 11:13 - Flow control and transfer type. This value indicates the flow controller and transfer type. The flow controller can be the DMAC, the source peripheral, or the destination peripheral. The transfer type can be memory-to-memory, memory-to-peripheral, peripheral-to-memory, or peripheral-to-peripheral."]
    #[inline(always)]
    pub fn flow_cntrl(&self) -> FlowCntrlR {
        FlowCntrlR::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bit 14 - Interrupt error mask. When cleared, this bit masks out the error interrupt of the relevant channel."]
    #[inline(always)]
    pub fn iem(&self) -> IemR {
        IemR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Terminal count interrupt mask. When cleared, this bit masks out the terminal count interrupt of the relevant channel."]
    #[inline(always)]
    pub fn itc(&self) -> ItcR {
        ItcR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Lock. When set, this bit enables locked transfers. For details of how lock control works."]
    #[inline(always)]
    pub fn lock(&self) -> LockR {
        LockR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Active - 0: there channel FIFO has no data, 1: the channel FIFO has data."]
    #[inline(always)]
    pub fn active(&self) -> ActiveR {
        ActiveR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Halt - 0: enable DMA requests, 1: ignore extra source DMA requests. The contents of the channels FIFO are drained. You can use this value with the Active and Channel Enable bits to cleanly disable a DMA channel."]
    #[inline(always)]
    pub fn halt(&self) -> HaltR {
        HaltR::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel enable - 0: channel disabled, 1: channel enabled."]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> EnableW<ConfigurationSpec> {
        EnableW::new(self, 0)
    }
    #[doc = "Bits 1:4 - Source peripheral. This value selects the DMA source request peripheral. This field is ignored if the source of the transfer is from memory."]
    #[inline(always)]
    #[must_use]
    pub fn src_peripheral(&mut self) -> SrcPeripheralW<ConfigurationSpec> {
        SrcPeripheralW::new(self, 1)
    }
    #[doc = "Bits 6:9 - Destination peripheral. This value selects the DMA destination request peripheral. This field is ignored if the destination of the transfer is to memory."]
    #[inline(always)]
    #[must_use]
    pub fn dst_peripheral(&mut self) -> DstPeripheralW<ConfigurationSpec> {
        DstPeripheralW::new(self, 6)
    }
    #[doc = "Bits 11:13 - Flow control and transfer type. This value indicates the flow controller and transfer type. The flow controller can be the DMAC, the source peripheral, or the destination peripheral. The transfer type can be memory-to-memory, memory-to-peripheral, peripheral-to-memory, or peripheral-to-peripheral."]
    #[inline(always)]
    #[must_use]
    pub fn flow_cntrl(&mut self) -> FlowCntrlW<ConfigurationSpec> {
        FlowCntrlW::new(self, 11)
    }
    #[doc = "Bit 14 - Interrupt error mask. When cleared, this bit masks out the error interrupt of the relevant channel."]
    #[inline(always)]
    #[must_use]
    pub fn iem(&mut self) -> IemW<ConfigurationSpec> {
        IemW::new(self, 14)
    }
    #[doc = "Bit 15 - Terminal count interrupt mask. When cleared, this bit masks out the terminal count interrupt of the relevant channel."]
    #[inline(always)]
    #[must_use]
    pub fn itc(&mut self) -> ItcW<ConfigurationSpec> {
        ItcW::new(self, 15)
    }
    #[doc = "Bit 16 - Lock. When set, this bit enables locked transfers. For details of how lock control works."]
    #[inline(always)]
    #[must_use]
    pub fn lock(&mut self) -> LockW<ConfigurationSpec> {
        LockW::new(self, 16)
    }
    #[doc = "Bit 18 - Halt - 0: enable DMA requests, 1: ignore extra source DMA requests. The contents of the channels FIFO are drained. You can use this value with the Active and Channel Enable bits to cleanly disable a DMA channel."]
    #[inline(always)]
    #[must_use]
    pub fn halt(&mut self) -> HaltW<ConfigurationSpec> {
        HaltW::new(self, 18)
    }
}
#[doc = "DMA Channel Configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`configuration::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`configuration::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ConfigurationSpec;
impl crate::RegisterSpec for ConfigurationSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`configuration::R`](R) reader structure"]
impl crate::Readable for ConfigurationSpec {}
#[doc = "`write(|w| ..)` method takes [`configuration::W`](W) writer structure"]
impl crate::Writable for ConfigurationSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets configuration to value 0"]
impl crate::Resettable for ConfigurationSpec {
    const RESET_VALUE: u32 = 0;
}
