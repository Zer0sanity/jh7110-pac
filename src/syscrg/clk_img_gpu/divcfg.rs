#[doc = "Register `divcfg` reader"]
pub type R = crate::R<DivcfgSpec>;
#[doc = "Register `divcfg` writer"]
pub type W = crate::W<DivcfgSpec>;
#[doc = "Field `clk_divcfg` reader - Clock divider coefficient: Max=7, Default=3, Min=3, Typical=3"]
pub type ClkDivcfgR = crate::FieldReader<u32>;
#[doc = "Field `clk_divcfg` writer - Clock divider coefficient: Max=7, Default=3, Min=3, Typical=3"]
pub type ClkDivcfgW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - Clock divider coefficient: Max=7, Default=3, Min=3, Typical=3"]
    #[inline(always)]
    pub fn clk_divcfg(&self) -> ClkDivcfgR {
        ClkDivcfgR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - Clock divider coefficient: Max=7, Default=3, Min=3, Typical=3"]
    #[inline(always)]
    #[must_use]
    pub fn clk_divcfg(&mut self) -> ClkDivcfgW<DivcfgSpec> {
        ClkDivcfgW::new(self, 0)
    }
}
#[doc = "Clock IMG GPU Core DIVCFG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`divcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`divcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DivcfgSpec;
impl crate::RegisterSpec for DivcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`divcfg::R`](R) reader structure"]
impl crate::Readable for DivcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`divcfg::W`](W) writer structure"]
impl crate::Writable for DivcfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets divcfg to value 0x03"]
impl crate::Resettable for DivcfgSpec {
    const RESET_VALUE: u32 = 0x03;
}
