#[doc = "Register `aon_syscfg_5` reader"]
pub type R = crate::R<AonSyscfg5Spec>;
#[doc = "Field `gmac5_axi64_ptp_timestamp_o_32_63` reader - GMAC5 PTP timestamps 32-63 (little-endian)"]
pub type Gmac5Axi64PtpTimestampO32_63R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - GMAC5 PTP timestamps 32-63 (little-endian)"]
    #[inline(always)]
    pub fn gmac5_axi64_ptp_timestamp_o_32_63(&self) -> Gmac5Axi64PtpTimestampO32_63R {
        Gmac5Axi64PtpTimestampO32_63R::new(self.bits)
    }
}
#[doc = "AON SYSCONSAIF SYSCFG 20\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aon_syscfg_5::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AonSyscfg5Spec;
impl crate::RegisterSpec for AonSyscfg5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aon_syscfg_5::R`](R) reader structure"]
impl crate::Readable for AonSyscfg5Spec {}
#[doc = "`reset()` method sets aon_syscfg_5 to value 0"]
impl crate::Resettable for AonSyscfg5Spec {
    const RESET_VALUE: u32 = 0;
}
