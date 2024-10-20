#[doc = "Register `sys_syscfg2` reader"]
pub type R = crate::R<SysSyscfg2Spec>;
#[doc = "Register `sys_syscfg2` writer"]
pub type W = crate::W<SysSyscfg2Spec>;
#[doc = "Field `vout0_remap_awaddr` reader - "]
pub type Vout0RemapAwaddrR = crate::FieldReader;
#[doc = "Field `vout0_remap_awaddr` writer - "]
pub type Vout0RemapAwaddrW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `vout1_remap_araddr` reader - "]
pub type Vout1RemapAraddrR = crate::FieldReader;
#[doc = "Field `vout1_remap_araddr` writer - "]
pub type Vout1RemapAraddrW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `vout1_remap_awaddr` reader - "]
pub type Vout1RemapAwaddrR = crate::FieldReader;
#[doc = "Field `vout1_remap_awaddr` writer - "]
pub type Vout1RemapAwaddrW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn vout0_remap_awaddr(&self) -> Vout0RemapAwaddrR {
        Vout0RemapAwaddrR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn vout1_remap_araddr(&self) -> Vout1RemapAraddrR {
        Vout1RemapAraddrR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn vout1_remap_awaddr(&self) -> Vout1RemapAwaddrR {
        Vout1RemapAwaddrR::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    #[must_use]
    pub fn vout0_remap_awaddr(&mut self) -> Vout0RemapAwaddrW<SysSyscfg2Spec> {
        Vout0RemapAwaddrW::new(self, 0)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    #[must_use]
    pub fn vout1_remap_araddr(&mut self) -> Vout1RemapAraddrW<SysSyscfg2Spec> {
        Vout1RemapAraddrW::new(self, 4)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    #[must_use]
    pub fn vout1_remap_awaddr(&mut self) -> Vout1RemapAwaddrW<SysSyscfg2Spec> {
        Vout1RemapAwaddrW::new(self, 8)
    }
}
#[doc = "SYS SYSCONSAIF SYSCFG 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_syscfg2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_syscfg2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysSyscfg2Spec;
impl crate::RegisterSpec for SysSyscfg2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_syscfg2::R`](R) reader structure"]
impl crate::Readable for SysSyscfg2Spec {}
#[doc = "`write(|w| ..)` method takes [`sys_syscfg2::W`](W) writer structure"]
impl crate::Writable for SysSyscfg2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets sys_syscfg2 to value 0"]
impl crate::Resettable for SysSyscfg2Spec {
    const RESET_VALUE: u32 = 0;
}
