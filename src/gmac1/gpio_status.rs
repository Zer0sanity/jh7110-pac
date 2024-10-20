#[doc = "Register `gpio_status` reader"]
pub type R = crate::R<GpioStatusSpec>;
#[doc = "Register `gpio_status` writer"]
pub type W = crate::W<GpioStatusSpec>;
#[doc = "Field `gpo(0-3)` reader - MAC GPIO GPO Status"]
pub type GpoR = crate::BitReader;
#[doc = "Field `gpo(0-3)` writer - MAC GPIO GPO Status"]
pub type GpoW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "MAC GPIO GPO Status"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `gpo0` field"]
    #[inline(always)]
    pub fn gpo(&self, n: u8) -> GpoR {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        GpoR::new(((self.bits >> (n + 16)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "MAC GPIO GPO Status"]
    #[inline(always)]
    pub fn gpo_iter(&self) -> impl Iterator<Item = GpoR> + '_ {
        (0..4).map(move |n| GpoR::new(((self.bits >> (n + 16)) & 1) != 0))
    }
    #[doc = "Bit 16 - MAC GPIO GPO Status"]
    #[inline(always)]
    pub fn gpo0(&self) -> GpoR {
        GpoR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - MAC GPIO GPO Status"]
    #[inline(always)]
    pub fn gpo1(&self) -> GpoR {
        GpoR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - MAC GPIO GPO Status"]
    #[inline(always)]
    pub fn gpo2(&self) -> GpoR {
        GpoR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - MAC GPIO GPO Status"]
    #[inline(always)]
    pub fn gpo3(&self) -> GpoR {
        GpoR::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "MAC GPIO GPO Status"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `gpo0` field"]
    #[inline(always)]
    #[must_use]
    pub fn gpo(&mut self, n: u8) -> GpoW<GpioStatusSpec> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        GpoW::new(self, n + 16)
    }
    #[doc = "Bit 16 - MAC GPIO GPO Status"]
    #[inline(always)]
    #[must_use]
    pub fn gpo0(&mut self) -> GpoW<GpioStatusSpec> {
        GpoW::new(self, 16)
    }
    #[doc = "Bit 17 - MAC GPIO GPO Status"]
    #[inline(always)]
    #[must_use]
    pub fn gpo1(&mut self) -> GpoW<GpioStatusSpec> {
        GpoW::new(self, 17)
    }
    #[doc = "Bit 18 - MAC GPIO GPO Status"]
    #[inline(always)]
    #[must_use]
    pub fn gpo2(&mut self) -> GpoW<GpioStatusSpec> {
        GpoW::new(self, 18)
    }
    #[doc = "Bit 19 - MAC GPIO GPO Status"]
    #[inline(always)]
    #[must_use]
    pub fn gpo3(&mut self) -> GpoW<GpioStatusSpec> {
        GpoW::new(self, 19)
    }
}
#[doc = "MAC GPIO Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GpioStatusSpec;
impl crate::RegisterSpec for GpioStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_status::R`](R) reader structure"]
impl crate::Readable for GpioStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`gpio_status::W`](W) writer structure"]
impl crate::Writable for GpioStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets gpio_status to value 0"]
impl crate::Resettable for GpioStatusSpec {
    const RESET_VALUE: u32 = 0;
}
