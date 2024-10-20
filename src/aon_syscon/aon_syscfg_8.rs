#[doc = "Register `aon_syscfg_8` reader"]
pub type R = crate::R<AonSyscfg8Spec>;
#[doc = "Field `u0_otpc_fl_pll0_lock` reader - "]
pub type U0OtpcFlPll0LockR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn u0_otpc_fl_pll0_lock(&self) -> U0OtpcFlPll0LockR {
        U0OtpcFlPll0LockR::new(self.bits)
    }
}
#[doc = "AON SYSCONSAIF SYSCFG 32\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aon_syscfg_8::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AonSyscfg8Spec;
impl crate::RegisterSpec for AonSyscfg8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aon_syscfg_8::R`](R) reader structure"]
impl crate::Readable for AonSyscfg8Spec {}
#[doc = "`reset()` method sets aon_syscfg_8 to value 0"]
impl crate::Resettable for AonSyscfg8Spec {
    const RESET_VALUE: u32 = 0;
}
