#[doc = "Register `rst3` reader"]
pub type R = crate::R<Rst3Spec>;
#[doc = "Register `rst3` writer"]
pub type W = crate::W<Rst3Spec>;
#[doc = "Reset selector: u0_pwmdac_apb\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U0PwmdacApb {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U0PwmdacApb> for bool {
    #[inline(always)]
    fn from(variant: U0PwmdacApb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u0_pwmdac_apb` reader - Reset selector: u0_pwmdac_apb"]
pub type U0PwmdacApbR = crate::BitReader<U0PwmdacApb>;
impl U0PwmdacApbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U0PwmdacApb {
        match self.bits {
            false => U0PwmdacApb::None,
            true => U0PwmdacApb::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U0PwmdacApb::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U0PwmdacApb::Reset
    }
}
#[doc = "Field `u0_pwmdac_apb` writer - Reset selector: u0_pwmdac_apb"]
pub type U0PwmdacApbW<'a, REG> = crate::BitWriter<'a, REG, U0PwmdacApb>;
impl<'a, REG> U0PwmdacApbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U0PwmdacApb::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U0PwmdacApb::Reset)
    }
}
#[doc = "Reset selector: u0_pdm_4mic_dmic\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U0Pdm4micDmic {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U0Pdm4micDmic> for bool {
    #[inline(always)]
    fn from(variant: U0Pdm4micDmic) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u0_pdm_4mic_dmic` reader - Reset selector: u0_pdm_4mic_dmic"]
pub type U0Pdm4micDmicR = crate::BitReader<U0Pdm4micDmic>;
impl U0Pdm4micDmicR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U0Pdm4micDmic {
        match self.bits {
            false => U0Pdm4micDmic::None,
            true => U0Pdm4micDmic::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U0Pdm4micDmic::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U0Pdm4micDmic::Reset
    }
}
#[doc = "Field `u0_pdm_4mic_dmic` writer - Reset selector: u0_pdm_4mic_dmic"]
pub type U0Pdm4micDmicW<'a, REG> = crate::BitWriter<'a, REG, U0Pdm4micDmic>;
impl<'a, REG> U0Pdm4micDmicW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U0Pdm4micDmic::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U0Pdm4micDmic::Reset)
    }
}
#[doc = "Reset selector: u0_pdm_4mic_apb\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U0Pdm4micApb {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U0Pdm4micApb> for bool {
    #[inline(always)]
    fn from(variant: U0Pdm4micApb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u0_pdm_4mic_apb` reader - Reset selector: u0_pdm_4mic_apb"]
pub type U0Pdm4micApbR = crate::BitReader<U0Pdm4micApb>;
impl U0Pdm4micApbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U0Pdm4micApb {
        match self.bits {
            false => U0Pdm4micApb::None,
            true => U0Pdm4micApb::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U0Pdm4micApb::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U0Pdm4micApb::Reset
    }
}
#[doc = "Field `u0_pdm_4mic_apb` writer - Reset selector: u0_pdm_4mic_apb"]
pub type U0Pdm4micApbW<'a, REG> = crate::BitWriter<'a, REG, U0Pdm4micApb>;
impl<'a, REG> U0Pdm4micApbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U0Pdm4micApb::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U0Pdm4micApb::Reset)
    }
}
#[doc = "Reset selector: u0_i2srx_apb\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U0I2srxApb {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U0I2srxApb> for bool {
    #[inline(always)]
    fn from(variant: U0I2srxApb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u0_i2srx_apb` reader - Reset selector: u0_i2srx_apb"]
pub type U0I2srxApbR = crate::BitReader<U0I2srxApb>;
impl U0I2srxApbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U0I2srxApb {
        match self.bits {
            false => U0I2srxApb::None,
            true => U0I2srxApb::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U0I2srxApb::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U0I2srxApb::Reset
    }
}
#[doc = "Field `u0_i2srx_apb` writer - Reset selector: u0_i2srx_apb"]
pub type U0I2srxApbW<'a, REG> = crate::BitWriter<'a, REG, U0I2srxApb>;
impl<'a, REG> U0I2srxApbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U0I2srxApb::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U0I2srxApb::Reset)
    }
}
#[doc = "Reset selector: u0_i2srx_bclk\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U0I2srxBclk {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U0I2srxBclk> for bool {
    #[inline(always)]
    fn from(variant: U0I2srxBclk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u0_i2srx_bclk` reader - Reset selector: u0_i2srx_bclk"]
pub type U0I2srxBclkR = crate::BitReader<U0I2srxBclk>;
impl U0I2srxBclkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U0I2srxBclk {
        match self.bits {
            false => U0I2srxBclk::None,
            true => U0I2srxBclk::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U0I2srxBclk::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U0I2srxBclk::Reset
    }
}
#[doc = "Field `u0_i2srx_bclk` writer - Reset selector: u0_i2srx_bclk"]
pub type U0I2srxBclkW<'a, REG> = crate::BitWriter<'a, REG, U0I2srxBclk>;
impl<'a, REG> U0I2srxBclkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U0I2srxBclk::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U0I2srxBclk::Reset)
    }
}
#[doc = "Reset selector: u0_i2stx_apb\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U0I2stxApb {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U0I2stxApb> for bool {
    #[inline(always)]
    fn from(variant: U0I2stxApb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u0_i2stx_apb` reader - Reset selector: u0_i2stx_apb"]
pub type U0I2stxApbR = crate::BitReader<U0I2stxApb>;
impl U0I2stxApbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U0I2stxApb {
        match self.bits {
            false => U0I2stxApb::None,
            true => U0I2stxApb::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U0I2stxApb::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U0I2stxApb::Reset
    }
}
#[doc = "Field `u0_i2stx_apb` writer - Reset selector: u0_i2stx_apb"]
pub type U0I2stxApbW<'a, REG> = crate::BitWriter<'a, REG, U0I2stxApb>;
impl<'a, REG> U0I2stxApbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U0I2stxApb::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U0I2stxApb::Reset)
    }
}
#[doc = "Reset selector: u0_i2stx_bclk\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U0I2stxBclk {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U0I2stxBclk> for bool {
    #[inline(always)]
    fn from(variant: U0I2stxBclk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u0_i2stx_bclk` reader - Reset selector: u0_i2stx_bclk"]
pub type U0I2stxBclkR = crate::BitReader<U0I2stxBclk>;
impl U0I2stxBclkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U0I2stxBclk {
        match self.bits {
            false => U0I2stxBclk::None,
            true => U0I2stxBclk::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U0I2stxBclk::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U0I2stxBclk::Reset
    }
}
#[doc = "Field `u0_i2stx_bclk` writer - Reset selector: u0_i2stx_bclk"]
pub type U0I2stxBclkW<'a, REG> = crate::BitWriter<'a, REG, U0I2stxBclk>;
impl<'a, REG> U0I2stxBclkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U0I2stxBclk::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U0I2stxBclk::Reset)
    }
}
#[doc = "Reset selector: u1_i2stx_apb\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U1I2stxApb {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U1I2stxApb> for bool {
    #[inline(always)]
    fn from(variant: U1I2stxApb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u1_i2stx_apb` reader - Reset selector: u1_i2stx_apb"]
pub type U1I2stxApbR = crate::BitReader<U1I2stxApb>;
impl U1I2stxApbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U1I2stxApb {
        match self.bits {
            false => U1I2stxApb::None,
            true => U1I2stxApb::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U1I2stxApb::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U1I2stxApb::Reset
    }
}
#[doc = "Field `u1_i2stx_apb` writer - Reset selector: u1_i2stx_apb"]
pub type U1I2stxApbW<'a, REG> = crate::BitWriter<'a, REG, U1I2stxApb>;
impl<'a, REG> U1I2stxApbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U1I2stxApb::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U1I2stxApb::Reset)
    }
}
#[doc = "Reset selector: u1_i2stx_bclk\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U1I2stxBclk {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U1I2stxBclk> for bool {
    #[inline(always)]
    fn from(variant: U1I2stxBclk) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u1_i2stx_bclk` reader - Reset selector: u1_i2stx_bclk"]
pub type U1I2stxBclkR = crate::BitReader<U1I2stxBclk>;
impl U1I2stxBclkR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U1I2stxBclk {
        match self.bits {
            false => U1I2stxBclk::None,
            true => U1I2stxBclk::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U1I2stxBclk::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U1I2stxBclk::Reset
    }
}
#[doc = "Field `u1_i2stx_bclk` writer - Reset selector: u1_i2stx_bclk"]
pub type U1I2stxBclkW<'a, REG> = crate::BitWriter<'a, REG, U1I2stxBclk>;
impl<'a, REG> U1I2stxBclkW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U1I2stxBclk::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U1I2stxBclk::Reset)
    }
}
#[doc = "Reset selector: u0_tdm16slot_ahb\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U0Tdm16slotAhb {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U0Tdm16slotAhb> for bool {
    #[inline(always)]
    fn from(variant: U0Tdm16slotAhb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u0_tdm16slot_ahb` reader - Reset selector: u0_tdm16slot_ahb"]
pub type U0Tdm16slotAhbR = crate::BitReader<U0Tdm16slotAhb>;
impl U0Tdm16slotAhbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U0Tdm16slotAhb {
        match self.bits {
            false => U0Tdm16slotAhb::None,
            true => U0Tdm16slotAhb::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U0Tdm16slotAhb::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U0Tdm16slotAhb::Reset
    }
}
#[doc = "Field `u0_tdm16slot_ahb` writer - Reset selector: u0_tdm16slot_ahb"]
pub type U0Tdm16slotAhbW<'a, REG> = crate::BitWriter<'a, REG, U0Tdm16slotAhb>;
impl<'a, REG> U0Tdm16slotAhbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U0Tdm16slotAhb::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U0Tdm16slotAhb::Reset)
    }
}
#[doc = "Reset selector: u0_tdm16slot_tdm\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U0Tdm16slotTdm {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U0Tdm16slotTdm> for bool {
    #[inline(always)]
    fn from(variant: U0Tdm16slotTdm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u0_tdm16slot_tdm` reader - Reset selector: u0_tdm16slot_tdm"]
pub type U0Tdm16slotTdmR = crate::BitReader<U0Tdm16slotTdm>;
impl U0Tdm16slotTdmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U0Tdm16slotTdm {
        match self.bits {
            false => U0Tdm16slotTdm::None,
            true => U0Tdm16slotTdm::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U0Tdm16slotTdm::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U0Tdm16slotTdm::Reset
    }
}
#[doc = "Field `u0_tdm16slot_tdm` writer - Reset selector: u0_tdm16slot_tdm"]
pub type U0Tdm16slotTdmW<'a, REG> = crate::BitWriter<'a, REG, U0Tdm16slotTdm>;
impl<'a, REG> U0Tdm16slotTdmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U0Tdm16slotTdm::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U0Tdm16slotTdm::Reset)
    }
}
#[doc = "Reset selector: u0_tdm16slot_apb\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U0Tdm16slotApb {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U0Tdm16slotApb> for bool {
    #[inline(always)]
    fn from(variant: U0Tdm16slotApb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u0_tdm16slot_apb` reader - Reset selector: u0_tdm16slot_apb"]
pub type U0Tdm16slotApbR = crate::BitReader<U0Tdm16slotApb>;
impl U0Tdm16slotApbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U0Tdm16slotApb {
        match self.bits {
            false => U0Tdm16slotApb::None,
            true => U0Tdm16slotApb::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U0Tdm16slotApb::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U0Tdm16slotApb::Reset
    }
}
#[doc = "Field `u0_tdm16slot_apb` writer - Reset selector: u0_tdm16slot_apb"]
pub type U0Tdm16slotApbW<'a, REG> = crate::BitWriter<'a, REG, U0Tdm16slotApb>;
impl<'a, REG> U0Tdm16slotApbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U0Tdm16slotApb::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U0Tdm16slotApb::Reset)
    }
}
#[doc = "Reset selector: u0_pwm_apb\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U0PwmApb {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U0PwmApb> for bool {
    #[inline(always)]
    fn from(variant: U0PwmApb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u0_pwm_apb` reader - Reset selector: u0_pwm_apb"]
pub type U0PwmApbR = crate::BitReader<U0PwmApb>;
impl U0PwmApbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U0PwmApb {
        match self.bits {
            false => U0PwmApb::None,
            true => U0PwmApb::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U0PwmApb::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U0PwmApb::Reset
    }
}
#[doc = "Field `u0_pwm_apb` writer - Reset selector: u0_pwm_apb"]
pub type U0PwmApbW<'a, REG> = crate::BitWriter<'a, REG, U0PwmApb>;
impl<'a, REG> U0PwmApbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U0PwmApb::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U0PwmApb::Reset)
    }
}
#[doc = "Reset selector: u0_dskit_wdt_rstn_apb\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U0DskitWdtRstnApb {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U0DskitWdtRstnApb> for bool {
    #[inline(always)]
    fn from(variant: U0DskitWdtRstnApb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u0_dskit_wdt_rstn_apb` reader - Reset selector: u0_dskit_wdt_rstn_apb"]
pub type U0DskitWdtRstnApbR = crate::BitReader<U0DskitWdtRstnApb>;
impl U0DskitWdtRstnApbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U0DskitWdtRstnApb {
        match self.bits {
            false => U0DskitWdtRstnApb::None,
            true => U0DskitWdtRstnApb::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U0DskitWdtRstnApb::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U0DskitWdtRstnApb::Reset
    }
}
#[doc = "Field `u0_dskit_wdt_rstn_apb` writer - Reset selector: u0_dskit_wdt_rstn_apb"]
pub type U0DskitWdtRstnApbW<'a, REG> = crate::BitWriter<'a, REG, U0DskitWdtRstnApb>;
impl<'a, REG> U0DskitWdtRstnApbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U0DskitWdtRstnApb::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U0DskitWdtRstnApb::Reset)
    }
}
#[doc = "Reset selector: u0_dskit_wdt\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U0DskitWdt {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U0DskitWdt> for bool {
    #[inline(always)]
    fn from(variant: U0DskitWdt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u0_dskit_wdt` reader - Reset selector: u0_dskit_wdt"]
pub type U0DskitWdtR = crate::BitReader<U0DskitWdt>;
impl U0DskitWdtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U0DskitWdt {
        match self.bits {
            false => U0DskitWdt::None,
            true => U0DskitWdt::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U0DskitWdt::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U0DskitWdt::Reset
    }
}
#[doc = "Field `u0_dskit_wdt` writer - Reset selector: u0_dskit_wdt"]
pub type U0DskitWdtW<'a, REG> = crate::BitWriter<'a, REG, U0DskitWdt>;
impl<'a, REG> U0DskitWdtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U0DskitWdt::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U0DskitWdt::Reset)
    }
}
#[doc = "Reset selector: u0_can_ctrl_apb\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U0CanCtrlApb {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U0CanCtrlApb> for bool {
    #[inline(always)]
    fn from(variant: U0CanCtrlApb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u0_can_ctrl_apb` reader - Reset selector: u0_can_ctrl_apb"]
pub type U0CanCtrlApbR = crate::BitReader<U0CanCtrlApb>;
impl U0CanCtrlApbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U0CanCtrlApb {
        match self.bits {
            false => U0CanCtrlApb::None,
            true => U0CanCtrlApb::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U0CanCtrlApb::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U0CanCtrlApb::Reset
    }
}
#[doc = "Field `u0_can_ctrl_apb` writer - Reset selector: u0_can_ctrl_apb"]
pub type U0CanCtrlApbW<'a, REG> = crate::BitWriter<'a, REG, U0CanCtrlApb>;
impl<'a, REG> U0CanCtrlApbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U0CanCtrlApb::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U0CanCtrlApb::Reset)
    }
}
#[doc = "Reset selector: u0_can_ctrl\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U0CanCtrl {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U0CanCtrl> for bool {
    #[inline(always)]
    fn from(variant: U0CanCtrl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u0_can_ctrl` reader - Reset selector: u0_can_ctrl"]
pub type U0CanCtrlR = crate::BitReader<U0CanCtrl>;
impl U0CanCtrlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U0CanCtrl {
        match self.bits {
            false => U0CanCtrl::None,
            true => U0CanCtrl::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U0CanCtrl::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U0CanCtrl::Reset
    }
}
#[doc = "Field `u0_can_ctrl` writer - Reset selector: u0_can_ctrl"]
pub type U0CanCtrlW<'a, REG> = crate::BitWriter<'a, REG, U0CanCtrl>;
impl<'a, REG> U0CanCtrlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U0CanCtrl::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U0CanCtrl::Reset)
    }
}
#[doc = "Reset selector: u0_can_ctrl_timer\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U0CanCtrlTimer {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U0CanCtrlTimer> for bool {
    #[inline(always)]
    fn from(variant: U0CanCtrlTimer) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u0_can_ctrl_timer` reader - Reset selector: u0_can_ctrl_timer"]
pub type U0CanCtrlTimerR = crate::BitReader<U0CanCtrlTimer>;
impl U0CanCtrlTimerR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U0CanCtrlTimer {
        match self.bits {
            false => U0CanCtrlTimer::None,
            true => U0CanCtrlTimer::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U0CanCtrlTimer::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U0CanCtrlTimer::Reset
    }
}
#[doc = "Field `u0_can_ctrl_timer` writer - Reset selector: u0_can_ctrl_timer"]
pub type U0CanCtrlTimerW<'a, REG> = crate::BitWriter<'a, REG, U0CanCtrlTimer>;
impl<'a, REG> U0CanCtrlTimerW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U0CanCtrlTimer::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U0CanCtrlTimer::Reset)
    }
}
#[doc = "Reset selector: u1_can_ctrl_apb\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U1CanCtrlApb {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U1CanCtrlApb> for bool {
    #[inline(always)]
    fn from(variant: U1CanCtrlApb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u1_can_ctrl_apb` reader - Reset selector: u1_can_ctrl_apb"]
pub type U1CanCtrlApbR = crate::BitReader<U1CanCtrlApb>;
impl U1CanCtrlApbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U1CanCtrlApb {
        match self.bits {
            false => U1CanCtrlApb::None,
            true => U1CanCtrlApb::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U1CanCtrlApb::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U1CanCtrlApb::Reset
    }
}
#[doc = "Field `u1_can_ctrl_apb` writer - Reset selector: u1_can_ctrl_apb"]
pub type U1CanCtrlApbW<'a, REG> = crate::BitWriter<'a, REG, U1CanCtrlApb>;
impl<'a, REG> U1CanCtrlApbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U1CanCtrlApb::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U1CanCtrlApb::Reset)
    }
}
#[doc = "Reset selector: u1_can_ctrl_can\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U1CanCtrlCan {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U1CanCtrlCan> for bool {
    #[inline(always)]
    fn from(variant: U1CanCtrlCan) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u1_can_ctrl_can` reader - Reset selector: u1_can_ctrl_can"]
pub type U1CanCtrlCanR = crate::BitReader<U1CanCtrlCan>;
impl U1CanCtrlCanR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U1CanCtrlCan {
        match self.bits {
            false => U1CanCtrlCan::None,
            true => U1CanCtrlCan::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U1CanCtrlCan::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U1CanCtrlCan::Reset
    }
}
#[doc = "Field `u1_can_ctrl_can` writer - Reset selector: u1_can_ctrl_can"]
pub type U1CanCtrlCanW<'a, REG> = crate::BitWriter<'a, REG, U1CanCtrlCan>;
impl<'a, REG> U1CanCtrlCanW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U1CanCtrlCan::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U1CanCtrlCan::Reset)
    }
}
#[doc = "Reset selector: u1_can_ctrl_timer\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U1CanCtrlTimer {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U1CanCtrlTimer> for bool {
    #[inline(always)]
    fn from(variant: U1CanCtrlTimer) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u1_can_ctrl_timer` reader - Reset selector: u1_can_ctrl_timer"]
pub type U1CanCtrlTimerR = crate::BitReader<U1CanCtrlTimer>;
impl U1CanCtrlTimerR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U1CanCtrlTimer {
        match self.bits {
            false => U1CanCtrlTimer::None,
            true => U1CanCtrlTimer::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U1CanCtrlTimer::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U1CanCtrlTimer::Reset
    }
}
#[doc = "Field `u1_can_ctrl_timer` writer - Reset selector: u1_can_ctrl_timer"]
pub type U1CanCtrlTimerW<'a, REG> = crate::BitWriter<'a, REG, U1CanCtrlTimer>;
impl<'a, REG> U1CanCtrlTimerW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U1CanCtrlTimer::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U1CanCtrlTimer::Reset)
    }
}
#[doc = "Reset selector: u0_si5_timer_apb\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U0Si5TimerApb {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U0Si5TimerApb> for bool {
    #[inline(always)]
    fn from(variant: U0Si5TimerApb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u0_si5_timer_apb` reader - Reset selector: u0_si5_timer_apb"]
pub type U0Si5TimerApbR = crate::BitReader<U0Si5TimerApb>;
impl U0Si5TimerApbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U0Si5TimerApb {
        match self.bits {
            false => U0Si5TimerApb::None,
            true => U0Si5TimerApb::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U0Si5TimerApb::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U0Si5TimerApb::Reset
    }
}
#[doc = "Field `u0_si5_timer_apb` writer - Reset selector: u0_si5_timer_apb"]
pub type U0Si5TimerApbW<'a, REG> = crate::BitWriter<'a, REG, U0Si5TimerApb>;
impl<'a, REG> U0Si5TimerApbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U0Si5TimerApb::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U0Si5TimerApb::Reset)
    }
}
#[doc = "Reset selector: u0_si5_timer_0\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U0Si5Timer0 {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U0Si5Timer0> for bool {
    #[inline(always)]
    fn from(variant: U0Si5Timer0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u0_si5_timer_0` reader - Reset selector: u0_si5_timer_0"]
pub type U0Si5Timer0R = crate::BitReader<U0Si5Timer0>;
impl U0Si5Timer0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U0Si5Timer0 {
        match self.bits {
            false => U0Si5Timer0::None,
            true => U0Si5Timer0::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U0Si5Timer0::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U0Si5Timer0::Reset
    }
}
#[doc = "Field `u0_si5_timer_0` writer - Reset selector: u0_si5_timer_0"]
pub type U0Si5Timer0W<'a, REG> = crate::BitWriter<'a, REG, U0Si5Timer0>;
impl<'a, REG> U0Si5Timer0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U0Si5Timer0::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U0Si5Timer0::Reset)
    }
}
#[doc = "Reset selector: u0_si5_timer_1\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U0Si5Timer1 {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U0Si5Timer1> for bool {
    #[inline(always)]
    fn from(variant: U0Si5Timer1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u0_si5_timer_1` reader - Reset selector: u0_si5_timer_1"]
pub type U0Si5Timer1R = crate::BitReader<U0Si5Timer1>;
impl U0Si5Timer1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U0Si5Timer1 {
        match self.bits {
            false => U0Si5Timer1::None,
            true => U0Si5Timer1::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U0Si5Timer1::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U0Si5Timer1::Reset
    }
}
#[doc = "Field `u0_si5_timer_1` writer - Reset selector: u0_si5_timer_1"]
pub type U0Si5Timer1W<'a, REG> = crate::BitWriter<'a, REG, U0Si5Timer1>;
impl<'a, REG> U0Si5Timer1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U0Si5Timer1::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U0Si5Timer1::Reset)
    }
}
#[doc = "Reset selector: u0_si5_timer_2\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U0Si5Timer2 {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U0Si5Timer2> for bool {
    #[inline(always)]
    fn from(variant: U0Si5Timer2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u0_si5_timer_2` reader - Reset selector: u0_si5_timer_2"]
pub type U0Si5Timer2R = crate::BitReader<U0Si5Timer2>;
impl U0Si5Timer2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U0Si5Timer2 {
        match self.bits {
            false => U0Si5Timer2::None,
            true => U0Si5Timer2::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U0Si5Timer2::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U0Si5Timer2::Reset
    }
}
#[doc = "Field `u0_si5_timer_2` writer - Reset selector: u0_si5_timer_2"]
pub type U0Si5Timer2W<'a, REG> = crate::BitWriter<'a, REG, U0Si5Timer2>;
impl<'a, REG> U0Si5Timer2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U0Si5Timer2::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U0Si5Timer2::Reset)
    }
}
#[doc = "Reset selector: u0_si5_timer_3\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U0Si5Timer3 {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U0Si5Timer3> for bool {
    #[inline(always)]
    fn from(variant: U0Si5Timer3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u0_si5_timer_3` reader - Reset selector: u0_si5_timer_3"]
pub type U0Si5Timer3R = crate::BitReader<U0Si5Timer3>;
impl U0Si5Timer3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U0Si5Timer3 {
        match self.bits {
            false => U0Si5Timer3::None,
            true => U0Si5Timer3::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U0Si5Timer3::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U0Si5Timer3::Reset
    }
}
#[doc = "Field `u0_si5_timer_3` writer - Reset selector: u0_si5_timer_3"]
pub type U0Si5Timer3W<'a, REG> = crate::BitWriter<'a, REG, U0Si5Timer3>;
impl<'a, REG> U0Si5Timer3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U0Si5Timer3::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U0Si5Timer3::Reset)
    }
}
#[doc = "Reset selector: u0_int_ctrl_apb\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U0IntCtrlApb {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U0IntCtrlApb> for bool {
    #[inline(always)]
    fn from(variant: U0IntCtrlApb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u0_int_ctrl_apb` reader - Reset selector: u0_int_ctrl_apb"]
pub type U0IntCtrlApbR = crate::BitReader<U0IntCtrlApb>;
impl U0IntCtrlApbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U0IntCtrlApb {
        match self.bits {
            false => U0IntCtrlApb::None,
            true => U0IntCtrlApb::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U0IntCtrlApb::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U0IntCtrlApb::Reset
    }
}
#[doc = "Field `u0_int_ctrl_apb` writer - Reset selector: u0_int_ctrl_apb"]
pub type U0IntCtrlApbW<'a, REG> = crate::BitWriter<'a, REG, U0IntCtrlApb>;
impl<'a, REG> U0IntCtrlApbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U0IntCtrlApb::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U0IntCtrlApb::Reset)
    }
}
#[doc = "Reset selector: u0_temp_sensor_apb\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U0TempSensorApb {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U0TempSensorApb> for bool {
    #[inline(always)]
    fn from(variant: U0TempSensorApb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u0_temp_sensor_apb` reader - Reset selector: u0_temp_sensor_apb"]
pub type U0TempSensorApbR = crate::BitReader<U0TempSensorApb>;
impl U0TempSensorApbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U0TempSensorApb {
        match self.bits {
            false => U0TempSensorApb::None,
            true => U0TempSensorApb::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U0TempSensorApb::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U0TempSensorApb::Reset
    }
}
#[doc = "Field `u0_temp_sensor_apb` writer - Reset selector: u0_temp_sensor_apb"]
pub type U0TempSensorApbW<'a, REG> = crate::BitWriter<'a, REG, U0TempSensorApb>;
impl<'a, REG> U0TempSensorApbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U0TempSensorApb::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U0TempSensorApb::Reset)
    }
}
#[doc = "Reset selector: u0_temp_sensor\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U0TempSensor {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U0TempSensor> for bool {
    #[inline(always)]
    fn from(variant: U0TempSensor) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u0_temp_sensor` reader - Reset selector: u0_temp_sensor"]
pub type U0TempSensorR = crate::BitReader<U0TempSensor>;
impl U0TempSensorR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U0TempSensor {
        match self.bits {
            false => U0TempSensor::None,
            true => U0TempSensor::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U0TempSensor::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U0TempSensor::Reset
    }
}
#[doc = "Field `u0_temp_sensor` writer - Reset selector: u0_temp_sensor"]
pub type U0TempSensorW<'a, REG> = crate::BitWriter<'a, REG, U0TempSensor>;
impl<'a, REG> U0TempSensorW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U0TempSensor::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U0TempSensor::Reset)
    }
}
#[doc = "Reset selector: u0_jtag_rst\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U0JtagRst {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U0JtagRst> for bool {
    #[inline(always)]
    fn from(variant: U0JtagRst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u0_jtag_rst` reader - Reset selector: u0_jtag_rst"]
pub type U0JtagRstR = crate::BitReader<U0JtagRst>;
impl U0JtagRstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U0JtagRst {
        match self.bits {
            false => U0JtagRst::None,
            true => U0JtagRst::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U0JtagRst::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U0JtagRst::Reset
    }
}
#[doc = "Field `u0_jtag_rst` writer - Reset selector: u0_jtag_rst"]
pub type U0JtagRstW<'a, REG> = crate::BitWriter<'a, REG, U0JtagRst>;
impl<'a, REG> U0JtagRstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U0JtagRst::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U0JtagRst::Reset)
    }
}
impl R {
    #[doc = "Bit 0 - Reset selector: u0_pwmdac_apb"]
    #[inline(always)]
    pub fn u0_pwmdac_apb(&self) -> U0PwmdacApbR {
        U0PwmdacApbR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reset selector: u0_pdm_4mic_dmic"]
    #[inline(always)]
    pub fn u0_pdm_4mic_dmic(&self) -> U0Pdm4micDmicR {
        U0Pdm4micDmicR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reset selector: u0_pdm_4mic_apb"]
    #[inline(always)]
    pub fn u0_pdm_4mic_apb(&self) -> U0Pdm4micApbR {
        U0Pdm4micApbR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reset selector: u0_i2srx_apb"]
    #[inline(always)]
    pub fn u0_i2srx_apb(&self) -> U0I2srxApbR {
        U0I2srxApbR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Reset selector: u0_i2srx_bclk"]
    #[inline(always)]
    pub fn u0_i2srx_bclk(&self) -> U0I2srxBclkR {
        U0I2srxBclkR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Reset selector: u0_i2stx_apb"]
    #[inline(always)]
    pub fn u0_i2stx_apb(&self) -> U0I2stxApbR {
        U0I2stxApbR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Reset selector: u0_i2stx_bclk"]
    #[inline(always)]
    pub fn u0_i2stx_bclk(&self) -> U0I2stxBclkR {
        U0I2stxBclkR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Reset selector: u1_i2stx_apb"]
    #[inline(always)]
    pub fn u1_i2stx_apb(&self) -> U1I2stxApbR {
        U1I2stxApbR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Reset selector: u1_i2stx_bclk"]
    #[inline(always)]
    pub fn u1_i2stx_bclk(&self) -> U1I2stxBclkR {
        U1I2stxBclkR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Reset selector: u0_tdm16slot_ahb"]
    #[inline(always)]
    pub fn u0_tdm16slot_ahb(&self) -> U0Tdm16slotAhbR {
        U0Tdm16slotAhbR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Reset selector: u0_tdm16slot_tdm"]
    #[inline(always)]
    pub fn u0_tdm16slot_tdm(&self) -> U0Tdm16slotTdmR {
        U0Tdm16slotTdmR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Reset selector: u0_tdm16slot_apb"]
    #[inline(always)]
    pub fn u0_tdm16slot_apb(&self) -> U0Tdm16slotApbR {
        U0Tdm16slotApbR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Reset selector: u0_pwm_apb"]
    #[inline(always)]
    pub fn u0_pwm_apb(&self) -> U0PwmApbR {
        U0PwmApbR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Reset selector: u0_dskit_wdt_rstn_apb"]
    #[inline(always)]
    pub fn u0_dskit_wdt_rstn_apb(&self) -> U0DskitWdtRstnApbR {
        U0DskitWdtRstnApbR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Reset selector: u0_dskit_wdt"]
    #[inline(always)]
    pub fn u0_dskit_wdt(&self) -> U0DskitWdtR {
        U0DskitWdtR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Reset selector: u0_can_ctrl_apb"]
    #[inline(always)]
    pub fn u0_can_ctrl_apb(&self) -> U0CanCtrlApbR {
        U0CanCtrlApbR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Reset selector: u0_can_ctrl"]
    #[inline(always)]
    pub fn u0_can_ctrl(&self) -> U0CanCtrlR {
        U0CanCtrlR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Reset selector: u0_can_ctrl_timer"]
    #[inline(always)]
    pub fn u0_can_ctrl_timer(&self) -> U0CanCtrlTimerR {
        U0CanCtrlTimerR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Reset selector: u1_can_ctrl_apb"]
    #[inline(always)]
    pub fn u1_can_ctrl_apb(&self) -> U1CanCtrlApbR {
        U1CanCtrlApbR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Reset selector: u1_can_ctrl_can"]
    #[inline(always)]
    pub fn u1_can_ctrl_can(&self) -> U1CanCtrlCanR {
        U1CanCtrlCanR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Reset selector: u1_can_ctrl_timer"]
    #[inline(always)]
    pub fn u1_can_ctrl_timer(&self) -> U1CanCtrlTimerR {
        U1CanCtrlTimerR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Reset selector: u0_si5_timer_apb"]
    #[inline(always)]
    pub fn u0_si5_timer_apb(&self) -> U0Si5TimerApbR {
        U0Si5TimerApbR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Reset selector: u0_si5_timer_0"]
    #[inline(always)]
    pub fn u0_si5_timer_0(&self) -> U0Si5Timer0R {
        U0Si5Timer0R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Reset selector: u0_si5_timer_1"]
    #[inline(always)]
    pub fn u0_si5_timer_1(&self) -> U0Si5Timer1R {
        U0Si5Timer1R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Reset selector: u0_si5_timer_2"]
    #[inline(always)]
    pub fn u0_si5_timer_2(&self) -> U0Si5Timer2R {
        U0Si5Timer2R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Reset selector: u0_si5_timer_3"]
    #[inline(always)]
    pub fn u0_si5_timer_3(&self) -> U0Si5Timer3R {
        U0Si5Timer3R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Reset selector: u0_int_ctrl_apb"]
    #[inline(always)]
    pub fn u0_int_ctrl_apb(&self) -> U0IntCtrlApbR {
        U0IntCtrlApbR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Reset selector: u0_temp_sensor_apb"]
    #[inline(always)]
    pub fn u0_temp_sensor_apb(&self) -> U0TempSensorApbR {
        U0TempSensorApbR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Reset selector: u0_temp_sensor"]
    #[inline(always)]
    pub fn u0_temp_sensor(&self) -> U0TempSensorR {
        U0TempSensorR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Reset selector: u0_jtag_rst"]
    #[inline(always)]
    pub fn u0_jtag_rst(&self) -> U0JtagRstR {
        U0JtagRstR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Reset selector: u0_pwmdac_apb"]
    #[inline(always)]
    #[must_use]
    pub fn u0_pwmdac_apb(&mut self) -> U0PwmdacApbW<Rst3Spec> {
        U0PwmdacApbW::new(self, 0)
    }
    #[doc = "Bit 1 - Reset selector: u0_pdm_4mic_dmic"]
    #[inline(always)]
    #[must_use]
    pub fn u0_pdm_4mic_dmic(&mut self) -> U0Pdm4micDmicW<Rst3Spec> {
        U0Pdm4micDmicW::new(self, 1)
    }
    #[doc = "Bit 2 - Reset selector: u0_pdm_4mic_apb"]
    #[inline(always)]
    #[must_use]
    pub fn u0_pdm_4mic_apb(&mut self) -> U0Pdm4micApbW<Rst3Spec> {
        U0Pdm4micApbW::new(self, 2)
    }
    #[doc = "Bit 3 - Reset selector: u0_i2srx_apb"]
    #[inline(always)]
    #[must_use]
    pub fn u0_i2srx_apb(&mut self) -> U0I2srxApbW<Rst3Spec> {
        U0I2srxApbW::new(self, 3)
    }
    #[doc = "Bit 4 - Reset selector: u0_i2srx_bclk"]
    #[inline(always)]
    #[must_use]
    pub fn u0_i2srx_bclk(&mut self) -> U0I2srxBclkW<Rst3Spec> {
        U0I2srxBclkW::new(self, 4)
    }
    #[doc = "Bit 5 - Reset selector: u0_i2stx_apb"]
    #[inline(always)]
    #[must_use]
    pub fn u0_i2stx_apb(&mut self) -> U0I2stxApbW<Rst3Spec> {
        U0I2stxApbW::new(self, 5)
    }
    #[doc = "Bit 6 - Reset selector: u0_i2stx_bclk"]
    #[inline(always)]
    #[must_use]
    pub fn u0_i2stx_bclk(&mut self) -> U0I2stxBclkW<Rst3Spec> {
        U0I2stxBclkW::new(self, 6)
    }
    #[doc = "Bit 7 - Reset selector: u1_i2stx_apb"]
    #[inline(always)]
    #[must_use]
    pub fn u1_i2stx_apb(&mut self) -> U1I2stxApbW<Rst3Spec> {
        U1I2stxApbW::new(self, 7)
    }
    #[doc = "Bit 8 - Reset selector: u1_i2stx_bclk"]
    #[inline(always)]
    #[must_use]
    pub fn u1_i2stx_bclk(&mut self) -> U1I2stxBclkW<Rst3Spec> {
        U1I2stxBclkW::new(self, 8)
    }
    #[doc = "Bit 9 - Reset selector: u0_tdm16slot_ahb"]
    #[inline(always)]
    #[must_use]
    pub fn u0_tdm16slot_ahb(&mut self) -> U0Tdm16slotAhbW<Rst3Spec> {
        U0Tdm16slotAhbW::new(self, 9)
    }
    #[doc = "Bit 10 - Reset selector: u0_tdm16slot_tdm"]
    #[inline(always)]
    #[must_use]
    pub fn u0_tdm16slot_tdm(&mut self) -> U0Tdm16slotTdmW<Rst3Spec> {
        U0Tdm16slotTdmW::new(self, 10)
    }
    #[doc = "Bit 11 - Reset selector: u0_tdm16slot_apb"]
    #[inline(always)]
    #[must_use]
    pub fn u0_tdm16slot_apb(&mut self) -> U0Tdm16slotApbW<Rst3Spec> {
        U0Tdm16slotApbW::new(self, 11)
    }
    #[doc = "Bit 12 - Reset selector: u0_pwm_apb"]
    #[inline(always)]
    #[must_use]
    pub fn u0_pwm_apb(&mut self) -> U0PwmApbW<Rst3Spec> {
        U0PwmApbW::new(self, 12)
    }
    #[doc = "Bit 13 - Reset selector: u0_dskit_wdt_rstn_apb"]
    #[inline(always)]
    #[must_use]
    pub fn u0_dskit_wdt_rstn_apb(&mut self) -> U0DskitWdtRstnApbW<Rst3Spec> {
        U0DskitWdtRstnApbW::new(self, 13)
    }
    #[doc = "Bit 14 - Reset selector: u0_dskit_wdt"]
    #[inline(always)]
    #[must_use]
    pub fn u0_dskit_wdt(&mut self) -> U0DskitWdtW<Rst3Spec> {
        U0DskitWdtW::new(self, 14)
    }
    #[doc = "Bit 15 - Reset selector: u0_can_ctrl_apb"]
    #[inline(always)]
    #[must_use]
    pub fn u0_can_ctrl_apb(&mut self) -> U0CanCtrlApbW<Rst3Spec> {
        U0CanCtrlApbW::new(self, 15)
    }
    #[doc = "Bit 16 - Reset selector: u0_can_ctrl"]
    #[inline(always)]
    #[must_use]
    pub fn u0_can_ctrl(&mut self) -> U0CanCtrlW<Rst3Spec> {
        U0CanCtrlW::new(self, 16)
    }
    #[doc = "Bit 17 - Reset selector: u0_can_ctrl_timer"]
    #[inline(always)]
    #[must_use]
    pub fn u0_can_ctrl_timer(&mut self) -> U0CanCtrlTimerW<Rst3Spec> {
        U0CanCtrlTimerW::new(self, 17)
    }
    #[doc = "Bit 18 - Reset selector: u1_can_ctrl_apb"]
    #[inline(always)]
    #[must_use]
    pub fn u1_can_ctrl_apb(&mut self) -> U1CanCtrlApbW<Rst3Spec> {
        U1CanCtrlApbW::new(self, 18)
    }
    #[doc = "Bit 19 - Reset selector: u1_can_ctrl_can"]
    #[inline(always)]
    #[must_use]
    pub fn u1_can_ctrl_can(&mut self) -> U1CanCtrlCanW<Rst3Spec> {
        U1CanCtrlCanW::new(self, 19)
    }
    #[doc = "Bit 20 - Reset selector: u1_can_ctrl_timer"]
    #[inline(always)]
    #[must_use]
    pub fn u1_can_ctrl_timer(&mut self) -> U1CanCtrlTimerW<Rst3Spec> {
        U1CanCtrlTimerW::new(self, 20)
    }
    #[doc = "Bit 21 - Reset selector: u0_si5_timer_apb"]
    #[inline(always)]
    #[must_use]
    pub fn u0_si5_timer_apb(&mut self) -> U0Si5TimerApbW<Rst3Spec> {
        U0Si5TimerApbW::new(self, 21)
    }
    #[doc = "Bit 22 - Reset selector: u0_si5_timer_0"]
    #[inline(always)]
    #[must_use]
    pub fn u0_si5_timer_0(&mut self) -> U0Si5Timer0W<Rst3Spec> {
        U0Si5Timer0W::new(self, 22)
    }
    #[doc = "Bit 23 - Reset selector: u0_si5_timer_1"]
    #[inline(always)]
    #[must_use]
    pub fn u0_si5_timer_1(&mut self) -> U0Si5Timer1W<Rst3Spec> {
        U0Si5Timer1W::new(self, 23)
    }
    #[doc = "Bit 24 - Reset selector: u0_si5_timer_2"]
    #[inline(always)]
    #[must_use]
    pub fn u0_si5_timer_2(&mut self) -> U0Si5Timer2W<Rst3Spec> {
        U0Si5Timer2W::new(self, 24)
    }
    #[doc = "Bit 25 - Reset selector: u0_si5_timer_3"]
    #[inline(always)]
    #[must_use]
    pub fn u0_si5_timer_3(&mut self) -> U0Si5Timer3W<Rst3Spec> {
        U0Si5Timer3W::new(self, 25)
    }
    #[doc = "Bit 26 - Reset selector: u0_int_ctrl_apb"]
    #[inline(always)]
    #[must_use]
    pub fn u0_int_ctrl_apb(&mut self) -> U0IntCtrlApbW<Rst3Spec> {
        U0IntCtrlApbW::new(self, 26)
    }
    #[doc = "Bit 27 - Reset selector: u0_temp_sensor_apb"]
    #[inline(always)]
    #[must_use]
    pub fn u0_temp_sensor_apb(&mut self) -> U0TempSensorApbW<Rst3Spec> {
        U0TempSensorApbW::new(self, 27)
    }
    #[doc = "Bit 28 - Reset selector: u0_temp_sensor"]
    #[inline(always)]
    #[must_use]
    pub fn u0_temp_sensor(&mut self) -> U0TempSensorW<Rst3Spec> {
        U0TempSensorW::new(self, 28)
    }
    #[doc = "Bit 29 - Reset selector: u0_jtag_rst"]
    #[inline(always)]
    #[must_use]
    pub fn u0_jtag_rst(&mut self) -> U0JtagRstW<Rst3Spec> {
        U0JtagRstW::new(self, 29)
    }
}
#[doc = "RESET 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rst3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rst3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rst3Spec;
impl crate::RegisterSpec for Rst3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rst3::R`](R) reader structure"]
impl crate::Readable for Rst3Spec {}
#[doc = "`write(|w| ..)` method takes [`rst3::W`](W) writer structure"]
impl crate::Writable for Rst3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets rst3 to value 0x07ff_ffff"]
impl crate::Resettable for Rst3Spec {
    const RESET_VALUE: u32 = 0x07ff_ffff;
}
