#[doc = "Register `sys_syscfg7` reader"]
pub type R = crate::R<SysSyscfg7Spec>;
#[doc = "Register `sys_syscfg7` writer"]
pub type W = crate::W<SysSyscfg7Spec>;
#[doc = "Field `pll0_fbdiv` reader - "]
pub type Pll0FbdivR = crate::FieldReader<u16>;
#[doc = "Field `pll0_fbdiv` writer - "]
pub type Pll0FbdivW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn pll0_fbdiv(&self) -> Pll0FbdivR {
        Pll0FbdivR::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    #[must_use]
    pub fn pll0_fbdiv(&mut self) -> Pll0FbdivW<SysSyscfg7Spec> {
        Pll0FbdivW::new(self, 0)
    }
}
#[doc = "SYS SYSCONSAIF SYSCFG 28\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_syscfg7::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_syscfg7::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysSyscfg7Spec;
impl crate::RegisterSpec for SysSyscfg7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_syscfg7::R`](R) reader structure"]
impl crate::Readable for SysSyscfg7Spec {}
#[doc = "`write(|w| ..)` method takes [`sys_syscfg7::W`](W) writer structure"]
impl crate::Writable for SysSyscfg7Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets sys_syscfg7 to value 0x53"]
impl crate::Resettable for SysSyscfg7Spec {
    const RESET_VALUE: u32 = 0x53;
}
