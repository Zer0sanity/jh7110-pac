#[doc = "Register `sys_syscfg37` reader"]
pub type R = crate::R<SysSyscfg37Spec>;
#[doc = "Field `gmac5_axi64_ptp_timestamp_31_0` reader - "]
pub type Gmac5Axi64PtpTimestamp31_0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn gmac5_axi64_ptp_timestamp_31_0(&self) -> Gmac5Axi64PtpTimestamp31_0R {
        Gmac5Axi64PtpTimestamp31_0R::new(self.bits)
    }
}
#[doc = "SYS SYSCONSAIF SYSCFG 148\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_syscfg37::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysSyscfg37Spec;
impl crate::RegisterSpec for SysSyscfg37Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_syscfg37::R`](R) reader structure"]
impl crate::Readable for SysSyscfg37Spec {}
#[doc = "`reset()` method sets sys_syscfg37 to value 0"]
impl crate::Resettable for SysSyscfg37Spec {
    const RESET_VALUE: u32 = 0;
}
