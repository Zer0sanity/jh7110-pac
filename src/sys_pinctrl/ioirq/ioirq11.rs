#[doc = "Register `ioirq11` reader"]
pub type R = crate::R<Ioirq11Spec>;
#[doc = "Field `ris0` reader - Status of the edge trigger. The register can be cleared by writing gpio ic"]
pub type Ris0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Status of the edge trigger. The register can be cleared by writing gpio ic"]
    #[inline(always)]
    pub fn ris0(&self) -> Ris0R {
        Ris0R::new(self.bits)
    }
}
#[doc = "SYS IOMUX CFGSAIF SYSCFG IOIRQ 44: GPIO Register Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioirq11::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ioirq11Spec;
impl crate::RegisterSpec for Ioirq11Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ioirq11::R`](R) reader structure"]
impl crate::Readable for Ioirq11Spec {}
#[doc = "`reset()` method sets ioirq11 to value 0"]
impl crate::Resettable for Ioirq11Spec {
    const RESET_VALUE: u32 = 0;
}
