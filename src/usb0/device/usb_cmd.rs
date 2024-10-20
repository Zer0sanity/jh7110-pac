#[doc = "Register `usb_cmd` reader"]
pub type R = crate::R<UsbCmdSpec>;
#[doc = "Register `usb_cmd` writer"]
pub type W = crate::W<UsbCmdSpec>;
#[doc = "Field `set_addr` reader - Set function address"]
pub type SetAddrR = crate::BitReader;
#[doc = "Field `set_addr` writer - Set function address"]
pub type SetAddrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `faddr` reader - Function address - saves the address to the device only when `set_addr` is set to `1`."]
pub type FaddrR = crate::FieldReader;
#[doc = "Field `faddr` writer - Function address - saves the address to the device only when `set_addr` is set to `1`."]
pub type FaddrW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `sdnfw` reader - Send Function Wake Device Notification TP - used only in SS mode."]
pub type SdnfwR = crate::BitReader;
#[doc = "Field `sdnfw` writer - Send Function Wake Device Notification TP - used only in SS mode."]
pub type SdnfwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `stmode` reader - Set Test Mode - used only in FS/HS mode."]
pub type StmodeR = crate::BitReader;
#[doc = "Field `stmode` writer - Set Test Mode - used only in FS/HS mode."]
pub type StmodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tmode_sel` reader - Test Mode Selector - used only in FS/HS mode."]
pub type TmodeSelR = crate::FieldReader;
#[doc = "Field `tmode_sel` writer - Test Mode Selector - used only in FS/HS mode."]
pub type TmodeSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `sdnltm` reader - Send Latency Tolerance Message Device Notification TP - used only in SS mode."]
pub type SdnltmR = crate::BitReader;
#[doc = "Field `sdnltm` writer - Send Latency Tolerance Message Device Notification TP - used only in SS mode."]
pub type SdnltmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `spkt` reader - Send Custom Transaction Packet - used only in SS mode."]
pub type SpktR = crate::BitReader;
#[doc = "Field `spkt` writer - Send Custom Transaction Packet - used only in SS mode."]
pub type SpktW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dnfw_int_dnltm_belt` reader - Device Notification `Function Wake` Interface Value / Device Notification `Latency Tolerance Message` BELT Value - used only in SS mode."]
pub type DnfwIntDnltmBeltR = crate::FieldReader;
#[doc = "Field `dnfw_int_dnltm_belt` writer - Device Notification `Function Wake` Interface Value / Device Notification `Latency Tolerance Message` BELT Value - used only in SS mode."]
pub type DnfwIntDnltmBeltW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - Set function address"]
    #[inline(always)]
    pub fn set_addr(&self) -> SetAddrR {
        SetAddrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - Function address - saves the address to the device only when `set_addr` is set to `1`."]
    #[inline(always)]
    pub fn faddr(&self) -> FaddrR {
        FaddrR::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bit 8 - Send Function Wake Device Notification TP - used only in SS mode."]
    #[inline(always)]
    pub fn sdnfw(&self) -> SdnfwR {
        SdnfwR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Set Test Mode - used only in FS/HS mode."]
    #[inline(always)]
    pub fn stmode(&self) -> StmodeR {
        StmodeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - Test Mode Selector - used only in FS/HS mode."]
    #[inline(always)]
    pub fn tmode_sel(&self) -> TmodeSelR {
        TmodeSelR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - Send Latency Tolerance Message Device Notification TP - used only in SS mode."]
    #[inline(always)]
    pub fn sdnltm(&self) -> SdnltmR {
        SdnltmR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Send Custom Transaction Packet - used only in SS mode."]
    #[inline(always)]
    pub fn spkt(&self) -> SpktR {
        SpktR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Device Notification `Function Wake` Interface Value / Device Notification `Latency Tolerance Message` BELT Value - used only in SS mode."]
    #[inline(always)]
    pub fn dnfw_int_dnltm_belt(&self) -> DnfwIntDnltmBeltR {
        DnfwIntDnltmBeltR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Set function address"]
    #[inline(always)]
    #[must_use]
    pub fn set_addr(&mut self) -> SetAddrW<UsbCmdSpec> {
        SetAddrW::new(self, 0)
    }
    #[doc = "Bits 1:7 - Function address - saves the address to the device only when `set_addr` is set to `1`."]
    #[inline(always)]
    #[must_use]
    pub fn faddr(&mut self) -> FaddrW<UsbCmdSpec> {
        FaddrW::new(self, 1)
    }
    #[doc = "Bit 8 - Send Function Wake Device Notification TP - used only in SS mode."]
    #[inline(always)]
    #[must_use]
    pub fn sdnfw(&mut self) -> SdnfwW<UsbCmdSpec> {
        SdnfwW::new(self, 8)
    }
    #[doc = "Bit 9 - Set Test Mode - used only in FS/HS mode."]
    #[inline(always)]
    #[must_use]
    pub fn stmode(&mut self) -> StmodeW<UsbCmdSpec> {
        StmodeW::new(self, 9)
    }
    #[doc = "Bits 10:11 - Test Mode Selector - used only in FS/HS mode."]
    #[inline(always)]
    #[must_use]
    pub fn tmode_sel(&mut self) -> TmodeSelW<UsbCmdSpec> {
        TmodeSelW::new(self, 10)
    }
    #[doc = "Bit 12 - Send Latency Tolerance Message Device Notification TP - used only in SS mode."]
    #[inline(always)]
    #[must_use]
    pub fn sdnltm(&mut self) -> SdnltmW<UsbCmdSpec> {
        SdnltmW::new(self, 12)
    }
    #[doc = "Bit 13 - Send Custom Transaction Packet - used only in SS mode."]
    #[inline(always)]
    #[must_use]
    pub fn spkt(&mut self) -> SpktW<UsbCmdSpec> {
        SpktW::new(self, 13)
    }
    #[doc = "Bits 16:23 - Device Notification `Function Wake` Interface Value / Device Notification `Latency Tolerance Message` BELT Value - used only in SS mode."]
    #[inline(always)]
    #[must_use]
    pub fn dnfw_int_dnltm_belt(&mut self) -> DnfwIntDnltmBeltW<UsbCmdSpec> {
        DnfwIntDnltmBeltW::new(self, 16)
    }
}
#[doc = "USB3 Global command.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb_cmd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb_cmd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UsbCmdSpec;
impl crate::RegisterSpec for UsbCmdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usb_cmd::R`](R) reader structure"]
impl crate::Readable for UsbCmdSpec {}
#[doc = "`write(|w| ..)` method takes [`usb_cmd::W`](W) writer structure"]
impl crate::Writable for UsbCmdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets usb_cmd to value 0"]
impl crate::Resettable for UsbCmdSpec {
    const RESET_VALUE: u32 = 0;
}
