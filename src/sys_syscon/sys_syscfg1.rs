#[doc = "Register `sys_syscfg1` reader"]
pub type R = crate::R<SysSyscfg1Spec>;
#[doc = "Register `sys_syscfg1` writer"]
pub type W = crate::W<SysSyscfg1Spec>;
#[doc = "Field `sd1_remap_awaddr` reader - "]
pub type Sd1RemapAwaddrR = crate::FieldReader;
#[doc = "Field `sd1_remap_awaddr` writer - "]
pub type Sd1RemapAwaddrW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `sec_haddr_remap` reader - "]
pub type SecHaddrRemapR = crate::FieldReader;
#[doc = "Field `sec_haddr_remap` writer - "]
pub type SecHaddrRemapW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `usb_araddr_remap` reader - "]
pub type UsbAraddrRemapR = crate::FieldReader;
#[doc = "Field `usb_araddr_remap` writer - "]
pub type UsbAraddrRemapW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `usb_awaddr_remap` reader - "]
pub type UsbAwaddrRemapR = crate::FieldReader;
#[doc = "Field `usb_awaddr_remap` writer - "]
pub type UsbAwaddrRemapW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `vdec_remap_awaddr` reader - "]
pub type VdecRemapAwaddrR = crate::FieldReader;
#[doc = "Field `vdec_remap_awaddr` writer - "]
pub type VdecRemapAwaddrW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `venc_remap_araddr` reader - "]
pub type VencRemapAraddrR = crate::FieldReader;
#[doc = "Field `venc_remap_araddr` writer - "]
pub type VencRemapAraddrW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `venc_remap_awaddr` reader - "]
pub type VencRemapAwaddrR = crate::FieldReader;
#[doc = "Field `venc_remap_awaddr` writer - "]
pub type VencRemapAwaddrW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `vout0_remap_araddr` reader - "]
pub type Vout0RemapAraddrR = crate::FieldReader;
#[doc = "Field `vout0_remap_araddr` writer - "]
pub type Vout0RemapAraddrW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn sd1_remap_awaddr(&self) -> Sd1RemapAwaddrR {
        Sd1RemapAwaddrR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    pub fn sec_haddr_remap(&self) -> SecHaddrRemapR {
        SecHaddrRemapR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn usb_araddr_remap(&self) -> UsbAraddrRemapR {
        UsbAraddrRemapR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    pub fn usb_awaddr_remap(&self) -> UsbAwaddrRemapR {
        UsbAwaddrRemapR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn vdec_remap_awaddr(&self) -> VdecRemapAwaddrR {
        VdecRemapAwaddrR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn venc_remap_araddr(&self) -> VencRemapAraddrR {
        VencRemapAraddrR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    pub fn venc_remap_awaddr(&self) -> VencRemapAwaddrR {
        VencRemapAwaddrR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    pub fn vout0_remap_araddr(&self) -> Vout0RemapAraddrR {
        Vout0RemapAraddrR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    #[must_use]
    pub fn sd1_remap_awaddr(&mut self) -> Sd1RemapAwaddrW<SysSyscfg1Spec> {
        Sd1RemapAwaddrW::new(self, 0)
    }
    #[doc = "Bits 4:7"]
    #[inline(always)]
    #[must_use]
    pub fn sec_haddr_remap(&mut self) -> SecHaddrRemapW<SysSyscfg1Spec> {
        SecHaddrRemapW::new(self, 4)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    #[must_use]
    pub fn usb_araddr_remap(&mut self) -> UsbAraddrRemapW<SysSyscfg1Spec> {
        UsbAraddrRemapW::new(self, 8)
    }
    #[doc = "Bits 12:15"]
    #[inline(always)]
    #[must_use]
    pub fn usb_awaddr_remap(&mut self) -> UsbAwaddrRemapW<SysSyscfg1Spec> {
        UsbAwaddrRemapW::new(self, 12)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    #[must_use]
    pub fn vdec_remap_awaddr(&mut self) -> VdecRemapAwaddrW<SysSyscfg1Spec> {
        VdecRemapAwaddrW::new(self, 16)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    #[must_use]
    pub fn venc_remap_araddr(&mut self) -> VencRemapAraddrW<SysSyscfg1Spec> {
        VencRemapAraddrW::new(self, 20)
    }
    #[doc = "Bits 24:27"]
    #[inline(always)]
    #[must_use]
    pub fn venc_remap_awaddr(&mut self) -> VencRemapAwaddrW<SysSyscfg1Spec> {
        VencRemapAwaddrW::new(self, 24)
    }
    #[doc = "Bits 28:31"]
    #[inline(always)]
    #[must_use]
    pub fn vout0_remap_araddr(&mut self) -> Vout0RemapAraddrW<SysSyscfg1Spec> {
        Vout0RemapAraddrW::new(self, 28)
    }
}
#[doc = "SYS SYSCONSAIF SYSCFG 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_syscfg1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_syscfg1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysSyscfg1Spec;
impl crate::RegisterSpec for SysSyscfg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_syscfg1::R`](R) reader structure"]
impl crate::Readable for SysSyscfg1Spec {}
#[doc = "`write(|w| ..)` method takes [`sys_syscfg1::W`](W) writer structure"]
impl crate::Writable for SysSyscfg1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets sys_syscfg1 to value 0"]
impl crate::Resettable for SysSyscfg1Spec {
    const RESET_VALUE: u32 = 0;
}
