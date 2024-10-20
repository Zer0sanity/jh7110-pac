#[doc = "Register `usb_sts` reader"]
pub type R = crate::R<UsbStsSpec>;
#[doc = "Register `usb_sts` writer"]
pub type W = crate::W<UsbStsSpec>;
#[doc = "Device configuration status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cfgsts {
    #[doc = "0: Device not configured."]
    NotCfg = 0,
    #[doc = "1: Device configured."]
    Cfg = 1,
}
impl From<Cfgsts> for bool {
    #[inline(always)]
    fn from(variant: Cfgsts) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `cfgsts` reader - Device configuration status."]
pub type CfgstsR = crate::BitReader<Cfgsts>;
impl CfgstsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cfgsts {
        match self.bits {
            false => Cfgsts::NotCfg,
            true => Cfgsts::Cfg,
        }
    }
    #[doc = "Device not configured."]
    #[inline(always)]
    pub fn is_not_cfg(&self) -> bool {
        *self == Cfgsts::NotCfg
    }
    #[doc = "Device configured."]
    #[inline(always)]
    pub fn is_cfg(&self) -> bool {
        *self == Cfgsts::Cfg
    }
}
#[doc = "Field `cfgsts` writer - Device configuration status."]
pub type CfgstsW<'a, REG> = crate::BitWriter<'a, REG, Cfgsts>;
impl<'a, REG> CfgstsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Device not configured."]
    #[inline(always)]
    pub fn not_cfg(self) -> &'a mut crate::W<REG> {
        self.variant(Cfgsts::NotCfg)
    }
    #[doc = "Device configured."]
    #[inline(always)]
    pub fn cfg(self) -> &'a mut crate::W<REG> {
        self.variant(Cfgsts::Cfg)
    }
}
#[doc = "On-chip memory overflow.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ov {
    #[doc = "0: On-chip memory status OK."]
    Ok = 0,
    #[doc = "1: On-chip memory overflow."]
    Overflow = 1,
}
impl From<Ov> for bool {
    #[inline(always)]
    fn from(variant: Ov) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ov` reader - On-chip memory overflow."]
pub type OvR = crate::BitReader<Ov>;
impl OvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ov {
        match self.bits {
            false => Ov::Ok,
            true => Ov::Overflow,
        }
    }
    #[doc = "On-chip memory status OK."]
    #[inline(always)]
    pub fn is_ok(&self) -> bool {
        *self == Ov::Ok
    }
    #[doc = "On-chip memory overflow."]
    #[inline(always)]
    pub fn is_overflow(&self) -> bool {
        *self == Ov::Overflow
    }
}
#[doc = "Field `ov` writer - On-chip memory overflow."]
pub type OvW<'a, REG> = crate::BitWriter<'a, REG, Ov>;
impl<'a, REG> OvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "On-chip memory status OK."]
    #[inline(always)]
    pub fn ok(self) -> &'a mut crate::W<REG> {
        self.variant(Ov::Ok)
    }
    #[doc = "On-chip memory overflow."]
    #[inline(always)]
    pub fn overflow(self) -> &'a mut crate::W<REG> {
        self.variant(Ov::Overflow)
    }
}
#[doc = "Superspeed connection status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usb3cons {
    #[doc = "0: SuperSpeed mode disconnected."]
    Disconnected = 0,
    #[doc = "1: SuperSpeed mode connected."]
    Connected = 1,
}
impl From<Usb3cons> for bool {
    #[inline(always)]
    fn from(variant: Usb3cons) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `usb3cons` reader - Superspeed connection status."]
pub type Usb3consR = crate::BitReader<Usb3cons>;
impl Usb3consR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usb3cons {
        match self.bits {
            false => Usb3cons::Disconnected,
            true => Usb3cons::Connected,
        }
    }
    #[doc = "SuperSpeed mode disconnected."]
    #[inline(always)]
    pub fn is_disconnected(&self) -> bool {
        *self == Usb3cons::Disconnected
    }
    #[doc = "SuperSpeed mode connected."]
    #[inline(always)]
    pub fn is_connected(&self) -> bool {
        *self == Usb3cons::Connected
    }
}
#[doc = "Field `usb3cons` writer - Superspeed connection status."]
pub type Usb3consW<'a, REG> = crate::BitWriter<'a, REG, Usb3cons>;
impl<'a, REG> Usb3consW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SuperSpeed mode disconnected."]
    #[inline(always)]
    pub fn disconnected(self) -> &'a mut crate::W<REG> {
        self.variant(Usb3cons::Disconnected)
    }
    #[doc = "SuperSpeed mode connected."]
    #[inline(always)]
    pub fn connected(self) -> &'a mut crate::W<REG> {
        self.variant(Usb3cons::Connected)
    }
}
#[doc = "DMA transfer configuration status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dtrans {
    #[doc = "0: Single DMA request."]
    SingleRequest = 0,
    #[doc = "1: Multiple TRB chain."]
    MultipleTrbChain = 1,
}
impl From<Dtrans> for bool {
    #[inline(always)]
    fn from(variant: Dtrans) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `dtrans` reader - DMA transfer configuration status."]
pub type DtransR = crate::BitReader<Dtrans>;
impl DtransR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dtrans {
        match self.bits {
            false => Dtrans::SingleRequest,
            true => Dtrans::MultipleTrbChain,
        }
    }
    #[doc = "Single DMA request."]
    #[inline(always)]
    pub fn is_single_request(&self) -> bool {
        *self == Dtrans::SingleRequest
    }
    #[doc = "Multiple TRB chain."]
    #[inline(always)]
    pub fn is_multiple_trb_chain(&self) -> bool {
        *self == Dtrans::MultipleTrbChain
    }
}
#[doc = "Field `dtrans` writer - DMA transfer configuration status."]
pub type DtransW<'a, REG> = crate::BitWriter<'a, REG, Dtrans>;
impl<'a, REG> DtransW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Single DMA request."]
    #[inline(always)]
    pub fn single_request(self) -> &'a mut crate::W<REG> {
        self.variant(Dtrans::SingleRequest)
    }
    #[doc = "Multiple TRB chain."]
    #[inline(always)]
    pub fn multiple_trb_chain(self) -> &'a mut crate::W<REG> {
        self.variant(Dtrans::MultipleTrbChain)
    }
}
#[doc = "Device speed.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Usbspeed {
    #[doc = "0: Undefined speed - value after reset."]
    Undefined = 0,
    #[doc = "1: Low speed."]
    Low = 1,
    #[doc = "2: Full speed."]
    Full = 2,
    #[doc = "3: High speed."]
    High = 3,
    #[doc = "4: Super speed."]
    Super = 4,
}
impl From<Usbspeed> for u8 {
    #[inline(always)]
    fn from(variant: Usbspeed) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Usbspeed {
    type Ux = u8;
}
#[doc = "Field `usbspeed` reader - Device speed."]
pub type UsbspeedR = crate::FieldReader<Usbspeed>;
impl UsbspeedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Usbspeed> {
        match self.bits {
            0 => Some(Usbspeed::Undefined),
            1 => Some(Usbspeed::Low),
            2 => Some(Usbspeed::Full),
            3 => Some(Usbspeed::High),
            4 => Some(Usbspeed::Super),
            _ => None,
        }
    }
    #[doc = "Undefined speed - value after reset."]
    #[inline(always)]
    pub fn is_undefined(&self) -> bool {
        *self == Usbspeed::Undefined
    }
    #[doc = "Low speed."]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == Usbspeed::Low
    }
    #[doc = "Full speed."]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == Usbspeed::Full
    }
    #[doc = "High speed."]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Usbspeed::High
    }
    #[doc = "Super speed."]
    #[inline(always)]
    pub fn is_super(&self) -> bool {
        *self == Usbspeed::Super
    }
}
#[doc = "Field `usbspeed` writer - Device speed."]
pub type UsbspeedW<'a, REG> = crate::FieldWriter<'a, REG, 3, Usbspeed>;
impl<'a, REG> UsbspeedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Undefined speed - value after reset."]
    #[inline(always)]
    pub fn undefined(self) -> &'a mut crate::W<REG> {
        self.variant(Usbspeed::Undefined)
    }
    #[doc = "Low speed."]
    #[inline(always)]
    pub fn low(self) -> &'a mut crate::W<REG> {
        self.variant(Usbspeed::Low)
    }
    #[doc = "Full speed."]
    #[inline(always)]
    pub fn full(self) -> &'a mut crate::W<REG> {
        self.variant(Usbspeed::Full)
    }
    #[doc = "High speed."]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Usbspeed::High)
    }
    #[doc = "Super speed."]
    #[inline(always)]
    pub fn super_(self) -> &'a mut crate::W<REG> {
        self.variant(Usbspeed::Super)
    }
}
#[doc = "Endianess for SFR access.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Endian {
    #[doc = "0: Little Endian order - default after hardware reset"]
    Little = 0,
    #[doc = "1: Big Endian order"]
    Big = 1,
}
impl From<Endian> for bool {
    #[inline(always)]
    fn from(variant: Endian) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `endian` reader - Endianess for SFR access."]
pub type EndianR = crate::BitReader<Endian>;
impl EndianR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Endian {
        match self.bits {
            false => Endian::Little,
            true => Endian::Big,
        }
    }
    #[doc = "Little Endian order - default after hardware reset"]
    #[inline(always)]
    pub fn is_little(&self) -> bool {
        *self == Endian::Little
    }
    #[doc = "Big Endian order"]
    #[inline(always)]
    pub fn is_big(&self) -> bool {
        *self == Endian::Big
    }
}
#[doc = "Field `endian` writer - Endianess for SFR access."]
pub type EndianW<'a, REG> = crate::BitWriter<'a, REG, Endian>;
impl<'a, REG> EndianW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Little Endian order - default after hardware reset"]
    #[inline(always)]
    pub fn little(self) -> &'a mut crate::W<REG> {
        self.variant(Endian::Little)
    }
    #[doc = "Big Endian order"]
    #[inline(always)]
    pub fn big(self) -> &'a mut crate::W<REG> {
        self.variant(Endian::Big)
    }
}
#[doc = "FS/HS clock turn-off status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Clk2off {
    #[doc = "0: FS/HS clock is always on"]
    AlwaysOn = 0,
    #[doc = "1: FS/HS clock turn-off in L2 (FS/HS mode) is enabled - default afteer hardware reset"]
    TurnOff = 1,
}
impl From<Clk2off> for bool {
    #[inline(always)]
    fn from(variant: Clk2off) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `clk2off` reader - FS/HS clock turn-off status."]
pub type Clk2offR = crate::BitReader<Clk2off>;
impl Clk2offR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Clk2off {
        match self.bits {
            false => Clk2off::AlwaysOn,
            true => Clk2off::TurnOff,
        }
    }
    #[doc = "FS/HS clock is always on"]
    #[inline(always)]
    pub fn is_always_on(&self) -> bool {
        *self == Clk2off::AlwaysOn
    }
    #[doc = "FS/HS clock turn-off in L2 (FS/HS mode) is enabled - default afteer hardware reset"]
    #[inline(always)]
    pub fn is_turn_off(&self) -> bool {
        *self == Clk2off::TurnOff
    }
}
#[doc = "Field `clk2off` writer - FS/HS clock turn-off status."]
pub type Clk2offW<'a, REG> = crate::BitWriter<'a, REG, Clk2off>;
impl<'a, REG> Clk2offW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "FS/HS clock is always on"]
    #[inline(always)]
    pub fn always_on(self) -> &'a mut crate::W<REG> {
        self.variant(Clk2off::AlwaysOn)
    }
    #[doc = "FS/HS clock turn-off in L2 (FS/HS mode) is enabled - default afteer hardware reset"]
    #[inline(always)]
    pub fn turn_off(self) -> &'a mut crate::W<REG> {
        self.variant(Clk2off::TurnOff)
    }
}
#[doc = "PCLK clock turn-off status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Clk3off {
    #[doc = "0: PCLK clock is always on."]
    AlwaysOn = 0,
    #[doc = "1: PCLK clock turn-off in U3 (SS mode) is enabled - default afteer hardware reset."]
    TurnOff = 1,
}
impl From<Clk3off> for bool {
    #[inline(always)]
    fn from(variant: Clk3off) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `clk3off` reader - PCLK clock turn-off status."]
pub type Clk3offR = crate::BitReader<Clk3off>;
impl Clk3offR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Clk3off {
        match self.bits {
            false => Clk3off::AlwaysOn,
            true => Clk3off::TurnOff,
        }
    }
    #[doc = "PCLK clock is always on."]
    #[inline(always)]
    pub fn is_always_on(&self) -> bool {
        *self == Clk3off::AlwaysOn
    }
    #[doc = "PCLK clock turn-off in U3 (SS mode) is enabled - default afteer hardware reset."]
    #[inline(always)]
    pub fn is_turn_off(&self) -> bool {
        *self == Clk3off::TurnOff
    }
}
#[doc = "Field `clk3off` writer - PCLK clock turn-off status."]
pub type Clk3offW<'a, REG> = crate::BitWriter<'a, REG, Clk3off>;
impl<'a, REG> Clk3offW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PCLK clock is always on."]
    #[inline(always)]
    pub fn always_on(self) -> &'a mut crate::W<REG> {
        self.variant(Clk3off::AlwaysOn)
    }
    #[doc = "PCLK clock turn-off in U3 (SS mode) is enabled - default afteer hardware reset."]
    #[inline(always)]
    pub fn turn_off(self) -> &'a mut crate::W<REG> {
        self.variant(Clk3off::TurnOff)
    }
}
#[doc = "Controller in reset state.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InRst {
    #[doc = "0: Internal reset is active"]
    Active = 0,
    #[doc = "1: Internal reset is not active and controller is fully operational."]
    NotActive = 1,
}
impl From<InRst> for bool {
    #[inline(always)]
    fn from(variant: InRst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `in_rst` reader - Controller in reset state."]
pub type InRstR = crate::BitReader<InRst>;
impl InRstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> InRst {
        match self.bits {
            false => InRst::Active,
            true => InRst::NotActive,
        }
    }
    #[doc = "Internal reset is active"]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == InRst::Active
    }
    #[doc = "Internal reset is not active and controller is fully operational."]
    #[inline(always)]
    pub fn is_not_active(&self) -> bool {
        *self == InRst::NotActive
    }
}
#[doc = "Field `in_rst` writer - Controller in reset state."]
pub type InRstW<'a, REG> = crate::BitWriter<'a, REG, InRst>;
impl<'a, REG> InRstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal reset is active"]
    #[inline(always)]
    pub fn active(self) -> &'a mut crate::W<REG> {
        self.variant(InRst::Active)
    }
    #[doc = "Internal reset is not active and controller is fully operational."]
    #[inline(always)]
    pub fn not_active(self) -> &'a mut crate::W<REG> {
        self.variant(InRst::NotActive)
    }
}
#[doc = "Status of the `TDL calculation based on TRB` feature.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TdlTrbEn {
    #[doc = "0: TDL TRB calculation disabled."]
    Disabled = 0,
    #[doc = "1: TDL TRB calculation enabled."]
    Enabled = 1,
}
impl From<TdlTrbEn> for bool {
    #[inline(always)]
    fn from(variant: TdlTrbEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `tdl_trb_en` reader - Status of the `TDL calculation based on TRB` feature."]
pub type TdlTrbEnR = crate::BitReader<TdlTrbEn>;
impl TdlTrbEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TdlTrbEn {
        match self.bits {
            false => TdlTrbEn::Disabled,
            true => TdlTrbEn::Enabled,
        }
    }
    #[doc = "TDL TRB calculation disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TdlTrbEn::Disabled
    }
    #[doc = "TDL TRB calculation enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TdlTrbEn::Enabled
    }
}
#[doc = "Field `tdl_trb_en` writer - Status of the `TDL calculation based on TRB` feature."]
pub type TdlTrbEnW<'a, REG> = crate::BitWriter<'a, REG, TdlTrbEn>;
impl<'a, REG> TdlTrbEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TDL TRB calculation disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(TdlTrbEn::Disabled)
    }
    #[doc = "TDL TRB calculation enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(TdlTrbEn::Enabled)
    }
}
#[doc = "Device enable status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Devs {
    #[doc = "0: USB device is disabled - VBUS disconnected from internal logic."]
    Disabled = 0,
    #[doc = "1: USB device is enabled - VBUS connected to internal logic."]
    Enabled = 1,
}
impl From<Devs> for bool {
    #[inline(always)]
    fn from(variant: Devs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `devs` reader - Device enable status."]
pub type DevsR = crate::BitReader<Devs>;
impl DevsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Devs {
        match self.bits {
            false => Devs::Disabled,
            true => Devs::Enabled,
        }
    }
    #[doc = "USB device is disabled - VBUS disconnected from internal logic."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Devs::Disabled
    }
    #[doc = "USB device is enabled - VBUS connected to internal logic."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Devs::Enabled
    }
}
#[doc = "Field `devs` writer - Device enable status."]
pub type DevsW<'a, REG> = crate::BitWriter<'a, REG, Devs>;
impl<'a, REG> DevsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "USB device is disabled - VBUS disconnected from internal logic."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Devs::Disabled)
    }
    #[doc = "USB device is enabled - VBUS connected to internal logic."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Devs::Enabled)
    }
}
#[doc = "Address status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Addressed {
    #[doc = "0: USB device is in the default state."]
    Default = 0,
    #[doc = "1: USB device is at least in address state."]
    Addressed = 1,
}
impl From<Addressed> for bool {
    #[inline(always)]
    fn from(variant: Addressed) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `addressed` reader - Address status."]
pub type AddressedR = crate::BitReader<Addressed>;
impl AddressedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Addressed {
        match self.bits {
            false => Addressed::Default,
            true => Addressed::Addressed,
        }
    }
    #[doc = "USB device is in the default state."]
    #[inline(always)]
    pub fn is_default(&self) -> bool {
        *self == Addressed::Default
    }
    #[doc = "USB device is at least in address state."]
    #[inline(always)]
    pub fn is_addressed(&self) -> bool {
        *self == Addressed::Addressed
    }
}
#[doc = "Field `addressed` writer - Address status."]
pub type AddressedW<'a, REG> = crate::BitWriter<'a, REG, Addressed>;
impl<'a, REG> AddressedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "USB device is in the default state."]
    #[inline(always)]
    pub fn default(self) -> &'a mut crate::W<REG> {
        self.variant(Addressed::Default)
    }
    #[doc = "USB device is at least in address state."]
    #[inline(always)]
    pub fn addressed(self) -> &'a mut crate::W<REG> {
        self.variant(Addressed::Addressed)
    }
}
#[doc = "L1 LPM state enable status - used in FS/HS mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum L1ens {
    #[doc = "0: Entering into L1 LPM state disabled."]
    Disabled = 0,
    #[doc = "1: Entering into L1 LPM state enabled."]
    Enabled = 1,
}
impl From<L1ens> for bool {
    #[inline(always)]
    fn from(variant: L1ens) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `l1ens` reader - L1 LPM state enable status - used in FS/HS mode."]
pub type L1ensR = crate::BitReader<L1ens>;
impl L1ensR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> L1ens {
        match self.bits {
            false => L1ens::Disabled,
            true => L1ens::Enabled,
        }
    }
    #[doc = "Entering into L1 LPM state disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == L1ens::Disabled
    }
    #[doc = "Entering into L1 LPM state enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == L1ens::Enabled
    }
}
#[doc = "Field `l1ens` writer - L1 LPM state enable status - used in FS/HS mode."]
pub type L1ensW<'a, REG> = crate::BitWriter<'a, REG, L1ens>;
impl<'a, REG> L1ensW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Entering into L1 LPM state disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(L1ens::Disabled)
    }
    #[doc = "Entering into L1 LPM state enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(L1ens::Enabled)
    }
}
#[doc = "Internal VBUS connection status - used both in FS/HS and SS mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Vbuss {
    #[doc = "0: Internal VBUS is not detected."]
    NotDetected = 0,
    #[doc = "1: Internal VBUS is detected."]
    Detected = 1,
}
impl From<Vbuss> for bool {
    #[inline(always)]
    fn from(variant: Vbuss) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `vbuss` reader - Internal VBUS connection status - used both in FS/HS and SS mode."]
pub type VbussR = crate::BitReader<Vbuss>;
impl VbussR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Vbuss {
        match self.bits {
            false => Vbuss::NotDetected,
            true => Vbuss::Detected,
        }
    }
    #[doc = "Internal VBUS is not detected."]
    #[inline(always)]
    pub fn is_not_detected(&self) -> bool {
        *self == Vbuss::NotDetected
    }
    #[doc = "Internal VBUS is detected."]
    #[inline(always)]
    pub fn is_detected(&self) -> bool {
        *self == Vbuss::Detected
    }
}
#[doc = "Field `vbuss` writer - Internal VBUS connection status - used both in FS/HS and SS mode."]
pub type VbussW<'a, REG> = crate::BitWriter<'a, REG, Vbuss>;
impl<'a, REG> VbussW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal VBUS is not detected."]
    #[inline(always)]
    pub fn not_detected(self) -> &'a mut crate::W<REG> {
        self.variant(Vbuss::NotDetected)
    }
    #[doc = "Internal VBUS is detected."]
    #[inline(always)]
    pub fn detected(self) -> &'a mut crate::W<REG> {
        self.variant(Vbuss::Detected)
    }
}
#[doc = "FS/HS LPM state - used in FS/HS mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Lpmst {
    #[doc = "0: L0 state."]
    L0 = 0,
    #[doc = "1: L1 state."]
    L1 = 1,
    #[doc = "2: L2 state."]
    L2 = 2,
    #[doc = "3: L3 state."]
    L3 = 3,
}
impl From<Lpmst> for u8 {
    #[inline(always)]
    fn from(variant: Lpmst) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Lpmst {
    type Ux = u8;
}
#[doc = "Field `lpmst` reader - FS/HS LPM state - used in FS/HS mode."]
pub type LpmstR = crate::FieldReader<Lpmst>;
impl LpmstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lpmst {
        match self.bits {
            0 => Lpmst::L0,
            1 => Lpmst::L1,
            2 => Lpmst::L2,
            3 => Lpmst::L3,
            _ => unreachable!(),
        }
    }
    #[doc = "L0 state."]
    #[inline(always)]
    pub fn is_l0(&self) -> bool {
        *self == Lpmst::L0
    }
    #[doc = "L1 state."]
    #[inline(always)]
    pub fn is_l1(&self) -> bool {
        *self == Lpmst::L1
    }
    #[doc = "L2 state."]
    #[inline(always)]
    pub fn is_l2(&self) -> bool {
        *self == Lpmst::L2
    }
    #[doc = "L3 state."]
    #[inline(always)]
    pub fn is_l3(&self) -> bool {
        *self == Lpmst::L3
    }
}
#[doc = "Field `lpmst` writer - FS/HS LPM state - used in FS/HS mode."]
pub type LpmstW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Lpmst>;
impl<'a, REG> LpmstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "L0 state."]
    #[inline(always)]
    pub fn l0(self) -> &'a mut crate::W<REG> {
        self.variant(Lpmst::L0)
    }
    #[doc = "L1 state."]
    #[inline(always)]
    pub fn l1(self) -> &'a mut crate::W<REG> {
        self.variant(Lpmst::L1)
    }
    #[doc = "L2 state."]
    #[inline(always)]
    pub fn l2(self) -> &'a mut crate::W<REG> {
        self.variant(Lpmst::L2)
    }
    #[doc = "L3 state."]
    #[inline(always)]
    pub fn l3(self) -> &'a mut crate::W<REG> {
        self.variant(Lpmst::L3)
    }
}
#[doc = "Disable HS status - used in FS/HS mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Usb2cons {
    #[doc = "0: The disconnect bit for FS/HS mode is set."]
    Disconnect = 0,
    #[doc = "1: The disconnect bit for FS/HS mode is not set."]
    Connect = 1,
}
impl From<Usb2cons> for bool {
    #[inline(always)]
    fn from(variant: Usb2cons) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `usb2cons` reader - Disable HS status - used in FS/HS mode."]
pub type Usb2consR = crate::BitReader<Usb2cons>;
impl Usb2consR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Usb2cons {
        match self.bits {
            false => Usb2cons::Disconnect,
            true => Usb2cons::Connect,
        }
    }
    #[doc = "The disconnect bit for FS/HS mode is set."]
    #[inline(always)]
    pub fn is_disconnect(&self) -> bool {
        *self == Usb2cons::Disconnect
    }
    #[doc = "The disconnect bit for FS/HS mode is not set."]
    #[inline(always)]
    pub fn is_connect(&self) -> bool {
        *self == Usb2cons::Connect
    }
}
#[doc = "Field `usb2cons` writer - Disable HS status - used in FS/HS mode."]
pub type Usb2consW<'a, REG> = crate::BitWriter<'a, REG, Usb2cons>;
impl<'a, REG> Usb2consW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The disconnect bit for FS/HS mode is set."]
    #[inline(always)]
    pub fn disconnect(self) -> &'a mut crate::W<REG> {
        self.variant(Usb2cons::Disconnect)
    }
    #[doc = "The disconnect bit for FS/HS mode is not set."]
    #[inline(always)]
    pub fn connect(self) -> &'a mut crate::W<REG> {
        self.variant(Usb2cons::Connect)
    }
}
#[doc = "FS/HS mode connection status - used in FS/HS mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DisableHs {
    #[doc = "0: High Speed operations in USB2.0 mode are not disabled."]
    Enabled = 0,
    #[doc = "1: High Speed operations in USB2.0 mode are disabled."]
    Disabled = 1,
}
impl From<DisableHs> for bool {
    #[inline(always)]
    fn from(variant: DisableHs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `disable_hs` reader - FS/HS mode connection status - used in FS/HS mode."]
pub type DisableHsR = crate::BitReader<DisableHs>;
impl DisableHsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DisableHs {
        match self.bits {
            false => DisableHs::Enabled,
            true => DisableHs::Disabled,
        }
    }
    #[doc = "High Speed operations in USB2.0 mode are not disabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DisableHs::Enabled
    }
    #[doc = "High Speed operations in USB2.0 mode are disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DisableHs::Disabled
    }
}
#[doc = "Field `disable_hs` writer - FS/HS mode connection status - used in FS/HS mode."]
pub type DisableHsW<'a, REG> = crate::BitWriter<'a, REG, DisableHs>;
impl<'a, REG> DisableHsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "High Speed operations in USB2.0 mode are not disabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DisableHs::Enabled)
    }
    #[doc = "High Speed operations in USB2.0 mode are disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DisableHs::Disabled)
    }
}
#[doc = "U1/2 state enable status - used in SS mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnsU {
    #[doc = "0: Entering to U1/2 state disabled."]
    Disabled = 0,
    #[doc = "1: Entering to U1/2 state enabled."]
    Enabled = 1,
}
impl From<EnsU> for bool {
    #[inline(always)]
    fn from(variant: EnsU) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ens_u(1-2)` reader - U1/2 state enable status - used in SS mode."]
pub type EnsUR = crate::BitReader<EnsU>;
impl EnsUR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnsU {
        match self.bits {
            false => EnsU::Disabled,
            true => EnsU::Enabled,
        }
    }
    #[doc = "Entering to U1/2 state disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EnsU::Disabled
    }
    #[doc = "Entering to U1/2 state enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EnsU::Enabled
    }
}
#[doc = "Field `ens_u(1-2)` writer - U1/2 state enable status - used in SS mode."]
pub type EnsUW<'a, REG> = crate::BitWriter<'a, REG, EnsU>;
impl<'a, REG> EnsUW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Entering to U1/2 state disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EnsU::Disabled)
    }
    #[doc = "Entering to U1/2 state enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(EnsU::Enabled)
    }
}
#[doc = "SuperSpeed Link LTSSM state.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Lst {
    #[doc = "0: U0 link status."]
    U0 = 0,
    #[doc = "1: U1 link status."]
    U1 = 1,
    #[doc = "2: U2 link status."]
    U2 = 2,
    #[doc = "3: U3 link status."]
    U3 = 3,
    #[doc = "4: Link disabled."]
    Disabled = 4,
    #[doc = "5: Link detected receive."]
    Rxdetect = 5,
    #[doc = "6: Link inactive."]
    Inactive = 6,
    #[doc = "7: Link polling."]
    Polling = 7,
    #[doc = "8: Link is in recovery."]
    Recovery = 8,
    #[doc = "9: Link is hot reset."]
    HotReset = 9,
    #[doc = "10: Link is in COMP mode."]
    CompMode = 10,
    #[doc = "11: Link is in LB state."]
    LbState = 11,
}
impl From<Lst> for u8 {
    #[inline(always)]
    fn from(variant: Lst) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Lst {
    type Ux = u8;
}
#[doc = "Field `lst` reader - SuperSpeed Link LTSSM state."]
pub type LstR = crate::FieldReader<Lst>;
impl LstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Lst> {
        match self.bits {
            0 => Some(Lst::U0),
            1 => Some(Lst::U1),
            2 => Some(Lst::U2),
            3 => Some(Lst::U3),
            4 => Some(Lst::Disabled),
            5 => Some(Lst::Rxdetect),
            6 => Some(Lst::Inactive),
            7 => Some(Lst::Polling),
            8 => Some(Lst::Recovery),
            9 => Some(Lst::HotReset),
            10 => Some(Lst::CompMode),
            11 => Some(Lst::LbState),
            _ => None,
        }
    }
    #[doc = "U0 link status."]
    #[inline(always)]
    pub fn is_u0(&self) -> bool {
        *self == Lst::U0
    }
    #[doc = "U1 link status."]
    #[inline(always)]
    pub fn is_u1(&self) -> bool {
        *self == Lst::U1
    }
    #[doc = "U2 link status."]
    #[inline(always)]
    pub fn is_u2(&self) -> bool {
        *self == Lst::U2
    }
    #[doc = "U3 link status."]
    #[inline(always)]
    pub fn is_u3(&self) -> bool {
        *self == Lst::U3
    }
    #[doc = "Link disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Lst::Disabled
    }
    #[doc = "Link detected receive."]
    #[inline(always)]
    pub fn is_rxdetect(&self) -> bool {
        *self == Lst::Rxdetect
    }
    #[doc = "Link inactive."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == Lst::Inactive
    }
    #[doc = "Link polling."]
    #[inline(always)]
    pub fn is_polling(&self) -> bool {
        *self == Lst::Polling
    }
    #[doc = "Link is in recovery."]
    #[inline(always)]
    pub fn is_recovery(&self) -> bool {
        *self == Lst::Recovery
    }
    #[doc = "Link is hot reset."]
    #[inline(always)]
    pub fn is_hot_reset(&self) -> bool {
        *self == Lst::HotReset
    }
    #[doc = "Link is in COMP mode."]
    #[inline(always)]
    pub fn is_comp_mode(&self) -> bool {
        *self == Lst::CompMode
    }
    #[doc = "Link is in LB state."]
    #[inline(always)]
    pub fn is_lb_state(&self) -> bool {
        *self == Lst::LbState
    }
}
#[doc = "Field `lst` writer - SuperSpeed Link LTSSM state."]
pub type LstW<'a, REG> = crate::FieldWriter<'a, REG, 4, Lst>;
impl<'a, REG> LstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "U0 link status."]
    #[inline(always)]
    pub fn u0(self) -> &'a mut crate::W<REG> {
        self.variant(Lst::U0)
    }
    #[doc = "U1 link status."]
    #[inline(always)]
    pub fn u1(self) -> &'a mut crate::W<REG> {
        self.variant(Lst::U1)
    }
    #[doc = "U2 link status."]
    #[inline(always)]
    pub fn u2(self) -> &'a mut crate::W<REG> {
        self.variant(Lst::U2)
    }
    #[doc = "U3 link status."]
    #[inline(always)]
    pub fn u3(self) -> &'a mut crate::W<REG> {
        self.variant(Lst::U3)
    }
    #[doc = "Link disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Lst::Disabled)
    }
    #[doc = "Link detected receive."]
    #[inline(always)]
    pub fn rxdetect(self) -> &'a mut crate::W<REG> {
        self.variant(Lst::Rxdetect)
    }
    #[doc = "Link inactive."]
    #[inline(always)]
    pub fn inactive(self) -> &'a mut crate::W<REG> {
        self.variant(Lst::Inactive)
    }
    #[doc = "Link polling."]
    #[inline(always)]
    pub fn polling(self) -> &'a mut crate::W<REG> {
        self.variant(Lst::Polling)
    }
    #[doc = "Link is in recovery."]
    #[inline(always)]
    pub fn recovery(self) -> &'a mut crate::W<REG> {
        self.variant(Lst::Recovery)
    }
    #[doc = "Link is hot reset."]
    #[inline(always)]
    pub fn hot_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Lst::HotReset)
    }
    #[doc = "Link is in COMP mode."]
    #[inline(always)]
    pub fn comp_mode(self) -> &'a mut crate::W<REG> {
        self.variant(Lst::CompMode)
    }
    #[doc = "Link is in LB state."]
    #[inline(always)]
    pub fn lb_state(self) -> &'a mut crate::W<REG> {
        self.variant(Lst::LbState)
    }
}
#[doc = "DMA clock turn-off status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dmaoff {
    #[doc = "0: DMA clock is always on - default after hardware reset."]
    AlwaysOn = 0,
    #[doc = "1: DMA clock turn-off in U1, U2, and U3 (SS mode) is enabled."]
    TurnOff = 1,
}
impl From<Dmaoff> for bool {
    #[inline(always)]
    fn from(variant: Dmaoff) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `dmaoff` reader - DMA clock turn-off status."]
pub type DmaoffR = crate::BitReader<Dmaoff>;
impl DmaoffR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dmaoff {
        match self.bits {
            false => Dmaoff::AlwaysOn,
            true => Dmaoff::TurnOff,
        }
    }
    #[doc = "DMA clock is always on - default after hardware reset."]
    #[inline(always)]
    pub fn is_always_on(&self) -> bool {
        *self == Dmaoff::AlwaysOn
    }
    #[doc = "DMA clock turn-off in U1, U2, and U3 (SS mode) is enabled."]
    #[inline(always)]
    pub fn is_turn_off(&self) -> bool {
        *self == Dmaoff::TurnOff
    }
}
#[doc = "Field `dmaoff` writer - DMA clock turn-off status."]
pub type DmaoffW<'a, REG> = crate::BitWriter<'a, REG, Dmaoff>;
impl<'a, REG> DmaoffW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA clock is always on - default after hardware reset."]
    #[inline(always)]
    pub fn always_on(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaoff::AlwaysOn)
    }
    #[doc = "DMA clock turn-off in U1, U2, and U3 (SS mode) is enabled."]
    #[inline(always)]
    pub fn turn_off(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaoff::TurnOff)
    }
}
#[doc = "SFR Endian status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Endian2 {
    #[doc = "0: Little Endian order - default after hardware reset."]
    Little = 0,
    #[doc = "1: Big Endian order."]
    Big = 1,
}
impl From<Endian2> for bool {
    #[inline(always)]
    fn from(variant: Endian2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `endian2` reader - SFR Endian status."]
pub type Endian2R = crate::BitReader<Endian2>;
impl Endian2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Endian2 {
        match self.bits {
            false => Endian2::Little,
            true => Endian2::Big,
        }
    }
    #[doc = "Little Endian order - default after hardware reset."]
    #[inline(always)]
    pub fn is_little(&self) -> bool {
        *self == Endian2::Little
    }
    #[doc = "Big Endian order."]
    #[inline(always)]
    pub fn is_big(&self) -> bool {
        *self == Endian2::Big
    }
}
#[doc = "Field `endian2` writer - SFR Endian status."]
pub type Endian2W<'a, REG> = crate::BitWriter<'a, REG, Endian2>;
impl<'a, REG> Endian2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Little Endian order - default after hardware reset."]
    #[inline(always)]
    pub fn little(self) -> &'a mut crate::W<REG> {
        self.variant(Endian2::Little)
    }
    #[doc = "Big Endian order."]
    #[inline(always)]
    pub fn big(self) -> &'a mut crate::W<REG> {
        self.variant(Endian2::Big)
    }
}
impl R {
    #[doc = "Bit 0 - Device configuration status."]
    #[inline(always)]
    pub fn cfgsts(&self) -> CfgstsR {
        CfgstsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - On-chip memory overflow."]
    #[inline(always)]
    pub fn ov(&self) -> OvR {
        OvR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Superspeed connection status."]
    #[inline(always)]
    pub fn usb3cons(&self) -> Usb3consR {
        Usb3consR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMA transfer configuration status."]
    #[inline(always)]
    pub fn dtrans(&self) -> DtransR {
        DtransR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Device speed."]
    #[inline(always)]
    pub fn usbspeed(&self) -> UsbspeedR {
        UsbspeedR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Endianess for SFR access."]
    #[inline(always)]
    pub fn endian(&self) -> EndianR {
        EndianR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - FS/HS clock turn-off status."]
    #[inline(always)]
    pub fn clk2off(&self) -> Clk2offR {
        Clk2offR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PCLK clock turn-off status."]
    #[inline(always)]
    pub fn clk3off(&self) -> Clk3offR {
        Clk3offR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Controller in reset state."]
    #[inline(always)]
    pub fn in_rst(&self) -> InRstR {
        InRstR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Status of the `TDL calculation based on TRB` feature."]
    #[inline(always)]
    pub fn tdl_trb_en(&self) -> TdlTrbEnR {
        TdlTrbEnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 14 - Device enable status."]
    #[inline(always)]
    pub fn devs(&self) -> DevsR {
        DevsR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Address status."]
    #[inline(always)]
    pub fn addressed(&self) -> AddressedR {
        AddressedR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - L1 LPM state enable status - used in FS/HS mode."]
    #[inline(always)]
    pub fn l1ens(&self) -> L1ensR {
        L1ensR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Internal VBUS connection status - used both in FS/HS and SS mode."]
    #[inline(always)]
    pub fn vbuss(&self) -> VbussR {
        VbussR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - FS/HS LPM state - used in FS/HS mode."]
    #[inline(always)]
    pub fn lpmst(&self) -> LpmstR {
        LpmstR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20 - Disable HS status - used in FS/HS mode."]
    #[inline(always)]
    pub fn usb2cons(&self) -> Usb2consR {
        Usb2consR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - FS/HS mode connection status - used in FS/HS mode."]
    #[inline(always)]
    pub fn disable_hs(&self) -> DisableHsR {
        DisableHsR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "U1/2 state enable status - used in SS mode."]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `ens_u1` field"]
    #[inline(always)]
    pub fn ens_u(&self, n: u8) -> EnsUR {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        EnsUR::new(((self.bits >> (n + 24)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "U1/2 state enable status - used in SS mode."]
    #[inline(always)]
    pub fn ens_u_iter(&self) -> impl Iterator<Item = EnsUR> + '_ {
        (0..2).map(move |n| EnsUR::new(((self.bits >> (n + 24)) & 1) != 0))
    }
    #[doc = "Bit 24 - U1/2 state enable status - used in SS mode."]
    #[inline(always)]
    pub fn ens_u1(&self) -> EnsUR {
        EnsUR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - U1/2 state enable status - used in SS mode."]
    #[inline(always)]
    pub fn ens_u2(&self) -> EnsUR {
        EnsUR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:29 - SuperSpeed Link LTSSM state."]
    #[inline(always)]
    pub fn lst(&self) -> LstR {
        LstR::new(((self.bits >> 26) & 0x0f) as u8)
    }
    #[doc = "Bit 30 - DMA clock turn-off status."]
    #[inline(always)]
    pub fn dmaoff(&self) -> DmaoffR {
        DmaoffR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - SFR Endian status."]
    #[inline(always)]
    pub fn endian2(&self) -> Endian2R {
        Endian2R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Device configuration status."]
    #[inline(always)]
    #[must_use]
    pub fn cfgsts(&mut self) -> CfgstsW<UsbStsSpec> {
        CfgstsW::new(self, 0)
    }
    #[doc = "Bit 1 - On-chip memory overflow."]
    #[inline(always)]
    #[must_use]
    pub fn ov(&mut self) -> OvW<UsbStsSpec> {
        OvW::new(self, 1)
    }
    #[doc = "Bit 2 - Superspeed connection status."]
    #[inline(always)]
    #[must_use]
    pub fn usb3cons(&mut self) -> Usb3consW<UsbStsSpec> {
        Usb3consW::new(self, 2)
    }
    #[doc = "Bit 3 - DMA transfer configuration status."]
    #[inline(always)]
    #[must_use]
    pub fn dtrans(&mut self) -> DtransW<UsbStsSpec> {
        DtransW::new(self, 3)
    }
    #[doc = "Bits 4:6 - Device speed."]
    #[inline(always)]
    #[must_use]
    pub fn usbspeed(&mut self) -> UsbspeedW<UsbStsSpec> {
        UsbspeedW::new(self, 4)
    }
    #[doc = "Bit 7 - Endianess for SFR access."]
    #[inline(always)]
    #[must_use]
    pub fn endian(&mut self) -> EndianW<UsbStsSpec> {
        EndianW::new(self, 7)
    }
    #[doc = "Bit 8 - FS/HS clock turn-off status."]
    #[inline(always)]
    #[must_use]
    pub fn clk2off(&mut self) -> Clk2offW<UsbStsSpec> {
        Clk2offW::new(self, 8)
    }
    #[doc = "Bit 9 - PCLK clock turn-off status."]
    #[inline(always)]
    #[must_use]
    pub fn clk3off(&mut self) -> Clk3offW<UsbStsSpec> {
        Clk3offW::new(self, 9)
    }
    #[doc = "Bit 10 - Controller in reset state."]
    #[inline(always)]
    #[must_use]
    pub fn in_rst(&mut self) -> InRstW<UsbStsSpec> {
        InRstW::new(self, 10)
    }
    #[doc = "Bit 11 - Status of the `TDL calculation based on TRB` feature."]
    #[inline(always)]
    #[must_use]
    pub fn tdl_trb_en(&mut self) -> TdlTrbEnW<UsbStsSpec> {
        TdlTrbEnW::new(self, 11)
    }
    #[doc = "Bit 14 - Device enable status."]
    #[inline(always)]
    #[must_use]
    pub fn devs(&mut self) -> DevsW<UsbStsSpec> {
        DevsW::new(self, 14)
    }
    #[doc = "Bit 15 - Address status."]
    #[inline(always)]
    #[must_use]
    pub fn addressed(&mut self) -> AddressedW<UsbStsSpec> {
        AddressedW::new(self, 15)
    }
    #[doc = "Bit 16 - L1 LPM state enable status - used in FS/HS mode."]
    #[inline(always)]
    #[must_use]
    pub fn l1ens(&mut self) -> L1ensW<UsbStsSpec> {
        L1ensW::new(self, 16)
    }
    #[doc = "Bit 17 - Internal VBUS connection status - used both in FS/HS and SS mode."]
    #[inline(always)]
    #[must_use]
    pub fn vbuss(&mut self) -> VbussW<UsbStsSpec> {
        VbussW::new(self, 17)
    }
    #[doc = "Bits 18:19 - FS/HS LPM state - used in FS/HS mode."]
    #[inline(always)]
    #[must_use]
    pub fn lpmst(&mut self) -> LpmstW<UsbStsSpec> {
        LpmstW::new(self, 18)
    }
    #[doc = "Bit 20 - Disable HS status - used in FS/HS mode."]
    #[inline(always)]
    #[must_use]
    pub fn usb2cons(&mut self) -> Usb2consW<UsbStsSpec> {
        Usb2consW::new(self, 20)
    }
    #[doc = "Bit 21 - FS/HS mode connection status - used in FS/HS mode."]
    #[inline(always)]
    #[must_use]
    pub fn disable_hs(&mut self) -> DisableHsW<UsbStsSpec> {
        DisableHsW::new(self, 21)
    }
    #[doc = "U1/2 state enable status - used in SS mode."]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `ens_u1` field"]
    #[inline(always)]
    #[must_use]
    pub fn ens_u(&mut self, n: u8) -> EnsUW<UsbStsSpec> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        EnsUW::new(self, n + 24)
    }
    #[doc = "Bit 24 - U1/2 state enable status - used in SS mode."]
    #[inline(always)]
    #[must_use]
    pub fn ens_u1(&mut self) -> EnsUW<UsbStsSpec> {
        EnsUW::new(self, 24)
    }
    #[doc = "Bit 25 - U1/2 state enable status - used in SS mode."]
    #[inline(always)]
    #[must_use]
    pub fn ens_u2(&mut self) -> EnsUW<UsbStsSpec> {
        EnsUW::new(self, 25)
    }
    #[doc = "Bits 26:29 - SuperSpeed Link LTSSM state."]
    #[inline(always)]
    #[must_use]
    pub fn lst(&mut self) -> LstW<UsbStsSpec> {
        LstW::new(self, 26)
    }
    #[doc = "Bit 30 - DMA clock turn-off status."]
    #[inline(always)]
    #[must_use]
    pub fn dmaoff(&mut self) -> DmaoffW<UsbStsSpec> {
        DmaoffW::new(self, 30)
    }
    #[doc = "Bit 31 - SFR Endian status."]
    #[inline(always)]
    #[must_use]
    pub fn endian2(&mut self) -> Endian2W<UsbStsSpec> {
        Endian2W::new(self, 31)
    }
}
#[doc = "USB3 Global status.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb_sts::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb_sts::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UsbStsSpec;
impl crate::RegisterSpec for UsbStsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usb_sts::R`](R) reader structure"]
impl crate::Readable for UsbStsSpec {}
#[doc = "`write(|w| ..)` method takes [`usb_sts::W`](W) writer structure"]
impl crate::Writable for UsbStsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets usb_sts to value 0"]
impl crate::Resettable for UsbStsSpec {
    const RESET_VALUE: u32 = 0;
}
