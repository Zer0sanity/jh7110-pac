#[doc = "Register `pcell_id0` reader"]
pub type R = crate::R<PcellId0Spec>;
#[doc = "Field `pcell_id` reader - These bits read back as 0x80"]
pub type PcellIdR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - These bits read back as 0x80"]
    #[inline(always)]
    pub fn pcell_id(&self) -> PcellIdR {
        PcellIdR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "DMA PrimeCell ID 0 register - is hard-coded and the fields in the register determine the reset value.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcell_id0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcellId0Spec;
impl crate::RegisterSpec for PcellId0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcell_id0::R`](R) reader structure"]
impl crate::Readable for PcellId0Spec {}
#[doc = "`reset()` method sets pcell_id0 to value 0x0d"]
impl crate::Resettable for PcellId0Spec {
    const RESET_VALUE: u32 = 0x0d;
}
