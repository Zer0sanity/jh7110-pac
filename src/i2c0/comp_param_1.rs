#[doc = "Register `comp_param_1` reader"]
pub type R = crate::R<CompParam1Spec>;
#[doc = "Field `speed` reader - Speed mask - 01: Standard, 10: Full, 11: High"]
pub type SpeedR = crate::FieldReader;
impl R {
    #[doc = "Bits 2:3 - Speed mask - 01: Standard, 10: Full, 11: High"]
    #[inline(always)]
    pub fn speed(&self) -> SpeedR {
        SpeedR::new(((self.bits >> 2) & 3) as u8)
    }
}
#[doc = "DesignWare I2C Compatibility Parameter 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comp_param_1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CompParam1Spec;
impl crate::RegisterSpec for CompParam1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`comp_param_1::R`](R) reader structure"]
impl crate::Readable for CompParam1Spec {}
#[doc = "`reset()` method sets comp_param_1 to value 0"]
impl crate::Resettable for CompParam1Spec {
    const RESET_VALUE: u32 = 0;
}
