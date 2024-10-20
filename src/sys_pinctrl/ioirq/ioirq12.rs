#[doc = "Register `ioirq12` reader"]
pub type R = crate::R<Ioirq12Spec>;
#[doc = "Field `ris1` reader - Status of the edge trigger. The register can be cleared by writing gpio ic"]
pub type Ris1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Status of the edge trigger. The register can be cleared by writing gpio ic"]
    #[inline(always)]
    pub fn ris1(&self) -> Ris1R {
        Ris1R::new(self.bits)
    }
}
#[doc = "SYS IOMUX CFGSAIF SYSCFG IOIRQ 48: GPIO Register Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ioirq12::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ioirq12Spec;
impl crate::RegisterSpec for Ioirq12Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ioirq12::R`](R) reader structure"]
impl crate::Readable for Ioirq12Spec {}
#[doc = "`reset()` method sets ioirq12 to value 0"]
impl crate::Resettable for Ioirq12Spec {
    const RESET_VALUE: u32 = 0;
}
