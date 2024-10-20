#[doc = "Register `client_filter` reader"]
pub type R = crate::R<ClientFilterSpec>;
#[doc = "Register `client_filter` writer"]
pub type W = crate::W<ClientFilterSpec>;
#[doc = "Disable counter events originating from `Debug` client.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Debug {
    #[doc = "0: Enable events from the client"]
    Enable = 0,
    #[doc = "1: Disable events from the client"]
    Disable = 1,
}
impl From<Debug> for bool {
    #[inline(always)]
    fn from(variant: Debug) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `debug` reader - Disable counter events originating from `Debug` client."]
pub type DebugR = crate::BitReader<Debug>;
impl DebugR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Debug {
        match self.bits {
            false => Debug::Enable,
            true => Debug::Disable,
        }
    }
    #[doc = "Enable events from the client"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Debug::Enable
    }
    #[doc = "Disable events from the client"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Debug::Disable
    }
}
#[doc = "Field `debug` writer - Disable counter events originating from `Debug` client."]
pub type DebugW<'a, REG> = crate::BitWriter<'a, REG, Debug>;
impl<'a, REG> DebugW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable events from the client"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Debug::Enable)
    }
    #[doc = "Disable events from the client"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Debug::Disable)
    }
}
#[doc = "Disable counter events originating from `Debug` client.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Axi4FrontPort {
    #[doc = "0: Enable events from the client"]
    Enable = 0,
    #[doc = "1: Disable events from the client"]
    Disable = 1,
}
impl From<Axi4FrontPort> for bool {
    #[inline(always)]
    fn from(variant: Axi4FrontPort) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `axi4_front_port(0-3)` reader - Disable counter events originating from `Debug` client."]
pub type Axi4FrontPortR = crate::BitReader<Axi4FrontPort>;
impl Axi4FrontPortR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Axi4FrontPort {
        match self.bits {
            false => Axi4FrontPort::Enable,
            true => Axi4FrontPort::Disable,
        }
    }
    #[doc = "Enable events from the client"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Axi4FrontPort::Enable
    }
    #[doc = "Disable events from the client"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Axi4FrontPort::Disable
    }
}
#[doc = "Field `axi4_front_port(0-3)` writer - Disable counter events originating from `Debug` client."]
pub type Axi4FrontPortW<'a, REG> = crate::BitWriter<'a, REG, Axi4FrontPort>;
impl<'a, REG> Axi4FrontPortW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable events from the client"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Axi4FrontPort::Enable)
    }
    #[doc = "Disable events from the client"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Axi4FrontPort::Disable)
    }
}
#[doc = "Disable counter events originating from `Hart 0 Fetch Unit` client.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hart0FetchUnit {
    #[doc = "0: Enable events from the client"]
    Enable = 0,
    #[doc = "1: Disable events from the client"]
    Disable = 1,
}
impl From<Hart0FetchUnit> for bool {
    #[inline(always)]
    fn from(variant: Hart0FetchUnit) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `hart0_fetch_unit` reader - Disable counter events originating from `Hart 0 Fetch Unit` client."]
pub type Hart0FetchUnitR = crate::BitReader<Hart0FetchUnit>;
impl Hart0FetchUnitR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hart0FetchUnit {
        match self.bits {
            false => Hart0FetchUnit::Enable,
            true => Hart0FetchUnit::Disable,
        }
    }
    #[doc = "Enable events from the client"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Hart0FetchUnit::Enable
    }
    #[doc = "Disable events from the client"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Hart0FetchUnit::Disable
    }
}
#[doc = "Field `hart0_fetch_unit` writer - Disable counter events originating from `Hart 0 Fetch Unit` client."]
pub type Hart0FetchUnitW<'a, REG> = crate::BitWriter<'a, REG, Hart0FetchUnit>;
impl<'a, REG> Hart0FetchUnitW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable events from the client"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Hart0FetchUnit::Enable)
    }
    #[doc = "Disable events from the client"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Hart0FetchUnit::Disable)
    }
}
#[doc = "Disable counter events originating from `Hart 0 D-Cache MMIO` client.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hart0DcacheMmio {
    #[doc = "0: Enable events from the client"]
    Enable = 0,
    #[doc = "1: Disable events from the client"]
    Disable = 1,
}
impl From<Hart0DcacheMmio> for bool {
    #[inline(always)]
    fn from(variant: Hart0DcacheMmio) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `hart0_dcache_mmio` reader - Disable counter events originating from `Hart 0 D-Cache MMIO` client."]
pub type Hart0DcacheMmioR = crate::BitReader<Hart0DcacheMmio>;
impl Hart0DcacheMmioR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hart0DcacheMmio {
        match self.bits {
            false => Hart0DcacheMmio::Enable,
            true => Hart0DcacheMmio::Disable,
        }
    }
    #[doc = "Enable events from the client"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Hart0DcacheMmio::Enable
    }
    #[doc = "Disable events from the client"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Hart0DcacheMmio::Disable
    }
}
#[doc = "Field `hart0_dcache_mmio` writer - Disable counter events originating from `Hart 0 D-Cache MMIO` client."]
pub type Hart0DcacheMmioW<'a, REG> = crate::BitWriter<'a, REG, Hart0DcacheMmio>;
impl<'a, REG> Hart0DcacheMmioW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable events from the client"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Hart0DcacheMmio::Enable)
    }
    #[doc = "Disable events from the client"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Hart0DcacheMmio::Disable)
    }
}
#[doc = "Disable counter events originating from `Hart 1 Fetch Unit` client.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hart1FetchUnit {
    #[doc = "0: Enable events from the client"]
    Enable = 0,
    #[doc = "1: Disable events from the client"]
    Disable = 1,
}
impl From<Hart1FetchUnit> for bool {
    #[inline(always)]
    fn from(variant: Hart1FetchUnit) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `hart1_fetch_unit` reader - Disable counter events originating from `Hart 1 Fetch Unit` client."]
pub type Hart1FetchUnitR = crate::BitReader<Hart1FetchUnit>;
impl Hart1FetchUnitR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hart1FetchUnit {
        match self.bits {
            false => Hart1FetchUnit::Enable,
            true => Hart1FetchUnit::Disable,
        }
    }
    #[doc = "Enable events from the client"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Hart1FetchUnit::Enable
    }
    #[doc = "Disable events from the client"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Hart1FetchUnit::Disable
    }
}
#[doc = "Field `hart1_fetch_unit` writer - Disable counter events originating from `Hart 1 Fetch Unit` client."]
pub type Hart1FetchUnitW<'a, REG> = crate::BitWriter<'a, REG, Hart1FetchUnit>;
impl<'a, REG> Hart1FetchUnitW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable events from the client"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Hart1FetchUnit::Enable)
    }
    #[doc = "Disable events from the client"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Hart1FetchUnit::Disable)
    }
}
#[doc = "Disable counter events originating from `Hart 1 I-Cache` client.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hart1Icache {
    #[doc = "0: Enable events from the client"]
    Enable = 0,
    #[doc = "1: Disable events from the client"]
    Disable = 1,
}
impl From<Hart1Icache> for bool {
    #[inline(always)]
    fn from(variant: Hart1Icache) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `hart1_icache` reader - Disable counter events originating from `Hart 1 I-Cache` client."]
pub type Hart1IcacheR = crate::BitReader<Hart1Icache>;
impl Hart1IcacheR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hart1Icache {
        match self.bits {
            false => Hart1Icache::Enable,
            true => Hart1Icache::Disable,
        }
    }
    #[doc = "Enable events from the client"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Hart1Icache::Enable
    }
    #[doc = "Disable events from the client"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Hart1Icache::Disable
    }
}
#[doc = "Field `hart1_icache` writer - Disable counter events originating from `Hart 1 I-Cache` client."]
pub type Hart1IcacheW<'a, REG> = crate::BitWriter<'a, REG, Hart1Icache>;
impl<'a, REG> Hart1IcacheW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable events from the client"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Hart1Icache::Enable)
    }
    #[doc = "Disable events from the client"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Hart1Icache::Disable)
    }
}
#[doc = "Disable counter events originating from `Hart 1 D-Cache` client.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hart1Dcache {
    #[doc = "0: Enable events from the client"]
    Enable = 0,
    #[doc = "1: Disable events from the client"]
    Disable = 1,
}
impl From<Hart1Dcache> for bool {
    #[inline(always)]
    fn from(variant: Hart1Dcache) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `hart1_dcache` reader - Disable counter events originating from `Hart 1 D-Cache` client."]
pub type Hart1DcacheR = crate::BitReader<Hart1Dcache>;
impl Hart1DcacheR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hart1Dcache {
        match self.bits {
            false => Hart1Dcache::Enable,
            true => Hart1Dcache::Disable,
        }
    }
    #[doc = "Enable events from the client"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Hart1Dcache::Enable
    }
    #[doc = "Disable events from the client"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Hart1Dcache::Disable
    }
}
#[doc = "Field `hart1_dcache` writer - Disable counter events originating from `Hart 1 D-Cache` client."]
pub type Hart1DcacheW<'a, REG> = crate::BitWriter<'a, REG, Hart1Dcache>;
impl<'a, REG> Hart1DcacheW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable events from the client"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Hart1Dcache::Enable)
    }
    #[doc = "Disable events from the client"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Hart1Dcache::Disable)
    }
}
#[doc = "Disable counter events originating from `Hart 1 D-Cache MMIO` client.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hart1DcacheMmio {
    #[doc = "0: Enable events from the client"]
    Enable = 0,
    #[doc = "1: Disable events from the client"]
    Disable = 1,
}
impl From<Hart1DcacheMmio> for bool {
    #[inline(always)]
    fn from(variant: Hart1DcacheMmio) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `hart1_dcache_mmio` reader - Disable counter events originating from `Hart 1 D-Cache MMIO` client."]
pub type Hart1DcacheMmioR = crate::BitReader<Hart1DcacheMmio>;
impl Hart1DcacheMmioR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hart1DcacheMmio {
        match self.bits {
            false => Hart1DcacheMmio::Enable,
            true => Hart1DcacheMmio::Disable,
        }
    }
    #[doc = "Enable events from the client"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Hart1DcacheMmio::Enable
    }
    #[doc = "Disable events from the client"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Hart1DcacheMmio::Disable
    }
}
#[doc = "Field `hart1_dcache_mmio` writer - Disable counter events originating from `Hart 1 D-Cache MMIO` client."]
pub type Hart1DcacheMmioW<'a, REG> = crate::BitWriter<'a, REG, Hart1DcacheMmio>;
impl<'a, REG> Hart1DcacheMmioW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable events from the client"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Hart1DcacheMmio::Enable)
    }
    #[doc = "Disable events from the client"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Hart1DcacheMmio::Disable)
    }
}
#[doc = "Disable counter events originating from `Hart 1 L2 Prefetcher` client.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hart1L2Prefetcher {
    #[doc = "0: Enable events from the client"]
    Enable = 0,
    #[doc = "1: Disable events from the client"]
    Disable = 1,
}
impl From<Hart1L2Prefetcher> for bool {
    #[inline(always)]
    fn from(variant: Hart1L2Prefetcher) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `hart1_l2_prefetcher` reader - Disable counter events originating from `Hart 1 L2 Prefetcher` client."]
pub type Hart1L2PrefetcherR = crate::BitReader<Hart1L2Prefetcher>;
impl Hart1L2PrefetcherR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hart1L2Prefetcher {
        match self.bits {
            false => Hart1L2Prefetcher::Enable,
            true => Hart1L2Prefetcher::Disable,
        }
    }
    #[doc = "Enable events from the client"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Hart1L2Prefetcher::Enable
    }
    #[doc = "Disable events from the client"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Hart1L2Prefetcher::Disable
    }
}
#[doc = "Field `hart1_l2_prefetcher` writer - Disable counter events originating from `Hart 1 L2 Prefetcher` client."]
pub type Hart1L2PrefetcherW<'a, REG> = crate::BitWriter<'a, REG, Hart1L2Prefetcher>;
impl<'a, REG> Hart1L2PrefetcherW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable events from the client"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Hart1L2Prefetcher::Enable)
    }
    #[doc = "Disable events from the client"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Hart1L2Prefetcher::Disable)
    }
}
#[doc = "Disable counter events originating from `Hart 2 Fetch Unit` client.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hart2FetchUnit {
    #[doc = "0: Enable events from the client"]
    Enable = 0,
    #[doc = "1: Disable events from the client"]
    Disable = 1,
}
impl From<Hart2FetchUnit> for bool {
    #[inline(always)]
    fn from(variant: Hart2FetchUnit) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `hart2_fetch_unit` reader - Disable counter events originating from `Hart 2 Fetch Unit` client."]
pub type Hart2FetchUnitR = crate::BitReader<Hart2FetchUnit>;
impl Hart2FetchUnitR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hart2FetchUnit {
        match self.bits {
            false => Hart2FetchUnit::Enable,
            true => Hart2FetchUnit::Disable,
        }
    }
    #[doc = "Enable events from the client"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Hart2FetchUnit::Enable
    }
    #[doc = "Disable events from the client"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Hart2FetchUnit::Disable
    }
}
#[doc = "Field `hart2_fetch_unit` writer - Disable counter events originating from `Hart 2 Fetch Unit` client."]
pub type Hart2FetchUnitW<'a, REG> = crate::BitWriter<'a, REG, Hart2FetchUnit>;
impl<'a, REG> Hart2FetchUnitW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable events from the client"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Hart2FetchUnit::Enable)
    }
    #[doc = "Disable events from the client"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Hart2FetchUnit::Disable)
    }
}
#[doc = "Disable counter events originating from `Hart 2 I-Cache` client.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hart2Icache {
    #[doc = "0: Enable events from the client"]
    Enable = 0,
    #[doc = "1: Disable events from the client"]
    Disable = 1,
}
impl From<Hart2Icache> for bool {
    #[inline(always)]
    fn from(variant: Hart2Icache) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `hart2_icache` reader - Disable counter events originating from `Hart 2 I-Cache` client."]
pub type Hart2IcacheR = crate::BitReader<Hart2Icache>;
impl Hart2IcacheR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hart2Icache {
        match self.bits {
            false => Hart2Icache::Enable,
            true => Hart2Icache::Disable,
        }
    }
    #[doc = "Enable events from the client"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Hart2Icache::Enable
    }
    #[doc = "Disable events from the client"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Hart2Icache::Disable
    }
}
#[doc = "Field `hart2_icache` writer - Disable counter events originating from `Hart 2 I-Cache` client."]
pub type Hart2IcacheW<'a, REG> = crate::BitWriter<'a, REG, Hart2Icache>;
impl<'a, REG> Hart2IcacheW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable events from the client"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Hart2Icache::Enable)
    }
    #[doc = "Disable events from the client"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Hart2Icache::Disable)
    }
}
#[doc = "Disable counter events originating from `Hart 2 D-Cache` client.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hart2Dcache {
    #[doc = "0: Enable events from the client"]
    Enable = 0,
    #[doc = "1: Disable events from the client"]
    Disable = 1,
}
impl From<Hart2Dcache> for bool {
    #[inline(always)]
    fn from(variant: Hart2Dcache) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `hart2_dcache` reader - Disable counter events originating from `Hart 2 D-Cache` client."]
pub type Hart2DcacheR = crate::BitReader<Hart2Dcache>;
impl Hart2DcacheR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hart2Dcache {
        match self.bits {
            false => Hart2Dcache::Enable,
            true => Hart2Dcache::Disable,
        }
    }
    #[doc = "Enable events from the client"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Hart2Dcache::Enable
    }
    #[doc = "Disable events from the client"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Hart2Dcache::Disable
    }
}
#[doc = "Field `hart2_dcache` writer - Disable counter events originating from `Hart 2 D-Cache` client."]
pub type Hart2DcacheW<'a, REG> = crate::BitWriter<'a, REG, Hart2Dcache>;
impl<'a, REG> Hart2DcacheW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable events from the client"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Hart2Dcache::Enable)
    }
    #[doc = "Disable events from the client"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Hart2Dcache::Disable)
    }
}
#[doc = "Disable counter events originating from `Hart 2 D-Cache MMIO` client.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hart2DcacheMmio {
    #[doc = "0: Enable events from the client"]
    Enable = 0,
    #[doc = "1: Disable events from the client"]
    Disable = 1,
}
impl From<Hart2DcacheMmio> for bool {
    #[inline(always)]
    fn from(variant: Hart2DcacheMmio) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `hart2_dcache_mmio` reader - Disable counter events originating from `Hart 2 D-Cache MMIO` client."]
pub type Hart2DcacheMmioR = crate::BitReader<Hart2DcacheMmio>;
impl Hart2DcacheMmioR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hart2DcacheMmio {
        match self.bits {
            false => Hart2DcacheMmio::Enable,
            true => Hart2DcacheMmio::Disable,
        }
    }
    #[doc = "Enable events from the client"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Hart2DcacheMmio::Enable
    }
    #[doc = "Disable events from the client"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Hart2DcacheMmio::Disable
    }
}
#[doc = "Field `hart2_dcache_mmio` writer - Disable counter events originating from `Hart 2 D-Cache MMIO` client."]
pub type Hart2DcacheMmioW<'a, REG> = crate::BitWriter<'a, REG, Hart2DcacheMmio>;
impl<'a, REG> Hart2DcacheMmioW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable events from the client"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Hart2DcacheMmio::Enable)
    }
    #[doc = "Disable events from the client"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Hart2DcacheMmio::Disable)
    }
}
#[doc = "Disable counter events originating from `Hart 2 L2 Prefetcher` client.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hart2L2Prefetcher {
    #[doc = "0: Enable events from the client"]
    Enable = 0,
    #[doc = "1: Disable events from the client"]
    Disable = 1,
}
impl From<Hart2L2Prefetcher> for bool {
    #[inline(always)]
    fn from(variant: Hart2L2Prefetcher) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `hart2_l2_prefetcher` reader - Disable counter events originating from `Hart 2 L2 Prefetcher` client."]
pub type Hart2L2PrefetcherR = crate::BitReader<Hart2L2Prefetcher>;
impl Hart2L2PrefetcherR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hart2L2Prefetcher {
        match self.bits {
            false => Hart2L2Prefetcher::Enable,
            true => Hart2L2Prefetcher::Disable,
        }
    }
    #[doc = "Enable events from the client"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Hart2L2Prefetcher::Enable
    }
    #[doc = "Disable events from the client"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Hart2L2Prefetcher::Disable
    }
}
#[doc = "Field `hart2_l2_prefetcher` writer - Disable counter events originating from `Hart 2 L2 Prefetcher` client."]
pub type Hart2L2PrefetcherW<'a, REG> = crate::BitWriter<'a, REG, Hart2L2Prefetcher>;
impl<'a, REG> Hart2L2PrefetcherW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable events from the client"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Hart2L2Prefetcher::Enable)
    }
    #[doc = "Disable events from the client"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Hart2L2Prefetcher::Disable)
    }
}
#[doc = "Disable counter events originating from `Hart 3 Fetch Unit` client.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hart3FetchUnit {
    #[doc = "0: Enable events from the client"]
    Enable = 0,
    #[doc = "1: Disable events from the client"]
    Disable = 1,
}
impl From<Hart3FetchUnit> for bool {
    #[inline(always)]
    fn from(variant: Hart3FetchUnit) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `hart3_fetch_unit` reader - Disable counter events originating from `Hart 3 Fetch Unit` client."]
pub type Hart3FetchUnitR = crate::BitReader<Hart3FetchUnit>;
impl Hart3FetchUnitR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hart3FetchUnit {
        match self.bits {
            false => Hart3FetchUnit::Enable,
            true => Hart3FetchUnit::Disable,
        }
    }
    #[doc = "Enable events from the client"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Hart3FetchUnit::Enable
    }
    #[doc = "Disable events from the client"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Hart3FetchUnit::Disable
    }
}
#[doc = "Field `hart3_fetch_unit` writer - Disable counter events originating from `Hart 3 Fetch Unit` client."]
pub type Hart3FetchUnitW<'a, REG> = crate::BitWriter<'a, REG, Hart3FetchUnit>;
impl<'a, REG> Hart3FetchUnitW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable events from the client"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Hart3FetchUnit::Enable)
    }
    #[doc = "Disable events from the client"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Hart3FetchUnit::Disable)
    }
}
#[doc = "Disable counter events originating from `Hart 3 I-Cache` client.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hart3Icache {
    #[doc = "0: Enable events from the client"]
    Enable = 0,
    #[doc = "1: Disable events from the client"]
    Disable = 1,
}
impl From<Hart3Icache> for bool {
    #[inline(always)]
    fn from(variant: Hart3Icache) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `hart3_icache` reader - Disable counter events originating from `Hart 3 I-Cache` client."]
pub type Hart3IcacheR = crate::BitReader<Hart3Icache>;
impl Hart3IcacheR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hart3Icache {
        match self.bits {
            false => Hart3Icache::Enable,
            true => Hart3Icache::Disable,
        }
    }
    #[doc = "Enable events from the client"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Hart3Icache::Enable
    }
    #[doc = "Disable events from the client"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Hart3Icache::Disable
    }
}
#[doc = "Field `hart3_icache` writer - Disable counter events originating from `Hart 3 I-Cache` client."]
pub type Hart3IcacheW<'a, REG> = crate::BitWriter<'a, REG, Hart3Icache>;
impl<'a, REG> Hart3IcacheW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable events from the client"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Hart3Icache::Enable)
    }
    #[doc = "Disable events from the client"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Hart3Icache::Disable)
    }
}
#[doc = "Disable counter events originating from `Hart 3 D-Cache` client.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hart3Dcache {
    #[doc = "0: Enable events from the client"]
    Enable = 0,
    #[doc = "1: Disable events from the client"]
    Disable = 1,
}
impl From<Hart3Dcache> for bool {
    #[inline(always)]
    fn from(variant: Hart3Dcache) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `hart3_dcache` reader - Disable counter events originating from `Hart 3 D-Cache` client."]
pub type Hart3DcacheR = crate::BitReader<Hart3Dcache>;
impl Hart3DcacheR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hart3Dcache {
        match self.bits {
            false => Hart3Dcache::Enable,
            true => Hart3Dcache::Disable,
        }
    }
    #[doc = "Enable events from the client"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Hart3Dcache::Enable
    }
    #[doc = "Disable events from the client"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Hart3Dcache::Disable
    }
}
#[doc = "Field `hart3_dcache` writer - Disable counter events originating from `Hart 3 D-Cache` client."]
pub type Hart3DcacheW<'a, REG> = crate::BitWriter<'a, REG, Hart3Dcache>;
impl<'a, REG> Hart3DcacheW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable events from the client"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Hart3Dcache::Enable)
    }
    #[doc = "Disable events from the client"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Hart3Dcache::Disable)
    }
}
#[doc = "Disable counter events originating from `Hart 3 D-Cache MMIO` client.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hart3DcacheMmio {
    #[doc = "0: Enable events from the client"]
    Enable = 0,
    #[doc = "1: Disable events from the client"]
    Disable = 1,
}
impl From<Hart3DcacheMmio> for bool {
    #[inline(always)]
    fn from(variant: Hart3DcacheMmio) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `hart3_dcache_mmio` reader - Disable counter events originating from `Hart 3 D-Cache MMIO` client."]
pub type Hart3DcacheMmioR = crate::BitReader<Hart3DcacheMmio>;
impl Hart3DcacheMmioR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hart3DcacheMmio {
        match self.bits {
            false => Hart3DcacheMmio::Enable,
            true => Hart3DcacheMmio::Disable,
        }
    }
    #[doc = "Enable events from the client"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Hart3DcacheMmio::Enable
    }
    #[doc = "Disable events from the client"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Hart3DcacheMmio::Disable
    }
}
#[doc = "Field `hart3_dcache_mmio` writer - Disable counter events originating from `Hart 3 D-Cache MMIO` client."]
pub type Hart3DcacheMmioW<'a, REG> = crate::BitWriter<'a, REG, Hart3DcacheMmio>;
impl<'a, REG> Hart3DcacheMmioW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable events from the client"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Hart3DcacheMmio::Enable)
    }
    #[doc = "Disable events from the client"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Hart3DcacheMmio::Disable)
    }
}
#[doc = "Disable counter events originating from `Hart 3 L2 Prefetcher` client.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hart3L2Prefetcher {
    #[doc = "0: Enable events from the client"]
    Enable = 0,
    #[doc = "1: Disable events from the client"]
    Disable = 1,
}
impl From<Hart3L2Prefetcher> for bool {
    #[inline(always)]
    fn from(variant: Hart3L2Prefetcher) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `hart3_l2_prefetcher` reader - Disable counter events originating from `Hart 3 L2 Prefetcher` client."]
pub type Hart3L2PrefetcherR = crate::BitReader<Hart3L2Prefetcher>;
impl Hart3L2PrefetcherR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hart3L2Prefetcher {
        match self.bits {
            false => Hart3L2Prefetcher::Enable,
            true => Hart3L2Prefetcher::Disable,
        }
    }
    #[doc = "Enable events from the client"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Hart3L2Prefetcher::Enable
    }
    #[doc = "Disable events from the client"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Hart3L2Prefetcher::Disable
    }
}
#[doc = "Field `hart3_l2_prefetcher` writer - Disable counter events originating from `Hart 3 L2 Prefetcher` client."]
pub type Hart3L2PrefetcherW<'a, REG> = crate::BitWriter<'a, REG, Hart3L2Prefetcher>;
impl<'a, REG> Hart3L2PrefetcherW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable events from the client"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Hart3L2Prefetcher::Enable)
    }
    #[doc = "Disable events from the client"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Hart3L2Prefetcher::Disable)
    }
}
#[doc = "Disable counter events originating from `Hart 4 Fetch Unit` client.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hart4FetchUnit {
    #[doc = "0: Enable events from the client"]
    Enable = 0,
    #[doc = "1: Disable events from the client"]
    Disable = 1,
}
impl From<Hart4FetchUnit> for bool {
    #[inline(always)]
    fn from(variant: Hart4FetchUnit) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `hart4_fetch_unit` reader - Disable counter events originating from `Hart 4 Fetch Unit` client."]
pub type Hart4FetchUnitR = crate::BitReader<Hart4FetchUnit>;
impl Hart4FetchUnitR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hart4FetchUnit {
        match self.bits {
            false => Hart4FetchUnit::Enable,
            true => Hart4FetchUnit::Disable,
        }
    }
    #[doc = "Enable events from the client"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Hart4FetchUnit::Enable
    }
    #[doc = "Disable events from the client"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Hart4FetchUnit::Disable
    }
}
#[doc = "Field `hart4_fetch_unit` writer - Disable counter events originating from `Hart 4 Fetch Unit` client."]
pub type Hart4FetchUnitW<'a, REG> = crate::BitWriter<'a, REG, Hart4FetchUnit>;
impl<'a, REG> Hart4FetchUnitW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable events from the client"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Hart4FetchUnit::Enable)
    }
    #[doc = "Disable events from the client"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Hart4FetchUnit::Disable)
    }
}
#[doc = "Disable counter events originating from `Hart 4 I-Cache` client.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hart4Icache {
    #[doc = "0: Enable events from the client"]
    Enable = 0,
    #[doc = "1: Disable events from the client"]
    Disable = 1,
}
impl From<Hart4Icache> for bool {
    #[inline(always)]
    fn from(variant: Hart4Icache) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `hart4_icache` reader - Disable counter events originating from `Hart 4 I-Cache` client."]
pub type Hart4IcacheR = crate::BitReader<Hart4Icache>;
impl Hart4IcacheR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hart4Icache {
        match self.bits {
            false => Hart4Icache::Enable,
            true => Hart4Icache::Disable,
        }
    }
    #[doc = "Enable events from the client"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Hart4Icache::Enable
    }
    #[doc = "Disable events from the client"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Hart4Icache::Disable
    }
}
#[doc = "Field `hart4_icache` writer - Disable counter events originating from `Hart 4 I-Cache` client."]
pub type Hart4IcacheW<'a, REG> = crate::BitWriter<'a, REG, Hart4Icache>;
impl<'a, REG> Hart4IcacheW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable events from the client"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Hart4Icache::Enable)
    }
    #[doc = "Disable events from the client"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Hart4Icache::Disable)
    }
}
#[doc = "Disable counter events originating from `Hart 4 D-Cache` client.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hart4Dcache {
    #[doc = "0: Enable events from the client"]
    Enable = 0,
    #[doc = "1: Disable events from the client"]
    Disable = 1,
}
impl From<Hart4Dcache> for bool {
    #[inline(always)]
    fn from(variant: Hart4Dcache) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `hart4_dcache` reader - Disable counter events originating from `Hart 4 D-Cache` client."]
pub type Hart4DcacheR = crate::BitReader<Hart4Dcache>;
impl Hart4DcacheR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hart4Dcache {
        match self.bits {
            false => Hart4Dcache::Enable,
            true => Hart4Dcache::Disable,
        }
    }
    #[doc = "Enable events from the client"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Hart4Dcache::Enable
    }
    #[doc = "Disable events from the client"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Hart4Dcache::Disable
    }
}
#[doc = "Field `hart4_dcache` writer - Disable counter events originating from `Hart 4 D-Cache` client."]
pub type Hart4DcacheW<'a, REG> = crate::BitWriter<'a, REG, Hart4Dcache>;
impl<'a, REG> Hart4DcacheW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable events from the client"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Hart4Dcache::Enable)
    }
    #[doc = "Disable events from the client"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Hart4Dcache::Disable)
    }
}
#[doc = "Disable counter events originating from `Hart 4 D-Cache MMIO` client.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hart4DcacheMmio {
    #[doc = "0: Enable events from the client"]
    Enable = 0,
    #[doc = "1: Disable events from the client"]
    Disable = 1,
}
impl From<Hart4DcacheMmio> for bool {
    #[inline(always)]
    fn from(variant: Hart4DcacheMmio) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `hart4_dcache_mmio` reader - Disable counter events originating from `Hart 4 D-Cache MMIO` client."]
pub type Hart4DcacheMmioR = crate::BitReader<Hart4DcacheMmio>;
impl Hart4DcacheMmioR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hart4DcacheMmio {
        match self.bits {
            false => Hart4DcacheMmio::Enable,
            true => Hart4DcacheMmio::Disable,
        }
    }
    #[doc = "Enable events from the client"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Hart4DcacheMmio::Enable
    }
    #[doc = "Disable events from the client"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Hart4DcacheMmio::Disable
    }
}
#[doc = "Field `hart4_dcache_mmio` writer - Disable counter events originating from `Hart 4 D-Cache MMIO` client."]
pub type Hart4DcacheMmioW<'a, REG> = crate::BitWriter<'a, REG, Hart4DcacheMmio>;
impl<'a, REG> Hart4DcacheMmioW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable events from the client"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Hart4DcacheMmio::Enable)
    }
    #[doc = "Disable events from the client"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Hart4DcacheMmio::Disable)
    }
}
#[doc = "Disable counter events originating from `Hart 4 L2 Prefetcher` client.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hart4L2Prefetcher {
    #[doc = "0: Enable events from the client"]
    Enable = 0,
    #[doc = "1: Disable events from the client"]
    Disable = 1,
}
impl From<Hart4L2Prefetcher> for bool {
    #[inline(always)]
    fn from(variant: Hart4L2Prefetcher) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `hart4_l2_prefetcher` reader - Disable counter events originating from `Hart 4 L2 Prefetcher` client."]
pub type Hart4L2PrefetcherR = crate::BitReader<Hart4L2Prefetcher>;
impl Hart4L2PrefetcherR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hart4L2Prefetcher {
        match self.bits {
            false => Hart4L2Prefetcher::Enable,
            true => Hart4L2Prefetcher::Disable,
        }
    }
    #[doc = "Enable events from the client"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Hart4L2Prefetcher::Enable
    }
    #[doc = "Disable events from the client"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Hart4L2Prefetcher::Disable
    }
}
#[doc = "Field `hart4_l2_prefetcher` writer - Disable counter events originating from `Hart 4 L2 Prefetcher` client."]
pub type Hart4L2PrefetcherW<'a, REG> = crate::BitWriter<'a, REG, Hart4L2Prefetcher>;
impl<'a, REG> Hart4L2PrefetcherW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable events from the client"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Hart4L2Prefetcher::Enable)
    }
    #[doc = "Disable events from the client"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Hart4L2Prefetcher::Disable)
    }
}
impl R {
    #[doc = "Bit 0 - Disable counter events originating from `Debug` client."]
    #[inline(always)]
    pub fn debug(&self) -> DebugR {
        DebugR::new((self.bits & 1) != 0)
    }
    #[doc = "Disable counter events originating from `Debug` client."]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `axi4_front_port0` field"]
    #[inline(always)]
    pub fn axi4_front_port(&self, n: u8) -> Axi4FrontPortR {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        Axi4FrontPortR::new(((self.bits >> (n + 1)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Disable counter events originating from `Debug` client."]
    #[inline(always)]
    pub fn axi4_front_port_iter(&self) -> impl Iterator<Item = Axi4FrontPortR> + '_ {
        (0..4).map(move |n| Axi4FrontPortR::new(((self.bits >> (n + 1)) & 1) != 0))
    }
    #[doc = "Bit 1 - Disable counter events originating from `Debug` client."]
    #[inline(always)]
    pub fn axi4_front_port0(&self) -> Axi4FrontPortR {
        Axi4FrontPortR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Disable counter events originating from `Debug` client."]
    #[inline(always)]
    pub fn axi4_front_port1(&self) -> Axi4FrontPortR {
        Axi4FrontPortR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Disable counter events originating from `Debug` client."]
    #[inline(always)]
    pub fn axi4_front_port2(&self) -> Axi4FrontPortR {
        Axi4FrontPortR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Disable counter events originating from `Debug` client."]
    #[inline(always)]
    pub fn axi4_front_port3(&self) -> Axi4FrontPortR {
        Axi4FrontPortR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Disable counter events originating from `Hart 0 Fetch Unit` client."]
    #[inline(always)]
    pub fn hart0_fetch_unit(&self) -> Hart0FetchUnitR {
        Hart0FetchUnitR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Disable counter events originating from `Hart 0 D-Cache MMIO` client."]
    #[inline(always)]
    pub fn hart0_dcache_mmio(&self) -> Hart0DcacheMmioR {
        Hart0DcacheMmioR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Disable counter events originating from `Hart 1 Fetch Unit` client."]
    #[inline(always)]
    pub fn hart1_fetch_unit(&self) -> Hart1FetchUnitR {
        Hart1FetchUnitR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Disable counter events originating from `Hart 1 I-Cache` client."]
    #[inline(always)]
    pub fn hart1_icache(&self) -> Hart1IcacheR {
        Hart1IcacheR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Disable counter events originating from `Hart 1 D-Cache` client."]
    #[inline(always)]
    pub fn hart1_dcache(&self) -> Hart1DcacheR {
        Hart1DcacheR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Disable counter events originating from `Hart 1 D-Cache MMIO` client."]
    #[inline(always)]
    pub fn hart1_dcache_mmio(&self) -> Hart1DcacheMmioR {
        Hart1DcacheMmioR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Disable counter events originating from `Hart 1 L2 Prefetcher` client."]
    #[inline(always)]
    pub fn hart1_l2_prefetcher(&self) -> Hart1L2PrefetcherR {
        Hart1L2PrefetcherR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Disable counter events originating from `Hart 2 Fetch Unit` client."]
    #[inline(always)]
    pub fn hart2_fetch_unit(&self) -> Hart2FetchUnitR {
        Hart2FetchUnitR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Disable counter events originating from `Hart 2 I-Cache` client."]
    #[inline(always)]
    pub fn hart2_icache(&self) -> Hart2IcacheR {
        Hart2IcacheR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Disable counter events originating from `Hart 2 D-Cache` client."]
    #[inline(always)]
    pub fn hart2_dcache(&self) -> Hart2DcacheR {
        Hart2DcacheR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Disable counter events originating from `Hart 2 D-Cache MMIO` client."]
    #[inline(always)]
    pub fn hart2_dcache_mmio(&self) -> Hart2DcacheMmioR {
        Hart2DcacheMmioR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Disable counter events originating from `Hart 2 L2 Prefetcher` client."]
    #[inline(always)]
    pub fn hart2_l2_prefetcher(&self) -> Hart2L2PrefetcherR {
        Hart2L2PrefetcherR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Disable counter events originating from `Hart 3 Fetch Unit` client."]
    #[inline(always)]
    pub fn hart3_fetch_unit(&self) -> Hart3FetchUnitR {
        Hart3FetchUnitR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Disable counter events originating from `Hart 3 I-Cache` client."]
    #[inline(always)]
    pub fn hart3_icache(&self) -> Hart3IcacheR {
        Hart3IcacheR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Disable counter events originating from `Hart 3 D-Cache` client."]
    #[inline(always)]
    pub fn hart3_dcache(&self) -> Hart3DcacheR {
        Hart3DcacheR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Disable counter events originating from `Hart 3 D-Cache MMIO` client."]
    #[inline(always)]
    pub fn hart3_dcache_mmio(&self) -> Hart3DcacheMmioR {
        Hart3DcacheMmioR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Disable counter events originating from `Hart 3 L2 Prefetcher` client."]
    #[inline(always)]
    pub fn hart3_l2_prefetcher(&self) -> Hart3L2PrefetcherR {
        Hart3L2PrefetcherR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Disable counter events originating from `Hart 4 Fetch Unit` client."]
    #[inline(always)]
    pub fn hart4_fetch_unit(&self) -> Hart4FetchUnitR {
        Hart4FetchUnitR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Disable counter events originating from `Hart 4 I-Cache` client."]
    #[inline(always)]
    pub fn hart4_icache(&self) -> Hart4IcacheR {
        Hart4IcacheR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Disable counter events originating from `Hart 4 D-Cache` client."]
    #[inline(always)]
    pub fn hart4_dcache(&self) -> Hart4DcacheR {
        Hart4DcacheR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Disable counter events originating from `Hart 4 D-Cache MMIO` client."]
    #[inline(always)]
    pub fn hart4_dcache_mmio(&self) -> Hart4DcacheMmioR {
        Hart4DcacheMmioR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Disable counter events originating from `Hart 4 L2 Prefetcher` client."]
    #[inline(always)]
    pub fn hart4_l2_prefetcher(&self) -> Hart4L2PrefetcherR {
        Hart4L2PrefetcherR::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Disable counter events originating from `Debug` client."]
    #[inline(always)]
    #[must_use]
    pub fn debug(&mut self) -> DebugW<ClientFilterSpec> {
        DebugW::new(self, 0)
    }
    #[doc = "Disable counter events originating from `Debug` client."]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `axi4_front_port0` field"]
    #[inline(always)]
    #[must_use]
    pub fn axi4_front_port(&mut self, n: u8) -> Axi4FrontPortW<ClientFilterSpec> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        Axi4FrontPortW::new(self, n + 1)
    }
    #[doc = "Bit 1 - Disable counter events originating from `Debug` client."]
    #[inline(always)]
    #[must_use]
    pub fn axi4_front_port0(&mut self) -> Axi4FrontPortW<ClientFilterSpec> {
        Axi4FrontPortW::new(self, 1)
    }
    #[doc = "Bit 2 - Disable counter events originating from `Debug` client."]
    #[inline(always)]
    #[must_use]
    pub fn axi4_front_port1(&mut self) -> Axi4FrontPortW<ClientFilterSpec> {
        Axi4FrontPortW::new(self, 2)
    }
    #[doc = "Bit 3 - Disable counter events originating from `Debug` client."]
    #[inline(always)]
    #[must_use]
    pub fn axi4_front_port2(&mut self) -> Axi4FrontPortW<ClientFilterSpec> {
        Axi4FrontPortW::new(self, 3)
    }
    #[doc = "Bit 4 - Disable counter events originating from `Debug` client."]
    #[inline(always)]
    #[must_use]
    pub fn axi4_front_port3(&mut self) -> Axi4FrontPortW<ClientFilterSpec> {
        Axi4FrontPortW::new(self, 4)
    }
    #[doc = "Bit 5 - Disable counter events originating from `Hart 0 Fetch Unit` client."]
    #[inline(always)]
    #[must_use]
    pub fn hart0_fetch_unit(&mut self) -> Hart0FetchUnitW<ClientFilterSpec> {
        Hart0FetchUnitW::new(self, 5)
    }
    #[doc = "Bit 6 - Disable counter events originating from `Hart 0 D-Cache MMIO` client."]
    #[inline(always)]
    #[must_use]
    pub fn hart0_dcache_mmio(&mut self) -> Hart0DcacheMmioW<ClientFilterSpec> {
        Hart0DcacheMmioW::new(self, 6)
    }
    #[doc = "Bit 7 - Disable counter events originating from `Hart 1 Fetch Unit` client."]
    #[inline(always)]
    #[must_use]
    pub fn hart1_fetch_unit(&mut self) -> Hart1FetchUnitW<ClientFilterSpec> {
        Hart1FetchUnitW::new(self, 7)
    }
    #[doc = "Bit 8 - Disable counter events originating from `Hart 1 I-Cache` client."]
    #[inline(always)]
    #[must_use]
    pub fn hart1_icache(&mut self) -> Hart1IcacheW<ClientFilterSpec> {
        Hart1IcacheW::new(self, 8)
    }
    #[doc = "Bit 9 - Disable counter events originating from `Hart 1 D-Cache` client."]
    #[inline(always)]
    #[must_use]
    pub fn hart1_dcache(&mut self) -> Hart1DcacheW<ClientFilterSpec> {
        Hart1DcacheW::new(self, 9)
    }
    #[doc = "Bit 10 - Disable counter events originating from `Hart 1 D-Cache MMIO` client."]
    #[inline(always)]
    #[must_use]
    pub fn hart1_dcache_mmio(&mut self) -> Hart1DcacheMmioW<ClientFilterSpec> {
        Hart1DcacheMmioW::new(self, 10)
    }
    #[doc = "Bit 11 - Disable counter events originating from `Hart 1 L2 Prefetcher` client."]
    #[inline(always)]
    #[must_use]
    pub fn hart1_l2_prefetcher(&mut self) -> Hart1L2PrefetcherW<ClientFilterSpec> {
        Hart1L2PrefetcherW::new(self, 11)
    }
    #[doc = "Bit 12 - Disable counter events originating from `Hart 2 Fetch Unit` client."]
    #[inline(always)]
    #[must_use]
    pub fn hart2_fetch_unit(&mut self) -> Hart2FetchUnitW<ClientFilterSpec> {
        Hart2FetchUnitW::new(self, 12)
    }
    #[doc = "Bit 13 - Disable counter events originating from `Hart 2 I-Cache` client."]
    #[inline(always)]
    #[must_use]
    pub fn hart2_icache(&mut self) -> Hart2IcacheW<ClientFilterSpec> {
        Hart2IcacheW::new(self, 13)
    }
    #[doc = "Bit 14 - Disable counter events originating from `Hart 2 D-Cache` client."]
    #[inline(always)]
    #[must_use]
    pub fn hart2_dcache(&mut self) -> Hart2DcacheW<ClientFilterSpec> {
        Hart2DcacheW::new(self, 14)
    }
    #[doc = "Bit 15 - Disable counter events originating from `Hart 2 D-Cache MMIO` client."]
    #[inline(always)]
    #[must_use]
    pub fn hart2_dcache_mmio(&mut self) -> Hart2DcacheMmioW<ClientFilterSpec> {
        Hart2DcacheMmioW::new(self, 15)
    }
    #[doc = "Bit 16 - Disable counter events originating from `Hart 2 L2 Prefetcher` client."]
    #[inline(always)]
    #[must_use]
    pub fn hart2_l2_prefetcher(&mut self) -> Hart2L2PrefetcherW<ClientFilterSpec> {
        Hart2L2PrefetcherW::new(self, 16)
    }
    #[doc = "Bit 17 - Disable counter events originating from `Hart 3 Fetch Unit` client."]
    #[inline(always)]
    #[must_use]
    pub fn hart3_fetch_unit(&mut self) -> Hart3FetchUnitW<ClientFilterSpec> {
        Hart3FetchUnitW::new(self, 17)
    }
    #[doc = "Bit 18 - Disable counter events originating from `Hart 3 I-Cache` client."]
    #[inline(always)]
    #[must_use]
    pub fn hart3_icache(&mut self) -> Hart3IcacheW<ClientFilterSpec> {
        Hart3IcacheW::new(self, 18)
    }
    #[doc = "Bit 19 - Disable counter events originating from `Hart 3 D-Cache` client."]
    #[inline(always)]
    #[must_use]
    pub fn hart3_dcache(&mut self) -> Hart3DcacheW<ClientFilterSpec> {
        Hart3DcacheW::new(self, 19)
    }
    #[doc = "Bit 20 - Disable counter events originating from `Hart 3 D-Cache MMIO` client."]
    #[inline(always)]
    #[must_use]
    pub fn hart3_dcache_mmio(&mut self) -> Hart3DcacheMmioW<ClientFilterSpec> {
        Hart3DcacheMmioW::new(self, 20)
    }
    #[doc = "Bit 21 - Disable counter events originating from `Hart 3 L2 Prefetcher` client."]
    #[inline(always)]
    #[must_use]
    pub fn hart3_l2_prefetcher(&mut self) -> Hart3L2PrefetcherW<ClientFilterSpec> {
        Hart3L2PrefetcherW::new(self, 21)
    }
    #[doc = "Bit 22 - Disable counter events originating from `Hart 4 Fetch Unit` client."]
    #[inline(always)]
    #[must_use]
    pub fn hart4_fetch_unit(&mut self) -> Hart4FetchUnitW<ClientFilterSpec> {
        Hart4FetchUnitW::new(self, 22)
    }
    #[doc = "Bit 23 - Disable counter events originating from `Hart 4 I-Cache` client."]
    #[inline(always)]
    #[must_use]
    pub fn hart4_icache(&mut self) -> Hart4IcacheW<ClientFilterSpec> {
        Hart4IcacheW::new(self, 23)
    }
    #[doc = "Bit 24 - Disable counter events originating from `Hart 4 D-Cache` client."]
    #[inline(always)]
    #[must_use]
    pub fn hart4_dcache(&mut self) -> Hart4DcacheW<ClientFilterSpec> {
        Hart4DcacheW::new(self, 24)
    }
    #[doc = "Bit 25 - Disable counter events originating from `Hart 4 D-Cache MMIO` client."]
    #[inline(always)]
    #[must_use]
    pub fn hart4_dcache_mmio(&mut self) -> Hart4DcacheMmioW<ClientFilterSpec> {
        Hart4DcacheMmioW::new(self, 25)
    }
    #[doc = "Bit 26 - Disable counter events originating from `Hart 4 L2 Prefetcher` client."]
    #[inline(always)]
    #[must_use]
    pub fn hart4_l2_prefetcher(&mut self) -> Hart4L2PrefetcherW<ClientFilterSpec> {
        Hart4L2PrefetcherW::new(self, 26)
    }
}
#[doc = "L2PM Event Control Event Select configuration.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`client_filter::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`client_filter::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClientFilterSpec;
impl crate::RegisterSpec for ClientFilterSpec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`client_filter::R`](R) reader structure"]
impl crate::Readable for ClientFilterSpec {}
#[doc = "`write(|w| ..)` method takes [`client_filter::W`](W) writer structure"]
impl crate::Writable for ClientFilterSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
}
#[doc = "`reset()` method sets client_filter to value 0"]
impl crate::Resettable for ClientFilterSpec {
    const RESET_VALUE: u64 = 0;
}
