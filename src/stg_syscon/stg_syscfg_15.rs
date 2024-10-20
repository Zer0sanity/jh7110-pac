#[doc = "Register `stg_syscfg_15` reader"]
pub type R = crate::R<StgSyscfg15Spec>;
#[doc = "Register `stg_syscfg_15` writer"]
pub type W = crate::W<StgSyscfg15Spec>;
#[doc = "Field `u0_hifi4_scfg_dsp_mst_offset_master` reader - Indicates that master port remap address"]
pub type U0Hifi4ScfgDspMstOffsetMasterR = crate::FieldReader<u16>;
#[doc = "Field `u0_hifi4_scfg_dsp_mst_offset_master` writer - Indicates that master port remap address"]
pub type U0Hifi4ScfgDspMstOffsetMasterW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `u0_hifi4_scfg_dsp_mst_offset_dma` reader - Indicates the DMA port remap address"]
pub type U0Hifi4ScfgDspMstOffsetDmaR = crate::FieldReader<u16>;
#[doc = "Field `u0_hifi4_scfg_dsp_mst_offset_dma` writer - Indicates the DMA port remap address"]
pub type U0Hifi4ScfgDspMstOffsetDmaW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Indicates that master port remap address"]
    #[inline(always)]
    pub fn u0_hifi4_scfg_dsp_mst_offset_master(&self) -> U0Hifi4ScfgDspMstOffsetMasterR {
        U0Hifi4ScfgDspMstOffsetMasterR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Indicates the DMA port remap address"]
    #[inline(always)]
    pub fn u0_hifi4_scfg_dsp_mst_offset_dma(&self) -> U0Hifi4ScfgDspMstOffsetDmaR {
        U0Hifi4ScfgDspMstOffsetDmaR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Indicates that master port remap address"]
    #[inline(always)]
    #[must_use]
    pub fn u0_hifi4_scfg_dsp_mst_offset_master(
        &mut self,
    ) -> U0Hifi4ScfgDspMstOffsetMasterW<StgSyscfg15Spec> {
        U0Hifi4ScfgDspMstOffsetMasterW::new(self, 0)
    }
    #[doc = "Bits 16:27 - Indicates the DMA port remap address"]
    #[inline(always)]
    #[must_use]
    pub fn u0_hifi4_scfg_dsp_mst_offset_dma(
        &mut self,
    ) -> U0Hifi4ScfgDspMstOffsetDmaW<StgSyscfg15Spec> {
        U0Hifi4ScfgDspMstOffsetDmaW::new(self, 16)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 60\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_15::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stg_syscfg_15::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg15Spec;
impl crate::RegisterSpec for StgSyscfg15Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_15::R`](R) reader structure"]
impl crate::Readable for StgSyscfg15Spec {}
#[doc = "`write(|w| ..)` method takes [`stg_syscfg_15::W`](W) writer structure"]
impl crate::Writable for StgSyscfg15Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets stg_syscfg_15 to value 0"]
impl crate::Resettable for StgSyscfg15Spec {
    const RESET_VALUE: u32 = 0;
}
