#[doc = "Register `stg_syscfg_6` reader"]
pub type R = crate::R<StgSyscfg6Spec>;
#[doc = "Register `stg_syscfg_6` writer"]
pub type W = crate::W<StgSyscfg6Spec>;
#[doc = "Field `u0_usb_xhci_debug_sel` reader - "]
pub type U0UsbXhciDebugSelR = crate::FieldReader;
#[doc = "Field `u0_usb_xhci_debug_sel` writer - "]
pub type U0UsbXhciDebugSelW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `u0_usb_xhci_main_power_off_ack` reader - "]
pub type U0UsbXhciMainPowerOffAckR = crate::BitReader;
#[doc = "Field `u0_usb_xhci_main_power_off_req` reader - "]
pub type U0UsbXhciMainPowerOffReqR = crate::BitReader;
#[doc = "Field `u0_usb_xhci_main_power_off_req` writer - "]
pub type U0UsbXhciMainPowerOffReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_usb_xhci_main_power_on_ready` reader - "]
pub type U0UsbXhciMainPowerOnReadyR = crate::BitReader;
#[doc = "Field `u0_usb_xhci_main_power_on_req` reader - "]
pub type U0UsbXhciMainPowerOnReqR = crate::BitReader;
#[doc = "Field `u0_usb_xhci_main_power_on_valid` reader - "]
pub type U0UsbXhciMainPowerOnValidR = crate::BitReader;
#[doc = "Field `u0_usb_xhci_main_power_on_valid` writer - "]
pub type U0UsbXhciMainPowerOnValidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_usb_xhci_power_off_ack` reader - "]
pub type U0UsbXhciPowerOffAckR = crate::BitReader;
#[doc = "Field `u0_usb_xhci_power_off_ready` reader - "]
pub type U0UsbXhciPowerOffReadyR = crate::BitReader;
#[doc = "Field `u0_usb_xhci_power_off_req` reader - "]
pub type U0UsbXhciPowerOffReqR = crate::BitReader;
#[doc = "Field `u0_usb_xhci_power_off_req` writer - "]
pub type U0UsbXhciPowerOffReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_usb_xhci_power_on_ready` reader - "]
pub type U0UsbXhciPowerOnReadyR = crate::BitReader;
#[doc = "Field `u0_usb_xhci_power_on_req` reader - "]
pub type U0UsbXhciPowerOnReqR = crate::BitReader;
#[doc = "Field `u0_usb_xhci_power_on_valid` reader - "]
pub type U0UsbXhciPowerOnValidR = crate::BitReader;
#[doc = "Field `u0_usb_xhci_power_on_valid` writer - "]
pub type U0UsbXhciPowerOnValidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_e2_cease_from_tile_0` reader - "]
pub type U0E2CeaseFromTile0R = crate::BitReader;
#[doc = "Field `u0_e2_debug_from_tile_0` reader - "]
pub type U0E2DebugFromTile0R = crate::BitReader;
#[doc = "Field `u0_e2_halt_from_tile_0` reader - "]
pub type U0E2HaltFromTile0R = crate::BitReader;
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn u0_usb_xhci_debug_sel(&self) -> U0UsbXhciDebugSelR {
        U0UsbXhciDebugSelR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn u0_usb_xhci_main_power_off_ack(&self) -> U0UsbXhciMainPowerOffAckR {
        U0UsbXhciMainPowerOffAckR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn u0_usb_xhci_main_power_off_req(&self) -> U0UsbXhciMainPowerOffReqR {
        U0UsbXhciMainPowerOffReqR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn u0_usb_xhci_main_power_on_ready(&self) -> U0UsbXhciMainPowerOnReadyR {
        U0UsbXhciMainPowerOnReadyR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn u0_usb_xhci_main_power_on_req(&self) -> U0UsbXhciMainPowerOnReqR {
        U0UsbXhciMainPowerOnReqR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn u0_usb_xhci_main_power_on_valid(&self) -> U0UsbXhciMainPowerOnValidR {
        U0UsbXhciMainPowerOnValidR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn u0_usb_xhci_power_off_ack(&self) -> U0UsbXhciPowerOffAckR {
        U0UsbXhciPowerOffAckR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn u0_usb_xhci_power_off_ready(&self) -> U0UsbXhciPowerOffReadyR {
        U0UsbXhciPowerOffReadyR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn u0_usb_xhci_power_off_req(&self) -> U0UsbXhciPowerOffReqR {
        U0UsbXhciPowerOffReqR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn u0_usb_xhci_power_on_ready(&self) -> U0UsbXhciPowerOnReadyR {
        U0UsbXhciPowerOnReadyR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn u0_usb_xhci_power_on_req(&self) -> U0UsbXhciPowerOnReqR {
        U0UsbXhciPowerOnReqR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn u0_usb_xhci_power_on_valid(&self) -> U0UsbXhciPowerOnValidR {
        U0UsbXhciPowerOnValidR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn u0_e2_cease_from_tile_0(&self) -> U0E2CeaseFromTile0R {
        U0E2CeaseFromTile0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn u0_e2_debug_from_tile_0(&self) -> U0E2DebugFromTile0R {
        U0E2DebugFromTile0R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn u0_e2_halt_from_tile_0(&self) -> U0E2HaltFromTile0R {
        U0E2HaltFromTile0R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    #[must_use]
    pub fn u0_usb_xhci_debug_sel(&mut self) -> U0UsbXhciDebugSelW<StgSyscfg6Spec> {
        U0UsbXhciDebugSelW::new(self, 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn u0_usb_xhci_main_power_off_req(&mut self) -> U0UsbXhciMainPowerOffReqW<StgSyscfg6Spec> {
        U0UsbXhciMainPowerOffReqW::new(self, 6)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn u0_usb_xhci_main_power_on_valid(
        &mut self,
    ) -> U0UsbXhciMainPowerOnValidW<StgSyscfg6Spec> {
        U0UsbXhciMainPowerOnValidW::new(self, 9)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn u0_usb_xhci_power_off_req(&mut self) -> U0UsbXhciPowerOffReqW<StgSyscfg6Spec> {
        U0UsbXhciPowerOffReqW::new(self, 12)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn u0_usb_xhci_power_on_valid(&mut self) -> U0UsbXhciPowerOnValidW<StgSyscfg6Spec> {
        U0UsbXhciPowerOnValidW::new(self, 15)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 24\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stg_syscfg_6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg6Spec;
impl crate::RegisterSpec for StgSyscfg6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_6::R`](R) reader structure"]
impl crate::Readable for StgSyscfg6Spec {}
#[doc = "`write(|w| ..)` method takes [`stg_syscfg_6::W`](W) writer structure"]
impl crate::Writable for StgSyscfg6Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets stg_syscfg_6 to value 0x8200"]
impl crate::Resettable for StgSyscfg6Spec {
    const RESET_VALUE: u32 = 0x8200;
}
