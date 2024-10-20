#[doc = "Register `aon_syscfg_4` reader"]
pub type R = crate::R<AonSyscfg4Spec>;
#[doc = "Field `gmac5_axi64_ptp_timestamp_o_0_31` reader - GMAC5 PTP timestamps 0-31 (little-endian)"]
pub type Gmac5Axi64PtpTimestampO0_31R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - GMAC5 PTP timestamps 0-31 (little-endian)"]
    #[inline(always)]
    pub fn gmac5_axi64_ptp_timestamp_o_0_31(&self) -> Gmac5Axi64PtpTimestampO0_31R {
        Gmac5Axi64PtpTimestampO0_31R::new(self.bits)
    }
}
#[doc = "AON SYSCONSAIF SYSCFG 16\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aon_syscfg_4::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AonSyscfg4Spec;
impl crate::RegisterSpec for AonSyscfg4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aon_syscfg_4::R`](R) reader structure"]
impl crate::Readable for AonSyscfg4Spec {}
#[doc = "`reset()` method sets aon_syscfg_4 to value 0"]
impl crate::Resettable for AonSyscfg4Spec {
    const RESET_VALUE: u32 = 0;
}
