#[doc = "Register `ssp_pcell_id[%s]` reader"]
pub type R = crate::R<SspPcellIdSpec>;
#[doc = "Field `ssp_pcell_id` reader - The bits of the SSPCELLID are read as: \\[0x0d, 0xf0, 0x05, 0xb1\\]"]
pub type SspPcellIdR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - The bits of the SSPCELLID are read as: \\[0x0d, 0xf0, 0x05, 0xb1\\]"]
    #[inline(always)]
    pub fn ssp_pcell_id(&self) -> SspPcellIdR {
        SspPcellIdR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "The SSPPCellID0-3 registers are four 8-bit wide registers, that span address locations 0xFF0-0xFFC. The registers can conceptually be treated as a 32-bit register. The register is used as a standard cross-peripheral identification system. The SSPPCellID register is set to 0xB105F00D.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ssp_pcell_id::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SspPcellIdSpec;
impl crate::RegisterSpec for SspPcellIdSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ssp_pcell_id::R`](R) reader structure"]
impl crate::Readable for SspPcellIdSpec {}
#[doc = "`reset()` method sets ssp_pcell_id[%s]
to value 0"]
impl crate::Resettable for SspPcellIdSpec {
    const RESET_VALUE: u16 = 0;
}
