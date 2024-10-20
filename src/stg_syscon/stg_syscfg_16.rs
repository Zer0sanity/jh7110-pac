#[doc = "Register `stg_syscfg_16` reader"]
pub type R = crate::R<StgSyscfg16Spec>;
#[doc = "Register `stg_syscfg_16` writer"]
pub type W = crate::W<StgSyscfg16Spec>;
#[doc = "Field `u0_hifi4_scfg_dsp_slv_offset` reader - The value indicates the slave port remap address"]
pub type U0Hifi4ScfgDspSlvOffsetR = crate::FieldReader<u32>;
#[doc = "Field `u0_hifi4_scfg_dsp_slv_offset` writer - The value indicates the slave port remap address"]
pub type U0Hifi4ScfgDspSlvOffsetW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - The value indicates the slave port remap address"]
    #[inline(always)]
    pub fn u0_hifi4_scfg_dsp_slv_offset(&self) -> U0Hifi4ScfgDspSlvOffsetR {
        U0Hifi4ScfgDspSlvOffsetR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - The value indicates the slave port remap address"]
    #[inline(always)]
    #[must_use]
    pub fn u0_hifi4_scfg_dsp_slv_offset(&mut self) -> U0Hifi4ScfgDspSlvOffsetW<StgSyscfg16Spec> {
        U0Hifi4ScfgDspSlvOffsetW::new(self, 0)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 64\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_16::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stg_syscfg_16::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg16Spec;
impl crate::RegisterSpec for StgSyscfg16Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_16::R`](R) reader structure"]
impl crate::Readable for StgSyscfg16Spec {}
#[doc = "`write(|w| ..)` method takes [`stg_syscfg_16::W`](W) writer structure"]
impl crate::Writable for StgSyscfg16Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets stg_syscfg_16 to value 0x4000_0000"]
impl crate::Resettable for StgSyscfg16Spec {
    const RESET_VALUE: u32 = 0x4000_0000;
}
