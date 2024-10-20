#[doc = "Register `aon_syscfg_1` reader"]
pub type R = crate::R<AonSyscfg1Spec>;
#[doc = "Field `u0_boot_ctrl_boot_status` reader - "]
pub type U0BootCtrlBootStatusR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn u0_boot_ctrl_boot_status(&self) -> U0BootCtrlBootStatusR {
        U0BootCtrlBootStatusR::new((self.bits & 0x0f) as u8)
    }
}
#[doc = "AON SYSCONSAIF SYSCFG 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aon_syscfg_1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AonSyscfg1Spec;
impl crate::RegisterSpec for AonSyscfg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aon_syscfg_1::R`](R) reader structure"]
impl crate::Readable for AonSyscfg1Spec {}
#[doc = "`reset()` method sets aon_syscfg_1 to value 0"]
impl crate::Resettable for AonSyscfg1Spec {
    const RESET_VALUE: u32 = 0;
}
