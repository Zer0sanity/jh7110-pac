#[doc = "Register `pcell_id3` reader"]
pub type R = crate::R<PcellId3Spec>;
#[doc = "Field `pcell_id` reader - These bits read back as 0xB1"]
pub type PcellIdR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - These bits read back as 0xB1"]
    #[inline(always)]
    pub fn pcell_id(&self) -> PcellIdR {
        PcellIdR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "DMA PrimeCell ID 3 register - is hard-coded and the fields in the register determine the reset value.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcell_id3::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcellId3Spec;
impl crate::RegisterSpec for PcellId3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcell_id3::R`](R) reader structure"]
impl crate::Readable for PcellId3Spec {}
#[doc = "`reset()` method sets pcell_id3 to value 0xb1"]
impl crate::Resettable for PcellId3Spec {
    const RESET_VALUE: u32 = 0xb1;
}
