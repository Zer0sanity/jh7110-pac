#[doc = "Register `cntr` reader"]
pub type R = crate::R<CntrSpec>;
#[doc = "Register `cntr` writer"]
pub type W = crate::W<CntrSpec>;
#[doc = "Field `cntr` reader - "]
pub type CntrR = crate::FieldReader<u32>;
#[doc = "Field `cntr` writer - "]
pub type CntrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn cntr(&self) -> CntrR {
        CntrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn cntr(&mut self) -> CntrW<CntrSpec> {
        CntrW::new(self, 0)
    }
}
#[doc = "Opencores PTC PWM v1 CNTR is the actual counter register. It is incremented at every counter/timer clock cycle. Source clock is either system clock or ptc_ecgt eclk/gate input. Selection between both clocks is performed with the RPTC_CTRL\\[ECLK\\]. Active edge of external clock is selected with the RPTC_CTRL\\[NEC\\]. In order to count, RPTC_CNTR must first be enabled with the RPTC_CTRL\\[EN\\]. RPTC_CNTR can be reset with the RPTC_CTRL\\[RST\\]. RPTC_CNTR can operate in either single-run mode or continues mode. Mode is selected with the RPTC_CTRL\\[SINGLE\\].\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cntr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cntr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CntrSpec;
impl crate::RegisterSpec for CntrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cntr::R`](R) reader structure"]
impl crate::Readable for CntrSpec {}
#[doc = "`write(|w| ..)` method takes [`cntr::W`](W) writer structure"]
impl crate::Writable for CntrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets cntr to value 0"]
impl crate::Resettable for CntrSpec {
    const RESET_VALUE: u32 = 0;
}
