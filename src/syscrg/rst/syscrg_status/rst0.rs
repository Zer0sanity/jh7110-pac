#[doc = "Register `rst0` reader"]
pub type R = crate::R<Rst0Spec>;
#[doc = "Register `rst0` writer"]
pub type W = crate::W<Rst0Spec>;
#[doc = "Reset selector: u0_jtag2apb_presetn\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U0Jtag2apbPresetn {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U0Jtag2apbPresetn> for bool {
    #[inline(always)]
    fn from(variant: U0Jtag2apbPresetn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u0_jtag2apb_presetn` reader - Reset selector: u0_jtag2apb_presetn"]
pub type U0Jtag2apbPresetnR = crate::BitReader<U0Jtag2apbPresetn>;
impl U0Jtag2apbPresetnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U0Jtag2apbPresetn {
        match self.bits {
            false => U0Jtag2apbPresetn::None,
            true => U0Jtag2apbPresetn::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U0Jtag2apbPresetn::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U0Jtag2apbPresetn::Reset
    }
}
#[doc = "Field `u0_jtag2apb_presetn` writer - Reset selector: u0_jtag2apb_presetn"]
pub type U0Jtag2apbPresetnW<'a, REG> = crate::BitWriter<'a, REG, U0Jtag2apbPresetn>;
impl<'a, REG> U0Jtag2apbPresetnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U0Jtag2apbPresetn::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U0Jtag2apbPresetn::Reset)
    }
}
#[doc = "Reset selector: u0_sys_syscon_presetn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U0SysSysconPresetn {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U0SysSysconPresetn> for bool {
    #[inline(always)]
    fn from(variant: U0SysSysconPresetn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u0_sys_syscon_presetn` reader - Reset selector: u0_sys_syscon_presetn"]
pub type U0SysSysconPresetnR = crate::BitReader<U0SysSysconPresetn>;
impl U0SysSysconPresetnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U0SysSysconPresetn {
        match self.bits {
            false => U0SysSysconPresetn::None,
            true => U0SysSysconPresetn::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U0SysSysconPresetn::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U0SysSysconPresetn::Reset
    }
}
#[doc = "Field `u0_sys_syscon_presetn` writer - Reset selector: u0_sys_syscon_presetn"]
pub type U0SysSysconPresetnW<'a, REG> = crate::BitWriter<'a, REG, U0SysSysconPresetn>;
impl<'a, REG> U0SysSysconPresetnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U0SysSysconPresetn::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U0SysSysconPresetn::Reset)
    }
}
#[doc = "Reset selector: u0_sys_iomux_presetn\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U0SysIomuxPresetn {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U0SysIomuxPresetn> for bool {
    #[inline(always)]
    fn from(variant: U0SysIomuxPresetn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u0_sys_iomux_presetn` reader - Reset selector: u0_sys_iomux_presetn"]
pub type U0SysIomuxPresetnR = crate::BitReader<U0SysIomuxPresetn>;
impl U0SysIomuxPresetnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U0SysIomuxPresetn {
        match self.bits {
            false => U0SysIomuxPresetn::None,
            true => U0SysIomuxPresetn::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U0SysIomuxPresetn::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U0SysIomuxPresetn::Reset
    }
}
#[doc = "Field `u0_sys_iomux_presetn` writer - Reset selector: u0_sys_iomux_presetn"]
pub type U0SysIomuxPresetnW<'a, REG> = crate::BitWriter<'a, REG, U0SysIomuxPresetn>;
impl<'a, REG> U0SysIomuxPresetnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U0SysIomuxPresetn::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U0SysIomuxPresetn::Reset)
    }
}
#[doc = "Reset selector: u0_bus\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U0Bus {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U0Bus> for bool {
    #[inline(always)]
    fn from(variant: U0Bus) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u0_bus` reader - Reset selector: u0_bus"]
pub type U0BusR = crate::BitReader<U0Bus>;
impl U0BusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U0Bus {
        match self.bits {
            false => U0Bus::None,
            true => U0Bus::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U0Bus::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U0Bus::Reset
    }
}
#[doc = "Field `u0_bus` writer - Reset selector: u0_bus"]
pub type U0BusW<'a, REG> = crate::BitWriter<'a, REG, U0Bus>;
impl<'a, REG> U0BusW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U0Bus::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U0Bus::Reset)
    }
}
#[doc = "Reset selector: u0_debug\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U0Debug {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U0Debug> for bool {
    #[inline(always)]
    fn from(variant: U0Debug) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u0_debug` reader - Reset selector: u0_debug"]
pub type U0DebugR = crate::BitReader<U0Debug>;
impl U0DebugR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U0Debug {
        match self.bits {
            false => U0Debug::None,
            true => U0Debug::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U0Debug::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U0Debug::Reset
    }
}
#[doc = "Field `u0_debug` writer - Reset selector: u0_debug"]
pub type U0DebugW<'a, REG> = crate::BitWriter<'a, REG, U0Debug>;
impl<'a, REG> U0DebugW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U0Debug::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U0Debug::Reset)
    }
}
#[doc = "Reset selector: u0_core_0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U0Core0 {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U0Core0> for bool {
    #[inline(always)]
    fn from(variant: U0Core0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u0_core_0` reader - Reset selector: u0_core_0"]
pub type U0Core0R = crate::BitReader<U0Core0>;
impl U0Core0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U0Core0 {
        match self.bits {
            false => U0Core0::None,
            true => U0Core0::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U0Core0::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U0Core0::Reset
    }
}
#[doc = "Field `u0_core_0` writer - Reset selector: u0_core_0"]
pub type U0Core0W<'a, REG> = crate::BitWriter<'a, REG, U0Core0>;
impl<'a, REG> U0Core0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U0Core0::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U0Core0::Reset)
    }
}
#[doc = "Reset selector: u0_core_1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U0Core1 {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U0Core1> for bool {
    #[inline(always)]
    fn from(variant: U0Core1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u0_core_1` reader - Reset selector: u0_core_1"]
pub type U0Core1R = crate::BitReader<U0Core1>;
impl U0Core1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U0Core1 {
        match self.bits {
            false => U0Core1::None,
            true => U0Core1::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U0Core1::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U0Core1::Reset
    }
}
#[doc = "Field `u0_core_1` writer - Reset selector: u0_core_1"]
pub type U0Core1W<'a, REG> = crate::BitWriter<'a, REG, U0Core1>;
impl<'a, REG> U0Core1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U0Core1::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U0Core1::Reset)
    }
}
#[doc = "Reset selector: u0_core_2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U0Core2 {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U0Core2> for bool {
    #[inline(always)]
    fn from(variant: U0Core2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u0_core_2` reader - Reset selector: u0_core_2"]
pub type U0Core2R = crate::BitReader<U0Core2>;
impl U0Core2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U0Core2 {
        match self.bits {
            false => U0Core2::None,
            true => U0Core2::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U0Core2::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U0Core2::Reset
    }
}
#[doc = "Field `u0_core_2` writer - Reset selector: u0_core_2"]
pub type U0Core2W<'a, REG> = crate::BitWriter<'a, REG, U0Core2>;
impl<'a, REG> U0Core2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U0Core2::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U0Core2::Reset)
    }
}
#[doc = "Reset selector: u0_core3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U0Core3 {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U0Core3> for bool {
    #[inline(always)]
    fn from(variant: U0Core3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u0_core3` reader - Reset selector: u0_core3"]
pub type U0Core3R = crate::BitReader<U0Core3>;
impl U0Core3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U0Core3 {
        match self.bits {
            false => U0Core3::None,
            true => U0Core3::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U0Core3::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U0Core3::Reset
    }
}
#[doc = "Field `u0_core3` writer - Reset selector: u0_core3"]
pub type U0Core3W<'a, REG> = crate::BitWriter<'a, REG, U0Core3>;
impl<'a, REG> U0Core3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U0Core3::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U0Core3::Reset)
    }
}
#[doc = "Reset selector: u0_core4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U0Core4 {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U0Core4> for bool {
    #[inline(always)]
    fn from(variant: U0Core4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u0_core4` reader - Reset selector: u0_core4"]
pub type U0Core4R = crate::BitReader<U0Core4>;
impl U0Core4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U0Core4 {
        match self.bits {
            false => U0Core4::None,
            true => U0Core4::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U0Core4::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U0Core4::Reset
    }
}
#[doc = "Field `u0_core4` writer - Reset selector: u0_core4"]
pub type U0Core4W<'a, REG> = crate::BitWriter<'a, REG, U0Core4>;
impl<'a, REG> U0Core4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U0Core4::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U0Core4::Reset)
    }
}
#[doc = "Reset selector: u0_core_st_0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U0CoreSt0 {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U0CoreSt0> for bool {
    #[inline(always)]
    fn from(variant: U0CoreSt0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u0_core_st_0` reader - Reset selector: u0_core_st_0"]
pub type U0CoreSt0R = crate::BitReader<U0CoreSt0>;
impl U0CoreSt0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U0CoreSt0 {
        match self.bits {
            false => U0CoreSt0::None,
            true => U0CoreSt0::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U0CoreSt0::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U0CoreSt0::Reset
    }
}
#[doc = "Field `u0_core_st_0` writer - Reset selector: u0_core_st_0"]
pub type U0CoreSt0W<'a, REG> = crate::BitWriter<'a, REG, U0CoreSt0>;
impl<'a, REG> U0CoreSt0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U0CoreSt0::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U0CoreSt0::Reset)
    }
}
#[doc = "Reset selector: u0_core_st_1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U0CoreSt1 {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U0CoreSt1> for bool {
    #[inline(always)]
    fn from(variant: U0CoreSt1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u0_core_st_1` reader - Reset selector: u0_core_st_1"]
pub type U0CoreSt1R = crate::BitReader<U0CoreSt1>;
impl U0CoreSt1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U0CoreSt1 {
        match self.bits {
            false => U0CoreSt1::None,
            true => U0CoreSt1::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U0CoreSt1::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U0CoreSt1::Reset
    }
}
#[doc = "Field `u0_core_st_1` writer - Reset selector: u0_core_st_1"]
pub type U0CoreSt1W<'a, REG> = crate::BitWriter<'a, REG, U0CoreSt1>;
impl<'a, REG> U0CoreSt1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U0CoreSt1::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U0CoreSt1::Reset)
    }
}
#[doc = "Reset selector: u0_core_st_2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U0CoreSt2 {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U0CoreSt2> for bool {
    #[inline(always)]
    fn from(variant: U0CoreSt2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u0_core_st_2` reader - Reset selector: u0_core_st_2"]
pub type U0CoreSt2R = crate::BitReader<U0CoreSt2>;
impl U0CoreSt2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U0CoreSt2 {
        match self.bits {
            false => U0CoreSt2::None,
            true => U0CoreSt2::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U0CoreSt2::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U0CoreSt2::Reset
    }
}
#[doc = "Field `u0_core_st_2` writer - Reset selector: u0_core_st_2"]
pub type U0CoreSt2W<'a, REG> = crate::BitWriter<'a, REG, U0CoreSt2>;
impl<'a, REG> U0CoreSt2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U0CoreSt2::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U0CoreSt2::Reset)
    }
}
#[doc = "Reset selector: u0_core_st_3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U0CoreSt3 {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U0CoreSt3> for bool {
    #[inline(always)]
    fn from(variant: U0CoreSt3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u0_core_st_3` reader - Reset selector: u0_core_st_3"]
pub type U0CoreSt3R = crate::BitReader<U0CoreSt3>;
impl U0CoreSt3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U0CoreSt3 {
        match self.bits {
            false => U0CoreSt3::None,
            true => U0CoreSt3::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U0CoreSt3::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U0CoreSt3::Reset
    }
}
#[doc = "Field `u0_core_st_3` writer - Reset selector: u0_core_st_3"]
pub type U0CoreSt3W<'a, REG> = crate::BitWriter<'a, REG, U0CoreSt3>;
impl<'a, REG> U0CoreSt3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U0CoreSt3::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U0CoreSt3::Reset)
    }
}
#[doc = "Reset selector: u0_core_st_4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U0CoreSt4 {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U0CoreSt4> for bool {
    #[inline(always)]
    fn from(variant: U0CoreSt4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u0_core_st_4` reader - Reset selector: u0_core_st_4"]
pub type U0CoreSt4R = crate::BitReader<U0CoreSt4>;
impl U0CoreSt4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U0CoreSt4 {
        match self.bits {
            false => U0CoreSt4::None,
            true => U0CoreSt4::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U0CoreSt4::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U0CoreSt4::Reset
    }
}
#[doc = "Field `u0_core_st_4` writer - Reset selector: u0_core_st_4"]
pub type U0CoreSt4W<'a, REG> = crate::BitWriter<'a, REG, U0CoreSt4>;
impl<'a, REG> U0CoreSt4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U0CoreSt4::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U0CoreSt4::Reset)
    }
}
#[doc = "Reset selector: u0_trace_0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U0Trace0 {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U0Trace0> for bool {
    #[inline(always)]
    fn from(variant: U0Trace0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u0_trace_0` reader - Reset selector: u0_trace_0"]
pub type U0Trace0R = crate::BitReader<U0Trace0>;
impl U0Trace0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U0Trace0 {
        match self.bits {
            false => U0Trace0::None,
            true => U0Trace0::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U0Trace0::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U0Trace0::Reset
    }
}
#[doc = "Field `u0_trace_0` writer - Reset selector: u0_trace_0"]
pub type U0Trace0W<'a, REG> = crate::BitWriter<'a, REG, U0Trace0>;
impl<'a, REG> U0Trace0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U0Trace0::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U0Trace0::Reset)
    }
}
#[doc = "Reset selector: u0_trace_1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U0Trace1 {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U0Trace1> for bool {
    #[inline(always)]
    fn from(variant: U0Trace1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u0_trace_1` reader - Reset selector: u0_trace_1"]
pub type U0Trace1R = crate::BitReader<U0Trace1>;
impl U0Trace1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U0Trace1 {
        match self.bits {
            false => U0Trace1::None,
            true => U0Trace1::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U0Trace1::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U0Trace1::Reset
    }
}
#[doc = "Field `u0_trace_1` writer - Reset selector: u0_trace_1"]
pub type U0Trace1W<'a, REG> = crate::BitWriter<'a, REG, U0Trace1>;
impl<'a, REG> U0Trace1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U0Trace1::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U0Trace1::Reset)
    }
}
#[doc = "Reset selector: u0_trace_2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U0Trace2 {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U0Trace2> for bool {
    #[inline(always)]
    fn from(variant: U0Trace2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u0_trace_2` reader - Reset selector: u0_trace_2"]
pub type U0Trace2R = crate::BitReader<U0Trace2>;
impl U0Trace2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U0Trace2 {
        match self.bits {
            false => U0Trace2::None,
            true => U0Trace2::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U0Trace2::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U0Trace2::Reset
    }
}
#[doc = "Field `u0_trace_2` writer - Reset selector: u0_trace_2"]
pub type U0Trace2W<'a, REG> = crate::BitWriter<'a, REG, U0Trace2>;
impl<'a, REG> U0Trace2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U0Trace2::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U0Trace2::Reset)
    }
}
#[doc = "Reset selector: u0_trace_3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U0Trace3 {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U0Trace3> for bool {
    #[inline(always)]
    fn from(variant: U0Trace3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u0_trace_3` reader - Reset selector: u0_trace_3"]
pub type U0Trace3R = crate::BitReader<U0Trace3>;
impl U0Trace3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U0Trace3 {
        match self.bits {
            false => U0Trace3::None,
            true => U0Trace3::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U0Trace3::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U0Trace3::Reset
    }
}
#[doc = "Field `u0_trace_3` writer - Reset selector: u0_trace_3"]
pub type U0Trace3W<'a, REG> = crate::BitWriter<'a, REG, U0Trace3>;
impl<'a, REG> U0Trace3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U0Trace3::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U0Trace3::Reset)
    }
}
#[doc = "Reset selector: u0_trace_4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U0Trace4 {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U0Trace4> for bool {
    #[inline(always)]
    fn from(variant: U0Trace4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u0_trace_4` reader - Reset selector: u0_trace_4"]
pub type U0Trace4R = crate::BitReader<U0Trace4>;
impl U0Trace4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U0Trace4 {
        match self.bits {
            false => U0Trace4::None,
            true => U0Trace4::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U0Trace4::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U0Trace4::Reset
    }
}
#[doc = "Field `u0_trace_4` writer - Reset selector: u0_trace_4"]
pub type U0Trace4W<'a, REG> = crate::BitWriter<'a, REG, U0Trace4>;
impl<'a, REG> U0Trace4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U0Trace4::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U0Trace4::Reset)
    }
}
#[doc = "Reset selector: u0_trace_com\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U0TraceCom {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U0TraceCom> for bool {
    #[inline(always)]
    fn from(variant: U0TraceCom) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u0_trace_com` reader - Reset selector: u0_trace_com"]
pub type U0TraceComR = crate::BitReader<U0TraceCom>;
impl U0TraceComR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U0TraceCom {
        match self.bits {
            false => U0TraceCom::None,
            true => U0TraceCom::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U0TraceCom::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U0TraceCom::Reset
    }
}
#[doc = "Field `u0_trace_com` writer - Reset selector: u0_trace_com"]
pub type U0TraceComW<'a, REG> = crate::BitWriter<'a, REG, U0TraceCom>;
impl<'a, REG> U0TraceComW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U0TraceCom::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U0TraceCom::Reset)
    }
}
#[doc = "Reset selector: u0_img_gpu_apb\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U0ImgGpuApb {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U0ImgGpuApb> for bool {
    #[inline(always)]
    fn from(variant: U0ImgGpuApb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u0_img_gpu_apb` reader - Reset selector: u0_img_gpu_apb"]
pub type U0ImgGpuApbR = crate::BitReader<U0ImgGpuApb>;
impl U0ImgGpuApbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U0ImgGpuApb {
        match self.bits {
            false => U0ImgGpuApb::None,
            true => U0ImgGpuApb::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U0ImgGpuApb::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U0ImgGpuApb::Reset
    }
}
#[doc = "Field `u0_img_gpu_apb` writer - Reset selector: u0_img_gpu_apb"]
pub type U0ImgGpuApbW<'a, REG> = crate::BitWriter<'a, REG, U0ImgGpuApb>;
impl<'a, REG> U0ImgGpuApbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U0ImgGpuApb::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U0ImgGpuApb::Reset)
    }
}
#[doc = "Reset selector: u0_img_gpu_doma\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U0ImgGpuDoma {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U0ImgGpuDoma> for bool {
    #[inline(always)]
    fn from(variant: U0ImgGpuDoma) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u0_img_gpu_doma` reader - Reset selector: u0_img_gpu_doma"]
pub type U0ImgGpuDomaR = crate::BitReader<U0ImgGpuDoma>;
impl U0ImgGpuDomaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U0ImgGpuDoma {
        match self.bits {
            false => U0ImgGpuDoma::None,
            true => U0ImgGpuDoma::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U0ImgGpuDoma::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U0ImgGpuDoma::Reset
    }
}
#[doc = "Field `u0_img_gpu_doma` writer - Reset selector: u0_img_gpu_doma"]
pub type U0ImgGpuDomaW<'a, REG> = crate::BitWriter<'a, REG, U0ImgGpuDoma>;
impl<'a, REG> U0ImgGpuDomaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U0ImgGpuDoma::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U0ImgGpuDoma::Reset)
    }
}
#[doc = "Reset selector: u0_noc_bus_apb\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U0NocBusApb {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U0NocBusApb> for bool {
    #[inline(always)]
    fn from(variant: U0NocBusApb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u0_noc_bus_apb` reader - Reset selector: u0_noc_bus_apb"]
pub type U0NocBusApbR = crate::BitReader<U0NocBusApb>;
impl U0NocBusApbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U0NocBusApb {
        match self.bits {
            false => U0NocBusApb::None,
            true => U0NocBusApb::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U0NocBusApb::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U0NocBusApb::Reset
    }
}
#[doc = "Field `u0_noc_bus_apb` writer - Reset selector: u0_noc_bus_apb"]
pub type U0NocBusApbW<'a, REG> = crate::BitWriter<'a, REG, U0NocBusApb>;
impl<'a, REG> U0NocBusApbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U0NocBusApb::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U0NocBusApb::Reset)
    }
}
#[doc = "Reset selector: u0_noc_bus_axicfg0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U0NocBusAxicfg0 {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U0NocBusAxicfg0> for bool {
    #[inline(always)]
    fn from(variant: U0NocBusAxicfg0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u0_noc_bus_axicfg0` reader - Reset selector: u0_noc_bus_axicfg0"]
pub type U0NocBusAxicfg0R = crate::BitReader<U0NocBusAxicfg0>;
impl U0NocBusAxicfg0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U0NocBusAxicfg0 {
        match self.bits {
            false => U0NocBusAxicfg0::None,
            true => U0NocBusAxicfg0::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U0NocBusAxicfg0::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U0NocBusAxicfg0::Reset
    }
}
#[doc = "Field `u0_noc_bus_axicfg0` writer - Reset selector: u0_noc_bus_axicfg0"]
pub type U0NocBusAxicfg0W<'a, REG> = crate::BitWriter<'a, REG, U0NocBusAxicfg0>;
impl<'a, REG> U0NocBusAxicfg0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U0NocBusAxicfg0::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U0NocBusAxicfg0::Reset)
    }
}
#[doc = "Reset selector: u0_noc_bus_cpu_axi\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U0NocBusCpuAxi {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U0NocBusCpuAxi> for bool {
    #[inline(always)]
    fn from(variant: U0NocBusCpuAxi) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u0_noc_bus_cpu_axi` reader - Reset selector: u0_noc_bus_cpu_axi"]
pub type U0NocBusCpuAxiR = crate::BitReader<U0NocBusCpuAxi>;
impl U0NocBusCpuAxiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U0NocBusCpuAxi {
        match self.bits {
            false => U0NocBusCpuAxi::None,
            true => U0NocBusCpuAxi::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U0NocBusCpuAxi::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U0NocBusCpuAxi::Reset
    }
}
#[doc = "Field `u0_noc_bus_cpu_axi` writer - Reset selector: u0_noc_bus_cpu_axi"]
pub type U0NocBusCpuAxiW<'a, REG> = crate::BitWriter<'a, REG, U0NocBusCpuAxi>;
impl<'a, REG> U0NocBusCpuAxiW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U0NocBusCpuAxi::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U0NocBusCpuAxi::Reset)
    }
}
#[doc = "Reset selector: u0_noc_bus_disp_axi\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U0NocBusDispAxi {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U0NocBusDispAxi> for bool {
    #[inline(always)]
    fn from(variant: U0NocBusDispAxi) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u0_noc_bus_disp_axi` reader - Reset selector: u0_noc_bus_disp_axi"]
pub type U0NocBusDispAxiR = crate::BitReader<U0NocBusDispAxi>;
impl U0NocBusDispAxiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U0NocBusDispAxi {
        match self.bits {
            false => U0NocBusDispAxi::None,
            true => U0NocBusDispAxi::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U0NocBusDispAxi::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U0NocBusDispAxi::Reset
    }
}
#[doc = "Field `u0_noc_bus_disp_axi` writer - Reset selector: u0_noc_bus_disp_axi"]
pub type U0NocBusDispAxiW<'a, REG> = crate::BitWriter<'a, REG, U0NocBusDispAxi>;
impl<'a, REG> U0NocBusDispAxiW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U0NocBusDispAxi::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U0NocBusDispAxi::Reset)
    }
}
#[doc = "Reset selector: u0_noc_bus_gpu_axi\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U0NocBusGpuAxi {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U0NocBusGpuAxi> for bool {
    #[inline(always)]
    fn from(variant: U0NocBusGpuAxi) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u0_noc_bus_gpu_axi` reader - Reset selector: u0_noc_bus_gpu_axi"]
pub type U0NocBusGpuAxiR = crate::BitReader<U0NocBusGpuAxi>;
impl U0NocBusGpuAxiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U0NocBusGpuAxi {
        match self.bits {
            false => U0NocBusGpuAxi::None,
            true => U0NocBusGpuAxi::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U0NocBusGpuAxi::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U0NocBusGpuAxi::Reset
    }
}
#[doc = "Field `u0_noc_bus_gpu_axi` writer - Reset selector: u0_noc_bus_gpu_axi"]
pub type U0NocBusGpuAxiW<'a, REG> = crate::BitWriter<'a, REG, U0NocBusGpuAxi>;
impl<'a, REG> U0NocBusGpuAxiW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U0NocBusGpuAxi::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U0NocBusGpuAxi::Reset)
    }
}
#[doc = "Reset selector: u0_noc_bus_isp_axi\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U0NocBusIspAxi {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U0NocBusIspAxi> for bool {
    #[inline(always)]
    fn from(variant: U0NocBusIspAxi) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u0_noc_bus_isp_axi` reader - Reset selector: u0_noc_bus_isp_axi"]
pub type U0NocBusIspAxiR = crate::BitReader<U0NocBusIspAxi>;
impl U0NocBusIspAxiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U0NocBusIspAxi {
        match self.bits {
            false => U0NocBusIspAxi::None,
            true => U0NocBusIspAxi::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U0NocBusIspAxi::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U0NocBusIspAxi::Reset
    }
}
#[doc = "Field `u0_noc_bus_isp_axi` writer - Reset selector: u0_noc_bus_isp_axi"]
pub type U0NocBusIspAxiW<'a, REG> = crate::BitWriter<'a, REG, U0NocBusIspAxi>;
impl<'a, REG> U0NocBusIspAxiW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U0NocBusIspAxi::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U0NocBusIspAxi::Reset)
    }
}
#[doc = "Reset selector: u0_noc_bus_ddrc\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U0NocBusDdrc {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U0NocBusDdrc> for bool {
    #[inline(always)]
    fn from(variant: U0NocBusDdrc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u0_noc_bus_ddrc` reader - Reset selector: u0_noc_bus_ddrc"]
pub type U0NocBusDdrcR = crate::BitReader<U0NocBusDdrc>;
impl U0NocBusDdrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U0NocBusDdrc {
        match self.bits {
            false => U0NocBusDdrc::None,
            true => U0NocBusDdrc::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U0NocBusDdrc::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U0NocBusDdrc::Reset
    }
}
#[doc = "Field `u0_noc_bus_ddrc` writer - Reset selector: u0_noc_bus_ddrc"]
pub type U0NocBusDdrcW<'a, REG> = crate::BitWriter<'a, REG, U0NocBusDdrc>;
impl<'a, REG> U0NocBusDdrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U0NocBusDdrc::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U0NocBusDdrc::Reset)
    }
}
#[doc = "Reset selector: u0_noc_bus_stg_axi\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U0NocBusStgAxi {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U0NocBusStgAxi> for bool {
    #[inline(always)]
    fn from(variant: U0NocBusStgAxi) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u0_noc_bus_stg_axi` reader - Reset selector: u0_noc_bus_stg_axi"]
pub type U0NocBusStgAxiR = crate::BitReader<U0NocBusStgAxi>;
impl U0NocBusStgAxiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U0NocBusStgAxi {
        match self.bits {
            false => U0NocBusStgAxi::None,
            true => U0NocBusStgAxi::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U0NocBusStgAxi::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U0NocBusStgAxi::Reset
    }
}
#[doc = "Field `u0_noc_bus_stg_axi` writer - Reset selector: u0_noc_bus_stg_axi"]
pub type U0NocBusStgAxiW<'a, REG> = crate::BitWriter<'a, REG, U0NocBusStgAxi>;
impl<'a, REG> U0NocBusStgAxiW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U0NocBusStgAxi::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U0NocBusStgAxi::Reset)
    }
}
#[doc = "Reset selector: u0_noc_bus_vdec_axi\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U0NocBusVdecAxi {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U0NocBusVdecAxi> for bool {
    #[inline(always)]
    fn from(variant: U0NocBusVdecAxi) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u0_noc_bus_vdec_axi` reader - Reset selector: u0_noc_bus_vdec_axi"]
pub type U0NocBusVdecAxiR = crate::BitReader<U0NocBusVdecAxi>;
impl U0NocBusVdecAxiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U0NocBusVdecAxi {
        match self.bits {
            false => U0NocBusVdecAxi::None,
            true => U0NocBusVdecAxi::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U0NocBusVdecAxi::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U0NocBusVdecAxi::Reset
    }
}
#[doc = "Field `u0_noc_bus_vdec_axi` writer - Reset selector: u0_noc_bus_vdec_axi"]
pub type U0NocBusVdecAxiW<'a, REG> = crate::BitWriter<'a, REG, U0NocBusVdecAxi>;
impl<'a, REG> U0NocBusVdecAxiW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U0NocBusVdecAxi::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U0NocBusVdecAxi::Reset)
    }
}
impl R {
    #[doc = "Bit 0 - Reset selector: u0_jtag2apb_presetn"]
    #[inline(always)]
    pub fn u0_jtag2apb_presetn(&self) -> U0Jtag2apbPresetnR {
        U0Jtag2apbPresetnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reset selector: u0_sys_syscon_presetn"]
    #[inline(always)]
    pub fn u0_sys_syscon_presetn(&self) -> U0SysSysconPresetnR {
        U0SysSysconPresetnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reset selector: u0_sys_iomux_presetn"]
    #[inline(always)]
    pub fn u0_sys_iomux_presetn(&self) -> U0SysIomuxPresetnR {
        U0SysIomuxPresetnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reset selector: u0_bus"]
    #[inline(always)]
    pub fn u0_bus(&self) -> U0BusR {
        U0BusR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Reset selector: u0_debug"]
    #[inline(always)]
    pub fn u0_debug(&self) -> U0DebugR {
        U0DebugR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Reset selector: u0_core_0"]
    #[inline(always)]
    pub fn u0_core_0(&self) -> U0Core0R {
        U0Core0R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Reset selector: u0_core_1"]
    #[inline(always)]
    pub fn u0_core_1(&self) -> U0Core1R {
        U0Core1R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Reset selector: u0_core_2"]
    #[inline(always)]
    pub fn u0_core_2(&self) -> U0Core2R {
        U0Core2R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Reset selector: u0_core3"]
    #[inline(always)]
    pub fn u0_core3(&self) -> U0Core3R {
        U0Core3R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Reset selector: u0_core4"]
    #[inline(always)]
    pub fn u0_core4(&self) -> U0Core4R {
        U0Core4R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Reset selector: u0_core_st_0"]
    #[inline(always)]
    pub fn u0_core_st_0(&self) -> U0CoreSt0R {
        U0CoreSt0R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Reset selector: u0_core_st_1"]
    #[inline(always)]
    pub fn u0_core_st_1(&self) -> U0CoreSt1R {
        U0CoreSt1R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Reset selector: u0_core_st_2"]
    #[inline(always)]
    pub fn u0_core_st_2(&self) -> U0CoreSt2R {
        U0CoreSt2R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Reset selector: u0_core_st_3"]
    #[inline(always)]
    pub fn u0_core_st_3(&self) -> U0CoreSt3R {
        U0CoreSt3R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Reset selector: u0_core_st_4"]
    #[inline(always)]
    pub fn u0_core_st_4(&self) -> U0CoreSt4R {
        U0CoreSt4R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Reset selector: u0_trace_0"]
    #[inline(always)]
    pub fn u0_trace_0(&self) -> U0Trace0R {
        U0Trace0R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Reset selector: u0_trace_1"]
    #[inline(always)]
    pub fn u0_trace_1(&self) -> U0Trace1R {
        U0Trace1R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Reset selector: u0_trace_2"]
    #[inline(always)]
    pub fn u0_trace_2(&self) -> U0Trace2R {
        U0Trace2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Reset selector: u0_trace_3"]
    #[inline(always)]
    pub fn u0_trace_3(&self) -> U0Trace3R {
        U0Trace3R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Reset selector: u0_trace_4"]
    #[inline(always)]
    pub fn u0_trace_4(&self) -> U0Trace4R {
        U0Trace4R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Reset selector: u0_trace_com"]
    #[inline(always)]
    pub fn u0_trace_com(&self) -> U0TraceComR {
        U0TraceComR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Reset selector: u0_img_gpu_apb"]
    #[inline(always)]
    pub fn u0_img_gpu_apb(&self) -> U0ImgGpuApbR {
        U0ImgGpuApbR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Reset selector: u0_img_gpu_doma"]
    #[inline(always)]
    pub fn u0_img_gpu_doma(&self) -> U0ImgGpuDomaR {
        U0ImgGpuDomaR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Reset selector: u0_noc_bus_apb"]
    #[inline(always)]
    pub fn u0_noc_bus_apb(&self) -> U0NocBusApbR {
        U0NocBusApbR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Reset selector: u0_noc_bus_axicfg0"]
    #[inline(always)]
    pub fn u0_noc_bus_axicfg0(&self) -> U0NocBusAxicfg0R {
        U0NocBusAxicfg0R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Reset selector: u0_noc_bus_cpu_axi"]
    #[inline(always)]
    pub fn u0_noc_bus_cpu_axi(&self) -> U0NocBusCpuAxiR {
        U0NocBusCpuAxiR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Reset selector: u0_noc_bus_disp_axi"]
    #[inline(always)]
    pub fn u0_noc_bus_disp_axi(&self) -> U0NocBusDispAxiR {
        U0NocBusDispAxiR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Reset selector: u0_noc_bus_gpu_axi"]
    #[inline(always)]
    pub fn u0_noc_bus_gpu_axi(&self) -> U0NocBusGpuAxiR {
        U0NocBusGpuAxiR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Reset selector: u0_noc_bus_isp_axi"]
    #[inline(always)]
    pub fn u0_noc_bus_isp_axi(&self) -> U0NocBusIspAxiR {
        U0NocBusIspAxiR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Reset selector: u0_noc_bus_ddrc"]
    #[inline(always)]
    pub fn u0_noc_bus_ddrc(&self) -> U0NocBusDdrcR {
        U0NocBusDdrcR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Reset selector: u0_noc_bus_stg_axi"]
    #[inline(always)]
    pub fn u0_noc_bus_stg_axi(&self) -> U0NocBusStgAxiR {
        U0NocBusStgAxiR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Reset selector: u0_noc_bus_vdec_axi"]
    #[inline(always)]
    pub fn u0_noc_bus_vdec_axi(&self) -> U0NocBusVdecAxiR {
        U0NocBusVdecAxiR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Reset selector: u0_jtag2apb_presetn"]
    #[inline(always)]
    #[must_use]
    pub fn u0_jtag2apb_presetn(&mut self) -> U0Jtag2apbPresetnW<Rst0Spec> {
        U0Jtag2apbPresetnW::new(self, 0)
    }
    #[doc = "Bit 1 - Reset selector: u0_sys_syscon_presetn"]
    #[inline(always)]
    #[must_use]
    pub fn u0_sys_syscon_presetn(&mut self) -> U0SysSysconPresetnW<Rst0Spec> {
        U0SysSysconPresetnW::new(self, 1)
    }
    #[doc = "Bit 2 - Reset selector: u0_sys_iomux_presetn"]
    #[inline(always)]
    #[must_use]
    pub fn u0_sys_iomux_presetn(&mut self) -> U0SysIomuxPresetnW<Rst0Spec> {
        U0SysIomuxPresetnW::new(self, 2)
    }
    #[doc = "Bit 3 - Reset selector: u0_bus"]
    #[inline(always)]
    #[must_use]
    pub fn u0_bus(&mut self) -> U0BusW<Rst0Spec> {
        U0BusW::new(self, 3)
    }
    #[doc = "Bit 4 - Reset selector: u0_debug"]
    #[inline(always)]
    #[must_use]
    pub fn u0_debug(&mut self) -> U0DebugW<Rst0Spec> {
        U0DebugW::new(self, 4)
    }
    #[doc = "Bit 5 - Reset selector: u0_core_0"]
    #[inline(always)]
    #[must_use]
    pub fn u0_core_0(&mut self) -> U0Core0W<Rst0Spec> {
        U0Core0W::new(self, 5)
    }
    #[doc = "Bit 6 - Reset selector: u0_core_1"]
    #[inline(always)]
    #[must_use]
    pub fn u0_core_1(&mut self) -> U0Core1W<Rst0Spec> {
        U0Core1W::new(self, 6)
    }
    #[doc = "Bit 7 - Reset selector: u0_core_2"]
    #[inline(always)]
    #[must_use]
    pub fn u0_core_2(&mut self) -> U0Core2W<Rst0Spec> {
        U0Core2W::new(self, 7)
    }
    #[doc = "Bit 8 - Reset selector: u0_core3"]
    #[inline(always)]
    #[must_use]
    pub fn u0_core3(&mut self) -> U0Core3W<Rst0Spec> {
        U0Core3W::new(self, 8)
    }
    #[doc = "Bit 9 - Reset selector: u0_core4"]
    #[inline(always)]
    #[must_use]
    pub fn u0_core4(&mut self) -> U0Core4W<Rst0Spec> {
        U0Core4W::new(self, 9)
    }
    #[doc = "Bit 10 - Reset selector: u0_core_st_0"]
    #[inline(always)]
    #[must_use]
    pub fn u0_core_st_0(&mut self) -> U0CoreSt0W<Rst0Spec> {
        U0CoreSt0W::new(self, 10)
    }
    #[doc = "Bit 11 - Reset selector: u0_core_st_1"]
    #[inline(always)]
    #[must_use]
    pub fn u0_core_st_1(&mut self) -> U0CoreSt1W<Rst0Spec> {
        U0CoreSt1W::new(self, 11)
    }
    #[doc = "Bit 12 - Reset selector: u0_core_st_2"]
    #[inline(always)]
    #[must_use]
    pub fn u0_core_st_2(&mut self) -> U0CoreSt2W<Rst0Spec> {
        U0CoreSt2W::new(self, 12)
    }
    #[doc = "Bit 13 - Reset selector: u0_core_st_3"]
    #[inline(always)]
    #[must_use]
    pub fn u0_core_st_3(&mut self) -> U0CoreSt3W<Rst0Spec> {
        U0CoreSt3W::new(self, 13)
    }
    #[doc = "Bit 14 - Reset selector: u0_core_st_4"]
    #[inline(always)]
    #[must_use]
    pub fn u0_core_st_4(&mut self) -> U0CoreSt4W<Rst0Spec> {
        U0CoreSt4W::new(self, 14)
    }
    #[doc = "Bit 15 - Reset selector: u0_trace_0"]
    #[inline(always)]
    #[must_use]
    pub fn u0_trace_0(&mut self) -> U0Trace0W<Rst0Spec> {
        U0Trace0W::new(self, 15)
    }
    #[doc = "Bit 16 - Reset selector: u0_trace_1"]
    #[inline(always)]
    #[must_use]
    pub fn u0_trace_1(&mut self) -> U0Trace1W<Rst0Spec> {
        U0Trace1W::new(self, 16)
    }
    #[doc = "Bit 17 - Reset selector: u0_trace_2"]
    #[inline(always)]
    #[must_use]
    pub fn u0_trace_2(&mut self) -> U0Trace2W<Rst0Spec> {
        U0Trace2W::new(self, 17)
    }
    #[doc = "Bit 18 - Reset selector: u0_trace_3"]
    #[inline(always)]
    #[must_use]
    pub fn u0_trace_3(&mut self) -> U0Trace3W<Rst0Spec> {
        U0Trace3W::new(self, 18)
    }
    #[doc = "Bit 19 - Reset selector: u0_trace_4"]
    #[inline(always)]
    #[must_use]
    pub fn u0_trace_4(&mut self) -> U0Trace4W<Rst0Spec> {
        U0Trace4W::new(self, 19)
    }
    #[doc = "Bit 20 - Reset selector: u0_trace_com"]
    #[inline(always)]
    #[must_use]
    pub fn u0_trace_com(&mut self) -> U0TraceComW<Rst0Spec> {
        U0TraceComW::new(self, 20)
    }
    #[doc = "Bit 21 - Reset selector: u0_img_gpu_apb"]
    #[inline(always)]
    #[must_use]
    pub fn u0_img_gpu_apb(&mut self) -> U0ImgGpuApbW<Rst0Spec> {
        U0ImgGpuApbW::new(self, 21)
    }
    #[doc = "Bit 22 - Reset selector: u0_img_gpu_doma"]
    #[inline(always)]
    #[must_use]
    pub fn u0_img_gpu_doma(&mut self) -> U0ImgGpuDomaW<Rst0Spec> {
        U0ImgGpuDomaW::new(self, 22)
    }
    #[doc = "Bit 23 - Reset selector: u0_noc_bus_apb"]
    #[inline(always)]
    #[must_use]
    pub fn u0_noc_bus_apb(&mut self) -> U0NocBusApbW<Rst0Spec> {
        U0NocBusApbW::new(self, 23)
    }
    #[doc = "Bit 24 - Reset selector: u0_noc_bus_axicfg0"]
    #[inline(always)]
    #[must_use]
    pub fn u0_noc_bus_axicfg0(&mut self) -> U0NocBusAxicfg0W<Rst0Spec> {
        U0NocBusAxicfg0W::new(self, 24)
    }
    #[doc = "Bit 25 - Reset selector: u0_noc_bus_cpu_axi"]
    #[inline(always)]
    #[must_use]
    pub fn u0_noc_bus_cpu_axi(&mut self) -> U0NocBusCpuAxiW<Rst0Spec> {
        U0NocBusCpuAxiW::new(self, 25)
    }
    #[doc = "Bit 26 - Reset selector: u0_noc_bus_disp_axi"]
    #[inline(always)]
    #[must_use]
    pub fn u0_noc_bus_disp_axi(&mut self) -> U0NocBusDispAxiW<Rst0Spec> {
        U0NocBusDispAxiW::new(self, 26)
    }
    #[doc = "Bit 27 - Reset selector: u0_noc_bus_gpu_axi"]
    #[inline(always)]
    #[must_use]
    pub fn u0_noc_bus_gpu_axi(&mut self) -> U0NocBusGpuAxiW<Rst0Spec> {
        U0NocBusGpuAxiW::new(self, 27)
    }
    #[doc = "Bit 28 - Reset selector: u0_noc_bus_isp_axi"]
    #[inline(always)]
    #[must_use]
    pub fn u0_noc_bus_isp_axi(&mut self) -> U0NocBusIspAxiW<Rst0Spec> {
        U0NocBusIspAxiW::new(self, 28)
    }
    #[doc = "Bit 29 - Reset selector: u0_noc_bus_ddrc"]
    #[inline(always)]
    #[must_use]
    pub fn u0_noc_bus_ddrc(&mut self) -> U0NocBusDdrcW<Rst0Spec> {
        U0NocBusDdrcW::new(self, 29)
    }
    #[doc = "Bit 30 - Reset selector: u0_noc_bus_stg_axi"]
    #[inline(always)]
    #[must_use]
    pub fn u0_noc_bus_stg_axi(&mut self) -> U0NocBusStgAxiW<Rst0Spec> {
        U0NocBusStgAxiW::new(self, 30)
    }
    #[doc = "Bit 31 - Reset selector: u0_noc_bus_vdec_axi"]
    #[inline(always)]
    #[must_use]
    pub fn u0_noc_bus_vdec_axi(&mut self) -> U0NocBusVdecAxiW<Rst0Spec> {
        U0NocBusVdecAxiW::new(self, 31)
    }
}
#[doc = "RESET 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rst0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rst0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rst0Spec;
impl crate::RegisterSpec for Rst0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rst0::R`](R) reader structure"]
impl crate::Readable for Rst0Spec {}
#[doc = "`write(|w| ..)` method takes [`rst0::W`](W) writer structure"]
impl crate::Writable for Rst0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets rst0 to value 0x0060_0001"]
impl crate::Resettable for Rst0Spec {
    const RESET_VALUE: u32 = 0x0060_0001;
}
