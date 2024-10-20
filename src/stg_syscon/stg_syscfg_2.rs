#[doc = "Register `stg_syscfg_2` reader"]
pub type R = crate::R<StgSyscfg2Spec>;
#[doc = "Register `stg_syscfg_2` writer"]
pub type W = crate::W<StgSyscfg2Spec>;
#[doc = "Field `u0_usb_rx_dp` reader - "]
pub type U0UsbRxDpR = crate::BitReader;
#[doc = "Field `u0_usb_rx_rcv` reader - "]
pub type U0UsbRxRcvR = crate::BitReader;
#[doc = "Field `u0_usb_self_test` reader - For software bist_test"]
pub type U0UsbSelfTestR = crate::BitReader;
#[doc = "Field `u0_usb_self_test` writer - For software bist_test"]
pub type U0UsbSelfTestW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_usb_sessend` reader - "]
pub type U0UsbSessendR = crate::BitReader;
#[doc = "Field `u0_usb_sessvalid` reader - "]
pub type U0UsbSessvalidR = crate::BitReader;
#[doc = "Field `u0_usb_sof` reader - "]
pub type U0UsbSofR = crate::BitReader;
#[doc = "Field `u0_usb_test_bist` reader - For software bist_test"]
pub type U0UsbTestBistR = crate::BitReader;
#[doc = "Field `u0_usb_usbdev_main_power_off_ack` reader - "]
pub type U0UsbUsbdevMainPowerOffAckR = crate::BitReader;
#[doc = "Field `u0_usb_usbdev_main_power_off_ready` reader - "]
pub type U0UsbUsbdevMainPowerOffReadyR = crate::BitReader;
#[doc = "Field `u0_usb_usbdev_main_power_off_req` reader - "]
pub type U0UsbUsbdevMainPowerOffReqR = crate::BitReader;
#[doc = "Field `u0_usb_usbdev_main_power_off_req` writer - "]
pub type U0UsbUsbdevMainPowerOffReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_usb_usbdev_main_power_on_ready` reader - "]
pub type U0UsbUsbdevMainPowerOnReadyR = crate::BitReader;
#[doc = "Field `u0_usb_usbdev_main_power_on_req` reader - "]
pub type U0UsbUsbdevMainPowerOnReqR = crate::BitReader;
#[doc = "Field `u0_usb_usbdev_main_power_on_valid` reader - "]
pub type U0UsbUsbdevMainPowerOnValidR = crate::BitReader;
#[doc = "Field `u0_usb_usbdev_main_power_on_valid` writer - "]
pub type U0UsbUsbdevMainPowerOnValidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_usb_usbdev_power_off_ack` reader - "]
pub type U0UsbUsbdevPowerOffAckR = crate::BitReader;
#[doc = "Field `u0_usb_usbdev_power_off_ready` reader - "]
pub type U0UsbUsbdevPowerOffReadyR = crate::BitReader;
#[doc = "Field `u0_usb_usbdev_power_off_req` reader - "]
pub type U0UsbUsbdevPowerOffReqR = crate::BitReader;
#[doc = "Field `u0_usb_usbdev_power_off_req` writer - "]
pub type U0UsbUsbdevPowerOffReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_usb_usbdev_power_on_ready` reader - "]
pub type U0UsbUsbdevPowerOnReadyR = crate::BitReader;
#[doc = "Field `u0_usb_usbdev_power_on_req` reader - "]
pub type U0UsbUsbdevPowerOnReqR = crate::BitReader;
#[doc = "Field `u0_usb_usbdev_power_on_valid` reader - "]
pub type U0UsbUsbdevPowerOnValidR = crate::BitReader;
#[doc = "Field `u0_usb_usbdev_power_on_valid` writer - "]
pub type U0UsbUsbdevPowerOnValidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_usb_utmi_dmpulldown_sit` reader - "]
pub type U0UsbUtmiDmpulldownSitR = crate::BitReader;
#[doc = "Field `u0_usb_utmi_dmpulldown_sit` writer - "]
pub type U0UsbUtmiDmpulldownSitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_usb_utmi_dppulldown_sit` reader - "]
pub type U0UsbUtmiDppulldownSitR = crate::BitReader;
#[doc = "Field `u0_usb_utmi_dppulldown_sit` writer - "]
pub type U0UsbUtmiDppulldownSitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_usb_utmi_fslsserialmode_sit` reader - "]
pub type U0UsbUtmiFslsserialmodeSitR = crate::BitReader;
#[doc = "Field `u0_usb_utmi_fslsserialmode_sit` writer - "]
pub type U0UsbUtmiFslsserialmodeSitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_usb_utmi_hostdisconnect_sit` reader - "]
pub type U0UsbUtmiHostdisconnectSitR = crate::BitReader;
#[doc = "Field `u0_usb_utmi_iddig_sit` reader - "]
pub type U0UsbUtmiIddigSitR = crate::BitReader;
#[doc = "Field `u0_usb_utmi_idpullup_sit` reader - "]
pub type U0UsbUtmiIdpullupSitR = crate::BitReader;
#[doc = "Field `u0_usb_utmi_idpullup_sit` writer - "]
pub type U0UsbUtmiIdpullupSitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_usb_utmi_linestate_sit` reader - "]
pub type U0UsbUtmiLinestateSitR = crate::FieldReader;
#[doc = "Field `u0_usb_utmi_opmode_sit` reader - "]
pub type U0UsbUtmiOpmodeSitR = crate::FieldReader;
#[doc = "Field `u0_usb_utmi_opmode_sit` writer - "]
pub type U0UsbUtmiOpmodeSitW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `u0_usb_utmi_rxactive_sit` reader - "]
pub type U0UsbUtmiRxactiveSitR = crate::BitReader;
#[doc = "Field `u0_usb_utmi_rxerror_sit` reader - "]
pub type U0UsbUtmiRxerrorSitR = crate::BitReader;
#[doc = "Field `u0_usb_utmi_rxvalid_sit` reader - "]
pub type U0UsbUtmiRxvalidSitR = crate::BitReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn u0_usb_rx_dp(&self) -> U0UsbRxDpR {
        U0UsbRxDpR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn u0_usb_rx_rcv(&self) -> U0UsbRxRcvR {
        U0UsbRxRcvR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - For software bist_test"]
    #[inline(always)]
    pub fn u0_usb_self_test(&self) -> U0UsbSelfTestR {
        U0UsbSelfTestR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn u0_usb_sessend(&self) -> U0UsbSessendR {
        U0UsbSessendR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn u0_usb_sessvalid(&self) -> U0UsbSessvalidR {
        U0UsbSessvalidR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn u0_usb_sof(&self) -> U0UsbSofR {
        U0UsbSofR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - For software bist_test"]
    #[inline(always)]
    pub fn u0_usb_test_bist(&self) -> U0UsbTestBistR {
        U0UsbTestBistR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn u0_usb_usbdev_main_power_off_ack(&self) -> U0UsbUsbdevMainPowerOffAckR {
        U0UsbUsbdevMainPowerOffAckR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn u0_usb_usbdev_main_power_off_ready(&self) -> U0UsbUsbdevMainPowerOffReadyR {
        U0UsbUsbdevMainPowerOffReadyR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn u0_usb_usbdev_main_power_off_req(&self) -> U0UsbUsbdevMainPowerOffReqR {
        U0UsbUsbdevMainPowerOffReqR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn u0_usb_usbdev_main_power_on_ready(&self) -> U0UsbUsbdevMainPowerOnReadyR {
        U0UsbUsbdevMainPowerOnReadyR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn u0_usb_usbdev_main_power_on_req(&self) -> U0UsbUsbdevMainPowerOnReqR {
        U0UsbUsbdevMainPowerOnReqR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn u0_usb_usbdev_main_power_on_valid(&self) -> U0UsbUsbdevMainPowerOnValidR {
        U0UsbUsbdevMainPowerOnValidR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn u0_usb_usbdev_power_off_ack(&self) -> U0UsbUsbdevPowerOffAckR {
        U0UsbUsbdevPowerOffAckR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn u0_usb_usbdev_power_off_ready(&self) -> U0UsbUsbdevPowerOffReadyR {
        U0UsbUsbdevPowerOffReadyR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn u0_usb_usbdev_power_off_req(&self) -> U0UsbUsbdevPowerOffReqR {
        U0UsbUsbdevPowerOffReqR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn u0_usb_usbdev_power_on_ready(&self) -> U0UsbUsbdevPowerOnReadyR {
        U0UsbUsbdevPowerOnReadyR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn u0_usb_usbdev_power_on_req(&self) -> U0UsbUsbdevPowerOnReqR {
        U0UsbUsbdevPowerOnReqR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn u0_usb_usbdev_power_on_valid(&self) -> U0UsbUsbdevPowerOnValidR {
        U0UsbUsbdevPowerOnValidR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn u0_usb_utmi_dmpulldown_sit(&self) -> U0UsbUtmiDmpulldownSitR {
        U0UsbUtmiDmpulldownSitR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn u0_usb_utmi_dppulldown_sit(&self) -> U0UsbUtmiDppulldownSitR {
        U0UsbUtmiDppulldownSitR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn u0_usb_utmi_fslsserialmode_sit(&self) -> U0UsbUtmiFslsserialmodeSitR {
        U0UsbUtmiFslsserialmodeSitR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn u0_usb_utmi_hostdisconnect_sit(&self) -> U0UsbUtmiHostdisconnectSitR {
        U0UsbUtmiHostdisconnectSitR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn u0_usb_utmi_iddig_sit(&self) -> U0UsbUtmiIddigSitR {
        U0UsbUtmiIddigSitR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn u0_usb_utmi_idpullup_sit(&self) -> U0UsbUtmiIdpullupSitR {
        U0UsbUtmiIdpullupSitR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:26"]
    #[inline(always)]
    pub fn u0_usb_utmi_linestate_sit(&self) -> U0UsbUtmiLinestateSitR {
        U0UsbUtmiLinestateSitR::new(((self.bits >> 25) & 3) as u8)
    }
    #[doc = "Bits 27:28"]
    #[inline(always)]
    pub fn u0_usb_utmi_opmode_sit(&self) -> U0UsbUtmiOpmodeSitR {
        U0UsbUtmiOpmodeSitR::new(((self.bits >> 27) & 3) as u8)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn u0_usb_utmi_rxactive_sit(&self) -> U0UsbUtmiRxactiveSitR {
        U0UsbUtmiRxactiveSitR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn u0_usb_utmi_rxerror_sit(&self) -> U0UsbUtmiRxerrorSitR {
        U0UsbUtmiRxerrorSitR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn u0_usb_utmi_rxvalid_sit(&self) -> U0UsbUtmiRxvalidSitR {
        U0UsbUtmiRxvalidSitR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - For software bist_test"]
    #[inline(always)]
    #[must_use]
    pub fn u0_usb_self_test(&mut self) -> U0UsbSelfTestW<StgSyscfg2Spec> {
        U0UsbSelfTestW::new(self, 2)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn u0_usb_usbdev_main_power_off_req(
        &mut self,
    ) -> U0UsbUsbdevMainPowerOffReqW<StgSyscfg2Spec> {
        U0UsbUsbdevMainPowerOffReqW::new(self, 9)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn u0_usb_usbdev_main_power_on_valid(
        &mut self,
    ) -> U0UsbUsbdevMainPowerOnValidW<StgSyscfg2Spec> {
        U0UsbUsbdevMainPowerOnValidW::new(self, 12)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn u0_usb_usbdev_power_off_req(&mut self) -> U0UsbUsbdevPowerOffReqW<StgSyscfg2Spec> {
        U0UsbUsbdevPowerOffReqW::new(self, 15)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn u0_usb_usbdev_power_on_valid(&mut self) -> U0UsbUsbdevPowerOnValidW<StgSyscfg2Spec> {
        U0UsbUsbdevPowerOnValidW::new(self, 18)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn u0_usb_utmi_dmpulldown_sit(&mut self) -> U0UsbUtmiDmpulldownSitW<StgSyscfg2Spec> {
        U0UsbUtmiDmpulldownSitW::new(self, 19)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    #[must_use]
    pub fn u0_usb_utmi_dppulldown_sit(&mut self) -> U0UsbUtmiDppulldownSitW<StgSyscfg2Spec> {
        U0UsbUtmiDppulldownSitW::new(self, 20)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    #[must_use]
    pub fn u0_usb_utmi_fslsserialmode_sit(
        &mut self,
    ) -> U0UsbUtmiFslsserialmodeSitW<StgSyscfg2Spec> {
        U0UsbUtmiFslsserialmodeSitW::new(self, 21)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    #[must_use]
    pub fn u0_usb_utmi_idpullup_sit(&mut self) -> U0UsbUtmiIdpullupSitW<StgSyscfg2Spec> {
        U0UsbUtmiIdpullupSitW::new(self, 24)
    }
    #[doc = "Bits 27:28"]
    #[inline(always)]
    #[must_use]
    pub fn u0_usb_utmi_opmode_sit(&mut self) -> U0UsbUtmiOpmodeSitW<StgSyscfg2Spec> {
        U0UsbUtmiOpmodeSitW::new(self, 27)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stg_syscfg_2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg2Spec;
impl crate::RegisterSpec for StgSyscfg2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_2::R`](R) reader structure"]
impl crate::Readable for StgSyscfg2Spec {}
#[doc = "`write(|w| ..)` method takes [`stg_syscfg_2::W`](W) writer structure"]
impl crate::Writable for StgSyscfg2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets stg_syscfg_2 to value 0x0004_1000"]
impl crate::Resettable for StgSyscfg2Spec {
    const RESET_VALUE: u32 = 0x0004_1000;
}
