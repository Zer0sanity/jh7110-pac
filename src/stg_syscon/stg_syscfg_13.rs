#[doc = "Register `stg_syscfg_13` reader"]
pub type R = crate::R<StgSyscfg13Spec>;
#[doc = "Field `u0_hifi4_pfaultinfo` reader - Fault Handling Signals"]
pub type U0Hifi4PfaultinfoR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Fault Handling Signals"]
    #[inline(always)]
    pub fn u0_hifi4_pfaultinfo(&self) -> U0Hifi4PfaultinfoR {
        U0Hifi4PfaultinfoR::new(self.bits)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 52\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_13::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg13Spec;
impl crate::RegisterSpec for StgSyscfg13Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_13::R`](R) reader structure"]
impl crate::Readable for StgSyscfg13Spec {}
#[doc = "`reset()` method sets stg_syscfg_13 to value 0"]
impl crate::Resettable for StgSyscfg13Spec {
    const RESET_VALUE: u32 = 0;
}
