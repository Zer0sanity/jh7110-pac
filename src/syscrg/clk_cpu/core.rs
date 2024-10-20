#[doc = "Register `core` reader"]
pub type R = crate::R<CoreSpec>;
#[doc = "Register `core` writer"]
pub type W = crate::W<CoreSpec>;
#[doc = "Field `clk_divcfg` reader - Clock divider coefficient: Max=7, Default=1, Min=1, Typical=1"]
pub type ClkDivcfgR = crate::FieldReader<u32>;
#[doc = "Field `clk_divcfg` writer - Clock divider coefficient: Max=7, Default=1, Min=1, Typical=1"]
pub type ClkDivcfgW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - Clock divider coefficient: Max=7, Default=1, Min=1, Typical=1"]
    #[inline(always)]
    pub fn clk_divcfg(&self) -> ClkDivcfgR {
        ClkDivcfgR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - Clock divider coefficient: Max=7, Default=1, Min=1, Typical=1"]
    #[inline(always)]
    #[must_use]
    pub fn clk_divcfg(&mut self) -> ClkDivcfgW<CoreSpec> {
        ClkDivcfgW::new(self, 0)
    }
}
#[doc = "Clock CPU Core\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CoreSpec;
impl crate::RegisterSpec for CoreSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core::R`](R) reader structure"]
impl crate::Readable for CoreSpec {}
#[doc = "`write(|w| ..)` method takes [`core::W`](W) writer structure"]
impl crate::Writable for CoreSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets core to value 0x01"]
impl crate::Resettable for CoreSpec {
    const RESET_VALUE: u32 = 0x01;
}
