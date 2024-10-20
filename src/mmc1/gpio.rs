#[doc = "Register `gpio` reader"]
pub type R = crate::R<GpioSpec>;
#[doc = "Register `gpio` writer"]
pub type W = crate::W<GpioSpec>;
#[doc = "Field `gpio` reader - MMC GPIO"]
pub type GpioR = crate::FieldReader<u32>;
#[doc = "Field `gpio` writer - MMC GPIO"]
pub type GpioW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - MMC GPIO"]
    #[inline(always)]
    pub fn gpio(&self) -> GpioR {
        GpioR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - MMC GPIO"]
    #[inline(always)]
    #[must_use]
    pub fn gpio(&mut self) -> GpioW<GpioSpec> {
        GpioW::new(self, 0)
    }
}
#[doc = "MMC GPIO\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpioSpec;
impl crate::RegisterSpec for GpioSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio::R`](R) reader structure"]
impl crate::Readable for GpioSpec {}
#[doc = "`write(|w| ..)` method takes [`gpio::W`](W) writer structure"]
impl crate::Writable for GpioSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets gpio to value 0"]
impl crate::Resettable for GpioSpec {
    const RESET_VALUE: u32 = 0;
}
