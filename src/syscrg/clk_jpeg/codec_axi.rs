#[doc = "Register `codec_axi` reader"]
pub type R = crate::R<CodecAxiSpec>;
#[doc = "Register `codec_axi` writer"]
pub type W = crate::W<CodecAxiSpec>;
#[doc = "Field `clk_divcfg` reader - Clock divider coefficient: Max=16, Default=6, Min=6, Typical=6"]
pub type ClkDivcfgR = crate::FieldReader<u32>;
#[doc = "Field `clk_divcfg` writer - Clock divider coefficient: Max=16, Default=6, Min=6, Typical=6"]
pub type ClkDivcfgW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - Clock divider coefficient: Max=16, Default=6, Min=6, Typical=6"]
    #[inline(always)]
    pub fn clk_divcfg(&self) -> ClkDivcfgR {
        ClkDivcfgR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - Clock divider coefficient: Max=16, Default=6, Min=6, Typical=6"]
    #[inline(always)]
    #[must_use]
    pub fn clk_divcfg(&mut self) -> ClkDivcfgW<CodecAxiSpec> {
        ClkDivcfgW::new(self, 0)
    }
}
#[doc = "Clock JPEG Codec AXI\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`codec_axi::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`codec_axi::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CodecAxiSpec;
impl crate::RegisterSpec for CodecAxiSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`codec_axi::R`](R) reader structure"]
impl crate::Readable for CodecAxiSpec {}
#[doc = "`write(|w| ..)` method takes [`codec_axi::W`](W) writer structure"]
impl crate::Writable for CodecAxiSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets codec_axi to value 0x06"]
impl crate::Resettable for CodecAxiSpec {
    const RESET_VALUE: u32 = 0x06;
}
