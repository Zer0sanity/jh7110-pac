#[doc = "Register `sys_syscfg14` reader"]
pub type R = crate::R<SysSyscfg14Spec>;
#[doc = "Register `sys_syscfg14` writer"]
pub type W = crate::W<SysSyscfg14Spec>;
#[doc = "Field `noc_bus_oic_evemon_trigger6` reader - "]
pub type NocBusOicEvemonTrigger6R = crate::BitReader;
#[doc = "Field `noc_bus_oic_ignore_modifiable_0` reader - "]
pub type NocBusOicIgnoreModifiable0R = crate::BitReader;
#[doc = "Field `noc_bus_oic_ignore_modifiable_0` writer - "]
pub type NocBusOicIgnoreModifiable0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `noc_bus_oic_ignore_modifiable_1` reader - "]
pub type NocBusOicIgnoreModifiable1R = crate::BitReader;
#[doc = "Field `noc_bus_oic_ignore_modifiable_1` writer - "]
pub type NocBusOicIgnoreModifiable1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `noc_bus_oic_ignore_modifiable_2` reader - "]
pub type NocBusOicIgnoreModifiable2R = crate::BitReader;
#[doc = "Field `noc_bus_oic_ignore_modifiable_2` writer - "]
pub type NocBusOicIgnoreModifiable2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `noc_bus_oic_ignore_modifiable_3` reader - "]
pub type NocBusOicIgnoreModifiable3R = crate::BitReader;
#[doc = "Field `noc_bus_oic_ignore_modifiable_3` writer - "]
pub type NocBusOicIgnoreModifiable3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `noc_bus_oic_ignore_modifiable_4` reader - "]
pub type NocBusOicIgnoreModifiable4R = crate::BitReader;
#[doc = "Field `noc_bus_oic_ignore_modifiable_4` writer - "]
pub type NocBusOicIgnoreModifiable4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `noc_bus_oic_evemon_start7` reader - "]
pub type NocBusOicEvemonStart7R = crate::BitReader;
#[doc = "Field `noc_bus_oic_evemon_start7` writer - "]
pub type NocBusOicEvemonStart7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `noc_bus_oic_evemon_trigger7` reader - "]
pub type NocBusOicEvemonTrigger7R = crate::BitReader;
#[doc = "Field `noc_bus_oic_evemon_start8` reader - "]
pub type NocBusOicEvemonStart8R = crate::BitReader;
#[doc = "Field `noc_bus_oic_evemon_start8` writer - "]
pub type NocBusOicEvemonStart8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `noc_bus_oic_evemon_trigger8` reader - "]
pub type NocBusOicEvemonTrigger8R = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn noc_bus_oic_evemon_trigger6(&self) -> NocBusOicEvemonTrigger6R {
        NocBusOicEvemonTrigger6R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn noc_bus_oic_ignore_modifiable_0(&self) -> NocBusOicIgnoreModifiable0R {
        NocBusOicIgnoreModifiable0R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn noc_bus_oic_ignore_modifiable_1(&self) -> NocBusOicIgnoreModifiable1R {
        NocBusOicIgnoreModifiable1R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn noc_bus_oic_ignore_modifiable_2(&self) -> NocBusOicIgnoreModifiable2R {
        NocBusOicIgnoreModifiable2R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn noc_bus_oic_ignore_modifiable_3(&self) -> NocBusOicIgnoreModifiable3R {
        NocBusOicIgnoreModifiable3R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn noc_bus_oic_ignore_modifiable_4(&self) -> NocBusOicIgnoreModifiable4R {
        NocBusOicIgnoreModifiable4R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn noc_bus_oic_evemon_start7(&self) -> NocBusOicEvemonStart7R {
        NocBusOicEvemonStart7R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn noc_bus_oic_evemon_trigger7(&self) -> NocBusOicEvemonTrigger7R {
        NocBusOicEvemonTrigger7R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn noc_bus_oic_evemon_start8(&self) -> NocBusOicEvemonStart8R {
        NocBusOicEvemonStart8R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn noc_bus_oic_evemon_trigger8(&self) -> NocBusOicEvemonTrigger8R {
        NocBusOicEvemonTrigger8R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn noc_bus_oic_ignore_modifiable_0(
        &mut self,
    ) -> NocBusOicIgnoreModifiable0W<SysSyscfg14Spec> {
        NocBusOicIgnoreModifiable0W::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn noc_bus_oic_ignore_modifiable_1(
        &mut self,
    ) -> NocBusOicIgnoreModifiable1W<SysSyscfg14Spec> {
        NocBusOicIgnoreModifiable1W::new(self, 6)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn noc_bus_oic_ignore_modifiable_2(
        &mut self,
    ) -> NocBusOicIgnoreModifiable2W<SysSyscfg14Spec> {
        NocBusOicIgnoreModifiable2W::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn noc_bus_oic_ignore_modifiable_3(
        &mut self,
    ) -> NocBusOicIgnoreModifiable3W<SysSyscfg14Spec> {
        NocBusOicIgnoreModifiable3W::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn noc_bus_oic_ignore_modifiable_4(
        &mut self,
    ) -> NocBusOicIgnoreModifiable4W<SysSyscfg14Spec> {
        NocBusOicIgnoreModifiable4W::new(self, 9)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn noc_bus_oic_evemon_start7(&mut self) -> NocBusOicEvemonStart7W<SysSyscfg14Spec> {
        NocBusOicEvemonStart7W::new(self, 15)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn noc_bus_oic_evemon_start8(&mut self) -> NocBusOicEvemonStart8W<SysSyscfg14Spec> {
        NocBusOicEvemonStart8W::new(self, 17)
    }
}
#[doc = "SYS SYSCONSAIF SYSCFG 56\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_syscfg14::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_syscfg14::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysSyscfg14Spec;
impl crate::RegisterSpec for SysSyscfg14Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_syscfg14::R`](R) reader structure"]
impl crate::Readable for SysSyscfg14Spec {}
#[doc = "`write(|w| ..)` method takes [`sys_syscfg14::W`](W) writer structure"]
impl crate::Writable for SysSyscfg14Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets sys_syscfg14 to value 0"]
impl crate::Resettable for SysSyscfg14Spec {
    const RESET_VALUE: u32 = 0;
}
