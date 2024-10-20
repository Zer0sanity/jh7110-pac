#[doc = "Register `cmd` reader"]
pub type R = crate::R<CmdSpec>;
#[doc = "Register `cmd` writer"]
pub type W = crate::W<CmdSpec>;
#[doc = "OTG bus request.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BusReq {
    #[doc = "0: No request for bus mode."]
    None = 0,
    #[doc = "1: Request the bus for Device mode."]
    Dev = 1,
    #[doc = "2: Request the bus for Host mode."]
    Host = 2,
}
impl From<BusReq> for u8 {
    #[inline(always)]
    fn from(variant: BusReq) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BusReq {
    type Ux = u8;
}
#[doc = "Field `bus_req` reader - OTG bus request."]
pub type BusReqR = crate::FieldReader<BusReq>;
impl BusReqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<BusReq> {
        match self.bits {
            0 => Some(BusReq::None),
            1 => Some(BusReq::Dev),
            2 => Some(BusReq::Host),
            _ => None,
        }
    }
    #[doc = "No request for bus mode."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == BusReq::None
    }
    #[doc = "Request the bus for Device mode."]
    #[inline(always)]
    pub fn is_dev(&self) -> bool {
        *self == BusReq::Dev
    }
    #[doc = "Request the bus for Host mode."]
    #[inline(always)]
    pub fn is_host(&self) -> bool {
        *self == BusReq::Host
    }
}
#[doc = "Field `bus_req` writer - OTG bus request."]
pub type BusReqW<'a, REG> = crate::FieldWriter<'a, REG, 2, BusReq>;
impl<'a, REG> BusReqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No request for bus mode."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(BusReq::None)
    }
    #[doc = "Request the bus for Device mode."]
    #[inline(always)]
    pub fn dev(self) -> &'a mut crate::W<REG> {
        self.variant(BusReq::Dev)
    }
    #[doc = "Request the bus for Host mode."]
    #[inline(always)]
    pub fn host(self) -> &'a mut crate::W<REG> {
        self.variant(BusReq::Host)
    }
}
#[doc = "OTG control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Otg {
    #[doc = "0: OTG none."]
    None = 0,
    #[doc = "1: OTG enable."]
    En = 1,
    #[doc = "2: OTG disable."]
    Dis = 2,
}
impl From<Otg> for u8 {
    #[inline(always)]
    fn from(variant: Otg) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Otg {
    type Ux = u8;
}
#[doc = "Field `otg` reader - OTG control."]
pub type OtgR = crate::FieldReader<Otg>;
impl OtgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Otg> {
        match self.bits {
            0 => Some(Otg::None),
            1 => Some(Otg::En),
            2 => Some(Otg::Dis),
            _ => None,
        }
    }
    #[doc = "OTG none."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Otg::None
    }
    #[doc = "OTG enable."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Otg::En
    }
    #[doc = "OTG disable."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == Otg::Dis
    }
}
#[doc = "Field `otg` writer - OTG control."]
pub type OtgW<'a, REG> = crate::FieldWriter<'a, REG, 2, Otg>;
impl<'a, REG> OtgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "OTG none."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Otg::None)
    }
    #[doc = "OTG enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Otg::En)
    }
    #[doc = "OTG disable."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(Otg::Dis)
    }
}
#[doc = "Configure OTG as A-device.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADev {
    #[doc = "0: No OTG A-device configuration."]
    None = 0,
    #[doc = "1: Enable OTG configuration as A-device."]
    En = 1,
    #[doc = "2: Disable OTG configuration as A-device."]
    Dis = 2,
}
impl From<ADev> for u8 {
    #[inline(always)]
    fn from(variant: ADev) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ADev {
    type Ux = u8;
}
#[doc = "Field `a_dev` reader - Configure OTG as A-device."]
pub type ADevR = crate::FieldReader<ADev>;
impl ADevR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ADev> {
        match self.bits {
            0 => Some(ADev::None),
            1 => Some(ADev::En),
            2 => Some(ADev::Dis),
            _ => None,
        }
    }
    #[doc = "No OTG A-device configuration."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == ADev::None
    }
    #[doc = "Enable OTG configuration as A-device."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == ADev::En
    }
    #[doc = "Disable OTG configuration as A-device."]
    #[inline(always)]
    pub fn is_dis(&self) -> bool {
        *self == ADev::Dis
    }
}
#[doc = "Field `a_dev` writer - Configure OTG as A-device."]
pub type ADevW<'a, REG> = crate::FieldWriter<'a, REG, 2, ADev>;
impl<'a, REG> ADevW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No OTG A-device configuration."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(ADev::None)
    }
    #[doc = "Enable OTG configuration as A-device."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(ADev::En)
    }
    #[doc = "Disable OTG configuration as A-device."]
    #[inline(always)]
    pub fn dis(self) -> &'a mut crate::W<REG> {
        self.variant(ADev::Dis)
    }
}
#[doc = "OTG drop the bus.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BusDrop {
    #[doc = "0: No OTG bus drop."]
    None = 0,
    #[doc = "1: Drop the OTG bus for Device mode."]
    Dev = 1,
    #[doc = "2: Drop the OTG bus for Host mode."]
    Host = 2,
}
impl From<BusDrop> for u8 {
    #[inline(always)]
    fn from(variant: BusDrop) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BusDrop {
    type Ux = u8;
}
#[doc = "Field `bus_drop` reader - OTG drop the bus."]
pub type BusDropR = crate::FieldReader<BusDrop>;
impl BusDropR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<BusDrop> {
        match self.bits {
            0 => Some(BusDrop::None),
            1 => Some(BusDrop::Dev),
            2 => Some(BusDrop::Host),
            _ => None,
        }
    }
    #[doc = "No OTG bus drop."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == BusDrop::None
    }
    #[doc = "Drop the OTG bus for Device mode."]
    #[inline(always)]
    pub fn is_dev(&self) -> bool {
        *self == BusDrop::Dev
    }
    #[doc = "Drop the OTG bus for Host mode."]
    #[inline(always)]
    pub fn is_host(&self) -> bool {
        *self == BusDrop::Host
    }
}
#[doc = "Field `bus_drop` writer - OTG drop the bus."]
pub type BusDropW<'a, REG> = crate::FieldWriter<'a, REG, 2, BusDrop>;
impl<'a, REG> BusDropW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No OTG bus drop."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(BusDrop::None)
    }
    #[doc = "Drop the OTG bus for Device mode."]
    #[inline(always)]
    pub fn dev(self) -> &'a mut crate::W<REG> {
        self.variant(BusDrop::Dev)
    }
    #[doc = "Drop the OTG bus for Host mode."]
    #[inline(always)]
    pub fn host(self) -> &'a mut crate::W<REG> {
        self.variant(BusDrop::Host)
    }
}
#[doc = "OTG power down.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PowerOff {
    #[doc = "0: No power down."]
    None = 0,
    #[doc = "1: Power down USBSS-DEV."]
    Dev = 1,
    #[doc = "2: Power down CDNSXHCI."]
    Host = 2,
}
impl From<PowerOff> for u8 {
    #[inline(always)]
    fn from(variant: PowerOff) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PowerOff {
    type Ux = u8;
}
#[doc = "Field `power_off` reader - OTG power down."]
pub type PowerOffR = crate::FieldReader<PowerOff>;
impl PowerOffR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PowerOff> {
        match self.bits {
            0 => Some(PowerOff::None),
            1 => Some(PowerOff::Dev),
            2 => Some(PowerOff::Host),
            _ => None,
        }
    }
    #[doc = "No power down."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == PowerOff::None
    }
    #[doc = "Power down USBSS-DEV."]
    #[inline(always)]
    pub fn is_dev(&self) -> bool {
        *self == PowerOff::Dev
    }
    #[doc = "Power down CDNSXHCI."]
    #[inline(always)]
    pub fn is_host(&self) -> bool {
        *self == PowerOff::Host
    }
}
#[doc = "Field `power_off` writer - OTG power down."]
pub type PowerOffW<'a, REG> = crate::FieldWriter<'a, REG, 2, PowerOff>;
impl<'a, REG> PowerOffW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No power down."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(PowerOff::None)
    }
    #[doc = "Power down USBSS-DEV."]
    #[inline(always)]
    pub fn dev(self) -> &'a mut crate::W<REG> {
        self.variant(PowerOff::Dev)
    }
    #[doc = "Power down CDNSXHCI."]
    #[inline(always)]
    pub fn host(self) -> &'a mut crate::W<REG> {
        self.variant(PowerOff::Host)
    }
}
impl R {
    #[doc = "Bits 0:1 - OTG bus request."]
    #[inline(always)]
    pub fn bus_req(&self) -> BusReqR {
        BusReqR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - OTG control."]
    #[inline(always)]
    pub fn otg(&self) -> OtgR {
        OtgR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Configure OTG as A-device."]
    #[inline(always)]
    pub fn a_dev(&self) -> ADevR {
        ADevR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:9 - OTG drop the bus."]
    #[inline(always)]
    pub fn bus_drop(&self) -> BusDropR {
        BusDropR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 11:12 - OTG power down."]
    #[inline(always)]
    pub fn power_off(&self) -> PowerOffR {
        PowerOffR::new(((self.bits >> 11) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - OTG bus request."]
    #[inline(always)]
    #[must_use]
    pub fn bus_req(&mut self) -> BusReqW<CmdSpec> {
        BusReqW::new(self, 0)
    }
    #[doc = "Bits 2:3 - OTG control."]
    #[inline(always)]
    #[must_use]
    pub fn otg(&mut self) -> OtgW<CmdSpec> {
        OtgW::new(self, 2)
    }
    #[doc = "Bits 4:5 - Configure OTG as A-device."]
    #[inline(always)]
    #[must_use]
    pub fn a_dev(&mut self) -> ADevW<CmdSpec> {
        ADevW::new(self, 4)
    }
    #[doc = "Bits 8:9 - OTG drop the bus."]
    #[inline(always)]
    #[must_use]
    pub fn bus_drop(&mut self) -> BusDropW<CmdSpec> {
        BusDropW::new(self, 8)
    }
    #[doc = "Bits 11:12 - OTG power down."]
    #[inline(always)]
    #[must_use]
    pub fn power_off(&mut self) -> PowerOffW<CmdSpec> {
        PowerOffW::new(self, 11)
    }
}
#[doc = "USB3 OTG command.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmdSpec;
impl crate::RegisterSpec for CmdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmd::R`](R) reader structure"]
impl crate::Readable for CmdSpec {}
#[doc = "`write(|w| ..)` method takes [`cmd::W`](W) writer structure"]
impl crate::Writable for CmdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets cmd to value 0"]
impl crate::Resettable for CmdSpec {
    const RESET_VALUE: u32 = 0;
}
