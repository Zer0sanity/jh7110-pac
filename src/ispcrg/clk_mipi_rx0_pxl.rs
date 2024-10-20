#[doc = "Register `clk_mipi_rx0_pxl` reader"]
pub type R = crate::R<ClkMipiRx0PxlSpec>;
#[doc = "Register `clk_mipi_rx0_pxl` writer"]
pub type W = crate::W<ClkMipiRx0PxlSpec>;
#[doc = "Field `clk_divcfg` reader - Clock divider coefficient: Max=8, Default=3, Min=2, Typical=3"]
pub type ClkDivcfgR = crate::FieldReader<u32>;
#[doc = "Field `clk_divcfg` writer - Clock divider coefficient: Max=8, Default=3, Min=2, Typical=3"]
pub type ClkDivcfgW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - Clock divider coefficient: Max=8, Default=3, Min=2, Typical=3"]
    #[inline(always)]
    pub fn clk_divcfg(&self) -> ClkDivcfgR {
        ClkDivcfgR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - Clock divider coefficient: Max=8, Default=3, Min=2, Typical=3"]
    #[inline(always)]
    #[must_use]
    pub fn clk_divcfg(&mut self) -> ClkDivcfgW<ClkMipiRx0PxlSpec> {
        ClkDivcfgW::new(self, 0)
    }
}
#[doc = "Clock MIPI RX 0 Pixel\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_mipi_rx0_pxl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_mipi_rx0_pxl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkMipiRx0PxlSpec;
impl crate::RegisterSpec for ClkMipiRx0PxlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_mipi_rx0_pxl::R`](R) reader structure"]
impl crate::Readable for ClkMipiRx0PxlSpec {}
#[doc = "`write(|w| ..)` method takes [`clk_mipi_rx0_pxl::W`](W) writer structure"]
impl crate::Writable for ClkMipiRx0PxlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets clk_mipi_rx0_pxl to value 0x03"]
impl crate::Resettable for ClkMipiRx0PxlSpec {
    const RESET_VALUE: u32 = 0x03;
}
