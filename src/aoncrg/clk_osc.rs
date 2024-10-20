#[doc = "Register `clk_osc` reader"]
pub type R = crate::R<ClkOscSpec>;
#[doc = "Register `clk_osc` writer"]
pub type W = crate::W<ClkOscSpec>;
#[doc = "Field `clk_divcfg` reader - Clock divider coefficient: Max=4, Default=4, Min=4, Typical=4"]
pub type ClkDivcfgR = crate::FieldReader<u32>;
#[doc = "Field `clk_divcfg` writer - Clock divider coefficient: Max=4, Default=4, Min=4, Typical=4"]
pub type ClkDivcfgW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - Clock divider coefficient: Max=4, Default=4, Min=4, Typical=4"]
    #[inline(always)]
    pub fn clk_divcfg(&self) -> ClkDivcfgR {
        ClkDivcfgR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - Clock divider coefficient: Max=4, Default=4, Min=4, Typical=4"]
    #[inline(always)]
    #[must_use]
    pub fn clk_divcfg(&mut self) -> ClkDivcfgW<ClkOscSpec> {
        ClkDivcfgW::new(self, 0)
    }
}
#[doc = "Oscillator Clock\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_osc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_osc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkOscSpec;
impl crate::RegisterSpec for ClkOscSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_osc::R`](R) reader structure"]
impl crate::Readable for ClkOscSpec {}
#[doc = "`write(|w| ..)` method takes [`clk_osc::W`](W) writer structure"]
impl crate::Writable for ClkOscSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets clk_osc to value 0x04"]
impl crate::Resettable for ClkOscSpec {
    const RESET_VALUE: u32 = 0x04;
}
