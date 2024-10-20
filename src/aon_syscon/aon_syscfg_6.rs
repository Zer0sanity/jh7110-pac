#[doc = "Register `aon_syscfg_6` reader"]
pub type R = crate::R<AonSyscfg6Spec>;
#[doc = "Field `u0_otpc_chip_mode` reader - "]
pub type U0OtpcChipModeR = crate::BitReader;
#[doc = "Field `u0_otpc_crc_pass` reader - "]
pub type U0OtpcCrcPassR = crate::BitReader;
#[doc = "Field `u0_otpc_dbg_enable` reader - "]
pub type U0OtpcDbgEnableR = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn u0_otpc_chip_mode(&self) -> U0OtpcChipModeR {
        U0OtpcChipModeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn u0_otpc_crc_pass(&self) -> U0OtpcCrcPassR {
        U0OtpcCrcPassR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn u0_otpc_dbg_enable(&self) -> U0OtpcDbgEnableR {
        U0OtpcDbgEnableR::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "AON SYSCONSAIF SYSCFG 24\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`aon_syscfg_6::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AonSyscfg6Spec;
impl crate::RegisterSpec for AonSyscfg6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aon_syscfg_6::R`](R) reader structure"]
impl crate::Readable for AonSyscfg6Spec {}
#[doc = "`reset()` method sets aon_syscfg_6 to value 0"]
impl crate::Resettable for AonSyscfg6Spec {
    const RESET_VALUE: u32 = 0;
}
