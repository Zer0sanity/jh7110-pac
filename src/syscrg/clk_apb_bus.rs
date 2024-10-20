#[doc = "Register `clk_apb_bus` reader"]
pub type R = crate::R<ClkApbBusSpec>;
#[doc = "Register `clk_apb_bus` writer"]
pub type W = crate::W<ClkApbBusSpec>;
#[doc = "Field `clk_divcfg` reader - Clock divider coefficient: Max=8, Default=4, Min=4, Typical=4"]
pub type ClkDivcfgR = crate::FieldReader<u32>;
#[doc = "Field `clk_divcfg` writer - Clock divider coefficient: Max=8, Default=4, Min=4, Typical=4"]
pub type ClkDivcfgW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - Clock divider coefficient: Max=8, Default=4, Min=4, Typical=4"]
    #[inline(always)]
    pub fn clk_divcfg(&self) -> ClkDivcfgR {
        ClkDivcfgR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - Clock divider coefficient: Max=8, Default=4, Min=4, Typical=4"]
    #[inline(always)]
    #[must_use]
    pub fn clk_divcfg(&mut self) -> ClkDivcfgW<ClkApbBusSpec> {
        ClkDivcfgW::new(self, 0)
    }
}
#[doc = "Clock APB Bus\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_apb_bus::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_apb_bus::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkApbBusSpec;
impl crate::RegisterSpec for ClkApbBusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_apb_bus::R`](R) reader structure"]
impl crate::Readable for ClkApbBusSpec {}
#[doc = "`write(|w| ..)` method takes [`clk_apb_bus::W`](W) writer structure"]
impl crate::Writable for ClkApbBusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets clk_apb_bus to value 0x04"]
impl crate::Resettable for ClkApbBusSpec {
    const RESET_VALUE: u32 = 0x04;
}
