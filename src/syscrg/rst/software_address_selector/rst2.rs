#[doc = "Register `rst2` reader"]
pub type R = crate::R<Rst2Spec>;
#[doc = "Register `rst2` writer"]
pub type W = crate::W<Rst2Spec>;
#[doc = "Reset selector: u0_sdio_ahb\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U0SdioAhb {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U0SdioAhb> for bool {
    #[inline(always)]
    fn from(variant: U0SdioAhb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u0_sdio_ahb` reader - Reset selector: u0_sdio_ahb"]
pub type U0SdioAhbR = crate::BitReader<U0SdioAhb>;
impl U0SdioAhbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U0SdioAhb {
        match self.bits {
            false => U0SdioAhb::None,
            true => U0SdioAhb::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U0SdioAhb::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U0SdioAhb::Reset
    }
}
#[doc = "Field `u0_sdio_ahb` writer - Reset selector: u0_sdio_ahb"]
pub type U0SdioAhbW<'a, REG> = crate::BitWriter<'a, REG, U0SdioAhb>;
impl<'a, REG> U0SdioAhbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U0SdioAhb::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U0SdioAhb::Reset)
    }
}
#[doc = "Reset selector: u1_sdi_ahb\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U1SdiAhb {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U1SdiAhb> for bool {
    #[inline(always)]
    fn from(variant: U1SdiAhb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u1_sdi_ahb` reader - Reset selector: u1_sdi_ahb"]
pub type U1SdiAhbR = crate::BitReader<U1SdiAhb>;
impl U1SdiAhbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U1SdiAhb {
        match self.bits {
            false => U1SdiAhb::None,
            true => U1SdiAhb::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U1SdiAhb::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U1SdiAhb::Reset
    }
}
#[doc = "Field `u1_sdi_ahb` writer - Reset selector: u1_sdi_ahb"]
pub type U1SdiAhbW<'a, REG> = crate::BitWriter<'a, REG, U1SdiAhb>;
impl<'a, REG> U1SdiAhbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U1SdiAhb::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U1SdiAhb::Reset)
    }
}
#[doc = "Reset selector: u1_gmac5_axi64\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U1Gmac5Axi64 {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U1Gmac5Axi64> for bool {
    #[inline(always)]
    fn from(variant: U1Gmac5Axi64) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u1_gmac5_axi64` reader - Reset selector: u1_gmac5_axi64"]
pub type U1Gmac5Axi64R = crate::BitReader<U1Gmac5Axi64>;
impl U1Gmac5Axi64R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U1Gmac5Axi64 {
        match self.bits {
            false => U1Gmac5Axi64::None,
            true => U1Gmac5Axi64::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U1Gmac5Axi64::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U1Gmac5Axi64::Reset
    }
}
#[doc = "Field `u1_gmac5_axi64` writer - Reset selector: u1_gmac5_axi64"]
pub type U1Gmac5Axi64W<'a, REG> = crate::BitWriter<'a, REG, U1Gmac5Axi64>;
impl<'a, REG> U1Gmac5Axi64W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U1Gmac5Axi64::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U1Gmac5Axi64::Reset)
    }
}
#[doc = "Reset selector: u1_gmac5_axi64_hresetn\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U1Gmac5Axi64Hresetn {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U1Gmac5Axi64Hresetn> for bool {
    #[inline(always)]
    fn from(variant: U1Gmac5Axi64Hresetn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u1_gmac5_axi64_hresetn` reader - Reset selector: u1_gmac5_axi64_hresetn"]
pub type U1Gmac5Axi64HresetnR = crate::BitReader<U1Gmac5Axi64Hresetn>;
impl U1Gmac5Axi64HresetnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U1Gmac5Axi64Hresetn {
        match self.bits {
            false => U1Gmac5Axi64Hresetn::None,
            true => U1Gmac5Axi64Hresetn::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U1Gmac5Axi64Hresetn::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U1Gmac5Axi64Hresetn::Reset
    }
}
#[doc = "Field `u1_gmac5_axi64_hresetn` writer - Reset selector: u1_gmac5_axi64_hresetn"]
pub type U1Gmac5Axi64HresetnW<'a, REG> = crate::BitWriter<'a, REG, U1Gmac5Axi64Hresetn>;
impl<'a, REG> U1Gmac5Axi64HresetnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U1Gmac5Axi64Hresetn::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U1Gmac5Axi64Hresetn::Reset)
    }
}
#[doc = "Reset selector: u0_mailbox_presetn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U0MailboxPresetn {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U0MailboxPresetn> for bool {
    #[inline(always)]
    fn from(variant: U0MailboxPresetn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u0_mailbox_presetn` reader - Reset selector: u0_mailbox_presetn"]
pub type U0MailboxPresetnR = crate::BitReader<U0MailboxPresetn>;
impl U0MailboxPresetnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U0MailboxPresetn {
        match self.bits {
            false => U0MailboxPresetn::None,
            true => U0MailboxPresetn::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U0MailboxPresetn::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U0MailboxPresetn::Reset
    }
}
#[doc = "Field `u0_mailbox_presetn` writer - Reset selector: u0_mailbox_presetn"]
pub type U0MailboxPresetnW<'a, REG> = crate::BitWriter<'a, REG, U0MailboxPresetn>;
impl<'a, REG> U0MailboxPresetnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U0MailboxPresetn::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U0MailboxPresetn::Reset)
    }
}
#[doc = "Reset selector: u0_spi_apb\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U0SpiApb {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U0SpiApb> for bool {
    #[inline(always)]
    fn from(variant: U0SpiApb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u0_spi_apb` reader - Reset selector: u0_spi_apb"]
pub type U0SpiApbR = crate::BitReader<U0SpiApb>;
impl U0SpiApbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U0SpiApb {
        match self.bits {
            false => U0SpiApb::None,
            true => U0SpiApb::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U0SpiApb::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U0SpiApb::Reset
    }
}
#[doc = "Field `u0_spi_apb` writer - Reset selector: u0_spi_apb"]
pub type U0SpiApbW<'a, REG> = crate::BitWriter<'a, REG, U0SpiApb>;
impl<'a, REG> U0SpiApbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U0SpiApb::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U0SpiApb::Reset)
    }
}
#[doc = "Reset selector: u1_spi_apb\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U1SpiApb {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U1SpiApb> for bool {
    #[inline(always)]
    fn from(variant: U1SpiApb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u1_spi_apb` reader - Reset selector: u1_spi_apb"]
pub type U1SpiApbR = crate::BitReader<U1SpiApb>;
impl U1SpiApbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U1SpiApb {
        match self.bits {
            false => U1SpiApb::None,
            true => U1SpiApb::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U1SpiApb::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U1SpiApb::Reset
    }
}
#[doc = "Field `u1_spi_apb` writer - Reset selector: u1_spi_apb"]
pub type U1SpiApbW<'a, REG> = crate::BitWriter<'a, REG, U1SpiApb>;
impl<'a, REG> U1SpiApbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U1SpiApb::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U1SpiApb::Reset)
    }
}
#[doc = "Reset selector: u2_spi_apb\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U2SpiApb {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U2SpiApb> for bool {
    #[inline(always)]
    fn from(variant: U2SpiApb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u2_spi_apb` reader - Reset selector: u2_spi_apb"]
pub type U2SpiApbR = crate::BitReader<U2SpiApb>;
impl U2SpiApbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U2SpiApb {
        match self.bits {
            false => U2SpiApb::None,
            true => U2SpiApb::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U2SpiApb::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U2SpiApb::Reset
    }
}
#[doc = "Field `u2_spi_apb` writer - Reset selector: u2_spi_apb"]
pub type U2SpiApbW<'a, REG> = crate::BitWriter<'a, REG, U2SpiApb>;
impl<'a, REG> U2SpiApbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U2SpiApb::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U2SpiApb::Reset)
    }
}
#[doc = "Reset selector: u3_spi_apb\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U3SpiApb {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U3SpiApb> for bool {
    #[inline(always)]
    fn from(variant: U3SpiApb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u3_spi_apb` reader - Reset selector: u3_spi_apb"]
pub type U3SpiApbR = crate::BitReader<U3SpiApb>;
impl U3SpiApbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U3SpiApb {
        match self.bits {
            false => U3SpiApb::None,
            true => U3SpiApb::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U3SpiApb::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U3SpiApb::Reset
    }
}
#[doc = "Field `u3_spi_apb` writer - Reset selector: u3_spi_apb"]
pub type U3SpiApbW<'a, REG> = crate::BitWriter<'a, REG, U3SpiApb>;
impl<'a, REG> U3SpiApbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U3SpiApb::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U3SpiApb::Reset)
    }
}
#[doc = "Reset selector: u4_spi_apb\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U4SpiApb {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U4SpiApb> for bool {
    #[inline(always)]
    fn from(variant: U4SpiApb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u4_spi_apb` reader - Reset selector: u4_spi_apb"]
pub type U4SpiApbR = crate::BitReader<U4SpiApb>;
impl U4SpiApbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U4SpiApb {
        match self.bits {
            false => U4SpiApb::None,
            true => U4SpiApb::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U4SpiApb::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U4SpiApb::Reset
    }
}
#[doc = "Field `u4_spi_apb` writer - Reset selector: u4_spi_apb"]
pub type U4SpiApbW<'a, REG> = crate::BitWriter<'a, REG, U4SpiApb>;
impl<'a, REG> U4SpiApbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U4SpiApb::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U4SpiApb::Reset)
    }
}
#[doc = "Reset selector: u5_spi_apb\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U5SpiApb {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U5SpiApb> for bool {
    #[inline(always)]
    fn from(variant: U5SpiApb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u5_spi_apb` reader - Reset selector: u5_spi_apb"]
pub type U5SpiApbR = crate::BitReader<U5SpiApb>;
impl U5SpiApbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U5SpiApb {
        match self.bits {
            false => U5SpiApb::None,
            true => U5SpiApb::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U5SpiApb::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U5SpiApb::Reset
    }
}
#[doc = "Field `u5_spi_apb` writer - Reset selector: u5_spi_apb"]
pub type U5SpiApbW<'a, REG> = crate::BitWriter<'a, REG, U5SpiApb>;
impl<'a, REG> U5SpiApbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U5SpiApb::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U5SpiApb::Reset)
    }
}
#[doc = "Reset selector: u6_spi_apb\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U6SpiApb {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U6SpiApb> for bool {
    #[inline(always)]
    fn from(variant: U6SpiApb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u6_spi_apb` reader - Reset selector: u6_spi_apb"]
pub type U6SpiApbR = crate::BitReader<U6SpiApb>;
impl U6SpiApbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U6SpiApb {
        match self.bits {
            false => U6SpiApb::None,
            true => U6SpiApb::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U6SpiApb::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U6SpiApb::Reset
    }
}
#[doc = "Field `u6_spi_apb` writer - Reset selector: u6_spi_apb"]
pub type U6SpiApbW<'a, REG> = crate::BitWriter<'a, REG, U6SpiApb>;
impl<'a, REG> U6SpiApbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U6SpiApb::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U6SpiApb::Reset)
    }
}
#[doc = "Reset selector: u0_i2c_apb\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U0I2cApb {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U0I2cApb> for bool {
    #[inline(always)]
    fn from(variant: U0I2cApb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u0_i2c_apb` reader - Reset selector: u0_i2c_apb"]
pub type U0I2cApbR = crate::BitReader<U0I2cApb>;
impl U0I2cApbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U0I2cApb {
        match self.bits {
            false => U0I2cApb::None,
            true => U0I2cApb::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U0I2cApb::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U0I2cApb::Reset
    }
}
#[doc = "Field `u0_i2c_apb` writer - Reset selector: u0_i2c_apb"]
pub type U0I2cApbW<'a, REG> = crate::BitWriter<'a, REG, U0I2cApb>;
impl<'a, REG> U0I2cApbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U0I2cApb::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U0I2cApb::Reset)
    }
}
#[doc = "Reset selector: u1_i2c_apb\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U1I2cApb {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U1I2cApb> for bool {
    #[inline(always)]
    fn from(variant: U1I2cApb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u1_i2c_apb` reader - Reset selector: u1_i2c_apb"]
pub type U1I2cApbR = crate::BitReader<U1I2cApb>;
impl U1I2cApbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U1I2cApb {
        match self.bits {
            false => U1I2cApb::None,
            true => U1I2cApb::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U1I2cApb::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U1I2cApb::Reset
    }
}
#[doc = "Field `u1_i2c_apb` writer - Reset selector: u1_i2c_apb"]
pub type U1I2cApbW<'a, REG> = crate::BitWriter<'a, REG, U1I2cApb>;
impl<'a, REG> U1I2cApbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U1I2cApb::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U1I2cApb::Reset)
    }
}
#[doc = "Reset selector: u2_i2c_apb\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U2I2cApb {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U2I2cApb> for bool {
    #[inline(always)]
    fn from(variant: U2I2cApb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u2_i2c_apb` reader - Reset selector: u2_i2c_apb"]
pub type U2I2cApbR = crate::BitReader<U2I2cApb>;
impl U2I2cApbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U2I2cApb {
        match self.bits {
            false => U2I2cApb::None,
            true => U2I2cApb::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U2I2cApb::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U2I2cApb::Reset
    }
}
#[doc = "Field `u2_i2c_apb` writer - Reset selector: u2_i2c_apb"]
pub type U2I2cApbW<'a, REG> = crate::BitWriter<'a, REG, U2I2cApb>;
impl<'a, REG> U2I2cApbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U2I2cApb::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U2I2cApb::Reset)
    }
}
#[doc = "Reset selector: u3_i2c_apb\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U3I2cApb {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U3I2cApb> for bool {
    #[inline(always)]
    fn from(variant: U3I2cApb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u3_i2c_apb` reader - Reset selector: u3_i2c_apb"]
pub type U3I2cApbR = crate::BitReader<U3I2cApb>;
impl U3I2cApbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U3I2cApb {
        match self.bits {
            false => U3I2cApb::None,
            true => U3I2cApb::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U3I2cApb::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U3I2cApb::Reset
    }
}
#[doc = "Field `u3_i2c_apb` writer - Reset selector: u3_i2c_apb"]
pub type U3I2cApbW<'a, REG> = crate::BitWriter<'a, REG, U3I2cApb>;
impl<'a, REG> U3I2cApbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U3I2cApb::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U3I2cApb::Reset)
    }
}
#[doc = "Reset selector: u4_i2c_apb\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U4I2cApb {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U4I2cApb> for bool {
    #[inline(always)]
    fn from(variant: U4I2cApb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u4_i2c_apb` reader - Reset selector: u4_i2c_apb"]
pub type U4I2cApbR = crate::BitReader<U4I2cApb>;
impl U4I2cApbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U4I2cApb {
        match self.bits {
            false => U4I2cApb::None,
            true => U4I2cApb::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U4I2cApb::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U4I2cApb::Reset
    }
}
#[doc = "Field `u4_i2c_apb` writer - Reset selector: u4_i2c_apb"]
pub type U4I2cApbW<'a, REG> = crate::BitWriter<'a, REG, U4I2cApb>;
impl<'a, REG> U4I2cApbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U4I2cApb::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U4I2cApb::Reset)
    }
}
#[doc = "Reset selector: u5_i2c_apb\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U5I2cApb {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U5I2cApb> for bool {
    #[inline(always)]
    fn from(variant: U5I2cApb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u5_i2c_apb` reader - Reset selector: u5_i2c_apb"]
pub type U5I2cApbR = crate::BitReader<U5I2cApb>;
impl U5I2cApbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U5I2cApb {
        match self.bits {
            false => U5I2cApb::None,
            true => U5I2cApb::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U5I2cApb::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U5I2cApb::Reset
    }
}
#[doc = "Field `u5_i2c_apb` writer - Reset selector: u5_i2c_apb"]
pub type U5I2cApbW<'a, REG> = crate::BitWriter<'a, REG, U5I2cApb>;
impl<'a, REG> U5I2cApbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U5I2cApb::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U5I2cApb::Reset)
    }
}
#[doc = "Reset selector: u6_i2c_apb\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U6I2cApb {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U6I2cApb> for bool {
    #[inline(always)]
    fn from(variant: U6I2cApb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u6_i2c_apb` reader - Reset selector: u6_i2c_apb"]
pub type U6I2cApbR = crate::BitReader<U6I2cApb>;
impl U6I2cApbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U6I2cApb {
        match self.bits {
            false => U6I2cApb::None,
            true => U6I2cApb::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U6I2cApb::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U6I2cApb::Reset
    }
}
#[doc = "Field `u6_i2c_apb` writer - Reset selector: u6_i2c_apb"]
pub type U6I2cApbW<'a, REG> = crate::BitWriter<'a, REG, U6I2cApb>;
impl<'a, REG> U6I2cApbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U6I2cApb::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U6I2cApb::Reset)
    }
}
#[doc = "Reset selector: u0_uart_apb\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U0UartApb {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U0UartApb> for bool {
    #[inline(always)]
    fn from(variant: U0UartApb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u0_uart_apb` reader - Reset selector: u0_uart_apb"]
pub type U0UartApbR = crate::BitReader<U0UartApb>;
impl U0UartApbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U0UartApb {
        match self.bits {
            false => U0UartApb::None,
            true => U0UartApb::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U0UartApb::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U0UartApb::Reset
    }
}
#[doc = "Field `u0_uart_apb` writer - Reset selector: u0_uart_apb"]
pub type U0UartApbW<'a, REG> = crate::BitWriter<'a, REG, U0UartApb>;
impl<'a, REG> U0UartApbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U0UartApb::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U0UartApb::Reset)
    }
}
#[doc = "Reset selector: u0_uart_core\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U0UartCore {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U0UartCore> for bool {
    #[inline(always)]
    fn from(variant: U0UartCore) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u0_uart_core` reader - Reset selector: u0_uart_core"]
pub type U0UartCoreR = crate::BitReader<U0UartCore>;
impl U0UartCoreR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U0UartCore {
        match self.bits {
            false => U0UartCore::None,
            true => U0UartCore::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U0UartCore::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U0UartCore::Reset
    }
}
#[doc = "Field `u0_uart_core` writer - Reset selector: u0_uart_core"]
pub type U0UartCoreW<'a, REG> = crate::BitWriter<'a, REG, U0UartCore>;
impl<'a, REG> U0UartCoreW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U0UartCore::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U0UartCore::Reset)
    }
}
#[doc = "Reset selector: u1_uart_apb\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U1UartApb {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U1UartApb> for bool {
    #[inline(always)]
    fn from(variant: U1UartApb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u1_uart_apb` reader - Reset selector: u1_uart_apb"]
pub type U1UartApbR = crate::BitReader<U1UartApb>;
impl U1UartApbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U1UartApb {
        match self.bits {
            false => U1UartApb::None,
            true => U1UartApb::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U1UartApb::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U1UartApb::Reset
    }
}
#[doc = "Field `u1_uart_apb` writer - Reset selector: u1_uart_apb"]
pub type U1UartApbW<'a, REG> = crate::BitWriter<'a, REG, U1UartApb>;
impl<'a, REG> U1UartApbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U1UartApb::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U1UartApb::Reset)
    }
}
#[doc = "Reset selector: u1_uart_core\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U1UartCore {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U1UartCore> for bool {
    #[inline(always)]
    fn from(variant: U1UartCore) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u1_uart_core` reader - Reset selector: u1_uart_core"]
pub type U1UartCoreR = crate::BitReader<U1UartCore>;
impl U1UartCoreR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U1UartCore {
        match self.bits {
            false => U1UartCore::None,
            true => U1UartCore::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U1UartCore::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U1UartCore::Reset
    }
}
#[doc = "Field `u1_uart_core` writer - Reset selector: u1_uart_core"]
pub type U1UartCoreW<'a, REG> = crate::BitWriter<'a, REG, U1UartCore>;
impl<'a, REG> U1UartCoreW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U1UartCore::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U1UartCore::Reset)
    }
}
#[doc = "Reset selector: u2_uart_apb\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U2UartApb {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U2UartApb> for bool {
    #[inline(always)]
    fn from(variant: U2UartApb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u2_uart_apb` reader - Reset selector: u2_uart_apb"]
pub type U2UartApbR = crate::BitReader<U2UartApb>;
impl U2UartApbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U2UartApb {
        match self.bits {
            false => U2UartApb::None,
            true => U2UartApb::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U2UartApb::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U2UartApb::Reset
    }
}
#[doc = "Field `u2_uart_apb` writer - Reset selector: u2_uart_apb"]
pub type U2UartApbW<'a, REG> = crate::BitWriter<'a, REG, U2UartApb>;
impl<'a, REG> U2UartApbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U2UartApb::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U2UartApb::Reset)
    }
}
#[doc = "Reset selector: u2_uart_core\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U2UartCore {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U2UartCore> for bool {
    #[inline(always)]
    fn from(variant: U2UartCore) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u2_uart_core` reader - Reset selector: u2_uart_core"]
pub type U2UartCoreR = crate::BitReader<U2UartCore>;
impl U2UartCoreR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U2UartCore {
        match self.bits {
            false => U2UartCore::None,
            true => U2UartCore::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U2UartCore::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U2UartCore::Reset
    }
}
#[doc = "Field `u2_uart_core` writer - Reset selector: u2_uart_core"]
pub type U2UartCoreW<'a, REG> = crate::BitWriter<'a, REG, U2UartCore>;
impl<'a, REG> U2UartCoreW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U2UartCore::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U2UartCore::Reset)
    }
}
#[doc = "Reset selector: u3_uart_apb\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U3UartApb {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U3UartApb> for bool {
    #[inline(always)]
    fn from(variant: U3UartApb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u3_uart_apb` reader - Reset selector: u3_uart_apb"]
pub type U3UartApbR = crate::BitReader<U3UartApb>;
impl U3UartApbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U3UartApb {
        match self.bits {
            false => U3UartApb::None,
            true => U3UartApb::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U3UartApb::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U3UartApb::Reset
    }
}
#[doc = "Field `u3_uart_apb` writer - Reset selector: u3_uart_apb"]
pub type U3UartApbW<'a, REG> = crate::BitWriter<'a, REG, U3UartApb>;
impl<'a, REG> U3UartApbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U3UartApb::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U3UartApb::Reset)
    }
}
#[doc = "Reset selector: u3_uart_core\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U3UartCore {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U3UartCore> for bool {
    #[inline(always)]
    fn from(variant: U3UartCore) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u3_uart_core` reader - Reset selector: u3_uart_core"]
pub type U3UartCoreR = crate::BitReader<U3UartCore>;
impl U3UartCoreR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U3UartCore {
        match self.bits {
            false => U3UartCore::None,
            true => U3UartCore::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U3UartCore::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U3UartCore::Reset
    }
}
#[doc = "Field `u3_uart_core` writer - Reset selector: u3_uart_core"]
pub type U3UartCoreW<'a, REG> = crate::BitWriter<'a, REG, U3UartCore>;
impl<'a, REG> U3UartCoreW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U3UartCore::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U3UartCore::Reset)
    }
}
#[doc = "Reset selector: u4_uart_apb\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U4UartApb {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U4UartApb> for bool {
    #[inline(always)]
    fn from(variant: U4UartApb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u4_uart_apb` reader - Reset selector: u4_uart_apb"]
pub type U4UartApbR = crate::BitReader<U4UartApb>;
impl U4UartApbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U4UartApb {
        match self.bits {
            false => U4UartApb::None,
            true => U4UartApb::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U4UartApb::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U4UartApb::Reset
    }
}
#[doc = "Field `u4_uart_apb` writer - Reset selector: u4_uart_apb"]
pub type U4UartApbW<'a, REG> = crate::BitWriter<'a, REG, U4UartApb>;
impl<'a, REG> U4UartApbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U4UartApb::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U4UartApb::Reset)
    }
}
#[doc = "Reset selector: u4_uart_core\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U4UartCore {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U4UartCore> for bool {
    #[inline(always)]
    fn from(variant: U4UartCore) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u4_uart_core` reader - Reset selector: u4_uart_core"]
pub type U4UartCoreR = crate::BitReader<U4UartCore>;
impl U4UartCoreR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U4UartCore {
        match self.bits {
            false => U4UartCore::None,
            true => U4UartCore::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U4UartCore::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U4UartCore::Reset
    }
}
#[doc = "Field `u4_uart_core` writer - Reset selector: u4_uart_core"]
pub type U4UartCoreW<'a, REG> = crate::BitWriter<'a, REG, U4UartCore>;
impl<'a, REG> U4UartCoreW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U4UartCore::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U4UartCore::Reset)
    }
}
#[doc = "Reset selector: u5_uart_apb\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U5UartApb {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U5UartApb> for bool {
    #[inline(always)]
    fn from(variant: U5UartApb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u5_uart_apb` reader - Reset selector: u5_uart_apb"]
pub type U5UartApbR = crate::BitReader<U5UartApb>;
impl U5UartApbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U5UartApb {
        match self.bits {
            false => U5UartApb::None,
            true => U5UartApb::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U5UartApb::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U5UartApb::Reset
    }
}
#[doc = "Field `u5_uart_apb` writer - Reset selector: u5_uart_apb"]
pub type U5UartApbW<'a, REG> = crate::BitWriter<'a, REG, U5UartApb>;
impl<'a, REG> U5UartApbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U5UartApb::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U5UartApb::Reset)
    }
}
#[doc = "Reset selector: u6_uart_core\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U6UartCore {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U6UartCore> for bool {
    #[inline(always)]
    fn from(variant: U6UartCore) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u6_uart_core` reader - Reset selector: u6_uart_core"]
pub type U6UartCoreR = crate::BitReader<U6UartCore>;
impl U6UartCoreR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U6UartCore {
        match self.bits {
            false => U6UartCore::None,
            true => U6UartCore::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U6UartCore::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U6UartCore::Reset
    }
}
#[doc = "Field `u6_uart_core` writer - Reset selector: u6_uart_core"]
pub type U6UartCoreW<'a, REG> = crate::BitWriter<'a, REG, U6UartCore>;
impl<'a, REG> U6UartCoreW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U6UartCore::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U6UartCore::Reset)
    }
}
#[doc = "Reset selector: u0_spdif_apb\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U0SpdifApb {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U0SpdifApb> for bool {
    #[inline(always)]
    fn from(variant: U0SpdifApb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u0_spdif_apb` reader - Reset selector: u0_spdif_apb"]
pub type U0SpdifApbR = crate::BitReader<U0SpdifApb>;
impl U0SpdifApbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U0SpdifApb {
        match self.bits {
            false => U0SpdifApb::None,
            true => U0SpdifApb::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U0SpdifApb::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U0SpdifApb::Reset
    }
}
#[doc = "Field `u0_spdif_apb` writer - Reset selector: u0_spdif_apb"]
pub type U0SpdifApbW<'a, REG> = crate::BitWriter<'a, REG, U0SpdifApb>;
impl<'a, REG> U0SpdifApbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U0SpdifApb::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U0SpdifApb::Reset)
    }
}
impl R {
    #[doc = "Bit 0 - Reset selector: u0_sdio_ahb"]
    #[inline(always)]
    pub fn u0_sdio_ahb(&self) -> U0SdioAhbR {
        U0SdioAhbR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reset selector: u1_sdi_ahb"]
    #[inline(always)]
    pub fn u1_sdi_ahb(&self) -> U1SdiAhbR {
        U1SdiAhbR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reset selector: u1_gmac5_axi64"]
    #[inline(always)]
    pub fn u1_gmac5_axi64(&self) -> U1Gmac5Axi64R {
        U1Gmac5Axi64R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reset selector: u1_gmac5_axi64_hresetn"]
    #[inline(always)]
    pub fn u1_gmac5_axi64_hresetn(&self) -> U1Gmac5Axi64HresetnR {
        U1Gmac5Axi64HresetnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Reset selector: u0_mailbox_presetn"]
    #[inline(always)]
    pub fn u0_mailbox_presetn(&self) -> U0MailboxPresetnR {
        U0MailboxPresetnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Reset selector: u0_spi_apb"]
    #[inline(always)]
    pub fn u0_spi_apb(&self) -> U0SpiApbR {
        U0SpiApbR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Reset selector: u1_spi_apb"]
    #[inline(always)]
    pub fn u1_spi_apb(&self) -> U1SpiApbR {
        U1SpiApbR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Reset selector: u2_spi_apb"]
    #[inline(always)]
    pub fn u2_spi_apb(&self) -> U2SpiApbR {
        U2SpiApbR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Reset selector: u3_spi_apb"]
    #[inline(always)]
    pub fn u3_spi_apb(&self) -> U3SpiApbR {
        U3SpiApbR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Reset selector: u4_spi_apb"]
    #[inline(always)]
    pub fn u4_spi_apb(&self) -> U4SpiApbR {
        U4SpiApbR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Reset selector: u5_spi_apb"]
    #[inline(always)]
    pub fn u5_spi_apb(&self) -> U5SpiApbR {
        U5SpiApbR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Reset selector: u6_spi_apb"]
    #[inline(always)]
    pub fn u6_spi_apb(&self) -> U6SpiApbR {
        U6SpiApbR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Reset selector: u0_i2c_apb"]
    #[inline(always)]
    pub fn u0_i2c_apb(&self) -> U0I2cApbR {
        U0I2cApbR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Reset selector: u1_i2c_apb"]
    #[inline(always)]
    pub fn u1_i2c_apb(&self) -> U1I2cApbR {
        U1I2cApbR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Reset selector: u2_i2c_apb"]
    #[inline(always)]
    pub fn u2_i2c_apb(&self) -> U2I2cApbR {
        U2I2cApbR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Reset selector: u3_i2c_apb"]
    #[inline(always)]
    pub fn u3_i2c_apb(&self) -> U3I2cApbR {
        U3I2cApbR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Reset selector: u4_i2c_apb"]
    #[inline(always)]
    pub fn u4_i2c_apb(&self) -> U4I2cApbR {
        U4I2cApbR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Reset selector: u5_i2c_apb"]
    #[inline(always)]
    pub fn u5_i2c_apb(&self) -> U5I2cApbR {
        U5I2cApbR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Reset selector: u6_i2c_apb"]
    #[inline(always)]
    pub fn u6_i2c_apb(&self) -> U6I2cApbR {
        U6I2cApbR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Reset selector: u0_uart_apb"]
    #[inline(always)]
    pub fn u0_uart_apb(&self) -> U0UartApbR {
        U0UartApbR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Reset selector: u0_uart_core"]
    #[inline(always)]
    pub fn u0_uart_core(&self) -> U0UartCoreR {
        U0UartCoreR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Reset selector: u1_uart_apb"]
    #[inline(always)]
    pub fn u1_uart_apb(&self) -> U1UartApbR {
        U1UartApbR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Reset selector: u1_uart_core"]
    #[inline(always)]
    pub fn u1_uart_core(&self) -> U1UartCoreR {
        U1UartCoreR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Reset selector: u2_uart_apb"]
    #[inline(always)]
    pub fn u2_uart_apb(&self) -> U2UartApbR {
        U2UartApbR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Reset selector: u2_uart_core"]
    #[inline(always)]
    pub fn u2_uart_core(&self) -> U2UartCoreR {
        U2UartCoreR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Reset selector: u3_uart_apb"]
    #[inline(always)]
    pub fn u3_uart_apb(&self) -> U3UartApbR {
        U3UartApbR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Reset selector: u3_uart_core"]
    #[inline(always)]
    pub fn u3_uart_core(&self) -> U3UartCoreR {
        U3UartCoreR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Reset selector: u4_uart_apb"]
    #[inline(always)]
    pub fn u4_uart_apb(&self) -> U4UartApbR {
        U4UartApbR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Reset selector: u4_uart_core"]
    #[inline(always)]
    pub fn u4_uart_core(&self) -> U4UartCoreR {
        U4UartCoreR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Reset selector: u5_uart_apb"]
    #[inline(always)]
    pub fn u5_uart_apb(&self) -> U5UartApbR {
        U5UartApbR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Reset selector: u6_uart_core"]
    #[inline(always)]
    pub fn u6_uart_core(&self) -> U6UartCoreR {
        U6UartCoreR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Reset selector: u0_spdif_apb"]
    #[inline(always)]
    pub fn u0_spdif_apb(&self) -> U0SpdifApbR {
        U0SpdifApbR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Reset selector: u0_sdio_ahb"]
    #[inline(always)]
    #[must_use]
    pub fn u0_sdio_ahb(&mut self) -> U0SdioAhbW<Rst2Spec> {
        U0SdioAhbW::new(self, 0)
    }
    #[doc = "Bit 1 - Reset selector: u1_sdi_ahb"]
    #[inline(always)]
    #[must_use]
    pub fn u1_sdi_ahb(&mut self) -> U1SdiAhbW<Rst2Spec> {
        U1SdiAhbW::new(self, 1)
    }
    #[doc = "Bit 2 - Reset selector: u1_gmac5_axi64"]
    #[inline(always)]
    #[must_use]
    pub fn u1_gmac5_axi64(&mut self) -> U1Gmac5Axi64W<Rst2Spec> {
        U1Gmac5Axi64W::new(self, 2)
    }
    #[doc = "Bit 3 - Reset selector: u1_gmac5_axi64_hresetn"]
    #[inline(always)]
    #[must_use]
    pub fn u1_gmac5_axi64_hresetn(&mut self) -> U1Gmac5Axi64HresetnW<Rst2Spec> {
        U1Gmac5Axi64HresetnW::new(self, 3)
    }
    #[doc = "Bit 4 - Reset selector: u0_mailbox_presetn"]
    #[inline(always)]
    #[must_use]
    pub fn u0_mailbox_presetn(&mut self) -> U0MailboxPresetnW<Rst2Spec> {
        U0MailboxPresetnW::new(self, 4)
    }
    #[doc = "Bit 5 - Reset selector: u0_spi_apb"]
    #[inline(always)]
    #[must_use]
    pub fn u0_spi_apb(&mut self) -> U0SpiApbW<Rst2Spec> {
        U0SpiApbW::new(self, 5)
    }
    #[doc = "Bit 6 - Reset selector: u1_spi_apb"]
    #[inline(always)]
    #[must_use]
    pub fn u1_spi_apb(&mut self) -> U1SpiApbW<Rst2Spec> {
        U1SpiApbW::new(self, 6)
    }
    #[doc = "Bit 7 - Reset selector: u2_spi_apb"]
    #[inline(always)]
    #[must_use]
    pub fn u2_spi_apb(&mut self) -> U2SpiApbW<Rst2Spec> {
        U2SpiApbW::new(self, 7)
    }
    #[doc = "Bit 8 - Reset selector: u3_spi_apb"]
    #[inline(always)]
    #[must_use]
    pub fn u3_spi_apb(&mut self) -> U3SpiApbW<Rst2Spec> {
        U3SpiApbW::new(self, 8)
    }
    #[doc = "Bit 9 - Reset selector: u4_spi_apb"]
    #[inline(always)]
    #[must_use]
    pub fn u4_spi_apb(&mut self) -> U4SpiApbW<Rst2Spec> {
        U4SpiApbW::new(self, 9)
    }
    #[doc = "Bit 10 - Reset selector: u5_spi_apb"]
    #[inline(always)]
    #[must_use]
    pub fn u5_spi_apb(&mut self) -> U5SpiApbW<Rst2Spec> {
        U5SpiApbW::new(self, 10)
    }
    #[doc = "Bit 11 - Reset selector: u6_spi_apb"]
    #[inline(always)]
    #[must_use]
    pub fn u6_spi_apb(&mut self) -> U6SpiApbW<Rst2Spec> {
        U6SpiApbW::new(self, 11)
    }
    #[doc = "Bit 12 - Reset selector: u0_i2c_apb"]
    #[inline(always)]
    #[must_use]
    pub fn u0_i2c_apb(&mut self) -> U0I2cApbW<Rst2Spec> {
        U0I2cApbW::new(self, 12)
    }
    #[doc = "Bit 13 - Reset selector: u1_i2c_apb"]
    #[inline(always)]
    #[must_use]
    pub fn u1_i2c_apb(&mut self) -> U1I2cApbW<Rst2Spec> {
        U1I2cApbW::new(self, 13)
    }
    #[doc = "Bit 14 - Reset selector: u2_i2c_apb"]
    #[inline(always)]
    #[must_use]
    pub fn u2_i2c_apb(&mut self) -> U2I2cApbW<Rst2Spec> {
        U2I2cApbW::new(self, 14)
    }
    #[doc = "Bit 15 - Reset selector: u3_i2c_apb"]
    #[inline(always)]
    #[must_use]
    pub fn u3_i2c_apb(&mut self) -> U3I2cApbW<Rst2Spec> {
        U3I2cApbW::new(self, 15)
    }
    #[doc = "Bit 16 - Reset selector: u4_i2c_apb"]
    #[inline(always)]
    #[must_use]
    pub fn u4_i2c_apb(&mut self) -> U4I2cApbW<Rst2Spec> {
        U4I2cApbW::new(self, 16)
    }
    #[doc = "Bit 17 - Reset selector: u5_i2c_apb"]
    #[inline(always)]
    #[must_use]
    pub fn u5_i2c_apb(&mut self) -> U5I2cApbW<Rst2Spec> {
        U5I2cApbW::new(self, 17)
    }
    #[doc = "Bit 18 - Reset selector: u6_i2c_apb"]
    #[inline(always)]
    #[must_use]
    pub fn u6_i2c_apb(&mut self) -> U6I2cApbW<Rst2Spec> {
        U6I2cApbW::new(self, 18)
    }
    #[doc = "Bit 19 - Reset selector: u0_uart_apb"]
    #[inline(always)]
    #[must_use]
    pub fn u0_uart_apb(&mut self) -> U0UartApbW<Rst2Spec> {
        U0UartApbW::new(self, 19)
    }
    #[doc = "Bit 20 - Reset selector: u0_uart_core"]
    #[inline(always)]
    #[must_use]
    pub fn u0_uart_core(&mut self) -> U0UartCoreW<Rst2Spec> {
        U0UartCoreW::new(self, 20)
    }
    #[doc = "Bit 21 - Reset selector: u1_uart_apb"]
    #[inline(always)]
    #[must_use]
    pub fn u1_uart_apb(&mut self) -> U1UartApbW<Rst2Spec> {
        U1UartApbW::new(self, 21)
    }
    #[doc = "Bit 22 - Reset selector: u1_uart_core"]
    #[inline(always)]
    #[must_use]
    pub fn u1_uart_core(&mut self) -> U1UartCoreW<Rst2Spec> {
        U1UartCoreW::new(self, 22)
    }
    #[doc = "Bit 23 - Reset selector: u2_uart_apb"]
    #[inline(always)]
    #[must_use]
    pub fn u2_uart_apb(&mut self) -> U2UartApbW<Rst2Spec> {
        U2UartApbW::new(self, 23)
    }
    #[doc = "Bit 24 - Reset selector: u2_uart_core"]
    #[inline(always)]
    #[must_use]
    pub fn u2_uart_core(&mut self) -> U2UartCoreW<Rst2Spec> {
        U2UartCoreW::new(self, 24)
    }
    #[doc = "Bit 25 - Reset selector: u3_uart_apb"]
    #[inline(always)]
    #[must_use]
    pub fn u3_uart_apb(&mut self) -> U3UartApbW<Rst2Spec> {
        U3UartApbW::new(self, 25)
    }
    #[doc = "Bit 26 - Reset selector: u3_uart_core"]
    #[inline(always)]
    #[must_use]
    pub fn u3_uart_core(&mut self) -> U3UartCoreW<Rst2Spec> {
        U3UartCoreW::new(self, 26)
    }
    #[doc = "Bit 27 - Reset selector: u4_uart_apb"]
    #[inline(always)]
    #[must_use]
    pub fn u4_uart_apb(&mut self) -> U4UartApbW<Rst2Spec> {
        U4UartApbW::new(self, 27)
    }
    #[doc = "Bit 28 - Reset selector: u4_uart_core"]
    #[inline(always)]
    #[must_use]
    pub fn u4_uart_core(&mut self) -> U4UartCoreW<Rst2Spec> {
        U4UartCoreW::new(self, 28)
    }
    #[doc = "Bit 29 - Reset selector: u5_uart_apb"]
    #[inline(always)]
    #[must_use]
    pub fn u5_uart_apb(&mut self) -> U5UartApbW<Rst2Spec> {
        U5UartApbW::new(self, 29)
    }
    #[doc = "Bit 30 - Reset selector: u6_uart_core"]
    #[inline(always)]
    #[must_use]
    pub fn u6_uart_core(&mut self) -> U6UartCoreW<Rst2Spec> {
        U6UartCoreW::new(self, 30)
    }
    #[doc = "Bit 31 - Reset selector: u0_spdif_apb"]
    #[inline(always)]
    #[must_use]
    pub fn u0_spdif_apb(&mut self) -> U0SpdifApbW<Rst2Spec> {
        U0SpdifApbW::new(self, 31)
    }
}
#[doc = "RESET 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rst2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rst2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rst2Spec;
impl crate::RegisterSpec for Rst2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rst2::R`](R) reader structure"]
impl crate::Readable for Rst2Spec {}
#[doc = "`write(|w| ..)` method takes [`rst2::W`](W) writer structure"]
impl crate::Writable for Rst2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets rst2 to value 0xffe7_efcc"]
impl crate::Resettable for Rst2Spec {
    const RESET_VALUE: u32 = 0xffe7_efcc;
}
