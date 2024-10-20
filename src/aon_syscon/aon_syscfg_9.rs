#[doc = "Register `aon_syscfg_9` reader"]
pub type R = crate::R<AonSyscfg9Spec>;
#[doc = "Field `u0_otpc_fl_pll1_lock` reader - "]
pub type U0OtpcFlPll1LockR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn u0_otpc_fl_pll1_lock(&self) -> U0OtpcFlPll1LockR {
        U0OtpcFlPll1LockR::new(self.bits)
    }
}
#[doc = "AON SYSCONSAIF SYSCFG 36\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aon_syscfg_9::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AonSyscfg9Spec;
impl crate::RegisterSpec for AonSyscfg9Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aon_syscfg_9::R`](R) reader structure"]
impl crate::Readable for AonSyscfg9Spec {}
#[doc = "`reset()` method sets aon_syscfg_9 to value 0"]
impl crate::Resettable for AonSyscfg9Spec {
    const RESET_VALUE: u32 = 0;
}
