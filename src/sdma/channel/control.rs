#[doc = "Register `control` reader"]
pub type R = crate::R<ControlSpec>;
#[doc = "Register `control` writer"]
pub type W = crate::W<ControlSpec>;
#[doc = "Field `transfer_size` reader - Transfer size. A write to this field sets the size of the transfer when the DMAC is the flow controller. This value counts down from the original value to zero, and so its value indicates the number of transfers left to complete. A read from this field provides the number of transfers still to be completed on the destination bus. Reading the register when the channel is active does not give useful information because by the time the software has processed the value read, the channel might have progressed. Only use it when a channel is enabled, and then disabled. The ARM PrimeCell DMA Controller (PL080) Design Manual provides more information about the use of this field. Program the transfer size value to zero if the DMAC is not the flow controller. If you program the TransferSize to a non-zero value, the DMAC might attempt to use this value instead of ignoring the TransferSize."]
pub type TransferSizeR = crate::FieldReader<u16>;
#[doc = "Field `transfer_size` writer - Transfer size. A write to this field sets the size of the transfer when the DMAC is the flow controller. This value counts down from the original value to zero, and so its value indicates the number of transfers left to complete. A read from this field provides the number of transfers still to be completed on the destination bus. Reading the register when the channel is active does not give useful information because by the time the software has processed the value read, the channel might have progressed. Only use it when a channel is enabled, and then disabled. The ARM PrimeCell DMA Controller (PL080) Design Manual provides more information about the use of this field. Program the transfer size value to zero if the DMAC is not the flow controller. If you program the TransferSize to a non-zero value, the DMAC might attempt to use this value instead of ignoring the TransferSize."]
pub type TransferSizeW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `sb_size` reader - Source burst size - 0: 1, 1: 4, 2: 8, 3: 16, 4: 32, 5: 64, 6: 128, 7: 256. Indicates the number of transfers that make up a source burst. You must set this value to the burst size of the source peripheral, or if the source is memory, to the memory boundary size. The burst size is the amount of data that is transferred when the DMACxBREQ signal goes active in the source peripheral. The burst size is not related to the AHB HBURST signal."]
pub type SbSizeR = crate::FieldReader;
#[doc = "Field `sb_size` writer - Source burst size - 0: 1, 1: 4, 2: 8, 3: 16, 4: 32, 5: 64, 6: 128, 7: 256. Indicates the number of transfers that make up a source burst. You must set this value to the burst size of the source peripheral, or if the source is memory, to the memory boundary size. The burst size is the amount of data that is transferred when the DMACxBREQ signal goes active in the source peripheral. The burst size is not related to the AHB HBURST signal."]
pub type SbSizeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `db_size` reader - Destination burst size - 0: 1, 1: 4, 2: 8, 3: 16, 4: 32, 5: 64, 6: 128, 7: 256. Indicates the number of transfers that make up a destination burst transfer request. You must set this value to the burst size of the destination peripheral, or if the destination is memory, to the memory boundary size. The burst size is the amount of data that is transferred when the DMACxBREQ signal goes active in the destination peripheral. The burst size is not related to the AHB HBURST signal."]
pub type DbSizeR = crate::FieldReader;
#[doc = "Field `db_size` writer - Destination burst size - 0: 1, 1: 4, 2: 8, 3: 16, 4: 32, 5: 64, 6: 128, 7: 256. Indicates the number of transfers that make up a destination burst transfer request. You must set this value to the burst size of the destination peripheral, or if the destination is memory, to the memory boundary size. The burst size is the amount of data that is transferred when the DMACxBREQ signal goes active in the destination peripheral. The burst size is not related to the AHB HBURST signal."]
pub type DbSizeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `src_width` reader - Source transfer width - 0: 8-bit, 1: 16-bit, 2: 32-bit. Transfers wider than the AHB master bus width are illegal. The source and destination widths can be different from each other. The hardware automatically packs and unpacks the data when required."]
pub type SrcWidthR = crate::FieldReader;
#[doc = "Field `src_width` writer - Source transfer width - 0: 8-bit, 1: 16-bit, 2: 32-bit. Transfers wider than the AHB master bus width are illegal. The source and destination widths can be different from each other. The hardware automatically packs and unpacks the data when required."]
pub type SrcWidthW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `dst_width` reader - Destination transfer width - 0: 8-bit, 1: 16-bit, 2: 32-bit. Transfers wider than the AHB master bus width are illegal. The source and destination widths can be different from each other. The hardware automatically packs and unpacks the data when required."]
pub type DstWidthR = crate::FieldReader;
#[doc = "Field `dst_width` writer - Destination transfer width - 0: 8-bit, 1: 16-bit, 2: 32-bit. Transfers wider than the AHB master bus width are illegal. The source and destination widths can be different from each other. The hardware automatically packs and unpacks the data when required."]
pub type DstWidthW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `src` reader - Source AHB master select - 0: AHB master 1 selected, 1: AHB master 2 selected."]
pub type SrcR = crate::BitReader;
#[doc = "Field `src` writer - Source AHB master select - 0: AHB master 1 selected, 1: AHB master 2 selected."]
pub type SrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dst` reader - Destination AHB master select - 0: AHB master 1 selected, 1: AHB master 2 selected."]
pub type DstR = crate::BitReader;
#[doc = "Field `dst` writer - Destination AHB master select - 0: AHB master 1 selected, 1: AHB master 2 selected."]
pub type DstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `src_inc` reader - Source increment. When set, the source address is incremented after each transfer."]
pub type SrcIncR = crate::BitReader;
#[doc = "Field `src_inc` writer - Source increment. When set, the source address is incremented after each transfer."]
pub type SrcIncW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dst_inc` reader - Destination increment. When set, the destination address is incremented after each transfer."]
pub type DstIncR = crate::BitReader;
#[doc = "Field `dst_inc` writer - Destination increment. When set, the destination address is incremented after each transfer."]
pub type DstIncW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `prot_user` reader - Protection - 0: user mode, 1: privileged mode. Indicates whether the access is in User or Privileged mode. This bit controls the AHB HPROT\\[1\\]
signal."]
pub type ProtUserR = crate::BitReader;
#[doc = "Field `prot_user` writer - Protection - 0: user mode, 1: privileged mode. Indicates whether the access is in User or Privileged mode. This bit controls the AHB HPROT\\[1\\]
signal."]
pub type ProtUserW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `prot_buf` reader - Protection - 0: non-bufferable, 1: bufferable. Indicates whether or not the access can be buffered. This bit indicates whether or not the access is bufferable. For example, you can use this bit to indicate to an AMBA bridge that the read can complete in zero wait states on the source bus without waiting for it to arbitrate for the destination bus and for the slave to accept the data. This bit controls the AHB HPROT\\[2\\]
signal."]
pub type ProtBufR = crate::BitReader;
#[doc = "Field `prot_buf` writer - Protection - 0: non-bufferable, 1: bufferable. Indicates whether or not the access can be buffered. This bit indicates whether or not the access is bufferable. For example, you can use this bit to indicate to an AMBA bridge that the read can complete in zero wait states on the source bus without waiting for it to arbitrate for the destination bus and for the slave to accept the data. This bit controls the AHB HPROT\\[2\\]
signal."]
pub type ProtBufW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `prot_cache` reader - Protection - 0: non-cacheable, 1: cacheable. This bit indicates whether or not the access is cacheable. For example, you can use this bit to indicate to an AMBA bridge that when it saw the first read of a burst of eight it can transfer the whole burst of eight reads on the destination bus, rather than pass the transactions through one at a time. This bit controls the AHB HPROT\\[3\\]
signal."]
pub type ProtCacheR = crate::BitReader;
#[doc = "Field `prot_cache` writer - Protection - 0: non-cacheable, 1: cacheable. This bit indicates whether or not the access is cacheable. For example, you can use this bit to indicate to an AMBA bridge that when it saw the first read of a burst of eight it can transfer the whole burst of eight reads on the destination bus, rather than pass the transactions through one at a time. This bit controls the AHB HPROT\\[3\\]
signal."]
pub type ProtCacheW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ie` reader - Terminal count interrupt enable bit. It controls whether the current LLI is expected to trigger the terminal count interrupt."]
pub type IeR = crate::BitReader;
#[doc = "Field `ie` writer - Terminal count interrupt enable bit. It controls whether the current LLI is expected to trigger the terminal count interrupt."]
pub type IeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:11 - Transfer size. A write to this field sets the size of the transfer when the DMAC is the flow controller. This value counts down from the original value to zero, and so its value indicates the number of transfers left to complete. A read from this field provides the number of transfers still to be completed on the destination bus. Reading the register when the channel is active does not give useful information because by the time the software has processed the value read, the channel might have progressed. Only use it when a channel is enabled, and then disabled. The ARM PrimeCell DMA Controller (PL080) Design Manual provides more information about the use of this field. Program the transfer size value to zero if the DMAC is not the flow controller. If you program the TransferSize to a non-zero value, the DMAC might attempt to use this value instead of ignoring the TransferSize."]
    #[inline(always)]
    pub fn transfer_size(&self) -> TransferSizeR {
        TransferSizeR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:14 - Source burst size - 0: 1, 1: 4, 2: 8, 3: 16, 4: 32, 5: 64, 6: 128, 7: 256. Indicates the number of transfers that make up a source burst. You must set this value to the burst size of the source peripheral, or if the source is memory, to the memory boundary size. The burst size is the amount of data that is transferred when the DMACxBREQ signal goes active in the source peripheral. The burst size is not related to the AHB HBURST signal."]
    #[inline(always)]
    pub fn sb_size(&self) -> SbSizeR {
        SbSizeR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:17 - Destination burst size - 0: 1, 1: 4, 2: 8, 3: 16, 4: 32, 5: 64, 6: 128, 7: 256. Indicates the number of transfers that make up a destination burst transfer request. You must set this value to the burst size of the destination peripheral, or if the destination is memory, to the memory boundary size. The burst size is the amount of data that is transferred when the DMACxBREQ signal goes active in the destination peripheral. The burst size is not related to the AHB HBURST signal."]
    #[inline(always)]
    pub fn db_size(&self) -> DbSizeR {
        DbSizeR::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bits 18:20 - Source transfer width - 0: 8-bit, 1: 16-bit, 2: 32-bit. Transfers wider than the AHB master bus width are illegal. The source and destination widths can be different from each other. The hardware automatically packs and unpacks the data when required."]
    #[inline(always)]
    pub fn src_width(&self) -> SrcWidthR {
        SrcWidthR::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 21:23 - Destination transfer width - 0: 8-bit, 1: 16-bit, 2: 32-bit. Transfers wider than the AHB master bus width are illegal. The source and destination widths can be different from each other. The hardware automatically packs and unpacks the data when required."]
    #[inline(always)]
    pub fn dst_width(&self) -> DstWidthR {
        DstWidthR::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bit 24 - Source AHB master select - 0: AHB master 1 selected, 1: AHB master 2 selected."]
    #[inline(always)]
    pub fn src(&self) -> SrcR {
        SrcR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Destination AHB master select - 0: AHB master 1 selected, 1: AHB master 2 selected."]
    #[inline(always)]
    pub fn dst(&self) -> DstR {
        DstR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Source increment. When set, the source address is incremented after each transfer."]
    #[inline(always)]
    pub fn src_inc(&self) -> SrcIncR {
        SrcIncR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Destination increment. When set, the destination address is incremented after each transfer."]
    #[inline(always)]
    pub fn dst_inc(&self) -> DstIncR {
        DstIncR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Protection - 0: user mode, 1: privileged mode. Indicates whether the access is in User or Privileged mode. This bit controls the AHB HPROT\\[1\\]
signal."]
    #[inline(always)]
    pub fn prot_user(&self) -> ProtUserR {
        ProtUserR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Protection - 0: non-bufferable, 1: bufferable. Indicates whether or not the access can be buffered. This bit indicates whether or not the access is bufferable. For example, you can use this bit to indicate to an AMBA bridge that the read can complete in zero wait states on the source bus without waiting for it to arbitrate for the destination bus and for the slave to accept the data. This bit controls the AHB HPROT\\[2\\]
signal."]
    #[inline(always)]
    pub fn prot_buf(&self) -> ProtBufR {
        ProtBufR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Protection - 0: non-cacheable, 1: cacheable. This bit indicates whether or not the access is cacheable. For example, you can use this bit to indicate to an AMBA bridge that when it saw the first read of a burst of eight it can transfer the whole burst of eight reads on the destination bus, rather than pass the transactions through one at a time. This bit controls the AHB HPROT\\[3\\]
signal."]
    #[inline(always)]
    pub fn prot_cache(&self) -> ProtCacheR {
        ProtCacheR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Terminal count interrupt enable bit. It controls whether the current LLI is expected to trigger the terminal count interrupt."]
    #[inline(always)]
    pub fn ie(&self) -> IeR {
        IeR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - Transfer size. A write to this field sets the size of the transfer when the DMAC is the flow controller. This value counts down from the original value to zero, and so its value indicates the number of transfers left to complete. A read from this field provides the number of transfers still to be completed on the destination bus. Reading the register when the channel is active does not give useful information because by the time the software has processed the value read, the channel might have progressed. Only use it when a channel is enabled, and then disabled. The ARM PrimeCell DMA Controller (PL080) Design Manual provides more information about the use of this field. Program the transfer size value to zero if the DMAC is not the flow controller. If you program the TransferSize to a non-zero value, the DMAC might attempt to use this value instead of ignoring the TransferSize."]
    #[inline(always)]
    #[must_use]
    pub fn transfer_size(&mut self) -> TransferSizeW<ControlSpec> {
        TransferSizeW::new(self, 0)
    }
    #[doc = "Bits 12:14 - Source burst size - 0: 1, 1: 4, 2: 8, 3: 16, 4: 32, 5: 64, 6: 128, 7: 256. Indicates the number of transfers that make up a source burst. You must set this value to the burst size of the source peripheral, or if the source is memory, to the memory boundary size. The burst size is the amount of data that is transferred when the DMACxBREQ signal goes active in the source peripheral. The burst size is not related to the AHB HBURST signal."]
    #[inline(always)]
    #[must_use]
    pub fn sb_size(&mut self) -> SbSizeW<ControlSpec> {
        SbSizeW::new(self, 12)
    }
    #[doc = "Bits 15:17 - Destination burst size - 0: 1, 1: 4, 2: 8, 3: 16, 4: 32, 5: 64, 6: 128, 7: 256. Indicates the number of transfers that make up a destination burst transfer request. You must set this value to the burst size of the destination peripheral, or if the destination is memory, to the memory boundary size. The burst size is the amount of data that is transferred when the DMACxBREQ signal goes active in the destination peripheral. The burst size is not related to the AHB HBURST signal."]
    #[inline(always)]
    #[must_use]
    pub fn db_size(&mut self) -> DbSizeW<ControlSpec> {
        DbSizeW::new(self, 15)
    }
    #[doc = "Bits 18:20 - Source transfer width - 0: 8-bit, 1: 16-bit, 2: 32-bit. Transfers wider than the AHB master bus width are illegal. The source and destination widths can be different from each other. The hardware automatically packs and unpacks the data when required."]
    #[inline(always)]
    #[must_use]
    pub fn src_width(&mut self) -> SrcWidthW<ControlSpec> {
        SrcWidthW::new(self, 18)
    }
    #[doc = "Bits 21:23 - Destination transfer width - 0: 8-bit, 1: 16-bit, 2: 32-bit. Transfers wider than the AHB master bus width are illegal. The source and destination widths can be different from each other. The hardware automatically packs and unpacks the data when required."]
    #[inline(always)]
    #[must_use]
    pub fn dst_width(&mut self) -> DstWidthW<ControlSpec> {
        DstWidthW::new(self, 21)
    }
    #[doc = "Bit 24 - Source AHB master select - 0: AHB master 1 selected, 1: AHB master 2 selected."]
    #[inline(always)]
    #[must_use]
    pub fn src(&mut self) -> SrcW<ControlSpec> {
        SrcW::new(self, 24)
    }
    #[doc = "Bit 25 - Destination AHB master select - 0: AHB master 1 selected, 1: AHB master 2 selected."]
    #[inline(always)]
    #[must_use]
    pub fn dst(&mut self) -> DstW<ControlSpec> {
        DstW::new(self, 25)
    }
    #[doc = "Bit 26 - Source increment. When set, the source address is incremented after each transfer."]
    #[inline(always)]
    #[must_use]
    pub fn src_inc(&mut self) -> SrcIncW<ControlSpec> {
        SrcIncW::new(self, 26)
    }
    #[doc = "Bit 27 - Destination increment. When set, the destination address is incremented after each transfer."]
    #[inline(always)]
    #[must_use]
    pub fn dst_inc(&mut self) -> DstIncW<ControlSpec> {
        DstIncW::new(self, 27)
    }
    #[doc = "Bit 28 - Protection - 0: user mode, 1: privileged mode. Indicates whether the access is in User or Privileged mode. This bit controls the AHB HPROT\\[1\\]
signal."]
    #[inline(always)]
    #[must_use]
    pub fn prot_user(&mut self) -> ProtUserW<ControlSpec> {
        ProtUserW::new(self, 28)
    }
    #[doc = "Bit 29 - Protection - 0: non-bufferable, 1: bufferable. Indicates whether or not the access can be buffered. This bit indicates whether or not the access is bufferable. For example, you can use this bit to indicate to an AMBA bridge that the read can complete in zero wait states on the source bus without waiting for it to arbitrate for the destination bus and for the slave to accept the data. This bit controls the AHB HPROT\\[2\\]
signal."]
    #[inline(always)]
    #[must_use]
    pub fn prot_buf(&mut self) -> ProtBufW<ControlSpec> {
        ProtBufW::new(self, 29)
    }
    #[doc = "Bit 30 - Protection - 0: non-cacheable, 1: cacheable. This bit indicates whether or not the access is cacheable. For example, you can use this bit to indicate to an AMBA bridge that when it saw the first read of a burst of eight it can transfer the whole burst of eight reads on the destination bus, rather than pass the transactions through one at a time. This bit controls the AHB HPROT\\[3\\]
signal."]
    #[inline(always)]
    #[must_use]
    pub fn prot_cache(&mut self) -> ProtCacheW<ControlSpec> {
        ProtCacheW::new(self, 30)
    }
    #[doc = "Bit 31 - Terminal count interrupt enable bit. It controls whether the current LLI is expected to trigger the terminal count interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ie(&mut self) -> IeW<ControlSpec> {
        IeW::new(self, 31)
    }
}
#[doc = "DMA Channel Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`control::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`control::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ControlSpec;
impl crate::RegisterSpec for ControlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`control::R`](R) reader structure"]
impl crate::Readable for ControlSpec {}
#[doc = "`write(|w| ..)` method takes [`control::W`](W) writer structure"]
impl crate::Writable for ControlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets control to value 0"]
impl crate::Resettable for ControlSpec {
    const RESET_VALUE: u32 = 0;
}
