#[doc = "Register `usb_cpkt[%s]` reader"]
pub type R = crate::R<UsbCpktSpec>;
#[doc = "Register `usb_cpkt[%s]` writer"]
pub type W = crate::W<UsbCpktSpec>;
#[doc = "Field `cpkt` reader - Custom packet."]
pub type CpktR = crate::FieldReader<u32>;
#[doc = "Field `cpkt` writer - Custom packet."]
pub type CpktW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Custom packet."]
    #[inline(always)]
    pub fn cpkt(&self) -> CpktR {
        CpktR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Custom packet."]
    #[inline(always)]
    #[must_use]
    pub fn cpkt(&mut self) -> CpktW<UsbCpktSpec> {
        CpktW::new(self, 0)
    }
}
#[doc = "USB3 Global custom packet.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb_cpkt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb_cpkt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UsbCpktSpec;
impl crate::RegisterSpec for UsbCpktSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usb_cpkt::R`](R) reader structure"]
impl crate::Readable for UsbCpktSpec {}
#[doc = "`write(|w| ..)` method takes [`usb_cpkt::W`](W) writer structure"]
impl crate::Writable for UsbCpktSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets usb_cpkt[%s]
to value 0"]
impl crate::Resettable for UsbCpktSpec {
    const RESET_VALUE: u32 = 0;
}
