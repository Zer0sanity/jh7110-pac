#[doc = "Register `clk_usb_125m` reader"]
pub type R = crate::R<ClkUsb125mSpec>;
#[doc = "Register `clk_usb_125m` writer"]
pub type W = crate::W<ClkUsb125mSpec>;
#[doc = "Field `clk_divcfg` reader - Clock divider coefficient: Max=15, Default=8, Min=12, Typical=10"]
pub type ClkDivcfgR = crate::FieldReader<u32>;
#[doc = "Field `clk_divcfg` writer - Clock divider coefficient: Max=15, Default=8, Min=12, Typical=10"]
pub type ClkDivcfgW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - Clock divider coefficient: Max=15, Default=8, Min=12, Typical=10"]
    #[inline(always)]
    pub fn clk_divcfg(&self) -> ClkDivcfgR {
        ClkDivcfgR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - Clock divider coefficient: Max=15, Default=8, Min=12, Typical=10"]
    #[inline(always)]
    #[must_use]
    pub fn clk_divcfg(&mut self) -> ClkDivcfgW<ClkUsb125mSpec> {
        ClkDivcfgW::new(self, 0)
    }
}
#[doc = "Clock USB 125M\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_usb_125m::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_usb_125m::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkUsb125mSpec;
impl crate::RegisterSpec for ClkUsb125mSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_usb_125m::R`](R) reader structure"]
impl crate::Readable for ClkUsb125mSpec {}
#[doc = "`write(|w| ..)` method takes [`clk_usb_125m::W`](W) writer structure"]
impl crate::Writable for ClkUsb125mSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets clk_usb_125m to value 0x08"]
impl crate::Resettable for ClkUsb125mSpec {
    const RESET_VALUE: u32 = 0x08;
}
