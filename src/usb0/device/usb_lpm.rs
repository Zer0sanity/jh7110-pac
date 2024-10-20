#[doc = "Register `usb_lpm` reader"]
pub type R = crate::R<UsbLpmSpec>;
#[doc = "Register `usb_lpm` writer"]
pub type W = crate::W<UsbLpmSpec>;
#[doc = "Field `hird` reader - Host Initiated Resume Duration."]
pub type HirdR = crate::FieldReader;
#[doc = "Field `hird` writer - Host Initiated Resume Duration."]
pub type HirdW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `brw` reader - Remote Wakeup Enable - `bRemoteWake`."]
pub type BrwR = crate::BitReader;
#[doc = "Field `brw` writer - Remote Wakeup Enable - `bRemoteWake`."]
pub type BrwW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Host Initiated Resume Duration."]
    #[inline(always)]
    pub fn hird(&self) -> HirdR {
        HirdR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Remote Wakeup Enable - `bRemoteWake`."]
    #[inline(always)]
    pub fn brw(&self) -> BrwR {
        BrwR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Host Initiated Resume Duration."]
    #[inline(always)]
    #[must_use]
    pub fn hird(&mut self) -> HirdW<UsbLpmSpec> {
        HirdW::new(self, 0)
    }
    #[doc = "Bit 4 - Remote Wakeup Enable - `bRemoteWake`."]
    #[inline(always)]
    #[must_use]
    pub fn brw(&mut self) -> BrwW<UsbLpmSpec> {
        BrwW::new(self, 4)
    }
}
#[doc = "Global LPM.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb_lpm::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb_lpm::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UsbLpmSpec;
impl crate::RegisterSpec for UsbLpmSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usb_lpm::R`](R) reader structure"]
impl crate::Readable for UsbLpmSpec {}
#[doc = "`write(|w| ..)` method takes [`usb_lpm::W`](W) writer structure"]
impl crate::Writable for UsbLpmSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets usb_lpm to value 0"]
impl crate::Resettable for UsbLpmSpec {
    const RESET_VALUE: u32 = 0;
}
