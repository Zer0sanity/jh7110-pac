#[doc = "Register `usb_conf` reader"]
pub type R = crate::R<UsbConfSpec>;
#[doc = "Register `usb_conf` writer"]
pub type W = crate::W<UsbConfSpec>;
#[doc = "Reset/Set USB device configuration.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cfg {
    #[doc = "0: Do not set USB device configuration."]
    None = 0,
    #[doc = "1: Reset USB device configuration."]
    Rst = 1,
    #[doc = "2: Set configuration."]
    Set = 2,
}
impl From<Cfg> for u8 {
    #[inline(always)]
    fn from(variant: Cfg) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cfg {
    type Ux = u8;
}
#[doc = "Field `cfg` reader - Reset/Set USB device configuration."]
pub type CfgR = crate::FieldReader<Cfg>;
impl CfgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cfg> {
        match self.bits {
            0 => Some(Cfg::None),
            1 => Some(Cfg::Rst),
            2 => Some(Cfg::Set),
            _ => None,
        }
    }
    #[doc = "Do not set USB device configuration."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Cfg::None
    }
    #[doc = "Reset USB device configuration."]
    #[inline(always)]
    pub fn is_rst(&self) -> bool {
        *self == Cfg::Rst
    }
    #[doc = "Set configuration."]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Cfg::Set
    }
}
#[doc = "Field `cfg` writer - Reset/Set USB device configuration."]
pub type CfgW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cfg>;
impl<'a, REG> CfgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Do not set USB device configuration."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Cfg::None)
    }
    #[doc = "Reset USB device configuration."]
    #[inline(always)]
    pub fn rst(self) -> &'a mut crate::W<REG> {
        self.variant(Cfg::Rst)
    }
    #[doc = "Set configuration."]
    #[inline(always)]
    pub fn set(self) -> &'a mut crate::W<REG> {
        self.variant(Cfg::Set)
    }
}
#[doc = "Disconnect USB device.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dis {
    #[doc = "0: Do not disconnect USB device."]
    None = 0,
    #[doc = "1: Disconnect USB3 device in SuperSpeed mode."]
    Usb3 = 1,
    #[doc = "2: Disconnect USB2 device in FS/HS mode."]
    Usb2 = 2,
}
impl From<Dis> for u8 {
    #[inline(always)]
    fn from(variant: Dis) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dis {
    type Ux = u8;
}
#[doc = "Field `dis` reader - Disconnect USB device."]
pub type DisR = crate::FieldReader<Dis>;
impl DisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Dis> {
        match self.bits {
            0 => Some(Dis::None),
            1 => Some(Dis::Usb3),
            2 => Some(Dis::Usb2),
            _ => None,
        }
    }
    #[doc = "Do not disconnect USB device."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Dis::None
    }
    #[doc = "Disconnect USB3 device in SuperSpeed mode."]
    #[inline(always)]
    pub fn is_usb3(&self) -> bool {
        *self == Dis::Usb3
    }
    #[doc = "Disconnect USB2 device in FS/HS mode."]
    #[inline(always)]
    pub fn is_usb2(&self) -> bool {
        *self == Dis::Usb2
    }
}
#[doc = "Field `dis` writer - Disconnect USB device."]
pub type DisW<'a, REG> = crate::FieldWriter<'a, REG, 2, Dis>;
impl<'a, REG> DisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Do not disconnect USB device."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Dis::None)
    }
    #[doc = "Disconnect USB3 device in SuperSpeed mode."]
    #[inline(always)]
    pub fn usb3(self) -> &'a mut crate::W<REG> {
        self.variant(Dis::Usb3)
    }
    #[doc = "Disconnect USB2 device in FS/HS mode."]
    #[inline(always)]
    pub fn usb2(self) -> &'a mut crate::W<REG> {
        self.variant(Dis::Usb2)
    }
}
#[doc = "Endian access.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Endian {
    #[doc = "0: No endian access setting"]
    None = 0,
    #[doc = "1: Little endian access - default"]
    Little = 1,
    #[doc = "2: Big endian access"]
    Big = 2,
}
impl From<Endian> for u8 {
    #[inline(always)]
    fn from(variant: Endian) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Endian {
    type Ux = u8;
}
#[doc = "Field `endian` reader - Endian access."]
pub type EndianR = crate::FieldReader<Endian>;
impl EndianR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Endian> {
        match self.bits {
            0 => Some(Endian::None),
            1 => Some(Endian::Little),
            2 => Some(Endian::Big),
            _ => None,
        }
    }
    #[doc = "No endian access setting"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Endian::None
    }
    #[doc = "Little endian access - default"]
    #[inline(always)]
    pub fn is_little(&self) -> bool {
        *self == Endian::Little
    }
    #[doc = "Big endian access"]
    #[inline(always)]
    pub fn is_big(&self) -> bool {
        *self == Endian::Big
    }
}
#[doc = "Field `endian` writer - Endian access."]
pub type EndianW<'a, REG> = crate::FieldWriter<'a, REG, 2, Endian>;
impl<'a, REG> EndianW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No endian access setting"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Endian::None)
    }
    #[doc = "Little endian access - default"]
    #[inline(always)]
    pub fn little(self) -> &'a mut crate::W<REG> {
        self.variant(Endian::Little)
    }
    #[doc = "Big endian access"]
    #[inline(always)]
    pub fn big(self) -> &'a mut crate::W<REG> {
        self.variant(Endian::Big)
    }
}
#[doc = "Field `swrst` reader - Device software reset."]
pub type SwrstR = crate::BitReader;
#[doc = "Field `swrst` writer - Device software reset."]
pub type SwrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "DMA clock turn-off.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dmaoff {
    #[doc = "0: No DMA clock turn-off setting."]
    None = 0,
    #[doc = "1: DMA clock turn-off enable."]
    En = 1,
    #[doc = "2: DMA clock turn-off disable."]
    Ds = 2,
}
impl From<Dmaoff> for u8 {
    #[inline(always)]
    fn from(variant: Dmaoff) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dmaoff {
    type Ux = u8;
}
#[doc = "Field `dmaoff` reader - DMA clock turn-off."]
pub type DmaoffR = crate::FieldReader<Dmaoff>;
impl DmaoffR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Dmaoff> {
        match self.bits {
            0 => Some(Dmaoff::None),
            1 => Some(Dmaoff::En),
            2 => Some(Dmaoff::Ds),
            _ => None,
        }
    }
    #[doc = "No DMA clock turn-off setting."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Dmaoff::None
    }
    #[doc = "DMA clock turn-off enable."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Dmaoff::En
    }
    #[doc = "DMA clock turn-off disable."]
    #[inline(always)]
    pub fn is_ds(&self) -> bool {
        *self == Dmaoff::Ds
    }
}
#[doc = "Field `dmaoff` writer - DMA clock turn-off."]
pub type DmaoffW<'a, REG> = crate::FieldWriter<'a, REG, 2, Dmaoff>;
impl<'a, REG> DmaoffW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No DMA clock turn-off setting."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaoff::None)
    }
    #[doc = "DMA clock turn-off enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaoff::En)
    }
    #[doc = "DMA clock turn-off disable."]
    #[inline(always)]
    pub fn ds(self) -> &'a mut crate::W<REG> {
        self.variant(Dmaoff::Ds)
    }
}
#[doc = "Force Full Speed.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ForceFs {
    #[doc = "0: No Force Full Speed setting."]
    None = 0,
    #[doc = "1: Clear Force Full Speed."]
    Clear = 1,
    #[doc = "2: Set Force Full Speed."]
    Set = 2,
}
impl From<ForceFs> for u8 {
    #[inline(always)]
    fn from(variant: ForceFs) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ForceFs {
    type Ux = u8;
}
#[doc = "Field `force_fs` reader - Force Full Speed."]
pub type ForceFsR = crate::FieldReader<ForceFs>;
impl ForceFsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ForceFs> {
        match self.bits {
            0 => Some(ForceFs::None),
            1 => Some(ForceFs::Clear),
            2 => Some(ForceFs::Set),
            _ => None,
        }
    }
    #[doc = "No Force Full Speed setting."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == ForceFs::None
    }
    #[doc = "Clear Force Full Speed."]
    #[inline(always)]
    pub fn is_clear(&self) -> bool {
        *self == ForceFs::Clear
    }
    #[doc = "Set Force Full Speed."]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == ForceFs::Set
    }
}
#[doc = "Field `force_fs` writer - Force Full Speed."]
pub type ForceFsW<'a, REG> = crate::FieldWriter<'a, REG, 2, ForceFs>;
impl<'a, REG> ForceFsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No Force Full Speed setting."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(ForceFs::None)
    }
    #[doc = "Clear Force Full Speed."]
    #[inline(always)]
    pub fn clear(self) -> &'a mut crate::W<REG> {
        self.variant(ForceFs::Clear)
    }
    #[doc = "Set Force Full Speed."]
    #[inline(always)]
    pub fn set(self) -> &'a mut crate::W<REG> {
        self.variant(ForceFs::Set)
    }
}
#[doc = "Device enable/disable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dev {
    #[doc = "0: No Device enable/disable setting."]
    None = 0,
    #[doc = "1: Device enable."]
    En = 1,
    #[doc = "2: Device disable."]
    Ds = 2,
}
impl From<Dev> for u8 {
    #[inline(always)]
    fn from(variant: Dev) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dev {
    type Ux = u8;
}
#[doc = "Field `dev` reader - Device enable/disable."]
pub type DevR = crate::FieldReader<Dev>;
impl DevR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Dev> {
        match self.bits {
            0 => Some(Dev::None),
            1 => Some(Dev::En),
            2 => Some(Dev::Ds),
            _ => None,
        }
    }
    #[doc = "No Device enable/disable setting."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Dev::None
    }
    #[doc = "Device enable."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Dev::En
    }
    #[doc = "Device disable."]
    #[inline(always)]
    pub fn is_ds(&self) -> bool {
        *self == Dev::Ds
    }
}
#[doc = "Field `dev` writer - Device enable/disable."]
pub type DevW<'a, REG> = crate::FieldWriter<'a, REG, 2, Dev>;
impl<'a, REG> DevW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No Device enable/disable setting."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Dev::None)
    }
    #[doc = "Device enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Dev::En)
    }
    #[doc = "Device disable."]
    #[inline(always)]
    pub fn ds(self) -> &'a mut crate::W<REG> {
        self.variant(Dev::Ds)
    }
}
#[doc = "L1 LPM state entry enable/disable (used in HS/FS mode).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum L1 {
    #[doc = "0: No L1 LPM state entry setting."]
    None = 0,
    #[doc = "1: L1 LPM state entry enable."]
    En = 1,
    #[doc = "2: L1 LPM state entry disable."]
    Ds = 2,
}
impl From<L1> for u8 {
    #[inline(always)]
    fn from(variant: L1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for L1 {
    type Ux = u8;
}
#[doc = "Field `l1` reader - L1 LPM state entry enable/disable (used in HS/FS mode)."]
pub type L1R = crate::FieldReader<L1>;
impl L1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<L1> {
        match self.bits {
            0 => Some(L1::None),
            1 => Some(L1::En),
            2 => Some(L1::Ds),
            _ => None,
        }
    }
    #[doc = "No L1 LPM state entry setting."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == L1::None
    }
    #[doc = "L1 LPM state entry enable."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == L1::En
    }
    #[doc = "L1 LPM state entry disable."]
    #[inline(always)]
    pub fn is_ds(&self) -> bool {
        *self == L1::Ds
    }
}
#[doc = "Field `l1` writer - L1 LPM state entry enable/disable (used in HS/FS mode)."]
pub type L1W<'a, REG> = crate::FieldWriter<'a, REG, 2, L1>;
impl<'a, REG> L1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No L1 LPM state entry setting."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(L1::None)
    }
    #[doc = "L1 LPM state entry enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(L1::En)
    }
    #[doc = "L1 LPM state entry disable."]
    #[inline(always)]
    pub fn ds(self) -> &'a mut crate::W<REG> {
        self.variant(L1::Ds)
    }
}
#[doc = "USB 2.0 clock gate turn-off enable/disable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Clk2off {
    #[doc = "0: No USB 2.0 clock gate turn-off setting."]
    None = 0,
    #[doc = "1: USB 2.0 clock gate turn-off enable."]
    En = 1,
    #[doc = "2: USB 2.0 clock gate turn-off disable."]
    Ds = 2,
}
impl From<Clk2off> for u8 {
    #[inline(always)]
    fn from(variant: Clk2off) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Clk2off {
    type Ux = u8;
}
#[doc = "Field `clk2off` reader - USB 2.0 clock gate turn-off enable/disable."]
pub type Clk2offR = crate::FieldReader<Clk2off>;
impl Clk2offR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Clk2off> {
        match self.bits {
            0 => Some(Clk2off::None),
            1 => Some(Clk2off::En),
            2 => Some(Clk2off::Ds),
            _ => None,
        }
    }
    #[doc = "No USB 2.0 clock gate turn-off setting."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Clk2off::None
    }
    #[doc = "USB 2.0 clock gate turn-off enable."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Clk2off::En
    }
    #[doc = "USB 2.0 clock gate turn-off disable."]
    #[inline(always)]
    pub fn is_ds(&self) -> bool {
        *self == Clk2off::Ds
    }
}
#[doc = "Field `clk2off` writer - USB 2.0 clock gate turn-off enable/disable."]
pub type Clk2offW<'a, REG> = crate::FieldWriter<'a, REG, 2, Clk2off>;
impl<'a, REG> Clk2offW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No USB 2.0 clock gate turn-off setting."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Clk2off::None)
    }
    #[doc = "USB 2.0 clock gate turn-off enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Clk2off::En)
    }
    #[doc = "USB 2.0 clock gate turn-off disable."]
    #[inline(always)]
    pub fn ds(self) -> &'a mut crate::W<REG> {
        self.variant(Clk2off::Ds)
    }
}
#[doc = "Field `lgo_l0` reader - L0 LPM state entry request (used in HS/FS mode)."]
pub type LgoL0R = crate::BitReader;
#[doc = "Field `lgo_l0` writer - L0 LPM state entry request (used in HS/FS mode)."]
pub type LgoL0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "USB 3.0 clock gate turn-off enable/disable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Clk3off {
    #[doc = "0: USB 3.0 clock gate turn-off setting."]
    None = 0,
    #[doc = "1: USB 3.0 clock gate turn-off enable."]
    En = 1,
    #[doc = "2: USB 3.0 clock gate turn-off disable."]
    Ds = 2,
}
impl From<Clk3off> for u8 {
    #[inline(always)]
    fn from(variant: Clk3off) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Clk3off {
    type Ux = u8;
}
#[doc = "Field `clk3off` reader - USB 3.0 clock gate turn-off enable/disable."]
pub type Clk3offR = crate::FieldReader<Clk3off>;
impl Clk3offR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Clk3off> {
        match self.bits {
            0 => Some(Clk3off::None),
            1 => Some(Clk3off::En),
            2 => Some(Clk3off::Ds),
            _ => None,
        }
    }
    #[doc = "USB 3.0 clock gate turn-off setting."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Clk3off::None
    }
    #[doc = "USB 3.0 clock gate turn-off enable."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == Clk3off::En
    }
    #[doc = "USB 3.0 clock gate turn-off disable."]
    #[inline(always)]
    pub fn is_ds(&self) -> bool {
        *self == Clk3off::Ds
    }
}
#[doc = "Field `clk3off` writer - USB 3.0 clock gate turn-off enable/disable."]
pub type Clk3offW<'a, REG> = crate::FieldWriter<'a, REG, 2, Clk3off>;
impl<'a, REG> Clk3offW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "USB 3.0 clock gate turn-off setting."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Clk3off::None)
    }
    #[doc = "USB 3.0 clock gate turn-off enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(Clk3off::En)
    }
    #[doc = "USB 3.0 clock gate turn-off disable."]
    #[inline(always)]
    pub fn ds(self) -> &'a mut crate::W<REG> {
        self.variant(Clk3off::Ds)
    }
}
#[doc = "U1 state entry enable/disable (used in SS mode).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum U1 {
    #[doc = "0: No U1 state entry setting."]
    None = 0,
    #[doc = "1: U1 state entry enable."]
    En = 1,
    #[doc = "2: U1 state entry disable."]
    Ds = 2,
}
impl From<U1> for u8 {
    #[inline(always)]
    fn from(variant: U1) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for U1 {
    type Ux = u8;
}
#[doc = "Field `u1` reader - U1 state entry enable/disable (used in SS mode)."]
pub type U1R = crate::FieldReader<U1>;
impl U1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<U1> {
        match self.bits {
            0 => Some(U1::None),
            1 => Some(U1::En),
            2 => Some(U1::Ds),
            _ => None,
        }
    }
    #[doc = "No U1 state entry setting."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U1::None
    }
    #[doc = "U1 state entry enable."]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == U1::En
    }
    #[doc = "U1 state entry disable."]
    #[inline(always)]
    pub fn is_ds(&self) -> bool {
        *self == U1::Ds
    }
}
#[doc = "Field `u1` writer - U1 state entry enable/disable (used in SS mode)."]
pub type U1W<'a, REG> = crate::FieldWriter<'a, REG, 2, U1>;
impl<'a, REG> U1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No U1 state entry setting."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U1::None)
    }
    #[doc = "U1 state entry enable."]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(U1::En)
    }
    #[doc = "U1 state entry disable."]
    #[inline(always)]
    pub fn ds(self) -> &'a mut crate::W<REG> {
        self.variant(U1::Ds)
    }
}
#[doc = "U2 state entry enable/disable (used in SS mode).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum U2 {
    #[doc = "0: No U2 state entry setting."]
    None = 0,
    #[doc = "1: U2 state entry enable"]
    En = 1,
    #[doc = "2: U2 state entry disable"]
    Ds = 2,
}
impl From<U2> for u8 {
    #[inline(always)]
    fn from(variant: U2) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for U2 {
    type Ux = u8;
}
#[doc = "Field `u2` reader - U2 state entry enable/disable (used in SS mode)."]
pub type U2R = crate::FieldReader<U2>;
impl U2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<U2> {
        match self.bits {
            0 => Some(U2::None),
            1 => Some(U2::En),
            2 => Some(U2::Ds),
            _ => None,
        }
    }
    #[doc = "No U2 state entry setting."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U2::None
    }
    #[doc = "U2 state entry enable"]
    #[inline(always)]
    pub fn is_en(&self) -> bool {
        *self == U2::En
    }
    #[doc = "U2 state entry disable"]
    #[inline(always)]
    pub fn is_ds(&self) -> bool {
        *self == U2::Ds
    }
}
#[doc = "Field `u2` writer - U2 state entry enable/disable (used in SS mode)."]
pub type U2W<'a, REG> = crate::FieldWriter<'a, REG, 2, U2>;
impl<'a, REG> U2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No U2 state entry setting."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U2::None)
    }
    #[doc = "U2 state entry enable"]
    #[inline(always)]
    pub fn en(self) -> &'a mut crate::W<REG> {
        self.variant(U2::En)
    }
    #[doc = "U2 state entry disable"]
    #[inline(always)]
    pub fn ds(self) -> &'a mut crate::W<REG> {
        self.variant(U2::Ds)
    }
}
#[doc = "Field `lgo_u(0-2)` reader - U0-U2 state entry request - used in SS mode."]
pub type LgoUR = crate::BitReader;
#[doc = "Field `lgo_u(0-2)` writer - U0-U2 state entry request - used in SS mode."]
pub type LgoUW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `lgo_ssinact` reader - SS.Inactive state entry request - used in SS mode."]
pub type LgoSsinactR = crate::BitReader;
#[doc = "Field `lgo_ssinact` writer - SS.Inactive state entry request - used in SS mode."]
pub type LgoSsinactW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Reset/Set USB device configuration."]
    #[inline(always)]
    pub fn cfg(&self) -> CfgR {
        CfgR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 3:4 - Disconnect USB device."]
    #[inline(always)]
    pub fn dis(&self) -> DisR {
        DisR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:6 - Endian access."]
    #[inline(always)]
    pub fn endian(&self) -> EndianR {
        EndianR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Device software reset."]
    #[inline(always)]
    pub fn swrst(&self) -> SwrstR {
        SwrstR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 10:11 - DMA clock turn-off."]
    #[inline(always)]
    pub fn dmaoff(&self) -> DmaoffR {
        DmaoffR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Force Full Speed."]
    #[inline(always)]
    pub fn force_fs(&self) -> ForceFsR {
        ForceFsR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Device enable/disable."]
    #[inline(always)]
    pub fn dev(&self) -> DevR {
        DevR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - L1 LPM state entry enable/disable (used in HS/FS mode)."]
    #[inline(always)]
    pub fn l1(&self) -> L1R {
        L1R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - USB 2.0 clock gate turn-off enable/disable."]
    #[inline(always)]
    pub fn clk2off(&self) -> Clk2offR {
        Clk2offR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 20 - L0 LPM state entry request (used in HS/FS mode)."]
    #[inline(always)]
    pub fn lgo_l0(&self) -> LgoL0R {
        LgoL0R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:22 - USB 3.0 clock gate turn-off enable/disable."]
    #[inline(always)]
    pub fn clk3off(&self) -> Clk3offR {
        Clk3offR::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bits 24:25 - U1 state entry enable/disable (used in SS mode)."]
    #[inline(always)]
    pub fn u1(&self) -> U1R {
        U1R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 26:27 - U2 state entry enable/disable (used in SS mode)."]
    #[inline(always)]
    pub fn u2(&self) -> U2R {
        U2R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "U0-U2 state entry request - used in SS mode."]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `lgo_u0` field"]
    #[inline(always)]
    pub fn lgo_u(&self, n: u8) -> LgoUR {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        LgoUR::new(((self.bits >> (n + 28)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "U0-U2 state entry request - used in SS mode."]
    #[inline(always)]
    pub fn lgo_u_iter(&self) -> impl Iterator<Item = LgoUR> + '_ {
        (0..3).map(move |n| LgoUR::new(((self.bits >> (n + 28)) & 1) != 0))
    }
    #[doc = "Bit 28 - U0-U2 state entry request - used in SS mode."]
    #[inline(always)]
    pub fn lgo_u0(&self) -> LgoUR {
        LgoUR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - U0-U2 state entry request - used in SS mode."]
    #[inline(always)]
    pub fn lgo_u1(&self) -> LgoUR {
        LgoUR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - U0-U2 state entry request - used in SS mode."]
    #[inline(always)]
    pub fn lgo_u2(&self) -> LgoUR {
        LgoUR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - SS.Inactive state entry request - used in SS mode."]
    #[inline(always)]
    pub fn lgo_ssinact(&self) -> LgoSsinactR {
        LgoSsinactR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Reset/Set USB device configuration."]
    #[inline(always)]
    #[must_use]
    pub fn cfg(&mut self) -> CfgW<UsbConfSpec> {
        CfgW::new(self, 0)
    }
    #[doc = "Bits 3:4 - Disconnect USB device."]
    #[inline(always)]
    #[must_use]
    pub fn dis(&mut self) -> DisW<UsbConfSpec> {
        DisW::new(self, 3)
    }
    #[doc = "Bits 5:6 - Endian access."]
    #[inline(always)]
    #[must_use]
    pub fn endian(&mut self) -> EndianW<UsbConfSpec> {
        EndianW::new(self, 5)
    }
    #[doc = "Bit 7 - Device software reset."]
    #[inline(always)]
    #[must_use]
    pub fn swrst(&mut self) -> SwrstW<UsbConfSpec> {
        SwrstW::new(self, 7)
    }
    #[doc = "Bits 10:11 - DMA clock turn-off."]
    #[inline(always)]
    #[must_use]
    pub fn dmaoff(&mut self) -> DmaoffW<UsbConfSpec> {
        DmaoffW::new(self, 10)
    }
    #[doc = "Bits 12:13 - Force Full Speed."]
    #[inline(always)]
    #[must_use]
    pub fn force_fs(&mut self) -> ForceFsW<UsbConfSpec> {
        ForceFsW::new(self, 12)
    }
    #[doc = "Bits 14:15 - Device enable/disable."]
    #[inline(always)]
    #[must_use]
    pub fn dev(&mut self) -> DevW<UsbConfSpec> {
        DevW::new(self, 14)
    }
    #[doc = "Bits 16:17 - L1 LPM state entry enable/disable (used in HS/FS mode)."]
    #[inline(always)]
    #[must_use]
    pub fn l1(&mut self) -> L1W<UsbConfSpec> {
        L1W::new(self, 16)
    }
    #[doc = "Bits 18:19 - USB 2.0 clock gate turn-off enable/disable."]
    #[inline(always)]
    #[must_use]
    pub fn clk2off(&mut self) -> Clk2offW<UsbConfSpec> {
        Clk2offW::new(self, 18)
    }
    #[doc = "Bit 20 - L0 LPM state entry request (used in HS/FS mode)."]
    #[inline(always)]
    #[must_use]
    pub fn lgo_l0(&mut self) -> LgoL0W<UsbConfSpec> {
        LgoL0W::new(self, 20)
    }
    #[doc = "Bits 21:22 - USB 3.0 clock gate turn-off enable/disable."]
    #[inline(always)]
    #[must_use]
    pub fn clk3off(&mut self) -> Clk3offW<UsbConfSpec> {
        Clk3offW::new(self, 21)
    }
    #[doc = "Bits 24:25 - U1 state entry enable/disable (used in SS mode)."]
    #[inline(always)]
    #[must_use]
    pub fn u1(&mut self) -> U1W<UsbConfSpec> {
        U1W::new(self, 24)
    }
    #[doc = "Bits 26:27 - U2 state entry enable/disable (used in SS mode)."]
    #[inline(always)]
    #[must_use]
    pub fn u2(&mut self) -> U2W<UsbConfSpec> {
        U2W::new(self, 26)
    }
    #[doc = "U0-U2 state entry request - used in SS mode."]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `lgo_u0` field"]
    #[inline(always)]
    #[must_use]
    pub fn lgo_u(&mut self, n: u8) -> LgoUW<UsbConfSpec> {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        LgoUW::new(self, n + 28)
    }
    #[doc = "Bit 28 - U0-U2 state entry request - used in SS mode."]
    #[inline(always)]
    #[must_use]
    pub fn lgo_u0(&mut self) -> LgoUW<UsbConfSpec> {
        LgoUW::new(self, 28)
    }
    #[doc = "Bit 29 - U0-U2 state entry request - used in SS mode."]
    #[inline(always)]
    #[must_use]
    pub fn lgo_u1(&mut self) -> LgoUW<UsbConfSpec> {
        LgoUW::new(self, 29)
    }
    #[doc = "Bit 30 - U0-U2 state entry request - used in SS mode."]
    #[inline(always)]
    #[must_use]
    pub fn lgo_u2(&mut self) -> LgoUW<UsbConfSpec> {
        LgoUW::new(self, 30)
    }
    #[doc = "Bit 31 - SS.Inactive state entry request - used in SS mode."]
    #[inline(always)]
    #[must_use]
    pub fn lgo_ssinact(&mut self) -> LgoSsinactW<UsbConfSpec> {
        LgoSsinactW::new(self, 31)
    }
}
#[doc = "USB3 Global configuration.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UsbConfSpec;
impl crate::RegisterSpec for UsbConfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usb_conf::R`](R) reader structure"]
impl crate::Readable for UsbConfSpec {}
#[doc = "`write(|w| ..)` method takes [`usb_conf::W`](W) writer structure"]
impl crate::Writable for UsbConfSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets usb_conf to value 0"]
impl crate::Resettable for UsbConfSpec {
    const RESET_VALUE: u32 = 0;
}
