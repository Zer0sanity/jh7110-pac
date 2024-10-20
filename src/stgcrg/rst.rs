#[doc = "Register `rst[%s]` reader"]
pub type R = crate::R<RstSpec>;
#[doc = "Register `rst[%s]` writer"]
pub type W = crate::W<RstSpec>;
#[doc = "U0 STG SYSCON Presetn reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U0StgSysconPresetn {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U0StgSysconPresetn> for bool {
    #[inline(always)]
    fn from(variant: U0StgSysconPresetn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u0_stg_syscon_presetn` reader - U0 STG SYSCON Presetn reset."]
pub type U0StgSysconPresetnR = crate::BitReader<U0StgSysconPresetn>;
impl U0StgSysconPresetnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U0StgSysconPresetn {
        match self.bits {
            false => U0StgSysconPresetn::None,
            true => U0StgSysconPresetn::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U0StgSysconPresetn::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U0StgSysconPresetn::Reset
    }
}
#[doc = "Field `u0_stg_syscon_presetn` writer - U0 STG SYSCON Presetn reset."]
pub type U0StgSysconPresetnW<'a, REG> = crate::BitWriter<'a, REG, U0StgSysconPresetn>;
impl<'a, REG> U0StgSysconPresetnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U0StgSysconPresetn::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U0StgSysconPresetn::Reset)
    }
}
#[doc = "U0 HIFI4 Core reset.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U0Hifi4Core {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U0Hifi4Core> for bool {
    #[inline(always)]
    fn from(variant: U0Hifi4Core) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u0_hifi4_core` reader - U0 HIFI4 Core reset."]
pub type U0Hifi4CoreR = crate::BitReader<U0Hifi4Core>;
impl U0Hifi4CoreR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U0Hifi4Core {
        match self.bits {
            false => U0Hifi4Core::None,
            true => U0Hifi4Core::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U0Hifi4Core::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U0Hifi4Core::Reset
    }
}
#[doc = "Field `u0_hifi4_core` writer - U0 HIFI4 Core reset."]
pub type U0Hifi4CoreW<'a, REG> = crate::BitWriter<'a, REG, U0Hifi4Core>;
impl<'a, REG> U0Hifi4CoreW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U0Hifi4Core::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U0Hifi4Core::Reset)
    }
}
#[doc = "U0 HIFI4 AXI reset.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U0Hifi4Axi {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U0Hifi4Axi> for bool {
    #[inline(always)]
    fn from(variant: U0Hifi4Axi) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u0_hifi4_axi` reader - U0 HIFI4 AXI reset."]
pub type U0Hifi4AxiR = crate::BitReader<U0Hifi4Axi>;
impl U0Hifi4AxiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U0Hifi4Axi {
        match self.bits {
            false => U0Hifi4Axi::None,
            true => U0Hifi4Axi::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U0Hifi4Axi::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U0Hifi4Axi::Reset
    }
}
#[doc = "Field `u0_hifi4_axi` writer - U0 HIFI4 AXI reset."]
pub type U0Hifi4AxiW<'a, REG> = crate::BitWriter<'a, REG, U0Hifi4Axi>;
impl<'a, REG> U0Hifi4AxiW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U0Hifi4Axi::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U0Hifi4Axi::Reset)
    }
}
#[doc = "U0 SEC Top HResetn reset.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U0SecTopHresetn {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U0SecTopHresetn> for bool {
    #[inline(always)]
    fn from(variant: U0SecTopHresetn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u0_sec_top_hresetn` reader - U0 SEC Top HResetn reset."]
pub type U0SecTopHresetnR = crate::BitReader<U0SecTopHresetn>;
impl U0SecTopHresetnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U0SecTopHresetn {
        match self.bits {
            false => U0SecTopHresetn::None,
            true => U0SecTopHresetn::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U0SecTopHresetn::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U0SecTopHresetn::Reset
    }
}
#[doc = "Field `u0_sec_top_hresetn` writer - U0 SEC Top HResetn reset."]
pub type U0SecTopHresetnW<'a, REG> = crate::BitWriter<'a, REG, U0SecTopHresetn>;
impl<'a, REG> U0SecTopHresetnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U0SecTopHresetn::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U0SecTopHresetn::Reset)
    }
}
#[doc = "U0 E2 Core reset.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U0E2Core {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U0E2Core> for bool {
    #[inline(always)]
    fn from(variant: U0E2Core) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u0_e2_core` reader - U0 E2 Core reset."]
pub type U0E2CoreR = crate::BitReader<U0E2Core>;
impl U0E2CoreR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U0E2Core {
        match self.bits {
            false => U0E2Core::None,
            true => U0E2Core::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U0E2Core::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U0E2Core::Reset
    }
}
#[doc = "Field `u0_e2_core` writer - U0 E2 Core reset."]
pub type U0E2CoreW<'a, REG> = crate::BitWriter<'a, REG, U0E2Core>;
impl<'a, REG> U0E2CoreW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U0E2Core::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U0E2Core::Reset)
    }
}
#[doc = "U0 DMA AXI reset.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U0DmaAxi {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U0DmaAxi> for bool {
    #[inline(always)]
    fn from(variant: U0DmaAxi) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u0_dma_axi` reader - U0 DMA AXI reset."]
pub type U0DmaAxiR = crate::BitReader<U0DmaAxi>;
impl U0DmaAxiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U0DmaAxi {
        match self.bits {
            false => U0DmaAxi::None,
            true => U0DmaAxi::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U0DmaAxi::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U0DmaAxi::Reset
    }
}
#[doc = "Field `u0_dma_axi` writer - U0 DMA AXI reset."]
pub type U0DmaAxiW<'a, REG> = crate::BitWriter<'a, REG, U0DmaAxi>;
impl<'a, REG> U0DmaAxiW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U0DmaAxi::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U0DmaAxi::Reset)
    }
}
#[doc = "U0 DMA AHB reset.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U0DmaAhb {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U0DmaAhb> for bool {
    #[inline(always)]
    fn from(variant: U0DmaAhb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u0_dma_ahb` reader - U0 DMA AHB reset."]
pub type U0DmaAhbR = crate::BitReader<U0DmaAhb>;
impl U0DmaAhbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U0DmaAhb {
        match self.bits {
            false => U0DmaAhb::None,
            true => U0DmaAhb::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U0DmaAhb::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U0DmaAhb::Reset
    }
}
#[doc = "Field `u0_dma_ahb` writer - U0 DMA AHB reset."]
pub type U0DmaAhbW<'a, REG> = crate::BitWriter<'a, REG, U0DmaAhb>;
impl<'a, REG> U0DmaAhbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U0DmaAhb::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U0DmaAhb::Reset)
    }
}
#[doc = "U0 USB AXI reset.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U0UsbAxi {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U0UsbAxi> for bool {
    #[inline(always)]
    fn from(variant: U0UsbAxi) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u0_usb_axi` reader - U0 USB AXI reset."]
pub type U0UsbAxiR = crate::BitReader<U0UsbAxi>;
impl U0UsbAxiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U0UsbAxi {
        match self.bits {
            false => U0UsbAxi::None,
            true => U0UsbAxi::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U0UsbAxi::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U0UsbAxi::Reset
    }
}
#[doc = "Field `u0_usb_axi` writer - U0 USB AXI reset."]
pub type U0UsbAxiW<'a, REG> = crate::BitWriter<'a, REG, U0UsbAxi>;
impl<'a, REG> U0UsbAxiW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U0UsbAxi::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U0UsbAxi::Reset)
    }
}
#[doc = "U0 USB APB reset.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U0UsbApb {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U0UsbApb> for bool {
    #[inline(always)]
    fn from(variant: U0UsbApb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u0_usb_apb` reader - U0 USB APB reset."]
pub type U0UsbApbR = crate::BitReader<U0UsbApb>;
impl U0UsbApbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U0UsbApb {
        match self.bits {
            false => U0UsbApb::None,
            true => U0UsbApb::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U0UsbApb::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U0UsbApb::Reset
    }
}
#[doc = "Field `u0_usb_apb` writer - U0 USB APB reset."]
pub type U0UsbApbW<'a, REG> = crate::BitWriter<'a, REG, U0UsbApb>;
impl<'a, REG> U0UsbApbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U0UsbApb::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U0UsbApb::Reset)
    }
}
#[doc = "U0 USB UTMI APB reset.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U0UsbUtmiApb {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U0UsbUtmiApb> for bool {
    #[inline(always)]
    fn from(variant: U0UsbUtmiApb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u0_usb_utmi_apb` reader - U0 USB UTMI APB reset."]
pub type U0UsbUtmiApbR = crate::BitReader<U0UsbUtmiApb>;
impl U0UsbUtmiApbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U0UsbUtmiApb {
        match self.bits {
            false => U0UsbUtmiApb::None,
            true => U0UsbUtmiApb::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U0UsbUtmiApb::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U0UsbUtmiApb::Reset
    }
}
#[doc = "Field `u0_usb_utmi_apb` writer - U0 USB UTMI APB reset."]
pub type U0UsbUtmiApbW<'a, REG> = crate::BitWriter<'a, REG, U0UsbUtmiApb>;
impl<'a, REG> U0UsbUtmiApbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U0UsbUtmiApb::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U0UsbUtmiApb::Reset)
    }
}
#[doc = "U0 USB Power-up reset.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U0UsbPwrup {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U0UsbPwrup> for bool {
    #[inline(always)]
    fn from(variant: U0UsbPwrup) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u0_usb_pwrup` reader - U0 USB Power-up reset."]
pub type U0UsbPwrupR = crate::BitReader<U0UsbPwrup>;
impl U0UsbPwrupR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U0UsbPwrup {
        match self.bits {
            false => U0UsbPwrup::None,
            true => U0UsbPwrup::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U0UsbPwrup::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U0UsbPwrup::Reset
    }
}
#[doc = "Field `u0_usb_pwrup` writer - U0 USB Power-up reset."]
pub type U0UsbPwrupW<'a, REG> = crate::BitWriter<'a, REG, U0UsbPwrup>;
impl<'a, REG> U0UsbPwrupW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U0UsbPwrup::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U0UsbPwrup::Reset)
    }
}
#[doc = "U0 PCIE AXI MST0 reset.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U0PcieAxiMst0 {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U0PcieAxiMst0> for bool {
    #[inline(always)]
    fn from(variant: U0PcieAxiMst0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u0_pcie_axi_mst0` reader - U0 PCIE AXI MST0 reset."]
pub type U0PcieAxiMst0R = crate::BitReader<U0PcieAxiMst0>;
impl U0PcieAxiMst0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U0PcieAxiMst0 {
        match self.bits {
            false => U0PcieAxiMst0::None,
            true => U0PcieAxiMst0::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U0PcieAxiMst0::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U0PcieAxiMst0::Reset
    }
}
#[doc = "Field `u0_pcie_axi_mst0` writer - U0 PCIE AXI MST0 reset."]
pub type U0PcieAxiMst0W<'a, REG> = crate::BitWriter<'a, REG, U0PcieAxiMst0>;
impl<'a, REG> U0PcieAxiMst0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U0PcieAxiMst0::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U0PcieAxiMst0::Reset)
    }
}
#[doc = "U0 PCIE AXI SLV0 reset.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U0PcieAxiSlv0 {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U0PcieAxiSlv0> for bool {
    #[inline(always)]
    fn from(variant: U0PcieAxiSlv0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u0_pcie_axi_slv0` reader - U0 PCIE AXI SLV0 reset."]
pub type U0PcieAxiSlv0R = crate::BitReader<U0PcieAxiSlv0>;
impl U0PcieAxiSlv0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U0PcieAxiSlv0 {
        match self.bits {
            false => U0PcieAxiSlv0::None,
            true => U0PcieAxiSlv0::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U0PcieAxiSlv0::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U0PcieAxiSlv0::Reset
    }
}
#[doc = "Field `u0_pcie_axi_slv0` writer - U0 PCIE AXI SLV0 reset."]
pub type U0PcieAxiSlv0W<'a, REG> = crate::BitWriter<'a, REG, U0PcieAxiSlv0>;
impl<'a, REG> U0PcieAxiSlv0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U0PcieAxiSlv0::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U0PcieAxiSlv0::Reset)
    }
}
#[doc = "U0 PCIE AXI SLV reset.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U0PcieAxiSlv {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U0PcieAxiSlv> for bool {
    #[inline(always)]
    fn from(variant: U0PcieAxiSlv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u0_pcie_axi_slv` reader - U0 PCIE AXI SLV reset."]
pub type U0PcieAxiSlvR = crate::BitReader<U0PcieAxiSlv>;
impl U0PcieAxiSlvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U0PcieAxiSlv {
        match self.bits {
            false => U0PcieAxiSlv::None,
            true => U0PcieAxiSlv::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U0PcieAxiSlv::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U0PcieAxiSlv::Reset
    }
}
#[doc = "Field `u0_pcie_axi_slv` writer - U0 PCIE AXI SLV reset."]
pub type U0PcieAxiSlvW<'a, REG> = crate::BitWriter<'a, REG, U0PcieAxiSlv>;
impl<'a, REG> U0PcieAxiSlvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U0PcieAxiSlv::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U0PcieAxiSlv::Reset)
    }
}
#[doc = "U0 PCI BRG reset.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U0PciBrg {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U0PciBrg> for bool {
    #[inline(always)]
    fn from(variant: U0PciBrg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u0_pci_brg` reader - U0 PCI BRG reset."]
pub type U0PciBrgR = crate::BitReader<U0PciBrg>;
impl U0PciBrgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U0PciBrg {
        match self.bits {
            false => U0PciBrg::None,
            true => U0PciBrg::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U0PciBrg::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U0PciBrg::Reset
    }
}
#[doc = "Field `u0_pci_brg` writer - U0 PCI BRG reset."]
pub type U0PciBrgW<'a, REG> = crate::BitWriter<'a, REG, U0PciBrg>;
impl<'a, REG> U0PciBrgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U0PciBrg::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U0PciBrg::Reset)
    }
}
#[doc = "U0 PCIE main reset.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U0PciePcie {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U0PciePcie> for bool {
    #[inline(always)]
    fn from(variant: U0PciePcie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u0_pcie_pcie` reader - U0 PCIE main reset."]
pub type U0PciePcieR = crate::BitReader<U0PciePcie>;
impl U0PciePcieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U0PciePcie {
        match self.bits {
            false => U0PciePcie::None,
            true => U0PciePcie::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U0PciePcie::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U0PciePcie::Reset
    }
}
#[doc = "Field `u0_pcie_pcie` writer - U0 PCIE main reset."]
pub type U0PciePcieW<'a, REG> = crate::BitWriter<'a, REG, U0PciePcie>;
impl<'a, REG> U0PciePcieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U0PciePcie::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U0PciePcie::Reset)
    }
}
#[doc = "U0 PCIE APB reset.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U0PcieApb {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U0PcieApb> for bool {
    #[inline(always)]
    fn from(variant: U0PcieApb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u0_pcie_apb` reader - U0 PCIE APB reset."]
pub type U0PcieApbR = crate::BitReader<U0PcieApb>;
impl U0PcieApbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U0PcieApb {
        match self.bits {
            false => U0PcieApb::None,
            true => U0PcieApb::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U0PcieApb::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U0PcieApb::Reset
    }
}
#[doc = "Field `u0_pcie_apb` writer - U0 PCIE APB reset."]
pub type U0PcieApbW<'a, REG> = crate::BitWriter<'a, REG, U0PcieApb>;
impl<'a, REG> U0PcieApbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U0PcieApb::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U0PcieApb::Reset)
    }
}
#[doc = "U1 PCIE AXI MST0 reset.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U1PcieAxiMst0 {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U1PcieAxiMst0> for bool {
    #[inline(always)]
    fn from(variant: U1PcieAxiMst0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u1_pcie_axi_mst0` reader - U1 PCIE AXI MST0 reset."]
pub type U1PcieAxiMst0R = crate::BitReader<U1PcieAxiMst0>;
impl U1PcieAxiMst0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U1PcieAxiMst0 {
        match self.bits {
            false => U1PcieAxiMst0::None,
            true => U1PcieAxiMst0::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U1PcieAxiMst0::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U1PcieAxiMst0::Reset
    }
}
#[doc = "Field `u1_pcie_axi_mst0` writer - U1 PCIE AXI MST0 reset."]
pub type U1PcieAxiMst0W<'a, REG> = crate::BitWriter<'a, REG, U1PcieAxiMst0>;
impl<'a, REG> U1PcieAxiMst0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U1PcieAxiMst0::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U1PcieAxiMst0::Reset)
    }
}
#[doc = "U1 PCIE AXI SLV0 reset.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U1PcieAxiSlv0 {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U1PcieAxiSlv0> for bool {
    #[inline(always)]
    fn from(variant: U1PcieAxiSlv0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u1_pcie_axi_slv0` reader - U1 PCIE AXI SLV0 reset."]
pub type U1PcieAxiSlv0R = crate::BitReader<U1PcieAxiSlv0>;
impl U1PcieAxiSlv0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U1PcieAxiSlv0 {
        match self.bits {
            false => U1PcieAxiSlv0::None,
            true => U1PcieAxiSlv0::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U1PcieAxiSlv0::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U1PcieAxiSlv0::Reset
    }
}
#[doc = "Field `u1_pcie_axi_slv0` writer - U1 PCIE AXI SLV0 reset."]
pub type U1PcieAxiSlv0W<'a, REG> = crate::BitWriter<'a, REG, U1PcieAxiSlv0>;
impl<'a, REG> U1PcieAxiSlv0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U1PcieAxiSlv0::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U1PcieAxiSlv0::Reset)
    }
}
#[doc = "U1 PCIE AXI SLV reset.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U1PcieAxiSlv {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U1PcieAxiSlv> for bool {
    #[inline(always)]
    fn from(variant: U1PcieAxiSlv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u1_pcie_axi_slv` reader - U1 PCIE AXI SLV reset."]
pub type U1PcieAxiSlvR = crate::BitReader<U1PcieAxiSlv>;
impl U1PcieAxiSlvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U1PcieAxiSlv {
        match self.bits {
            false => U1PcieAxiSlv::None,
            true => U1PcieAxiSlv::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U1PcieAxiSlv::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U1PcieAxiSlv::Reset
    }
}
#[doc = "Field `u1_pcie_axi_slv` writer - U1 PCIE AXI SLV reset."]
pub type U1PcieAxiSlvW<'a, REG> = crate::BitWriter<'a, REG, U1PcieAxiSlv>;
impl<'a, REG> U1PcieAxiSlvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U1PcieAxiSlv::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U1PcieAxiSlv::Reset)
    }
}
#[doc = "U1 PCIE BRG reset.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U1PcieBrg {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U1PcieBrg> for bool {
    #[inline(always)]
    fn from(variant: U1PcieBrg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u1_pcie_brg` reader - U1 PCIE BRG reset."]
pub type U1PcieBrgR = crate::BitReader<U1PcieBrg>;
impl U1PcieBrgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U1PcieBrg {
        match self.bits {
            false => U1PcieBrg::None,
            true => U1PcieBrg::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U1PcieBrg::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U1PcieBrg::Reset
    }
}
#[doc = "Field `u1_pcie_brg` writer - U1 PCIE BRG reset."]
pub type U1PcieBrgW<'a, REG> = crate::BitWriter<'a, REG, U1PcieBrg>;
impl<'a, REG> U1PcieBrgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U1PcieBrg::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U1PcieBrg::Reset)
    }
}
#[doc = "U1 PCIE main reset.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U1PciePcie {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U1PciePcie> for bool {
    #[inline(always)]
    fn from(variant: U1PciePcie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u1_pcie_pcie` reader - U1 PCIE main reset."]
pub type U1PciePcieR = crate::BitReader<U1PciePcie>;
impl U1PciePcieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U1PciePcie {
        match self.bits {
            false => U1PciePcie::None,
            true => U1PciePcie::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U1PciePcie::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U1PciePcie::Reset
    }
}
#[doc = "Field `u1_pcie_pcie` writer - U1 PCIE main reset."]
pub type U1PciePcieW<'a, REG> = crate::BitWriter<'a, REG, U1PciePcie>;
impl<'a, REG> U1PciePcieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U1PciePcie::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U1PciePcie::Reset)
    }
}
#[doc = "U1 PCIE APB reset.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U1PcieApb {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U1PcieApb> for bool {
    #[inline(always)]
    fn from(variant: U1PcieApb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u1_pcie_apb` reader - U1 PCIE APB reset."]
pub type U1PcieApbR = crate::BitReader<U1PcieApb>;
impl U1PcieApbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U1PcieApb {
        match self.bits {
            false => U1PcieApb::None,
            true => U1PcieApb::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U1PcieApb::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U1PcieApb::Reset
    }
}
#[doc = "Field `u1_pcie_apb` writer - U1 PCIE APB reset."]
pub type U1PcieApbW<'a, REG> = crate::BitWriter<'a, REG, U1PcieApb>;
impl<'a, REG> U1PcieApbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U1PcieApb::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U1PcieApb::Reset)
    }
}
impl R {
    #[doc = "Bit 0 - U0 STG SYSCON Presetn reset."]
    #[inline(always)]
    pub fn u0_stg_syscon_presetn(&self) -> U0StgSysconPresetnR {
        U0StgSysconPresetnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - U0 HIFI4 Core reset."]
    #[inline(always)]
    pub fn u0_hifi4_core(&self) -> U0Hifi4CoreR {
        U0Hifi4CoreR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - U0 HIFI4 AXI reset."]
    #[inline(always)]
    pub fn u0_hifi4_axi(&self) -> U0Hifi4AxiR {
        U0Hifi4AxiR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - U0 SEC Top HResetn reset."]
    #[inline(always)]
    pub fn u0_sec_top_hresetn(&self) -> U0SecTopHresetnR {
        U0SecTopHresetnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - U0 E2 Core reset."]
    #[inline(always)]
    pub fn u0_e2_core(&self) -> U0E2CoreR {
        U0E2CoreR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - U0 DMA AXI reset."]
    #[inline(always)]
    pub fn u0_dma_axi(&self) -> U0DmaAxiR {
        U0DmaAxiR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - U0 DMA AHB reset."]
    #[inline(always)]
    pub fn u0_dma_ahb(&self) -> U0DmaAhbR {
        U0DmaAhbR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - U0 USB AXI reset."]
    #[inline(always)]
    pub fn u0_usb_axi(&self) -> U0UsbAxiR {
        U0UsbAxiR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - U0 USB APB reset."]
    #[inline(always)]
    pub fn u0_usb_apb(&self) -> U0UsbApbR {
        U0UsbApbR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - U0 USB UTMI APB reset."]
    #[inline(always)]
    pub fn u0_usb_utmi_apb(&self) -> U0UsbUtmiApbR {
        U0UsbUtmiApbR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - U0 USB Power-up reset."]
    #[inline(always)]
    pub fn u0_usb_pwrup(&self) -> U0UsbPwrupR {
        U0UsbPwrupR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - U0 PCIE AXI MST0 reset."]
    #[inline(always)]
    pub fn u0_pcie_axi_mst0(&self) -> U0PcieAxiMst0R {
        U0PcieAxiMst0R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - U0 PCIE AXI SLV0 reset."]
    #[inline(always)]
    pub fn u0_pcie_axi_slv0(&self) -> U0PcieAxiSlv0R {
        U0PcieAxiSlv0R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - U0 PCIE AXI SLV reset."]
    #[inline(always)]
    pub fn u0_pcie_axi_slv(&self) -> U0PcieAxiSlvR {
        U0PcieAxiSlvR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - U0 PCI BRG reset."]
    #[inline(always)]
    pub fn u0_pci_brg(&self) -> U0PciBrgR {
        U0PciBrgR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - U0 PCIE main reset."]
    #[inline(always)]
    pub fn u0_pcie_pcie(&self) -> U0PciePcieR {
        U0PciePcieR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - U0 PCIE APB reset."]
    #[inline(always)]
    pub fn u0_pcie_apb(&self) -> U0PcieApbR {
        U0PcieApbR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - U1 PCIE AXI MST0 reset."]
    #[inline(always)]
    pub fn u1_pcie_axi_mst0(&self) -> U1PcieAxiMst0R {
        U1PcieAxiMst0R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - U1 PCIE AXI SLV0 reset."]
    #[inline(always)]
    pub fn u1_pcie_axi_slv0(&self) -> U1PcieAxiSlv0R {
        U1PcieAxiSlv0R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - U1 PCIE AXI SLV reset."]
    #[inline(always)]
    pub fn u1_pcie_axi_slv(&self) -> U1PcieAxiSlvR {
        U1PcieAxiSlvR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - U1 PCIE BRG reset."]
    #[inline(always)]
    pub fn u1_pcie_brg(&self) -> U1PcieBrgR {
        U1PcieBrgR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - U1 PCIE main reset."]
    #[inline(always)]
    pub fn u1_pcie_pcie(&self) -> U1PciePcieR {
        U1PciePcieR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - U1 PCIE APB reset."]
    #[inline(always)]
    pub fn u1_pcie_apb(&self) -> U1PcieApbR {
        U1PcieApbR::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - U0 STG SYSCON Presetn reset."]
    #[inline(always)]
    #[must_use]
    pub fn u0_stg_syscon_presetn(&mut self) -> U0StgSysconPresetnW<RstSpec> {
        U0StgSysconPresetnW::new(self, 0)
    }
    #[doc = "Bit 1 - U0 HIFI4 Core reset."]
    #[inline(always)]
    #[must_use]
    pub fn u0_hifi4_core(&mut self) -> U0Hifi4CoreW<RstSpec> {
        U0Hifi4CoreW::new(self, 1)
    }
    #[doc = "Bit 2 - U0 HIFI4 AXI reset."]
    #[inline(always)]
    #[must_use]
    pub fn u0_hifi4_axi(&mut self) -> U0Hifi4AxiW<RstSpec> {
        U0Hifi4AxiW::new(self, 2)
    }
    #[doc = "Bit 3 - U0 SEC Top HResetn reset."]
    #[inline(always)]
    #[must_use]
    pub fn u0_sec_top_hresetn(&mut self) -> U0SecTopHresetnW<RstSpec> {
        U0SecTopHresetnW::new(self, 3)
    }
    #[doc = "Bit 4 - U0 E2 Core reset."]
    #[inline(always)]
    #[must_use]
    pub fn u0_e2_core(&mut self) -> U0E2CoreW<RstSpec> {
        U0E2CoreW::new(self, 4)
    }
    #[doc = "Bit 5 - U0 DMA AXI reset."]
    #[inline(always)]
    #[must_use]
    pub fn u0_dma_axi(&mut self) -> U0DmaAxiW<RstSpec> {
        U0DmaAxiW::new(self, 5)
    }
    #[doc = "Bit 6 - U0 DMA AHB reset."]
    #[inline(always)]
    #[must_use]
    pub fn u0_dma_ahb(&mut self) -> U0DmaAhbW<RstSpec> {
        U0DmaAhbW::new(self, 6)
    }
    #[doc = "Bit 7 - U0 USB AXI reset."]
    #[inline(always)]
    #[must_use]
    pub fn u0_usb_axi(&mut self) -> U0UsbAxiW<RstSpec> {
        U0UsbAxiW::new(self, 7)
    }
    #[doc = "Bit 8 - U0 USB APB reset."]
    #[inline(always)]
    #[must_use]
    pub fn u0_usb_apb(&mut self) -> U0UsbApbW<RstSpec> {
        U0UsbApbW::new(self, 8)
    }
    #[doc = "Bit 9 - U0 USB UTMI APB reset."]
    #[inline(always)]
    #[must_use]
    pub fn u0_usb_utmi_apb(&mut self) -> U0UsbUtmiApbW<RstSpec> {
        U0UsbUtmiApbW::new(self, 9)
    }
    #[doc = "Bit 10 - U0 USB Power-up reset."]
    #[inline(always)]
    #[must_use]
    pub fn u0_usb_pwrup(&mut self) -> U0UsbPwrupW<RstSpec> {
        U0UsbPwrupW::new(self, 10)
    }
    #[doc = "Bit 11 - U0 PCIE AXI MST0 reset."]
    #[inline(always)]
    #[must_use]
    pub fn u0_pcie_axi_mst0(&mut self) -> U0PcieAxiMst0W<RstSpec> {
        U0PcieAxiMst0W::new(self, 11)
    }
    #[doc = "Bit 12 - U0 PCIE AXI SLV0 reset."]
    #[inline(always)]
    #[must_use]
    pub fn u0_pcie_axi_slv0(&mut self) -> U0PcieAxiSlv0W<RstSpec> {
        U0PcieAxiSlv0W::new(self, 12)
    }
    #[doc = "Bit 13 - U0 PCIE AXI SLV reset."]
    #[inline(always)]
    #[must_use]
    pub fn u0_pcie_axi_slv(&mut self) -> U0PcieAxiSlvW<RstSpec> {
        U0PcieAxiSlvW::new(self, 13)
    }
    #[doc = "Bit 14 - U0 PCI BRG reset."]
    #[inline(always)]
    #[must_use]
    pub fn u0_pci_brg(&mut self) -> U0PciBrgW<RstSpec> {
        U0PciBrgW::new(self, 14)
    }
    #[doc = "Bit 15 - U0 PCIE main reset."]
    #[inline(always)]
    #[must_use]
    pub fn u0_pcie_pcie(&mut self) -> U0PciePcieW<RstSpec> {
        U0PciePcieW::new(self, 15)
    }
    #[doc = "Bit 16 - U0 PCIE APB reset."]
    #[inline(always)]
    #[must_use]
    pub fn u0_pcie_apb(&mut self) -> U0PcieApbW<RstSpec> {
        U0PcieApbW::new(self, 16)
    }
    #[doc = "Bit 17 - U1 PCIE AXI MST0 reset."]
    #[inline(always)]
    #[must_use]
    pub fn u1_pcie_axi_mst0(&mut self) -> U1PcieAxiMst0W<RstSpec> {
        U1PcieAxiMst0W::new(self, 17)
    }
    #[doc = "Bit 18 - U1 PCIE AXI SLV0 reset."]
    #[inline(always)]
    #[must_use]
    pub fn u1_pcie_axi_slv0(&mut self) -> U1PcieAxiSlv0W<RstSpec> {
        U1PcieAxiSlv0W::new(self, 18)
    }
    #[doc = "Bit 19 - U1 PCIE AXI SLV reset."]
    #[inline(always)]
    #[must_use]
    pub fn u1_pcie_axi_slv(&mut self) -> U1PcieAxiSlvW<RstSpec> {
        U1PcieAxiSlvW::new(self, 19)
    }
    #[doc = "Bit 20 - U1 PCIE BRG reset."]
    #[inline(always)]
    #[must_use]
    pub fn u1_pcie_brg(&mut self) -> U1PcieBrgW<RstSpec> {
        U1PcieBrgW::new(self, 20)
    }
    #[doc = "Bit 21 - U1 PCIE main reset."]
    #[inline(always)]
    #[must_use]
    pub fn u1_pcie_pcie(&mut self) -> U1PciePcieW<RstSpec> {
        U1PciePcieW::new(self, 21)
    }
    #[doc = "Bit 22 - U1 PCIE APB reset."]
    #[inline(always)]
    #[must_use]
    pub fn u1_pcie_apb(&mut self) -> U1PcieApbW<RstSpec> {
        U1PcieApbW::new(self, 22)
    }
}
#[doc = "STGCRG RESET\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rst::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rst::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RstSpec;
impl crate::RegisterSpec for RstSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rst::R`](R) reader structure"]
impl crate::Readable for RstSpec {}
#[doc = "`write(|w| ..)` method takes [`rst::W`](W) writer structure"]
impl crate::Writable for RstSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets rst[%s]
to value 0x007f_fffe"]
impl crate::Resettable for RstSpec {
    const RESET_VALUE: u32 = 0x007f_fffe;
}
