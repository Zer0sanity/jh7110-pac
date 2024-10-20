#[doc = "Register `aon_syscfg_2` reader"]
pub type R = crate::R<AonSyscfg2Spec>;
#[doc = "Field `u0_boot_ctrl_boot_vector_0_31` reader - Boot vectors 0-31 (little-endian)"]
pub type U0BootCtrlBootVector0_31R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Boot vectors 0-31 (little-endian)"]
    #[inline(always)]
    pub fn u0_boot_ctrl_boot_vector_0_31(&self) -> U0BootCtrlBootVector0_31R {
        U0BootCtrlBootVector0_31R::new(self.bits)
    }
}
#[doc = "AON SYSCONSAIF SYSCFG 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aon_syscfg_2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AonSyscfg2Spec;
impl crate::RegisterSpec for AonSyscfg2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aon_syscfg_2::R`](R) reader structure"]
impl crate::Readable for AonSyscfg2Spec {}
#[doc = "`reset()` method sets aon_syscfg_2 to value 0"]
impl crate::Resettable for AonSyscfg2Spec {
    const RESET_VALUE: u32 = 0;
}
