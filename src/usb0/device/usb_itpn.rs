#[doc = "Register `usb_itpn` reader"]
pub type R = crate::R<UsbItpnSpec>;
#[doc = "Register `usb_itpn` writer"]
pub type W = crate::W<UsbItpnSpec>;
#[doc = "Field `itpn` reader - SS: last ITP number, FS/HS: last SOF number."]
pub type ItpnR = crate::FieldReader<u16>;
#[doc = "Field `itpn` writer - SS: last ITP number, FS/HS: last SOF number."]
pub type ItpnW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13 - SS: last ITP number, FS/HS: last SOF number."]
    #[inline(always)]
    pub fn itpn(&self) -> ItpnR {
        ItpnR::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - SS: last ITP number, FS/HS: last SOF number."]
    #[inline(always)]
    #[must_use]
    pub fn itpn(&mut self) -> ItpnW<UsbItpnSpec> {
        ItpnW::new(self, 0)
    }
}
#[doc = "ITP (SS) / SOF (FS/HS) number - SS: last ITP number, FS/HS: last SOF number.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb_itpn::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb_itpn::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UsbItpnSpec;
impl crate::RegisterSpec for UsbItpnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usb_itpn::R`](R) reader structure"]
impl crate::Readable for UsbItpnSpec {}
#[doc = "`write(|w| ..)` method takes [`usb_itpn::W`](W) writer structure"]
impl crate::Writable for UsbItpnSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets usb_itpn to value 0"]
impl crate::Resettable for UsbItpnSpec {
    const RESET_VALUE: u32 = 0;
}
