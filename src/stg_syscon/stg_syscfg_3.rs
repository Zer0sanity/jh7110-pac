#[doc = "Register `stg_syscfg_3` reader"]
pub type R = crate::R<StgSyscfg3Spec>;
#[doc = "Register `stg_syscfg_3` writer"]
pub type W = crate::W<StgSyscfg3Spec>;
#[doc = "Field `u0_usb_utmi_rxvalidh_sit` reader - "]
pub type U0UsbUtmiRxvalidhSitR = crate::BitReader;
#[doc = "Field `u0_usb_utmi_sessvld` reader - "]
pub type U0UsbUtmiSessvldR = crate::BitReader;
#[doc = "Field `u0_usb_utmi_sessvld` writer - "]
pub type U0UsbUtmiSessvldW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_usb_utmi_termselect_sit` reader - "]
pub type U0UsbUtmiTermselectSitR = crate::BitReader;
#[doc = "Field `u0_usb_utmi_termselect_sit` writer - "]
pub type U0UsbUtmiTermselectSitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_usb_utmi_tx_dat_sit` reader - "]
pub type U0UsbUtmiTxDatSitR = crate::BitReader;
#[doc = "Field `u0_usb_utmi_tx_dat_sit` writer - "]
pub type U0UsbUtmiTxDatSitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_usb_utmi_tx_enable_n_sit` reader - "]
pub type U0UsbUtmiTxEnableNSitR = crate::BitReader;
#[doc = "Field `u0_usb_utmi_tx_enable_n_sit` writer - "]
pub type U0UsbUtmiTxEnableNSitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_usb_utmi_tx_se0_sit` reader - "]
pub type U0UsbUtmiTxSe0SitR = crate::BitReader;
#[doc = "Field `u0_usb_utmi_tx_se0_sit` writer - "]
pub type U0UsbUtmiTxSe0SitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_usb_utmi_txbitstuffenable_sit` reader - "]
pub type U0UsbUtmiTxbitstuffenableSitR = crate::BitReader;
#[doc = "Field `u0_usb_utmi_txbitstuffenable_sit` writer - "]
pub type U0UsbUtmiTxbitstuffenableSitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_usb_utmi_txready_sit` reader - "]
pub type U0UsbUtmiTxreadySitR = crate::BitReader;
#[doc = "Field `u0_usb_utmi_txvalid_sit` reader - "]
pub type U0UsbUtmiTxvalidSitR = crate::BitReader;
#[doc = "Field `u0_usb_utmi_txvalid_sit` writer - "]
pub type U0UsbUtmiTxvalidSitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_usb_utmi_txvalidh_sit` reader - "]
pub type U0UsbUtmiTxvalidhSitR = crate::BitReader;
#[doc = "Field `u0_usb_utmi_txvalidh_sit` writer - "]
pub type U0UsbUtmiTxvalidhSitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_usb_utmi_vbusvalid_sit` reader - "]
pub type U0UsbUtmiVbusvalidSitR = crate::BitReader;
#[doc = "Field `u0_usb_utmi_xcvrselect_sit` reader - "]
pub type U0UsbUtmiXcvrselectSitR = crate::FieldReader;
#[doc = "Field `u0_usb_utmi_xcvrselect_sit` writer - "]
pub type U0UsbUtmiXcvrselectSitW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `u0_usb_utmi_vdm_src_en` reader - "]
pub type U0UsbUtmiVdmSrcEnR = crate::BitReader;
#[doc = "Field `u0_usb_utmi_vdp_src_en` reader - "]
pub type U0UsbUtmiVdpSrcEnR = crate::BitReader;
#[doc = "Field `u0_usb_wakeup` reader - "]
pub type U0UsbWakeupR = crate::BitReader;
#[doc = "Field `u0_usb_wakeup` writer - "]
pub type U0UsbWakeupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_usb_xhc_d0_ack` reader - "]
pub type U0UsbXhcD0AckR = crate::BitReader;
#[doc = "Field `u0_usb_xhc_d0_req` reader - "]
pub type U0UsbXhcD0ReqR = crate::BitReader;
#[doc = "Field `u0_usb_xhc_d0_req` writer - "]
pub type U0UsbXhcD0ReqW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn u0_usb_utmi_rxvalidh_sit(&self) -> U0UsbUtmiRxvalidhSitR {
        U0UsbUtmiRxvalidhSitR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn u0_usb_utmi_sessvld(&self) -> U0UsbUtmiSessvldR {
        U0UsbUtmiSessvldR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn u0_usb_utmi_termselect_sit(&self) -> U0UsbUtmiTermselectSitR {
        U0UsbUtmiTermselectSitR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn u0_usb_utmi_tx_dat_sit(&self) -> U0UsbUtmiTxDatSitR {
        U0UsbUtmiTxDatSitR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn u0_usb_utmi_tx_enable_n_sit(&self) -> U0UsbUtmiTxEnableNSitR {
        U0UsbUtmiTxEnableNSitR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn u0_usb_utmi_tx_se0_sit(&self) -> U0UsbUtmiTxSe0SitR {
        U0UsbUtmiTxSe0SitR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn u0_usb_utmi_txbitstuffenable_sit(&self) -> U0UsbUtmiTxbitstuffenableSitR {
        U0UsbUtmiTxbitstuffenableSitR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn u0_usb_utmi_txready_sit(&self) -> U0UsbUtmiTxreadySitR {
        U0UsbUtmiTxreadySitR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn u0_usb_utmi_txvalid_sit(&self) -> U0UsbUtmiTxvalidSitR {
        U0UsbUtmiTxvalidSitR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn u0_usb_utmi_txvalidh_sit(&self) -> U0UsbUtmiTxvalidhSitR {
        U0UsbUtmiTxvalidhSitR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn u0_usb_utmi_vbusvalid_sit(&self) -> U0UsbUtmiVbusvalidSitR {
        U0UsbUtmiVbusvalidSitR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:12"]
    #[inline(always)]
    pub fn u0_usb_utmi_xcvrselect_sit(&self) -> U0UsbUtmiXcvrselectSitR {
        U0UsbUtmiXcvrselectSitR::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn u0_usb_utmi_vdm_src_en(&self) -> U0UsbUtmiVdmSrcEnR {
        U0UsbUtmiVdmSrcEnR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn u0_usb_utmi_vdp_src_en(&self) -> U0UsbUtmiVdpSrcEnR {
        U0UsbUtmiVdpSrcEnR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn u0_usb_wakeup(&self) -> U0UsbWakeupR {
        U0UsbWakeupR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn u0_usb_xhc_d0_ack(&self) -> U0UsbXhcD0AckR {
        U0UsbXhcD0AckR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn u0_usb_xhc_d0_req(&self) -> U0UsbXhcD0ReqR {
        U0UsbXhcD0ReqR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn u0_usb_utmi_sessvld(&mut self) -> U0UsbUtmiSessvldW<StgSyscfg3Spec> {
        U0UsbUtmiSessvldW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn u0_usb_utmi_termselect_sit(&mut self) -> U0UsbUtmiTermselectSitW<StgSyscfg3Spec> {
        U0UsbUtmiTermselectSitW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn u0_usb_utmi_tx_dat_sit(&mut self) -> U0UsbUtmiTxDatSitW<StgSyscfg3Spec> {
        U0UsbUtmiTxDatSitW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn u0_usb_utmi_tx_enable_n_sit(&mut self) -> U0UsbUtmiTxEnableNSitW<StgSyscfg3Spec> {
        U0UsbUtmiTxEnableNSitW::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn u0_usb_utmi_tx_se0_sit(&mut self) -> U0UsbUtmiTxSe0SitW<StgSyscfg3Spec> {
        U0UsbUtmiTxSe0SitW::new(self, 5)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn u0_usb_utmi_txbitstuffenable_sit(
        &mut self,
    ) -> U0UsbUtmiTxbitstuffenableSitW<StgSyscfg3Spec> {
        U0UsbUtmiTxbitstuffenableSitW::new(self, 6)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn u0_usb_utmi_txvalid_sit(&mut self) -> U0UsbUtmiTxvalidSitW<StgSyscfg3Spec> {
        U0UsbUtmiTxvalidSitW::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn u0_usb_utmi_txvalidh_sit(&mut self) -> U0UsbUtmiTxvalidhSitW<StgSyscfg3Spec> {
        U0UsbUtmiTxvalidhSitW::new(self, 9)
    }
    #[doc = "Bits 11:12"]
    #[inline(always)]
    #[must_use]
    pub fn u0_usb_utmi_xcvrselect_sit(&mut self) -> U0UsbUtmiXcvrselectSitW<StgSyscfg3Spec> {
        U0UsbUtmiXcvrselectSitW::new(self, 11)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn u0_usb_wakeup(&mut self) -> U0UsbWakeupW<StgSyscfg3Spec> {
        U0UsbWakeupW::new(self, 15)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn u0_usb_xhc_d0_req(&mut self) -> U0UsbXhcD0ReqW<StgSyscfg3Spec> {
        U0UsbXhcD0ReqW::new(self, 17)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 12\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stg_syscfg_3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg3Spec;
impl crate::RegisterSpec for StgSyscfg3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_3::R`](R) reader structure"]
impl crate::Readable for StgSyscfg3Spec {}
#[doc = "`write(|w| ..)` method takes [`stg_syscfg_3::W`](W) writer structure"]
impl crate::Writable for StgSyscfg3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets stg_syscfg_3 to value 0x02"]
impl crate::Resettable for StgSyscfg3Spec {
    const RESET_VALUE: u32 = 0x02;
}
