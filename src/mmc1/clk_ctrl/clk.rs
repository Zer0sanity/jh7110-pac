#[doc = "Register `clk[%s]` reader"]
pub type R = crate::R<ClkSpec>;
#[doc = "Register `clk[%s]` writer"]
pub type W = crate::W<ClkSpec>;
#[doc = "Field `clk` reader - MMC Clock Configuration"]
pub type ClkR = crate::FieldReader<u32>;
#[doc = "Field `clk` writer - MMC Clock Configuration"]
pub type ClkW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - MMC Clock Configuration"]
    #[inline(always)]
    pub fn clk(&self) -> ClkR {
        ClkR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - MMC Clock Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn clk(&mut self) -> ClkW<ClkSpec> {
        ClkW::new(self, 0)
    }
}
#[doc = "MMC Clock Configuration - 0: clkdiv, 1: clksrc\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkSpec;
impl crate::RegisterSpec for ClkSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk::R`](R) reader structure"]
impl crate::Readable for ClkSpec {}
#[doc = "`write(|w| ..)` method takes [`clk::W`](W) writer structure"]
impl crate::Writable for ClkSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets clk[%s]
to value 0"]
impl crate::Resettable for ClkSpec {
    const RESET_VALUE: u32 = 0;
}
