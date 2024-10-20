#[doc = "Register `aon_syscfg_7` reader"]
pub type R = crate::R<AonSyscfg7Spec>;
#[doc = "Field `u0_otpc_fl_func_lock` reader - "]
pub type U0OtpcFlFuncLockR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn u0_otpc_fl_func_lock(&self) -> U0OtpcFlFuncLockR {
        U0OtpcFlFuncLockR::new(self.bits)
    }
}
#[doc = "AON SYSCONSAIF SYSCFG 28\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aon_syscfg_7::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AonSyscfg7Spec;
impl crate::RegisterSpec for AonSyscfg7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aon_syscfg_7::R`](R) reader structure"]
impl crate::Readable for AonSyscfg7Spec {}
#[doc = "`reset()` method sets aon_syscfg_7 to value 0"]
impl crate::Resettable for AonSyscfg7Spec {
    const RESET_VALUE: u32 = 0;
}
