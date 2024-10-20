#[doc = "Register `pcell_id1` reader"]
pub type R = crate::R<PcellId1Spec>;
#[doc = "Field `pcell_id` reader - These bits read back as 0xF0"]
pub type PcellIdR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - These bits read back as 0xF0"]
    #[inline(always)]
    pub fn pcell_id(&self) -> PcellIdR {
        PcellIdR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "DMA PrimeCell ID 1 register - is hard-coded and the fields in the register determine the reset value.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcell_id1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcellId1Spec;
impl crate::RegisterSpec for PcellId1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcell_id1::R`](R) reader structure"]
impl crate::Readable for PcellId1Spec {}
#[doc = "`reset()` method sets pcell_id1 to value 0xf0"]
impl crate::Resettable for PcellId1Spec {
    const RESET_VALUE: u32 = 0xf0;
}
