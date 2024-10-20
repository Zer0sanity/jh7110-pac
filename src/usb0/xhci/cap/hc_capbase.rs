#[doc = "Register `hc_capbase` reader"]
pub type R = crate::R<HcCapbaseSpec>;
#[doc = "Field `hc_length` reader - USB3 XHCI length of the `hc_capbase` register."]
pub type HcLengthR = crate::FieldReader;
#[doc = "Field `hc_version` reader - USB3 XHCI length of the `hc_capbase` register."]
pub type HcVersionR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:7 - USB3 XHCI length of the `hc_capbase` register."]
    #[inline(always)]
    pub fn hc_length(&self) -> HcLengthR {
        HcLengthR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - USB3 XHCI length of the `hc_capbase` register."]
    #[inline(always)]
    pub fn hc_version(&self) -> HcVersionR {
        HcVersionR::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
#[doc = "USB3 XHCI host controller capability base - defines the offset of the `op` register cluster.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hc_capbase::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HcCapbaseSpec;
impl crate::RegisterSpec for HcCapbaseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hc_capbase::R`](R) reader structure"]
impl crate::Readable for HcCapbaseSpec {}
#[doc = "`reset()` method sets hc_capbase to value 0"]
impl crate::Resettable for HcCapbaseSpec {
    const RESET_VALUE: u32 = 0;
}
