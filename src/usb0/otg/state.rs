#[doc = "Register `state` reader"]
pub type R = crate::R<StateSpec>;
#[doc = "Register `state` writer"]
pub type W = crate::W<StateSpec>;
#[doc = "USB3 OTG Device state.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DevState {
    #[doc = "0: Bus is idle."]
    Idle = 0,
}
impl From<DevState> for u8 {
    #[inline(always)]
    fn from(variant: DevState) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DevState {
    type Ux = u8;
}
#[doc = "Field `dev_state` reader - USB3 OTG Device state."]
pub type DevStateR = crate::FieldReader<DevState>;
impl DevStateR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DevState> {
        match self.bits {
            0 => Some(DevState::Idle),
            _ => None,
        }
    }
    #[doc = "Bus is idle."]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == DevState::Idle
    }
}
#[doc = "Field `dev_state` writer - USB3 OTG Device state."]
pub type DevStateW<'a, REG> = crate::FieldWriter<'a, REG, 3, DevState>;
impl<'a, REG> DevStateW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Bus is idle."]
    #[inline(always)]
    pub fn idle(self) -> &'a mut crate::W<REG> {
        self.variant(DevState::Idle)
    }
}
#[doc = "USB3 OTG Device state.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HostState {
    #[doc = "0: Bus is idle."]
    Idle = 0,
    #[doc = "7: VBUS fall."]
    VbusFall = 7,
}
impl From<HostState> for u8 {
    #[inline(always)]
    fn from(variant: HostState) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for HostState {
    type Ux = u8;
}
#[doc = "Field `host_state` reader - USB3 OTG Device state."]
pub type HostStateR = crate::FieldReader<HostState>;
impl HostStateR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<HostState> {
        match self.bits {
            0 => Some(HostState::Idle),
            7 => Some(HostState::VbusFall),
            _ => None,
        }
    }
    #[doc = "Bus is idle."]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == HostState::Idle
    }
    #[doc = "VBUS fall."]
    #[inline(always)]
    pub fn is_vbus_fall(&self) -> bool {
        *self == HostState::VbusFall
    }
}
#[doc = "Field `host_state` writer - USB3 OTG Device state."]
pub type HostStateW<'a, REG> = crate::FieldWriter<'a, REG, 3, HostState>;
impl<'a, REG> HostStateW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Bus is idle."]
    #[inline(always)]
    pub fn idle(self) -> &'a mut crate::W<REG> {
        self.variant(HostState::Idle)
    }
    #[doc = "VBUS fall."]
    #[inline(always)]
    pub fn vbus_fall(self) -> &'a mut crate::W<REG> {
        self.variant(HostState::VbusFall)
    }
}
impl R {
    #[doc = "Bits 0:2 - USB3 OTG Device state."]
    #[inline(always)]
    pub fn dev_state(&self) -> DevStateR {
        DevStateR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - USB3 OTG Device state."]
    #[inline(always)]
    pub fn host_state(&self) -> HostStateR {
        HostStateR::new(((self.bits >> 3) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - USB3 OTG Device state."]
    #[inline(always)]
    #[must_use]
    pub fn dev_state(&mut self) -> DevStateW<StateSpec> {
        DevStateW::new(self, 0)
    }
    #[doc = "Bits 3:5 - USB3 OTG Device state."]
    #[inline(always)]
    #[must_use]
    pub fn host_state(&mut self) -> HostStateW<StateSpec> {
        HostStateW::new(self, 3)
    }
}
#[doc = "USB3 OTG state.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`state::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`state::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StateSpec;
impl crate::RegisterSpec for StateSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`state::R`](R) reader structure"]
impl crate::Readable for StateSpec {}
#[doc = "`write(|w| ..)` method takes [`state::W`](W) writer structure"]
impl crate::Writable for StateSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets state to value 0"]
impl crate::Resettable for StateSpec {
    const RESET_VALUE: u32 = 0;
}
