#[doc = "Register `sys_syscfg3` reader"]
pub type R = crate::R<SysSyscfg3Spec>;
#[doc = "Register `sys_syscfg3` writer"]
pub type W = crate::W<SysSyscfg3Spec>;
#[doc = "Field `vout0_remap_awaddr_gpio0` reader - 0: GPIO Group 0 (GPIO21-35) voltage select 3.3V, 1: GPIO Group 0 (GPIO21-35) voltage select 1.8V"]
pub type Vout0RemapAwaddrGpio0R = crate::BitReader;
#[doc = "Field `vout0_remap_awaddr_gpio0` writer - 0: GPIO Group 0 (GPIO21-35) voltage select 3.3V, 1: GPIO Group 0 (GPIO21-35) voltage select 1.8V"]
pub type Vout0RemapAwaddrGpio0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `vout0_remap_awaddr_gpio1` reader - 0: GPIO Group 1 (GPIO36-63) voltage select 3.3V, 1: GPIO Group 1 (GPIO36-63) voltage select 1.8V"]
pub type Vout0RemapAwaddrGpio1R = crate::BitReader;
#[doc = "Field `vout0_remap_awaddr_gpio1` writer - 0: GPIO Group 1 (GPIO36-63) voltage select 3.3V, 1: GPIO Group 1 (GPIO36-63) voltage select 1.8V"]
pub type Vout0RemapAwaddrGpio1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `vout0_remap_awaddr_gpio2` reader - 0: GPIO Group 2 (GPIO0-6) voltage select 3.3V, 1: GPIO Group 2 (GPIO0-6) voltage select 1.8V"]
pub type Vout0RemapAwaddrGpio2R = crate::BitReader;
#[doc = "Field `vout0_remap_awaddr_gpio2` writer - 0: GPIO Group 2 (GPIO0-6) voltage select 3.3V, 1: GPIO Group 2 (GPIO0-6) voltage select 1.8V"]
pub type Vout0RemapAwaddrGpio2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `vout0_remap_awaddr_gpio3` reader - 0: GPIO Group 3 (GPIO7-20) voltage select 3.3V, 1: GPIO Group 3 (GPIO7-20) voltage select 1.8V"]
pub type Vout0RemapAwaddrGpio3R = crate::BitReader;
#[doc = "Field `vout0_remap_awaddr_gpio3` writer - 0: GPIO Group 3 (GPIO7-20) voltage select 3.3V, 1: GPIO Group 3 (GPIO7-20) voltage select 1.8V"]
pub type Vout0RemapAwaddrGpio3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0: GPIO Group 0 (GPIO21-35) voltage select 3.3V, 1: GPIO Group 0 (GPIO21-35) voltage select 1.8V"]
    #[inline(always)]
    pub fn vout0_remap_awaddr_gpio0(&self) -> Vout0RemapAwaddrGpio0R {
        Vout0RemapAwaddrGpio0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 0: GPIO Group 1 (GPIO36-63) voltage select 3.3V, 1: GPIO Group 1 (GPIO36-63) voltage select 1.8V"]
    #[inline(always)]
    pub fn vout0_remap_awaddr_gpio1(&self) -> Vout0RemapAwaddrGpio1R {
        Vout0RemapAwaddrGpio1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 0: GPIO Group 2 (GPIO0-6) voltage select 3.3V, 1: GPIO Group 2 (GPIO0-6) voltage select 1.8V"]
    #[inline(always)]
    pub fn vout0_remap_awaddr_gpio2(&self) -> Vout0RemapAwaddrGpio2R {
        Vout0RemapAwaddrGpio2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 0: GPIO Group 3 (GPIO7-20) voltage select 3.3V, 1: GPIO Group 3 (GPIO7-20) voltage select 1.8V"]
    #[inline(always)]
    pub fn vout0_remap_awaddr_gpio3(&self) -> Vout0RemapAwaddrGpio3R {
        Vout0RemapAwaddrGpio3R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0: GPIO Group 0 (GPIO21-35) voltage select 3.3V, 1: GPIO Group 0 (GPIO21-35) voltage select 1.8V"]
    #[inline(always)]
    #[must_use]
    pub fn vout0_remap_awaddr_gpio0(&mut self) -> Vout0RemapAwaddrGpio0W<SysSyscfg3Spec> {
        Vout0RemapAwaddrGpio0W::new(self, 0)
    }
    #[doc = "Bit 1 - 0: GPIO Group 1 (GPIO36-63) voltage select 3.3V, 1: GPIO Group 1 (GPIO36-63) voltage select 1.8V"]
    #[inline(always)]
    #[must_use]
    pub fn vout0_remap_awaddr_gpio1(&mut self) -> Vout0RemapAwaddrGpio1W<SysSyscfg3Spec> {
        Vout0RemapAwaddrGpio1W::new(self, 1)
    }
    #[doc = "Bit 2 - 0: GPIO Group 2 (GPIO0-6) voltage select 3.3V, 1: GPIO Group 2 (GPIO0-6) voltage select 1.8V"]
    #[inline(always)]
    #[must_use]
    pub fn vout0_remap_awaddr_gpio2(&mut self) -> Vout0RemapAwaddrGpio2W<SysSyscfg3Spec> {
        Vout0RemapAwaddrGpio2W::new(self, 2)
    }
    #[doc = "Bit 3 - 0: GPIO Group 3 (GPIO7-20) voltage select 3.3V, 1: GPIO Group 3 (GPIO7-20) voltage select 1.8V"]
    #[inline(always)]
    #[must_use]
    pub fn vout0_remap_awaddr_gpio3(&mut self) -> Vout0RemapAwaddrGpio3W<SysSyscfg3Spec> {
        Vout0RemapAwaddrGpio3W::new(self, 3)
    }
}
#[doc = "SYS SYSCONSAIF SYSCFG 12\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_syscfg3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_syscfg3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysSyscfg3Spec;
impl crate::RegisterSpec for SysSyscfg3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_syscfg3::R`](R) reader structure"]
impl crate::Readable for SysSyscfg3Spec {}
#[doc = "`write(|w| ..)` method takes [`sys_syscfg3::W`](W) writer structure"]
impl crate::Writable for SysSyscfg3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets sys_syscfg3 to value 0"]
impl crate::Resettable for SysSyscfg3Spec {
    const RESET_VALUE: u32 = 0;
}
