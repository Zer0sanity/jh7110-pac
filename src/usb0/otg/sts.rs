#[doc = "Register `sts` reader"]
pub type R = crate::R<StsSpec>;
#[doc = "Register `sts` writer"]
pub type W = crate::W<StsSpec>;
#[doc = "Field `id_value` reader - USB3 OTG current value of the ID pin - only valid when idpullup in OTGCTRL1_TYPE set to `1`."]
pub type IdValueR = crate::BitReader;
#[doc = "Field `id_value` writer - USB3 OTG current value of the ID pin - only valid when idpullup in OTGCTRL1_TYPE set to `1`."]
pub type IdValueW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `vbus_valid` reader - USB3 OTG current value of the vbus_valid pin."]
pub type VbusValidR = crate::BitReader;
#[doc = "Field `vbus_valid` writer - USB3 OTG current value of the vbus_valid pin."]
pub type VbusValidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `session_valid` reader - USB3 OTG current value of the b_sess_vld pin."]
pub type SessionValidR = crate::BitReader;
#[doc = "Field `session_valid` writer - USB3 OTG current value of the b_sess_vld pin."]
pub type SessionValidW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "USB3 OTG active mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Active {
    #[doc = "0: No OTG mode is active."]
    None = 0,
    #[doc = "1: OTG Device mode is active."]
    Dev = 1,
    #[doc = "2: OTG Host mode is active."]
    Host = 2,
}
impl From<Active> for u8 {
    #[inline(always)]
    fn from(variant: Active) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Active {
    type Ux = u8;
}
#[doc = "Field `active` reader - USB3 OTG active mode."]
pub type ActiveR = crate::FieldReader<Active>;
impl ActiveR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Active> {
        match self.bits {
            0 => Some(Active::None),
            1 => Some(Active::Dev),
            2 => Some(Active::Host),
            _ => None,
        }
    }
    #[doc = "No OTG mode is active."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Active::None
    }
    #[doc = "OTG Device mode is active."]
    #[inline(always)]
    pub fn is_dev(&self) -> bool {
        *self == Active::Dev
    }
    #[doc = "OTG Host mode is active."]
    #[inline(always)]
    pub fn is_host(&self) -> bool {
        *self == Active::Host
    }
}
#[doc = "Field `active` writer - USB3 OTG active mode."]
pub type ActiveW<'a, REG> = crate::FieldWriter<'a, REG, 2, Active>;
impl<'a, REG> ActiveW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No OTG mode is active."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Active::None)
    }
    #[doc = "OTG Device mode is active."]
    #[inline(always)]
    pub fn dev(self) -> &'a mut crate::W<REG> {
        self.variant(Active::Dev)
    }
    #[doc = "OTG Host mode is active."]
    #[inline(always)]
    pub fn host(self) -> &'a mut crate::W<REG> {
        self.variant(Active::Host)
    }
}
#[doc = "USB3 OTG Controller (not) readiness status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OtgNrdy {
    #[doc = "0: OTG Controller is ready."]
    Ready = 0,
    #[doc = "1: OTG Controller is not ready."]
    NotReady = 1,
}
impl From<OtgNrdy> for bool {
    #[inline(always)]
    fn from(variant: OtgNrdy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `otg_nrdy` reader - USB3 OTG Controller (not) readiness status."]
pub type OtgNrdyR = crate::BitReader<OtgNrdy>;
impl OtgNrdyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OtgNrdy {
        match self.bits {
            false => OtgNrdy::Ready,
            true => OtgNrdy::NotReady,
        }
    }
    #[doc = "OTG Controller is ready."]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == OtgNrdy::Ready
    }
    #[doc = "OTG Controller is not ready."]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == OtgNrdy::NotReady
    }
}
#[doc = "Field `otg_nrdy` writer - USB3 OTG Controller (not) readiness status."]
pub type OtgNrdyW<'a, REG> = crate::BitWriter<'a, REG, OtgNrdy>;
impl<'a, REG> OtgNrdyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "OTG Controller is ready."]
    #[inline(always)]
    pub fn ready(self) -> &'a mut crate::W<REG> {
        self.variant(OtgNrdy::Ready)
    }
    #[doc = "OTG Controller is not ready."]
    #[inline(always)]
    pub fn not_ready(self) -> &'a mut crate::W<REG> {
        self.variant(OtgNrdy::NotReady)
    }
}
#[doc = "USB3 OTG value of the strap pins.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Strap {
    #[doc = "0: No default configuration."]
    NoDefaultCfg = 0,
    #[doc = "1: Initially configured as Host in OTG mode."]
    HostOtg = 1,
    #[doc = "2: Initially configured as Host."]
    Host = 2,
    #[doc = "4: Initially configured as Device."]
    Gadget = 4,
}
impl From<Strap> for u8 {
    #[inline(always)]
    fn from(variant: Strap) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Strap {
    type Ux = u8;
}
#[doc = "Field `strap` reader - USB3 OTG value of the strap pins."]
pub type StrapR = crate::FieldReader<Strap>;
impl StrapR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Strap> {
        match self.bits {
            0 => Some(Strap::NoDefaultCfg),
            1 => Some(Strap::HostOtg),
            2 => Some(Strap::Host),
            4 => Some(Strap::Gadget),
            _ => None,
        }
    }
    #[doc = "No default configuration."]
    #[inline(always)]
    pub fn is_no_default_cfg(&self) -> bool {
        *self == Strap::NoDefaultCfg
    }
    #[doc = "Initially configured as Host in OTG mode."]
    #[inline(always)]
    pub fn is_host_otg(&self) -> bool {
        *self == Strap::HostOtg
    }
    #[doc = "Initially configured as Host."]
    #[inline(always)]
    pub fn is_host(&self) -> bool {
        *self == Strap::Host
    }
    #[doc = "Initially configured as Device."]
    #[inline(always)]
    pub fn is_gadget(&self) -> bool {
        *self == Strap::Gadget
    }
}
#[doc = "Field `strap` writer - USB3 OTG value of the strap pins."]
pub type StrapW<'a, REG> = crate::FieldWriter<'a, REG, 3, Strap>;
impl<'a, REG> StrapW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No default configuration."]
    #[inline(always)]
    pub fn no_default_cfg(self) -> &'a mut crate::W<REG> {
        self.variant(Strap::NoDefaultCfg)
    }
    #[doc = "Initially configured as Host in OTG mode."]
    #[inline(always)]
    pub fn host_otg(self) -> &'a mut crate::W<REG> {
        self.variant(Strap::HostOtg)
    }
    #[doc = "Initially configured as Host."]
    #[inline(always)]
    pub fn host(self) -> &'a mut crate::W<REG> {
        self.variant(Strap::Host)
    }
    #[doc = "Initially configured as Device."]
    #[inline(always)]
    pub fn gadget(self) -> &'a mut crate::W<REG> {
        self.variant(Strap::Gadget)
    }
}
#[doc = "USB3 OTG readiness status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ready {
    #[doc = "0: OTG is not ready."]
    None = 0,
    #[doc = "1: OTG Host is ready - Host mode turned on."]
    Xhci = 1,
    #[doc = "2: OTG Device is ready - Device mode turned on."]
    Dev = 2,
}
impl From<Ready> for u8 {
    #[inline(always)]
    fn from(variant: Ready) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ready {
    type Ux = u8;
}
#[doc = "Field `ready` reader - USB3 OTG readiness status."]
pub type ReadyR = crate::FieldReader<Ready>;
impl ReadyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Ready> {
        match self.bits {
            0 => Some(Ready::None),
            1 => Some(Ready::Xhci),
            2 => Some(Ready::Dev),
            _ => None,
        }
    }
    #[doc = "OTG is not ready."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Ready::None
    }
    #[doc = "OTG Host is ready - Host mode turned on."]
    #[inline(always)]
    pub fn is_xhci(&self) -> bool {
        *self == Ready::Xhci
    }
    #[doc = "OTG Device is ready - Device mode turned on."]
    #[inline(always)]
    pub fn is_dev(&self) -> bool {
        *self == Ready::Dev
    }
}
#[doc = "Field `ready` writer - USB3 OTG readiness status."]
pub type ReadyW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ready>;
impl<'a, REG> ReadyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "OTG is not ready."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Ready::None)
    }
    #[doc = "OTG Host is ready - Host mode turned on."]
    #[inline(always)]
    pub fn xhci(self) -> &'a mut crate::W<REG> {
        self.variant(Ready::Xhci)
    }
    #[doc = "OTG Device is ready - Device mode turned on."]
    #[inline(always)]
    pub fn dev(self) -> &'a mut crate::W<REG> {
        self.variant(Ready::Dev)
    }
}
impl R {
    #[doc = "Bit 0 - USB3 OTG current value of the ID pin - only valid when idpullup in OTGCTRL1_TYPE set to `1`."]
    #[inline(always)]
    pub fn id_value(&self) -> IdValueR {
        IdValueR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - USB3 OTG current value of the vbus_valid pin."]
    #[inline(always)]
    pub fn vbus_valid(&self) -> VbusValidR {
        VbusValidR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - USB3 OTG current value of the b_sess_vld pin."]
    #[inline(always)]
    pub fn session_valid(&self) -> SessionValidR {
        SessionValidR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - USB3 OTG active mode."]
    #[inline(always)]
    pub fn active(&self) -> ActiveR {
        ActiveR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 11 - USB3 OTG Controller (not) readiness status."]
    #[inline(always)]
    pub fn otg_nrdy(&self) -> OtgNrdyR {
        OtgNrdyR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - USB3 OTG value of the strap pins."]
    #[inline(always)]
    pub fn strap(&self) -> StrapR {
        StrapR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 26:27 - USB3 OTG readiness status."]
    #[inline(always)]
    pub fn ready(&self) -> ReadyR {
        ReadyR::new(((self.bits >> 26) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - USB3 OTG current value of the ID pin - only valid when idpullup in OTGCTRL1_TYPE set to `1`."]
    #[inline(always)]
    #[must_use]
    pub fn id_value(&mut self) -> IdValueW<StsSpec> {
        IdValueW::new(self, 0)
    }
    #[doc = "Bit 1 - USB3 OTG current value of the vbus_valid pin."]
    #[inline(always)]
    #[must_use]
    pub fn vbus_valid(&mut self) -> VbusValidW<StsSpec> {
        VbusValidW::new(self, 1)
    }
    #[doc = "Bit 2 - USB3 OTG current value of the b_sess_vld pin."]
    #[inline(always)]
    #[must_use]
    pub fn session_valid(&mut self) -> SessionValidW<StsSpec> {
        SessionValidW::new(self, 2)
    }
    #[doc = "Bits 3:4 - USB3 OTG active mode."]
    #[inline(always)]
    #[must_use]
    pub fn active(&mut self) -> ActiveW<StsSpec> {
        ActiveW::new(self, 3)
    }
    #[doc = "Bit 11 - USB3 OTG Controller (not) readiness status."]
    #[inline(always)]
    #[must_use]
    pub fn otg_nrdy(&mut self) -> OtgNrdyW<StsSpec> {
        OtgNrdyW::new(self, 11)
    }
    #[doc = "Bits 12:14 - USB3 OTG value of the strap pins."]
    #[inline(always)]
    #[must_use]
    pub fn strap(&mut self) -> StrapW<StsSpec> {
        StrapW::new(self, 12)
    }
    #[doc = "Bits 26:27 - USB3 OTG readiness status."]
    #[inline(always)]
    #[must_use]
    pub fn ready(&mut self) -> ReadyW<StsSpec> {
        ReadyW::new(self, 26)
    }
}
#[doc = "USB3 OTG status.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sts::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sts::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StsSpec;
impl crate::RegisterSpec for StsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sts::R`](R) reader structure"]
impl crate::Readable for StsSpec {}
#[doc = "`write(|w| ..)` method takes [`sts::W`](W) writer structure"]
impl crate::Writable for StsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets sts to value 0"]
impl crate::Resettable for StsSpec {
    const RESET_VALUE: u32 = 0;
}
