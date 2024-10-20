#[doc = "Register `axi_divcfg` reader"]
pub type R = crate::R<AxiDivcfgSpec>;
#[doc = "Register `axi_divcfg` writer"]
pub type W = crate::W<AxiDivcfgSpec>;
#[doc = "Field `clk_divcfg` reader - Clock divider coefficient: Max=7, Default=2, Min=2, Typical=2"]
pub type ClkDivcfgR = crate::FieldReader<u32>;
#[doc = "Field `clk_divcfg` writer - Clock divider coefficient: Max=7, Default=2, Min=2, Typical=2"]
pub type ClkDivcfgW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - Clock divider coefficient: Max=7, Default=2, Min=2, Typical=2"]
    #[inline(always)]
    pub fn clk_divcfg(&self) -> ClkDivcfgR {
        ClkDivcfgR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - Clock divider coefficient: Max=7, Default=2, Min=2, Typical=2"]
    #[inline(always)]
    #[must_use]
    pub fn clk_divcfg(&mut self) -> ClkDivcfgW<AxiDivcfgSpec> {
        ClkDivcfgW::new(self, 0)
    }
}
#[doc = "Clock Video Output AXI DIVCFG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`axi_divcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`axi_divcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AxiDivcfgSpec;
impl crate::RegisterSpec for AxiDivcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`axi_divcfg::R`](R) reader structure"]
impl crate::Readable for AxiDivcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`axi_divcfg::W`](W) writer structure"]
impl crate::Writable for AxiDivcfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets axi_divcfg to value 0x02"]
impl crate::Resettable for AxiDivcfgSpec {
    const RESET_VALUE: u32 = 0x02;
}
