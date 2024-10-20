#[doc = "Register `cap1` reader"]
pub type R = crate::R<Cap1Spec>;
#[doc = "Register `cap1` writer"]
pub type W = crate::W<Cap1Spec>;
#[doc = "SFR interface type.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SfrType {
    #[doc = "0: SFR OCP interface."]
    Ocp = 0,
    #[doc = "1: SFR AHB interface."]
    Ahb = 1,
    #[doc = "2: SFR PLB interface."]
    Plb = 2,
    #[doc = "3: SFR AXI interface."]
    Axi = 3,
}
impl From<SfrType> for u8 {
    #[inline(always)]
    fn from(variant: SfrType) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SfrType {
    type Ux = u8;
}
#[doc = "Field `sfr_type` reader - SFR interface type."]
pub type SfrTypeR = crate::FieldReader<SfrType>;
impl SfrTypeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SfrType> {
        match self.bits {
            0 => Some(SfrType::Ocp),
            1 => Some(SfrType::Ahb),
            2 => Some(SfrType::Plb),
            3 => Some(SfrType::Axi),
            _ => None,
        }
    }
    #[doc = "SFR OCP interface."]
    #[inline(always)]
    pub fn is_ocp(&self) -> bool {
        *self == SfrType::Ocp
    }
    #[doc = "SFR AHB interface."]
    #[inline(always)]
    pub fn is_ahb(&self) -> bool {
        *self == SfrType::Ahb
    }
    #[doc = "SFR PLB interface."]
    #[inline(always)]
    pub fn is_plb(&self) -> bool {
        *self == SfrType::Plb
    }
    #[doc = "SFR AXI interface."]
    #[inline(always)]
    pub fn is_axi(&self) -> bool {
        *self == SfrType::Axi
    }
}
#[doc = "Field `sfr_type` writer - SFR interface type."]
pub type SfrTypeW<'a, REG> = crate::FieldWriter<'a, REG, 4, SfrType>;
impl<'a, REG> SfrTypeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SFR OCP interface."]
    #[inline(always)]
    pub fn ocp(self) -> &'a mut crate::W<REG> {
        self.variant(SfrType::Ocp)
    }
    #[doc = "SFR AHB interface."]
    #[inline(always)]
    pub fn ahb(self) -> &'a mut crate::W<REG> {
        self.variant(SfrType::Ahb)
    }
    #[doc = "SFR PLB interface."]
    #[inline(always)]
    pub fn plb(self) -> &'a mut crate::W<REG> {
        self.variant(SfrType::Plb)
    }
    #[doc = "SFR AXI interface."]
    #[inline(always)]
    pub fn axi(self) -> &'a mut crate::W<REG> {
        self.variant(SfrType::Axi)
    }
}
#[doc = "SFR interface width.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SfrWidth {
    #[doc = "0: SFR 8-bit interface."]
    Bit8 = 0,
    #[doc = "1: SFR 16-bit interface."]
    Bit16 = 1,
    #[doc = "2: SFR 32-bit interface."]
    Bit32 = 2,
    #[doc = "3: SFR 64-bit interface."]
    Bit64 = 3,
}
impl From<SfrWidth> for u8 {
    #[inline(always)]
    fn from(variant: SfrWidth) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SfrWidth {
    type Ux = u8;
}
#[doc = "Field `sfr_width` reader - SFR interface width."]
pub type SfrWidthR = crate::FieldReader<SfrWidth>;
impl SfrWidthR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SfrWidth> {
        match self.bits {
            0 => Some(SfrWidth::Bit8),
            1 => Some(SfrWidth::Bit16),
            2 => Some(SfrWidth::Bit32),
            3 => Some(SfrWidth::Bit64),
            _ => None,
        }
    }
    #[doc = "SFR 8-bit interface."]
    #[inline(always)]
    pub fn is_bit8(&self) -> bool {
        *self == SfrWidth::Bit8
    }
    #[doc = "SFR 16-bit interface."]
    #[inline(always)]
    pub fn is_bit16(&self) -> bool {
        *self == SfrWidth::Bit16
    }
    #[doc = "SFR 32-bit interface."]
    #[inline(always)]
    pub fn is_bit32(&self) -> bool {
        *self == SfrWidth::Bit32
    }
    #[doc = "SFR 64-bit interface."]
    #[inline(always)]
    pub fn is_bit64(&self) -> bool {
        *self == SfrWidth::Bit64
    }
}
#[doc = "Field `sfr_width` writer - SFR interface width."]
pub type SfrWidthW<'a, REG> = crate::FieldWriter<'a, REG, 4, SfrWidth>;
impl<'a, REG> SfrWidthW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SFR 8-bit interface."]
    #[inline(always)]
    pub fn bit8(self) -> &'a mut crate::W<REG> {
        self.variant(SfrWidth::Bit8)
    }
    #[doc = "SFR 16-bit interface."]
    #[inline(always)]
    pub fn bit16(self) -> &'a mut crate::W<REG> {
        self.variant(SfrWidth::Bit16)
    }
    #[doc = "SFR 32-bit interface."]
    #[inline(always)]
    pub fn bit32(self) -> &'a mut crate::W<REG> {
        self.variant(SfrWidth::Bit32)
    }
    #[doc = "SFR 64-bit interface."]
    #[inline(always)]
    pub fn bit64(self) -> &'a mut crate::W<REG> {
        self.variant(SfrWidth::Bit64)
    }
}
#[doc = "DMA interface type.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DmaType {
    #[doc = "0: DMA OCP interface."]
    Ocp = 0,
    #[doc = "1: DMA AHB interface."]
    Ahb = 1,
    #[doc = "2: DMA PLB interface."]
    Plb = 2,
    #[doc = "3: DMA AXI interface."]
    Axi = 3,
}
impl From<DmaType> for u8 {
    #[inline(always)]
    fn from(variant: DmaType) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DmaType {
    type Ux = u8;
}
#[doc = "Field `dma_type` reader - DMA interface type."]
pub type DmaTypeR = crate::FieldReader<DmaType>;
impl DmaTypeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DmaType> {
        match self.bits {
            0 => Some(DmaType::Ocp),
            1 => Some(DmaType::Ahb),
            2 => Some(DmaType::Plb),
            3 => Some(DmaType::Axi),
            _ => None,
        }
    }
    #[doc = "DMA OCP interface."]
    #[inline(always)]
    pub fn is_ocp(&self) -> bool {
        *self == DmaType::Ocp
    }
    #[doc = "DMA AHB interface."]
    #[inline(always)]
    pub fn is_ahb(&self) -> bool {
        *self == DmaType::Ahb
    }
    #[doc = "DMA PLB interface."]
    #[inline(always)]
    pub fn is_plb(&self) -> bool {
        *self == DmaType::Plb
    }
    #[doc = "DMA AXI interface."]
    #[inline(always)]
    pub fn is_axi(&self) -> bool {
        *self == DmaType::Axi
    }
}
#[doc = "Field `dma_type` writer - DMA interface type."]
pub type DmaTypeW<'a, REG> = crate::FieldWriter<'a, REG, 4, DmaType>;
impl<'a, REG> DmaTypeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DMA OCP interface."]
    #[inline(always)]
    pub fn ocp(self) -> &'a mut crate::W<REG> {
        self.variant(DmaType::Ocp)
    }
    #[doc = "DMA AHB interface."]
    #[inline(always)]
    pub fn ahb(self) -> &'a mut crate::W<REG> {
        self.variant(DmaType::Ahb)
    }
    #[doc = "DMA PLB interface."]
    #[inline(always)]
    pub fn plb(self) -> &'a mut crate::W<REG> {
        self.variant(DmaType::Plb)
    }
    #[doc = "DMA AXI interface."]
    #[inline(always)]
    pub fn axi(self) -> &'a mut crate::W<REG> {
        self.variant(DmaType::Axi)
    }
}
#[doc = "DMA interface width.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DmaWidth {
    #[doc = "2: DMA 32-bit interface."]
    Bit32 = 2,
    #[doc = "3: DMA 64-bit interface."]
    Bit64 = 3,
}
impl From<DmaWidth> for u8 {
    #[inline(always)]
    fn from(variant: DmaWidth) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DmaWidth {
    type Ux = u8;
}
#[doc = "Field `dma_width` reader - DMA interface width."]
pub type DmaWidthR = crate::FieldReader<DmaWidth>;
impl DmaWidthR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DmaWidth> {
        match self.bits {
            2 => Some(DmaWidth::Bit32),
            3 => Some(DmaWidth::Bit64),
            _ => None,
        }
    }
    #[doc = "DMA 32-bit interface."]
    #[inline(always)]
    pub fn is_bit32(&self) -> bool {
        *self == DmaWidth::Bit32
    }
    #[doc = "DMA 64-bit interface."]
    #[inline(always)]
    pub fn is_bit64(&self) -> bool {
        *self == DmaWidth::Bit64
    }
}
#[doc = "Field `dma_width` writer - DMA interface width."]
pub type DmaWidthW<'a, REG> = crate::FieldWriter<'a, REG, 4, DmaWidth>;
impl<'a, REG> DmaWidthW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "DMA 32-bit interface."]
    #[inline(always)]
    pub fn bit32(self) -> &'a mut crate::W<REG> {
        self.variant(DmaWidth::Bit32)
    }
    #[doc = "DMA 64-bit interface."]
    #[inline(always)]
    pub fn bit64(self) -> &'a mut crate::W<REG> {
        self.variant(DmaWidth::Bit64)
    }
}
#[doc = "USB3 PHY interface type.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum U3phyType {
    #[doc = "0: USB PIPE interface."]
    UsbPipe = 0,
    #[doc = "1: RMMI interface."]
    Rmmi = 1,
}
impl From<U3phyType> for u8 {
    #[inline(always)]
    fn from(variant: U3phyType) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for U3phyType {
    type Ux = u8;
}
#[doc = "Field `u3phy_type` reader - USB3 PHY interface type."]
pub type U3phyTypeR = crate::FieldReader<U3phyType>;
impl U3phyTypeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<U3phyType> {
        match self.bits {
            0 => Some(U3phyType::UsbPipe),
            1 => Some(U3phyType::Rmmi),
            _ => None,
        }
    }
    #[doc = "USB PIPE interface."]
    #[inline(always)]
    pub fn is_usb_pipe(&self) -> bool {
        *self == U3phyType::UsbPipe
    }
    #[doc = "RMMI interface."]
    #[inline(always)]
    pub fn is_rmmi(&self) -> bool {
        *self == U3phyType::Rmmi
    }
}
#[doc = "Field `u3phy_type` writer - USB3 PHY interface type."]
pub type U3phyTypeW<'a, REG> = crate::FieldWriter<'a, REG, 4, U3phyType>;
impl<'a, REG> U3phyTypeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "USB PIPE interface."]
    #[inline(always)]
    pub fn usb_pipe(self) -> &'a mut crate::W<REG> {
        self.variant(U3phyType::UsbPipe)
    }
    #[doc = "RMMI interface."]
    #[inline(always)]
    pub fn rmmi(self) -> &'a mut crate::W<REG> {
        self.variant(U3phyType::Rmmi)
    }
}
#[doc = "USB3 PHY interface width.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum U3phyWidth {
    #[doc = "0: USB3 PHY 8-bit interface."]
    Bit8 = 0,
    #[doc = "1: USB3 PHY 16-bit interface."]
    Bit16 = 1,
    #[doc = "2: USB3 PHY 32-bit interface."]
    Bit32 = 2,
    #[doc = "3: USB3 PHY 64-bit interface."]
    Bit64 = 3,
}
impl From<U3phyWidth> for u8 {
    #[inline(always)]
    fn from(variant: U3phyWidth) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for U3phyWidth {
    type Ux = u8;
}
#[doc = "Field `u3phy_width` reader - USB3 PHY interface width."]
pub type U3phyWidthR = crate::FieldReader<U3phyWidth>;
impl U3phyWidthR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<U3phyWidth> {
        match self.bits {
            0 => Some(U3phyWidth::Bit8),
            1 => Some(U3phyWidth::Bit16),
            2 => Some(U3phyWidth::Bit32),
            3 => Some(U3phyWidth::Bit64),
            _ => None,
        }
    }
    #[doc = "USB3 PHY 8-bit interface."]
    #[inline(always)]
    pub fn is_bit8(&self) -> bool {
        *self == U3phyWidth::Bit8
    }
    #[doc = "USB3 PHY 16-bit interface."]
    #[inline(always)]
    pub fn is_bit16(&self) -> bool {
        *self == U3phyWidth::Bit16
    }
    #[doc = "USB3 PHY 32-bit interface."]
    #[inline(always)]
    pub fn is_bit32(&self) -> bool {
        *self == U3phyWidth::Bit32
    }
    #[doc = "USB3 PHY 64-bit interface."]
    #[inline(always)]
    pub fn is_bit64(&self) -> bool {
        *self == U3phyWidth::Bit64
    }
}
#[doc = "Field `u3phy_width` writer - USB3 PHY interface width."]
pub type U3phyWidthW<'a, REG> = crate::FieldWriter<'a, REG, 4, U3phyWidth>;
impl<'a, REG> U3phyWidthW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "USB3 PHY 8-bit interface."]
    #[inline(always)]
    pub fn bit8(self) -> &'a mut crate::W<REG> {
        self.variant(U3phyWidth::Bit8)
    }
    #[doc = "USB3 PHY 16-bit interface."]
    #[inline(always)]
    pub fn bit16(self) -> &'a mut crate::W<REG> {
        self.variant(U3phyWidth::Bit16)
    }
    #[doc = "USB3 PHY 32-bit interface."]
    #[inline(always)]
    pub fn bit32(self) -> &'a mut crate::W<REG> {
        self.variant(U3phyWidth::Bit32)
    }
    #[doc = "USB3 PHY 64-bit interface."]
    #[inline(always)]
    pub fn bit64(self) -> &'a mut crate::W<REG> {
        self.variant(U3phyWidth::Bit64)
    }
}
#[doc = "Field `u2phy_en` reader - USB2 PHY interface enable."]
pub type U2phyEnR = crate::BitReader;
#[doc = "Field `u2phy_en` writer - USB2 PHY interface enable."]
pub type U2phyEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "USB2 PHY interface type.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U2phyType {
    #[doc = "0: USB2 PHY UTMI interface."]
    Utmi = 0,
    #[doc = "1: USB2 PHY ULPI interface."]
    Ulpi = 1,
}
impl From<U2phyType> for bool {
    #[inline(always)]
    fn from(variant: U2phyType) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u2phy_type` reader - USB2 PHY interface type."]
pub type U2phyTypeR = crate::BitReader<U2phyType>;
impl U2phyTypeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U2phyType {
        match self.bits {
            false => U2phyType::Utmi,
            true => U2phyType::Ulpi,
        }
    }
    #[doc = "USB2 PHY UTMI interface."]
    #[inline(always)]
    pub fn is_utmi(&self) -> bool {
        *self == U2phyType::Utmi
    }
    #[doc = "USB2 PHY ULPI interface."]
    #[inline(always)]
    pub fn is_ulpi(&self) -> bool {
        *self == U2phyType::Ulpi
    }
}
#[doc = "Field `u2phy_type` writer - USB2 PHY interface type."]
pub type U2phyTypeW<'a, REG> = crate::BitWriter<'a, REG, U2phyType>;
impl<'a, REG> U2phyTypeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "USB2 PHY UTMI interface."]
    #[inline(always)]
    pub fn utmi(self) -> &'a mut crate::W<REG> {
        self.variant(U2phyType::Utmi)
    }
    #[doc = "USB2 PHY ULPI interface."]
    #[inline(always)]
    pub fn ulpi(self) -> &'a mut crate::W<REG> {
        self.variant(U2phyType::Ulpi)
    }
}
#[doc = "USB2 PHY interface width - **NOTE**: The ULPI interface is always 8-bit wide.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U2phyWidth {
    #[doc = "0: USB2 PHY 8-bit interface width."]
    Bit8 = 0,
    #[doc = "1: USB2 PHY 16-bit interface width."]
    Bit16 = 1,
}
impl From<U2phyWidth> for bool {
    #[inline(always)]
    fn from(variant: U2phyWidth) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u2phy_width` reader - USB2 PHY interface width - **NOTE**: The ULPI interface is always 8-bit wide."]
pub type U2phyWidthR = crate::BitReader<U2phyWidth>;
impl U2phyWidthR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U2phyWidth {
        match self.bits {
            false => U2phyWidth::Bit8,
            true => U2phyWidth::Bit16,
        }
    }
    #[doc = "USB2 PHY 8-bit interface width."]
    #[inline(always)]
    pub fn is_bit8(&self) -> bool {
        *self == U2phyWidth::Bit8
    }
    #[doc = "USB2 PHY 16-bit interface width."]
    #[inline(always)]
    pub fn is_bit16(&self) -> bool {
        *self == U2phyWidth::Bit16
    }
}
#[doc = "Field `u2phy_width` writer - USB2 PHY interface width - **NOTE**: The ULPI interface is always 8-bit wide."]
pub type U2phyWidthW<'a, REG> = crate::BitWriter<'a, REG, U2phyWidth>;
impl<'a, REG> U2phyWidthW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "USB2 PHY 8-bit interface width."]
    #[inline(always)]
    pub fn bit8(self) -> &'a mut crate::W<REG> {
        self.variant(U2phyWidth::Bit8)
    }
    #[doc = "USB2 PHY 16-bit interface width."]
    #[inline(always)]
    pub fn bit16(self) -> &'a mut crate::W<REG> {
        self.variant(U2phyWidth::Bit16)
    }
}
#[doc = "OTG mode ready.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OtgReady {
    #[doc = "0: Pure device mode."]
    DevOnly = 0,
    #[doc = "1: Some features and ports for CDNS USB OTG controller are implemented."]
    Otg = 1,
}
impl From<OtgReady> for bool {
    #[inline(always)]
    fn from(variant: OtgReady) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `otg_ready` reader - OTG mode ready."]
pub type OtgReadyR = crate::BitReader<OtgReady>;
impl OtgReadyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OtgReady {
        match self.bits {
            false => OtgReady::DevOnly,
            true => OtgReady::Otg,
        }
    }
    #[doc = "Pure device mode."]
    #[inline(always)]
    pub fn is_dev_only(&self) -> bool {
        *self == OtgReady::DevOnly
    }
    #[doc = "Some features and ports for CDNS USB OTG controller are implemented."]
    #[inline(always)]
    pub fn is_otg(&self) -> bool {
        *self == OtgReady::Otg
    }
}
#[doc = "Field `otg_ready` writer - OTG mode ready."]
pub type OtgReadyW<'a, REG> = crate::BitWriter<'a, REG, OtgReady>;
impl<'a, REG> OtgReadyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Pure device mode."]
    #[inline(always)]
    pub fn dev_only(self) -> &'a mut crate::W<REG> {
        self.variant(OtgReady::DevOnly)
    }
    #[doc = "Some features and ports for CDNS USB OTG controller are implemented."]
    #[inline(always)]
    pub fn otg(self) -> &'a mut crate::W<REG> {
        self.variant(OtgReady::Otg)
    }
}
#[doc = "Field `tdl_from_trb` reader - Indicates the capability to automatically calculate internal TDL from TRB value for DMULT mode."]
pub type TdlFromTrbR = crate::BitReader;
#[doc = "Field `tdl_from_trb` writer - Indicates the capability to automatically calculate internal TDL from TRB value for DMULT mode."]
pub type TdlFromTrbW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - SFR interface type."]
    #[inline(always)]
    pub fn sfr_type(&self) -> SfrTypeR {
        SfrTypeR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - SFR interface width."]
    #[inline(always)]
    pub fn sfr_width(&self) -> SfrWidthR {
        SfrWidthR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - DMA interface type."]
    #[inline(always)]
    pub fn dma_type(&self) -> DmaTypeR {
        DmaTypeR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - DMA interface width."]
    #[inline(always)]
    pub fn dma_width(&self) -> DmaWidthR {
        DmaWidthR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - USB3 PHY interface type."]
    #[inline(always)]
    pub fn u3phy_type(&self) -> U3phyTypeR {
        U3phyTypeR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - USB3 PHY interface width."]
    #[inline(always)]
    pub fn u3phy_width(&self) -> U3phyWidthR {
        U3phyWidthR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - USB2 PHY interface enable."]
    #[inline(always)]
    pub fn u2phy_en(&self) -> U2phyEnR {
        U2phyEnR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - USB2 PHY interface type."]
    #[inline(always)]
    pub fn u2phy_type(&self) -> U2phyTypeR {
        U2phyTypeR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - USB2 PHY interface width - **NOTE**: The ULPI interface is always 8-bit wide."]
    #[inline(always)]
    pub fn u2phy_width(&self) -> U2phyWidthR {
        U2phyWidthR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - OTG mode ready."]
    #[inline(always)]
    pub fn otg_ready(&self) -> OtgReadyR {
        OtgReadyR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Indicates the capability to automatically calculate internal TDL from TRB value for DMULT mode."]
    #[inline(always)]
    pub fn tdl_from_trb(&self) -> TdlFromTrbR {
        TdlFromTrbR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - SFR interface type."]
    #[inline(always)]
    #[must_use]
    pub fn sfr_type(&mut self) -> SfrTypeW<Cap1Spec> {
        SfrTypeW::new(self, 0)
    }
    #[doc = "Bits 4:7 - SFR interface width."]
    #[inline(always)]
    #[must_use]
    pub fn sfr_width(&mut self) -> SfrWidthW<Cap1Spec> {
        SfrWidthW::new(self, 4)
    }
    #[doc = "Bits 8:11 - DMA interface type."]
    #[inline(always)]
    #[must_use]
    pub fn dma_type(&mut self) -> DmaTypeW<Cap1Spec> {
        DmaTypeW::new(self, 8)
    }
    #[doc = "Bits 12:15 - DMA interface width."]
    #[inline(always)]
    #[must_use]
    pub fn dma_width(&mut self) -> DmaWidthW<Cap1Spec> {
        DmaWidthW::new(self, 12)
    }
    #[doc = "Bits 16:19 - USB3 PHY interface type."]
    #[inline(always)]
    #[must_use]
    pub fn u3phy_type(&mut self) -> U3phyTypeW<Cap1Spec> {
        U3phyTypeW::new(self, 16)
    }
    #[doc = "Bits 20:23 - USB3 PHY interface width."]
    #[inline(always)]
    #[must_use]
    pub fn u3phy_width(&mut self) -> U3phyWidthW<Cap1Spec> {
        U3phyWidthW::new(self, 20)
    }
    #[doc = "Bit 24 - USB2 PHY interface enable."]
    #[inline(always)]
    #[must_use]
    pub fn u2phy_en(&mut self) -> U2phyEnW<Cap1Spec> {
        U2phyEnW::new(self, 24)
    }
    #[doc = "Bit 25 - USB2 PHY interface type."]
    #[inline(always)]
    #[must_use]
    pub fn u2phy_type(&mut self) -> U2phyTypeW<Cap1Spec> {
        U2phyTypeW::new(self, 25)
    }
    #[doc = "Bit 26 - USB2 PHY interface width - **NOTE**: The ULPI interface is always 8-bit wide."]
    #[inline(always)]
    #[must_use]
    pub fn u2phy_width(&mut self) -> U2phyWidthW<Cap1Spec> {
        U2phyWidthW::new(self, 26)
    }
    #[doc = "Bit 27 - OTG mode ready."]
    #[inline(always)]
    #[must_use]
    pub fn otg_ready(&mut self) -> OtgReadyW<Cap1Spec> {
        OtgReadyW::new(self, 27)
    }
    #[doc = "Bit 28 - Indicates the capability to automatically calculate internal TDL from TRB value for DMULT mode."]
    #[inline(always)]
    #[must_use]
    pub fn tdl_from_trb(&mut self) -> TdlFromTrbW<Cap1Spec> {
        TdlFromTrbW::new(self, 28)
    }
}
#[doc = "USB3 Global capability 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cap1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cap1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cap1Spec;
impl crate::RegisterSpec for Cap1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cap1::R`](R) reader structure"]
impl crate::Readable for Cap1Spec {}
#[doc = "`write(|w| ..)` method takes [`cap1::W`](W) writer structure"]
impl crate::Writable for Cap1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets cap1 to value 0"]
impl crate::Resettable for Cap1Spec {
    const RESET_VALUE: u32 = 0;
}
