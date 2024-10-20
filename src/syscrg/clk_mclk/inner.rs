#[doc = "Register `inner` reader"]
pub type R = crate::R<InnerSpec>;
#[doc = "Register `inner` writer"]
pub type W = crate::W<InnerSpec>;
#[doc = "Field `clk_divcfg` reader - Clock divider coefficient: Max=64, Default=12, Min=12, Typical=12"]
pub type ClkDivcfgR = crate::FieldReader<u32>;
#[doc = "Field `clk_divcfg` writer - Clock divider coefficient: Max=64, Default=12, Min=12, Typical=12"]
pub type ClkDivcfgW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - Clock divider coefficient: Max=64, Default=12, Min=12, Typical=12"]
    #[inline(always)]
    pub fn clk_divcfg(&self) -> ClkDivcfgR {
        ClkDivcfgR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - Clock divider coefficient: Max=64, Default=12, Min=12, Typical=12"]
    #[inline(always)]
    #[must_use]
    pub fn clk_divcfg(&mut self) -> ClkDivcfgW<InnerSpec> {
        ClkDivcfgW::new(self, 0)
    }
}
#[doc = "Clock MCLK Inner\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inner::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inner::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InnerSpec;
impl crate::RegisterSpec for InnerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inner::R`](R) reader structure"]
impl crate::Readable for InnerSpec {}
#[doc = "`write(|w| ..)` method takes [`inner::W`](W) writer structure"]
impl crate::Writable for InnerSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets inner to value 0x0c"]
impl crate::Resettable for InnerSpec {
    const RESET_VALUE: u32 = 0x0c;
}
