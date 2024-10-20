#[doc = "Register `clk_dom4_apb_func` reader"]
pub type R = crate::R<ClkDom4ApbFuncSpec>;
#[doc = "Register `clk_dom4_apb_func` writer"]
pub type W = crate::W<ClkDom4ApbFuncSpec>;
#[doc = "Field `clk_divcfg` reader - Clock divider coefficient: Max=15, Default=6, Min=6, Typical=6"]
pub type ClkDivcfgR = crate::FieldReader<u32>;
#[doc = "Field `clk_divcfg` writer - Clock divider coefficient: Max=15, Default=6, Min=6, Typical=6"]
pub type ClkDivcfgW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - Clock divider coefficient: Max=15, Default=6, Min=6, Typical=6"]
    #[inline(always)]
    pub fn clk_divcfg(&self) -> ClkDivcfgR {
        ClkDivcfgR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - Clock divider coefficient: Max=15, Default=6, Min=6, Typical=6"]
    #[inline(always)]
    #[must_use]
    pub fn clk_divcfg(&mut self) -> ClkDivcfgW<ClkDom4ApbFuncSpec> {
        ClkDivcfgW::new(self, 0)
    }
}
#[doc = "Clock DOM4 APB Function\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_dom4_apb_func::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_dom4_apb_func::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkDom4ApbFuncSpec;
impl crate::RegisterSpec for ClkDom4ApbFuncSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_dom4_apb_func::R`](R) reader structure"]
impl crate::Readable for ClkDom4ApbFuncSpec {}
#[doc = "`write(|w| ..)` method takes [`clk_dom4_apb_func::W`](W) writer structure"]
impl crate::Writable for ClkDom4ApbFuncSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets clk_dom4_apb_func to value 0x06"]
impl crate::Resettable for ClkDom4ApbFuncSpec {
    const RESET_VALUE: u32 = 0x06;
}
