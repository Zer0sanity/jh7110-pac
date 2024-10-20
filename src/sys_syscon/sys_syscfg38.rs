#[doc = "Register `sys_syscfg38` reader"]
pub type R = crate::R<SysSyscfg38Spec>;
#[doc = "Field `gmac5_axi64_ptp_timestamp_63_32` reader - "]
pub type Gmac5Axi64PtpTimestamp63_32R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn gmac5_axi64_ptp_timestamp_63_32(&self) -> Gmac5Axi64PtpTimestamp63_32R {
        Gmac5Axi64PtpTimestamp63_32R::new(self.bits)
    }
}
#[doc = "SYS SYSCONSAIF SYSCFG 152\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_syscfg38::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysSyscfg38Spec;
impl crate::RegisterSpec for SysSyscfg38Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_syscfg38::R`](R) reader structure"]
impl crate::Readable for SysSyscfg38Spec {}
#[doc = "`reset()` method sets sys_syscfg38 to value 0"]
impl crate::Resettable for SysSyscfg38Spec {
    const RESET_VALUE: u32 = 0;
}
