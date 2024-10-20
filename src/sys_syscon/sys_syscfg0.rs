#[doc = "Register `sys_syscfg0` reader"]
pub type R = crate::R<SysSyscfg0Spec>;
#[doc = "Register `sys_syscfg0` writer"]
pub type W = crate::W<SysSyscfg0Spec>;
#[doc = "Field `e24_remap_haddr` reader - "]
pub type E24RemapHaddrR = crate::FieldReader;
#[doc = "Field `e24_remap_haddr` writer - "]
pub type E24RemapHaddrW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `hifi4_idma_remap_araddr` reader - "]
pub type Hifi4IdmaRemapAraddrR = crate::FieldReader;
#[doc = "Field `hifi4_idma_remap_araddr` writer - "]
pub type Hifi4IdmaRemapAraddrW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `hifi4_idma_remap_awaddr` reader - "]
pub type Hifi4IdmaRemapAwaddrR = crate::FieldReader;
#[doc = "Field `hifi4_idma_remap_awaddr` writer - "]
pub type Hifi4IdmaRemapAwaddrW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `hifi4_sys_remap_araddr` reader - "]
pub type Hifi4SysRemapAraddrR = crate::FieldReader;
#[doc = "Field `hifi4_sys_remap_araddr` writer - "]
pub type Hifi4SysRemapAraddrW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `hifi4_sys_remap_awaddr` reader - "]
pub type Hifi4SysRemapAwaddrR = crate::FieldReader;
#[doc = "Field `hifi4_sys_remap_awaddr` writer - "]
pub type Hifi4SysRemapAwaddrW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `jpg_remap_araddr` reader - "]
pub type JpgRemapAraddrR = crate::FieldReader;
#[doc = "Field `jpg_remap_araddr` writer - "]
pub type JpgRemapAraddrW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `jpg_remap_awaddr` reader - "]
pub type JpgRemapAwaddrR = crate::FieldReader;
#[doc = "Field `jpg_remap_awaddr` writer - "]
pub type JpgRemapAwaddrW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `sd0_remap_araddr` reader - "]
pub type Sd0RemapAraddrR = crate::FieldReader;
#[doc = "Field `sd0_remap_araddr` writer - "]
pub type Sd0RemapAraddrW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn e24_remap_haddr(&self) -> E24RemapHaddrR {
        E24RemapHaddrR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn hifi4_idma_remap_araddr(&self) -> Hifi4IdmaRemapAraddrR {
        Hifi4IdmaRemapAraddrR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn hifi4_idma_remap_awaddr(&self) -> Hifi4IdmaRemapAwaddrR {
        Hifi4IdmaRemapAwaddrR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn hifi4_sys_remap_araddr(&self) -> Hifi4SysRemapAraddrR {
        Hifi4SysRemapAraddrR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn hifi4_sys_remap_awaddr(&self) -> Hifi4SysRemapAwaddrR {
        Hifi4SysRemapAwaddrR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn jpg_remap_araddr(&self) -> JpgRemapAraddrR {
        JpgRemapAraddrR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn jpg_remap_awaddr(&self) -> JpgRemapAwaddrR {
        JpgRemapAwaddrR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn sd0_remap_araddr(&self) -> Sd0RemapAraddrR {
        Sd0RemapAraddrR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    #[must_use]
    pub fn e24_remap_haddr(&mut self) -> E24RemapHaddrW<SysSyscfg0Spec> {
        E24RemapHaddrW::new(self, 0)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    #[must_use]
    pub fn hifi4_idma_remap_araddr(&mut self) -> Hifi4IdmaRemapAraddrW<SysSyscfg0Spec> {
        Hifi4IdmaRemapAraddrW::new(self, 4)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    #[must_use]
    pub fn hifi4_idma_remap_awaddr(&mut self) -> Hifi4IdmaRemapAwaddrW<SysSyscfg0Spec> {
        Hifi4IdmaRemapAwaddrW::new(self, 8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    #[must_use]
    pub fn hifi4_sys_remap_araddr(&mut self) -> Hifi4SysRemapAraddrW<SysSyscfg0Spec> {
        Hifi4SysRemapAraddrW::new(self, 12)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    #[must_use]
    pub fn hifi4_sys_remap_awaddr(&mut self) -> Hifi4SysRemapAwaddrW<SysSyscfg0Spec> {
        Hifi4SysRemapAwaddrW::new(self, 16)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    #[must_use]
    pub fn jpg_remap_araddr(&mut self) -> JpgRemapAraddrW<SysSyscfg0Spec> {
        JpgRemapAraddrW::new(self, 20)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    #[must_use]
    pub fn jpg_remap_awaddr(&mut self) -> JpgRemapAwaddrW<SysSyscfg0Spec> {
        JpgRemapAwaddrW::new(self, 24)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    #[must_use]
    pub fn sd0_remap_araddr(&mut self) -> Sd0RemapAraddrW<SysSyscfg0Spec> {
        Sd0RemapAraddrW::new(self, 28)
    }
}
#[doc = "SYS SYSCONSAIF SYSCFG 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_syscfg0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_syscfg0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysSyscfg0Spec;
impl crate::RegisterSpec for SysSyscfg0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_syscfg0::R`](R) reader structure"]
impl crate::Readable for SysSyscfg0Spec {}
#[doc = "`write(|w| ..)` method takes [`sys_syscfg0::W`](W) writer structure"]
impl crate::Writable for SysSyscfg0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets sys_syscfg0 to value 0"]
impl crate::Resettable for SysSyscfg0Spec {
    const RESET_VALUE: u32 = 0;
}
