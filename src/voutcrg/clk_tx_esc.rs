#[doc = "Register `clk_tx_esc` reader"]
pub type R = crate::R<ClkTxEscSpec>;
#[doc = "Register `clk_tx_esc` writer"]
pub type W = crate::W<ClkTxEscSpec>;
#[doc = "Field `clk_divcfg` reader - Clock divider coefficient: Max=31, Default=12, Min=10, Typical=12"]
pub type ClkDivcfgR = crate::FieldReader<u32>;
#[doc = "Field `clk_divcfg` writer - Clock divider coefficient: Max=31, Default=12, Min=10, Typical=12"]
pub type ClkDivcfgW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - Clock divider coefficient: Max=31, Default=12, Min=10, Typical=12"]
    #[inline(always)]
    pub fn clk_divcfg(&self) -> ClkDivcfgR {
        ClkDivcfgR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - Clock divider coefficient: Max=31, Default=12, Min=10, Typical=12"]
    #[inline(always)]
    #[must_use]
    pub fn clk_divcfg(&mut self) -> ClkDivcfgW<ClkTxEscSpec> {
        ClkDivcfgW::new(self, 0)
    }
}
#[doc = "Clock Transmit Escape\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_tx_esc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_tx_esc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkTxEscSpec;
impl crate::RegisterSpec for ClkTxEscSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_tx_esc::R`](R) reader structure"]
impl crate::Readable for ClkTxEscSpec {}
#[doc = "`write(|w| ..)` method takes [`clk_tx_esc::W`](W) writer structure"]
impl crate::Writable for ClkTxEscSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets clk_tx_esc to value 0x0c"]
impl crate::Resettable for ClkTxEscSpec {
    const RESET_VALUE: u32 = 0x0c;
}
