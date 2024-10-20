#[doc = "Register `rst1` reader"]
pub type R = crate::R<Rst1Spec>;
#[doc = "Register `rst1` writer"]
pub type W = crate::W<Rst1Spec>;
#[doc = "Reset selector: u0_noc_bus_venc_axi\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U0NocBusVencAxi {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U0NocBusVencAxi> for bool {
    #[inline(always)]
    fn from(variant: U0NocBusVencAxi) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u0_noc_bus_venc_axi` reader - Reset selector: u0_noc_bus_venc_axi"]
pub type U0NocBusVencAxiR = crate::BitReader<U0NocBusVencAxi>;
impl U0NocBusVencAxiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U0NocBusVencAxi {
        match self.bits {
            false => U0NocBusVencAxi::None,
            true => U0NocBusVencAxi::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U0NocBusVencAxi::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U0NocBusVencAxi::Reset
    }
}
#[doc = "Field `u0_noc_bus_venc_axi` writer - Reset selector: u0_noc_bus_venc_axi"]
pub type U0NocBusVencAxiW<'a, REG> = crate::BitWriter<'a, REG, U0NocBusVencAxi>;
impl<'a, REG> U0NocBusVencAxiW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U0NocBusVencAxi::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U0NocBusVencAxi::Reset)
    }
}
#[doc = "Reset selector: u0_axi_cfg1_dec_ahb\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U0AxiCfg1DecAhb {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U0AxiCfg1DecAhb> for bool {
    #[inline(always)]
    fn from(variant: U0AxiCfg1DecAhb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u0_axi_cfg1_dec_ahb` reader - Reset selector: u0_axi_cfg1_dec_ahb"]
pub type U0AxiCfg1DecAhbR = crate::BitReader<U0AxiCfg1DecAhb>;
impl U0AxiCfg1DecAhbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U0AxiCfg1DecAhb {
        match self.bits {
            false => U0AxiCfg1DecAhb::None,
            true => U0AxiCfg1DecAhb::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U0AxiCfg1DecAhb::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U0AxiCfg1DecAhb::Reset
    }
}
#[doc = "Field `u0_axi_cfg1_dec_ahb` writer - Reset selector: u0_axi_cfg1_dec_ahb"]
pub type U0AxiCfg1DecAhbW<'a, REG> = crate::BitWriter<'a, REG, U0AxiCfg1DecAhb>;
impl<'a, REG> U0AxiCfg1DecAhbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U0AxiCfg1DecAhb::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U0AxiCfg1DecAhb::Reset)
    }
}
#[doc = "Reset selector: u0_axi_cfg1_dec_main\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U0AxiCfg1DecMain {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U0AxiCfg1DecMain> for bool {
    #[inline(always)]
    fn from(variant: U0AxiCfg1DecMain) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u0_axi_cfg1_dec_main` reader - Reset selector: u0_axi_cfg1_dec_main"]
pub type U0AxiCfg1DecMainR = crate::BitReader<U0AxiCfg1DecMain>;
impl U0AxiCfg1DecMainR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U0AxiCfg1DecMain {
        match self.bits {
            false => U0AxiCfg1DecMain::None,
            true => U0AxiCfg1DecMain::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U0AxiCfg1DecMain::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U0AxiCfg1DecMain::Reset
    }
}
#[doc = "Field `u0_axi_cfg1_dec_main` writer - Reset selector: u0_axi_cfg1_dec_main"]
pub type U0AxiCfg1DecMainW<'a, REG> = crate::BitWriter<'a, REG, U0AxiCfg1DecMain>;
impl<'a, REG> U0AxiCfg1DecMainW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U0AxiCfg1DecMain::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U0AxiCfg1DecMain::Reset)
    }
}
#[doc = "Reset selector: u0_axi_cfg0_dec_main\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U0AxiCfg0DecMain {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U0AxiCfg0DecMain> for bool {
    #[inline(always)]
    fn from(variant: U0AxiCfg0DecMain) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u0_axi_cfg0_dec_main` reader - Reset selector: u0_axi_cfg0_dec_main"]
pub type U0AxiCfg0DecMainR = crate::BitReader<U0AxiCfg0DecMain>;
impl U0AxiCfg0DecMainR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U0AxiCfg0DecMain {
        match self.bits {
            false => U0AxiCfg0DecMain::None,
            true => U0AxiCfg0DecMain::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U0AxiCfg0DecMain::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U0AxiCfg0DecMain::Reset
    }
}
#[doc = "Field `u0_axi_cfg0_dec_main` writer - Reset selector: u0_axi_cfg0_dec_main"]
pub type U0AxiCfg0DecMainW<'a, REG> = crate::BitWriter<'a, REG, U0AxiCfg0DecMain>;
impl<'a, REG> U0AxiCfg0DecMainW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U0AxiCfg0DecMain::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U0AxiCfg0DecMain::Reset)
    }
}
#[doc = "Reset selector: u0_axi_cfg0_dec_main_div\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U0AxiCfg0DecMainDiv {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U0AxiCfg0DecMainDiv> for bool {
    #[inline(always)]
    fn from(variant: U0AxiCfg0DecMainDiv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u0_axi_cfg0_dec_main_div` reader - Reset selector: u0_axi_cfg0_dec_main_div"]
pub type U0AxiCfg0DecMainDivR = crate::BitReader<U0AxiCfg0DecMainDiv>;
impl U0AxiCfg0DecMainDivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U0AxiCfg0DecMainDiv {
        match self.bits {
            false => U0AxiCfg0DecMainDiv::None,
            true => U0AxiCfg0DecMainDiv::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U0AxiCfg0DecMainDiv::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U0AxiCfg0DecMainDiv::Reset
    }
}
#[doc = "Field `u0_axi_cfg0_dec_main_div` writer - Reset selector: u0_axi_cfg0_dec_main_div"]
pub type U0AxiCfg0DecMainDivW<'a, REG> = crate::BitWriter<'a, REG, U0AxiCfg0DecMainDiv>;
impl<'a, REG> U0AxiCfg0DecMainDivW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U0AxiCfg0DecMainDiv::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U0AxiCfg0DecMainDiv::Reset)
    }
}
#[doc = "Reset selector: u0_axi_cfg0_dec_hifi4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U0AxiCfg0DecHifi4 {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U0AxiCfg0DecHifi4> for bool {
    #[inline(always)]
    fn from(variant: U0AxiCfg0DecHifi4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u0_axi_cfg0_dec_hifi4` reader - Reset selector: u0_axi_cfg0_dec_hifi4"]
pub type U0AxiCfg0DecHifi4R = crate::BitReader<U0AxiCfg0DecHifi4>;
impl U0AxiCfg0DecHifi4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U0AxiCfg0DecHifi4 {
        match self.bits {
            false => U0AxiCfg0DecHifi4::None,
            true => U0AxiCfg0DecHifi4::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U0AxiCfg0DecHifi4::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U0AxiCfg0DecHifi4::Reset
    }
}
#[doc = "Field `u0_axi_cfg0_dec_hifi4` writer - Reset selector: u0_axi_cfg0_dec_hifi4"]
pub type U0AxiCfg0DecHifi4W<'a, REG> = crate::BitWriter<'a, REG, U0AxiCfg0DecHifi4>;
impl<'a, REG> U0AxiCfg0DecHifi4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U0AxiCfg0DecHifi4::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U0AxiCfg0DecHifi4::Reset)
    }
}
#[doc = "Reset selector: u0_ddr_axi\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U0DdrAxi {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U0DdrAxi> for bool {
    #[inline(always)]
    fn from(variant: U0DdrAxi) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u0_ddr_axi` reader - Reset selector: u0_ddr_axi"]
pub type U0DdrAxiR = crate::BitReader<U0DdrAxi>;
impl U0DdrAxiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U0DdrAxi {
        match self.bits {
            false => U0DdrAxi::None,
            true => U0DdrAxi::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U0DdrAxi::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U0DdrAxi::Reset
    }
}
#[doc = "Field `u0_ddr_axi` writer - Reset selector: u0_ddr_axi"]
pub type U0DdrAxiW<'a, REG> = crate::BitWriter<'a, REG, U0DdrAxi>;
impl<'a, REG> U0DdrAxiW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U0DdrAxi::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U0DdrAxi::Reset)
    }
}
#[doc = "Reset selector: u0_ddr_osc\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U0DdrOsc {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U0DdrOsc> for bool {
    #[inline(always)]
    fn from(variant: U0DdrOsc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u0_ddr_osc` reader - Reset selector: u0_ddr_osc"]
pub type U0DdrOscR = crate::BitReader<U0DdrOsc>;
impl U0DdrOscR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U0DdrOsc {
        match self.bits {
            false => U0DdrOsc::None,
            true => U0DdrOsc::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U0DdrOsc::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U0DdrOsc::Reset
    }
}
#[doc = "Field `u0_ddr_osc` writer - Reset selector: u0_ddr_osc"]
pub type U0DdrOscW<'a, REG> = crate::BitWriter<'a, REG, U0DdrOsc>;
impl<'a, REG> U0DdrOscW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U0DdrOsc::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U0DdrOsc::Reset)
    }
}
#[doc = "Reset selector: u0_ddr_apb\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U0DdrApb {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U0DdrApb> for bool {
    #[inline(always)]
    fn from(variant: U0DdrApb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u0_ddr_apb` reader - Reset selector: u0_ddr_apb"]
pub type U0DdrApbR = crate::BitReader<U0DdrApb>;
impl U0DdrApbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U0DdrApb {
        match self.bits {
            false => U0DdrApb::None,
            true => U0DdrApb::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U0DdrApb::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U0DdrApb::Reset
    }
}
#[doc = "Field `u0_ddr_apb` writer - Reset selector: u0_ddr_apb"]
pub type U0DdrApbW<'a, REG> = crate::BitWriter<'a, REG, U0DdrApb>;
impl<'a, REG> U0DdrApbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U0DdrApb::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U0DdrApb::Reset)
    }
}
#[doc = "Reset selector: u0_isp_top\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U0IspTop {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U0IspTop> for bool {
    #[inline(always)]
    fn from(variant: U0IspTop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u0_isp_top` reader - Reset selector: u0_isp_top"]
pub type U0IspTopR = crate::BitReader<U0IspTop>;
impl U0IspTopR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U0IspTop {
        match self.bits {
            false => U0IspTop::None,
            true => U0IspTop::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U0IspTop::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U0IspTop::Reset
    }
}
#[doc = "Field `u0_isp_top` writer - Reset selector: u0_isp_top"]
pub type U0IspTopW<'a, REG> = crate::BitWriter<'a, REG, U0IspTop>;
impl<'a, REG> U0IspTopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U0IspTop::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U0IspTop::Reset)
    }
}
#[doc = "Reset selector: u0_isp_axi\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U0IspAxi {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U0IspAxi> for bool {
    #[inline(always)]
    fn from(variant: U0IspAxi) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u0_isp_axi` reader - Reset selector: u0_isp_axi"]
pub type U0IspAxiR = crate::BitReader<U0IspAxi>;
impl U0IspAxiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U0IspAxi {
        match self.bits {
            false => U0IspAxi::None,
            true => U0IspAxi::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U0IspAxi::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U0IspAxi::Reset
    }
}
#[doc = "Field `u0_isp_axi` writer - Reset selector: u0_isp_axi"]
pub type U0IspAxiW<'a, REG> = crate::BitWriter<'a, REG, U0IspAxi>;
impl<'a, REG> U0IspAxiW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U0IspAxi::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U0IspAxi::Reset)
    }
}
#[doc = "Reset selector: u0_vout_src\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U0VoutSrc {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U0VoutSrc> for bool {
    #[inline(always)]
    fn from(variant: U0VoutSrc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u0_vout_src` reader - Reset selector: u0_vout_src"]
pub type U0VoutSrcR = crate::BitReader<U0VoutSrc>;
impl U0VoutSrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U0VoutSrc {
        match self.bits {
            false => U0VoutSrc::None,
            true => U0VoutSrc::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U0VoutSrc::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U0VoutSrc::Reset
    }
}
#[doc = "Field `u0_vout_src` writer - Reset selector: u0_vout_src"]
pub type U0VoutSrcW<'a, REG> = crate::BitWriter<'a, REG, U0VoutSrc>;
impl<'a, REG> U0VoutSrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U0VoutSrc::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U0VoutSrc::Reset)
    }
}
#[doc = "Reset selector: u0_codaj12_axi\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U0Codaj12Axi {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U0Codaj12Axi> for bool {
    #[inline(always)]
    fn from(variant: U0Codaj12Axi) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u0_codaj12_axi` reader - Reset selector: u0_codaj12_axi"]
pub type U0Codaj12AxiR = crate::BitReader<U0Codaj12Axi>;
impl U0Codaj12AxiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U0Codaj12Axi {
        match self.bits {
            false => U0Codaj12Axi::None,
            true => U0Codaj12Axi::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U0Codaj12Axi::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U0Codaj12Axi::Reset
    }
}
#[doc = "Field `u0_codaj12_axi` writer - Reset selector: u0_codaj12_axi"]
pub type U0Codaj12AxiW<'a, REG> = crate::BitWriter<'a, REG, U0Codaj12Axi>;
impl<'a, REG> U0Codaj12AxiW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U0Codaj12Axi::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U0Codaj12Axi::Reset)
    }
}
#[doc = "Reset selector: u0_codaj12_core\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U0Codaj12Core {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U0Codaj12Core> for bool {
    #[inline(always)]
    fn from(variant: U0Codaj12Core) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u0_codaj12_core` reader - Reset selector: u0_codaj12_core"]
pub type U0Codaj12CoreR = crate::BitReader<U0Codaj12Core>;
impl U0Codaj12CoreR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U0Codaj12Core {
        match self.bits {
            false => U0Codaj12Core::None,
            true => U0Codaj12Core::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U0Codaj12Core::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U0Codaj12Core::Reset
    }
}
#[doc = "Field `u0_codaj12_core` writer - Reset selector: u0_codaj12_core"]
pub type U0Codaj12CoreW<'a, REG> = crate::BitWriter<'a, REG, U0Codaj12Core>;
impl<'a, REG> U0Codaj12CoreW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U0Codaj12Core::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U0Codaj12Core::Reset)
    }
}
#[doc = "Reset selector: u0_codaj12_apb\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U0Codaj12Apb {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U0Codaj12Apb> for bool {
    #[inline(always)]
    fn from(variant: U0Codaj12Apb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u0_codaj12_apb` reader - Reset selector: u0_codaj12_apb"]
pub type U0Codaj12ApbR = crate::BitReader<U0Codaj12Apb>;
impl U0Codaj12ApbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U0Codaj12Apb {
        match self.bits {
            false => U0Codaj12Apb::None,
            true => U0Codaj12Apb::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U0Codaj12Apb::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U0Codaj12Apb::Reset
    }
}
#[doc = "Field `u0_codaj12_apb` writer - Reset selector: u0_codaj12_apb"]
pub type U0Codaj12ApbW<'a, REG> = crate::BitWriter<'a, REG, U0Codaj12Apb>;
impl<'a, REG> U0Codaj12ApbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U0Codaj12Apb::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U0Codaj12Apb::Reset)
    }
}
#[doc = "Reset selector: u0_wave511_axi\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U0Wave511Axi {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U0Wave511Axi> for bool {
    #[inline(always)]
    fn from(variant: U0Wave511Axi) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u0_wave511_axi` reader - Reset selector: u0_wave511_axi"]
pub type U0Wave511AxiR = crate::BitReader<U0Wave511Axi>;
impl U0Wave511AxiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U0Wave511Axi {
        match self.bits {
            false => U0Wave511Axi::None,
            true => U0Wave511Axi::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U0Wave511Axi::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U0Wave511Axi::Reset
    }
}
#[doc = "Field `u0_wave511_axi` writer - Reset selector: u0_wave511_axi"]
pub type U0Wave511AxiW<'a, REG> = crate::BitWriter<'a, REG, U0Wave511Axi>;
impl<'a, REG> U0Wave511AxiW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U0Wave511Axi::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U0Wave511Axi::Reset)
    }
}
#[doc = "Reset selector: u0_wave511_bpu\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U0Wave511Bpu {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U0Wave511Bpu> for bool {
    #[inline(always)]
    fn from(variant: U0Wave511Bpu) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u0_wave511_bpu` reader - Reset selector: u0_wave511_bpu"]
pub type U0Wave511BpuR = crate::BitReader<U0Wave511Bpu>;
impl U0Wave511BpuR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U0Wave511Bpu {
        match self.bits {
            false => U0Wave511Bpu::None,
            true => U0Wave511Bpu::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U0Wave511Bpu::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U0Wave511Bpu::Reset
    }
}
#[doc = "Field `u0_wave511_bpu` writer - Reset selector: u0_wave511_bpu"]
pub type U0Wave511BpuW<'a, REG> = crate::BitWriter<'a, REG, U0Wave511Bpu>;
impl<'a, REG> U0Wave511BpuW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U0Wave511Bpu::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U0Wave511Bpu::Reset)
    }
}
#[doc = "Reset selector: u0_wave511_vce\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U0Wave511Vce {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U0Wave511Vce> for bool {
    #[inline(always)]
    fn from(variant: U0Wave511Vce) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u0_wave511_vce` reader - Reset selector: u0_wave511_vce"]
pub type U0Wave511VceR = crate::BitReader<U0Wave511Vce>;
impl U0Wave511VceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U0Wave511Vce {
        match self.bits {
            false => U0Wave511Vce::None,
            true => U0Wave511Vce::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U0Wave511Vce::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U0Wave511Vce::Reset
    }
}
#[doc = "Field `u0_wave511_vce` writer - Reset selector: u0_wave511_vce"]
pub type U0Wave511VceW<'a, REG> = crate::BitWriter<'a, REG, U0Wave511Vce>;
impl<'a, REG> U0Wave511VceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U0Wave511Vce::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U0Wave511Vce::Reset)
    }
}
#[doc = "Reset selector: u0_wave511_apb\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U0Wave511Apb {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U0Wave511Apb> for bool {
    #[inline(always)]
    fn from(variant: U0Wave511Apb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u0_wave511_apb` reader - Reset selector: u0_wave511_apb"]
pub type U0Wave511ApbR = crate::BitReader<U0Wave511Apb>;
impl U0Wave511ApbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U0Wave511Apb {
        match self.bits {
            false => U0Wave511Apb::None,
            true => U0Wave511Apb::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U0Wave511Apb::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U0Wave511Apb::Reset
    }
}
#[doc = "Field `u0_wave511_apb` writer - Reset selector: u0_wave511_apb"]
pub type U0Wave511ApbW<'a, REG> = crate::BitWriter<'a, REG, U0Wave511Apb>;
impl<'a, REG> U0Wave511ApbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U0Wave511Apb::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U0Wave511Apb::Reset)
    }
}
#[doc = "Reset selector: u0_vdec_jpg_arb\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U0VdecJpgArb {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U0VdecJpgArb> for bool {
    #[inline(always)]
    fn from(variant: U0VdecJpgArb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u0_vdec_jpg_arb` reader - Reset selector: u0_vdec_jpg_arb"]
pub type U0VdecJpgArbR = crate::BitReader<U0VdecJpgArb>;
impl U0VdecJpgArbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U0VdecJpgArb {
        match self.bits {
            false => U0VdecJpgArb::None,
            true => U0VdecJpgArb::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U0VdecJpgArb::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U0VdecJpgArb::Reset
    }
}
#[doc = "Field `u0_vdec_jpg_arb` writer - Reset selector: u0_vdec_jpg_arb"]
pub type U0VdecJpgArbW<'a, REG> = crate::BitWriter<'a, REG, U0VdecJpgArb>;
impl<'a, REG> U0VdecJpgArbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U0VdecJpgArb::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U0VdecJpgArb::Reset)
    }
}
#[doc = "Reset selector: u0_vdec_jpg_arb_main\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U0VdecJpgArbMain {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U0VdecJpgArbMain> for bool {
    #[inline(always)]
    fn from(variant: U0VdecJpgArbMain) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u0_vdec_jpg_arb_main` reader - Reset selector: u0_vdec_jpg_arb_main"]
pub type U0VdecJpgArbMainR = crate::BitReader<U0VdecJpgArbMain>;
impl U0VdecJpgArbMainR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U0VdecJpgArbMain {
        match self.bits {
            false => U0VdecJpgArbMain::None,
            true => U0VdecJpgArbMain::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U0VdecJpgArbMain::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U0VdecJpgArbMain::Reset
    }
}
#[doc = "Field `u0_vdec_jpg_arb_main` writer - Reset selector: u0_vdec_jpg_arb_main"]
pub type U0VdecJpgArbMainW<'a, REG> = crate::BitWriter<'a, REG, U0VdecJpgArbMain>;
impl<'a, REG> U0VdecJpgArbMainW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U0VdecJpgArbMain::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U0VdecJpgArbMain::Reset)
    }
}
#[doc = "Reset selector: u0_aximem_128b_axi\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U0Aximem128bAxi {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U0Aximem128bAxi> for bool {
    #[inline(always)]
    fn from(variant: U0Aximem128bAxi) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u0_aximem_128b_axi` reader - Reset selector: u0_aximem_128b_axi"]
pub type U0Aximem128bAxiR = crate::BitReader<U0Aximem128bAxi>;
impl U0Aximem128bAxiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U0Aximem128bAxi {
        match self.bits {
            false => U0Aximem128bAxi::None,
            true => U0Aximem128bAxi::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U0Aximem128bAxi::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U0Aximem128bAxi::Reset
    }
}
#[doc = "Field `u0_aximem_128b_axi` writer - Reset selector: u0_aximem_128b_axi"]
pub type U0Aximem128bAxiW<'a, REG> = crate::BitWriter<'a, REG, U0Aximem128bAxi>;
impl<'a, REG> U0Aximem128bAxiW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U0Aximem128bAxi::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U0Aximem128bAxi::Reset)
    }
}
#[doc = "Reset selector: u0_wave420l_axi\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U0Wave420lAxi {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U0Wave420lAxi> for bool {
    #[inline(always)]
    fn from(variant: U0Wave420lAxi) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u0_wave420l_axi` reader - Reset selector: u0_wave420l_axi"]
pub type U0Wave420lAxiR = crate::BitReader<U0Wave420lAxi>;
impl U0Wave420lAxiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U0Wave420lAxi {
        match self.bits {
            false => U0Wave420lAxi::None,
            true => U0Wave420lAxi::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U0Wave420lAxi::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U0Wave420lAxi::Reset
    }
}
#[doc = "Field `u0_wave420l_axi` writer - Reset selector: u0_wave420l_axi"]
pub type U0Wave420lAxiW<'a, REG> = crate::BitWriter<'a, REG, U0Wave420lAxi>;
impl<'a, REG> U0Wave420lAxiW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U0Wave420lAxi::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U0Wave420lAxi::Reset)
    }
}
#[doc = "Reset selector: u0_wave420l_bpu\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U0Wave420lBpu {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U0Wave420lBpu> for bool {
    #[inline(always)]
    fn from(variant: U0Wave420lBpu) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u0_wave420l_bpu` reader - Reset selector: u0_wave420l_bpu"]
pub type U0Wave420lBpuR = crate::BitReader<U0Wave420lBpu>;
impl U0Wave420lBpuR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U0Wave420lBpu {
        match self.bits {
            false => U0Wave420lBpu::None,
            true => U0Wave420lBpu::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U0Wave420lBpu::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U0Wave420lBpu::Reset
    }
}
#[doc = "Field `u0_wave420l_bpu` writer - Reset selector: u0_wave420l_bpu"]
pub type U0Wave420lBpuW<'a, REG> = crate::BitWriter<'a, REG, U0Wave420lBpu>;
impl<'a, REG> U0Wave420lBpuW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U0Wave420lBpu::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U0Wave420lBpu::Reset)
    }
}
#[doc = "Reset selector: u0_wave420l_vce\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U0Wave420lVce {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U0Wave420lVce> for bool {
    #[inline(always)]
    fn from(variant: U0Wave420lVce) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u0_wave420l_vce` reader - Reset selector: u0_wave420l_vce"]
pub type U0Wave420lVceR = crate::BitReader<U0Wave420lVce>;
impl U0Wave420lVceR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U0Wave420lVce {
        match self.bits {
            false => U0Wave420lVce::None,
            true => U0Wave420lVce::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U0Wave420lVce::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U0Wave420lVce::Reset
    }
}
#[doc = "Field `u0_wave420l_vce` writer - Reset selector: u0_wave420l_vce"]
pub type U0Wave420lVceW<'a, REG> = crate::BitWriter<'a, REG, U0Wave420lVce>;
impl<'a, REG> U0Wave420lVceW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U0Wave420lVce::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U0Wave420lVce::Reset)
    }
}
#[doc = "Reset selector: u0_wave420l_apb\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U0Wave420lApb {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U0Wave420lApb> for bool {
    #[inline(always)]
    fn from(variant: U0Wave420lApb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u0_wave420l_apb` reader - Reset selector: u0_wave420l_apb"]
pub type U0Wave420lApbR = crate::BitReader<U0Wave420lApb>;
impl U0Wave420lApbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U0Wave420lApb {
        match self.bits {
            false => U0Wave420lApb::None,
            true => U0Wave420lApb::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U0Wave420lApb::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U0Wave420lApb::Reset
    }
}
#[doc = "Field `u0_wave420l_apb` writer - Reset selector: u0_wave420l_apb"]
pub type U0Wave420lApbW<'a, REG> = crate::BitWriter<'a, REG, U0Wave420lApb>;
impl<'a, REG> U0Wave420lApbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U0Wave420lApb::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U0Wave420lApb::Reset)
    }
}
#[doc = "Reset selector: u1_aximem\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U1Aximem {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U1Aximem> for bool {
    #[inline(always)]
    fn from(variant: U1Aximem) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u1_aximem` reader - Reset selector: u1_aximem"]
pub type U1AximemR = crate::BitReader<U1Aximem>;
impl U1AximemR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U1Aximem {
        match self.bits {
            false => U1Aximem::None,
            true => U1Aximem::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U1Aximem::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U1Aximem::Reset
    }
}
#[doc = "Field `u1_aximem` writer - Reset selector: u1_aximem"]
pub type U1AximemW<'a, REG> = crate::BitWriter<'a, REG, U1Aximem>;
impl<'a, REG> U1AximemW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U1Aximem::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U1Aximem::Reset)
    }
}
#[doc = "Reset selector: u2_aximem\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U2Aximem {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U2Aximem> for bool {
    #[inline(always)]
    fn from(variant: U2Aximem) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u2_aximem` reader - Reset selector: u2_aximem"]
pub type U2AximemR = crate::BitReader<U2Aximem>;
impl U2AximemR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U2Aximem {
        match self.bits {
            false => U2Aximem::None,
            true => U2Aximem::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U2Aximem::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U2Aximem::Reset
    }
}
#[doc = "Field `u2_aximem` writer - Reset selector: u2_aximem"]
pub type U2AximemW<'a, REG> = crate::BitWriter<'a, REG, U2Aximem>;
impl<'a, REG> U2AximemW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U2Aximem::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U2Aximem::Reset)
    }
}
#[doc = "Reset selector: u0_intmem_rom_sram\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U0IntmemRomSram {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U0IntmemRomSram> for bool {
    #[inline(always)]
    fn from(variant: U0IntmemRomSram) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u0_intmem_rom_sram` reader - Reset selector: u0_intmem_rom_sram"]
pub type U0IntmemRomSramR = crate::BitReader<U0IntmemRomSram>;
impl U0IntmemRomSramR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U0IntmemRomSram {
        match self.bits {
            false => U0IntmemRomSram::None,
            true => U0IntmemRomSram::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U0IntmemRomSram::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U0IntmemRomSram::Reset
    }
}
#[doc = "Field `u0_intmem_rom_sram` writer - Reset selector: u0_intmem_rom_sram"]
pub type U0IntmemRomSramW<'a, REG> = crate::BitWriter<'a, REG, U0IntmemRomSram>;
impl<'a, REG> U0IntmemRomSramW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U0IntmemRomSram::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U0IntmemRomSram::Reset)
    }
}
#[doc = "Reset selector: u0_qspi_ahb\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U0QspiAhb {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U0QspiAhb> for bool {
    #[inline(always)]
    fn from(variant: U0QspiAhb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u0_qspi_ahb` reader - Reset selector: u0_qspi_ahb"]
pub type U0QspiAhbR = crate::BitReader<U0QspiAhb>;
impl U0QspiAhbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U0QspiAhb {
        match self.bits {
            false => U0QspiAhb::None,
            true => U0QspiAhb::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U0QspiAhb::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U0QspiAhb::Reset
    }
}
#[doc = "Field `u0_qspi_ahb` writer - Reset selector: u0_qspi_ahb"]
pub type U0QspiAhbW<'a, REG> = crate::BitWriter<'a, REG, U0QspiAhb>;
impl<'a, REG> U0QspiAhbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U0QspiAhb::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U0QspiAhb::Reset)
    }
}
#[doc = "Reset selector: u0_qspi_apb\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U0QspiApb {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U0QspiApb> for bool {
    #[inline(always)]
    fn from(variant: U0QspiApb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u0_qspi_apb` reader - Reset selector: u0_qspi_apb"]
pub type U0QspiApbR = crate::BitReader<U0QspiApb>;
impl U0QspiApbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U0QspiApb {
        match self.bits {
            false => U0QspiApb::None,
            true => U0QspiApb::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U0QspiApb::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U0QspiApb::Reset
    }
}
#[doc = "Field `u0_qspi_apb` writer - Reset selector: u0_qspi_apb"]
pub type U0QspiApbW<'a, REG> = crate::BitWriter<'a, REG, U0QspiApb>;
impl<'a, REG> U0QspiApbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U0QspiApb::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U0QspiApb::Reset)
    }
}
#[doc = "Reset selector: u0_qspi_ref\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum U0QspiRef {
    #[doc = "0: De-assert reset."]
    None = 0,
    #[doc = "1: Assert reset."]
    Reset = 1,
}
impl From<U0QspiRef> for bool {
    #[inline(always)]
    fn from(variant: U0QspiRef) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `u0_qspi_ref` reader - Reset selector: u0_qspi_ref"]
pub type U0QspiRefR = crate::BitReader<U0QspiRef>;
impl U0QspiRefR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> U0QspiRef {
        match self.bits {
            false => U0QspiRef::None,
            true => U0QspiRef::Reset,
        }
    }
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == U0QspiRef::None
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == U0QspiRef::Reset
    }
}
#[doc = "Field `u0_qspi_ref` writer - Reset selector: u0_qspi_ref"]
pub type U0QspiRefW<'a, REG> = crate::BitWriter<'a, REG, U0QspiRef>;
impl<'a, REG> U0QspiRefW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "De-assert reset."]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(U0QspiRef::None)
    }
    #[doc = "Assert reset."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(U0QspiRef::Reset)
    }
}
impl R {
    #[doc = "Bit 0 - Reset selector: u0_noc_bus_venc_axi"]
    #[inline(always)]
    pub fn u0_noc_bus_venc_axi(&self) -> U0NocBusVencAxiR {
        U0NocBusVencAxiR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reset selector: u0_axi_cfg1_dec_ahb"]
    #[inline(always)]
    pub fn u0_axi_cfg1_dec_ahb(&self) -> U0AxiCfg1DecAhbR {
        U0AxiCfg1DecAhbR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Reset selector: u0_axi_cfg1_dec_main"]
    #[inline(always)]
    pub fn u0_axi_cfg1_dec_main(&self) -> U0AxiCfg1DecMainR {
        U0AxiCfg1DecMainR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reset selector: u0_axi_cfg0_dec_main"]
    #[inline(always)]
    pub fn u0_axi_cfg0_dec_main(&self) -> U0AxiCfg0DecMainR {
        U0AxiCfg0DecMainR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Reset selector: u0_axi_cfg0_dec_main_div"]
    #[inline(always)]
    pub fn u0_axi_cfg0_dec_main_div(&self) -> U0AxiCfg0DecMainDivR {
        U0AxiCfg0DecMainDivR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Reset selector: u0_axi_cfg0_dec_hifi4"]
    #[inline(always)]
    pub fn u0_axi_cfg0_dec_hifi4(&self) -> U0AxiCfg0DecHifi4R {
        U0AxiCfg0DecHifi4R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Reset selector: u0_ddr_axi"]
    #[inline(always)]
    pub fn u0_ddr_axi(&self) -> U0DdrAxiR {
        U0DdrAxiR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Reset selector: u0_ddr_osc"]
    #[inline(always)]
    pub fn u0_ddr_osc(&self) -> U0DdrOscR {
        U0DdrOscR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Reset selector: u0_ddr_apb"]
    #[inline(always)]
    pub fn u0_ddr_apb(&self) -> U0DdrApbR {
        U0DdrApbR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Reset selector: u0_isp_top"]
    #[inline(always)]
    pub fn u0_isp_top(&self) -> U0IspTopR {
        U0IspTopR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Reset selector: u0_isp_axi"]
    #[inline(always)]
    pub fn u0_isp_axi(&self) -> U0IspAxiR {
        U0IspAxiR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Reset selector: u0_vout_src"]
    #[inline(always)]
    pub fn u0_vout_src(&self) -> U0VoutSrcR {
        U0VoutSrcR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Reset selector: u0_codaj12_axi"]
    #[inline(always)]
    pub fn u0_codaj12_axi(&self) -> U0Codaj12AxiR {
        U0Codaj12AxiR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Reset selector: u0_codaj12_core"]
    #[inline(always)]
    pub fn u0_codaj12_core(&self) -> U0Codaj12CoreR {
        U0Codaj12CoreR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Reset selector: u0_codaj12_apb"]
    #[inline(always)]
    pub fn u0_codaj12_apb(&self) -> U0Codaj12ApbR {
        U0Codaj12ApbR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Reset selector: u0_wave511_axi"]
    #[inline(always)]
    pub fn u0_wave511_axi(&self) -> U0Wave511AxiR {
        U0Wave511AxiR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Reset selector: u0_wave511_bpu"]
    #[inline(always)]
    pub fn u0_wave511_bpu(&self) -> U0Wave511BpuR {
        U0Wave511BpuR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Reset selector: u0_wave511_vce"]
    #[inline(always)]
    pub fn u0_wave511_vce(&self) -> U0Wave511VceR {
        U0Wave511VceR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Reset selector: u0_wave511_apb"]
    #[inline(always)]
    pub fn u0_wave511_apb(&self) -> U0Wave511ApbR {
        U0Wave511ApbR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Reset selector: u0_vdec_jpg_arb"]
    #[inline(always)]
    pub fn u0_vdec_jpg_arb(&self) -> U0VdecJpgArbR {
        U0VdecJpgArbR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Reset selector: u0_vdec_jpg_arb_main"]
    #[inline(always)]
    pub fn u0_vdec_jpg_arb_main(&self) -> U0VdecJpgArbMainR {
        U0VdecJpgArbMainR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Reset selector: u0_aximem_128b_axi"]
    #[inline(always)]
    pub fn u0_aximem_128b_axi(&self) -> U0Aximem128bAxiR {
        U0Aximem128bAxiR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Reset selector: u0_wave420l_axi"]
    #[inline(always)]
    pub fn u0_wave420l_axi(&self) -> U0Wave420lAxiR {
        U0Wave420lAxiR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Reset selector: u0_wave420l_bpu"]
    #[inline(always)]
    pub fn u0_wave420l_bpu(&self) -> U0Wave420lBpuR {
        U0Wave420lBpuR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Reset selector: u0_wave420l_vce"]
    #[inline(always)]
    pub fn u0_wave420l_vce(&self) -> U0Wave420lVceR {
        U0Wave420lVceR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Reset selector: u0_wave420l_apb"]
    #[inline(always)]
    pub fn u0_wave420l_apb(&self) -> U0Wave420lApbR {
        U0Wave420lApbR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Reset selector: u1_aximem"]
    #[inline(always)]
    pub fn u1_aximem(&self) -> U1AximemR {
        U1AximemR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Reset selector: u2_aximem"]
    #[inline(always)]
    pub fn u2_aximem(&self) -> U2AximemR {
        U2AximemR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Reset selector: u0_intmem_rom_sram"]
    #[inline(always)]
    pub fn u0_intmem_rom_sram(&self) -> U0IntmemRomSramR {
        U0IntmemRomSramR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Reset selector: u0_qspi_ahb"]
    #[inline(always)]
    pub fn u0_qspi_ahb(&self) -> U0QspiAhbR {
        U0QspiAhbR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Reset selector: u0_qspi_apb"]
    #[inline(always)]
    pub fn u0_qspi_apb(&self) -> U0QspiApbR {
        U0QspiApbR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Reset selector: u0_qspi_ref"]
    #[inline(always)]
    pub fn u0_qspi_ref(&self) -> U0QspiRefR {
        U0QspiRefR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Reset selector: u0_noc_bus_venc_axi"]
    #[inline(always)]
    #[must_use]
    pub fn u0_noc_bus_venc_axi(&mut self) -> U0NocBusVencAxiW<Rst1Spec> {
        U0NocBusVencAxiW::new(self, 0)
    }
    #[doc = "Bit 1 - Reset selector: u0_axi_cfg1_dec_ahb"]
    #[inline(always)]
    #[must_use]
    pub fn u0_axi_cfg1_dec_ahb(&mut self) -> U0AxiCfg1DecAhbW<Rst1Spec> {
        U0AxiCfg1DecAhbW::new(self, 1)
    }
    #[doc = "Bit 2 - Reset selector: u0_axi_cfg1_dec_main"]
    #[inline(always)]
    #[must_use]
    pub fn u0_axi_cfg1_dec_main(&mut self) -> U0AxiCfg1DecMainW<Rst1Spec> {
        U0AxiCfg1DecMainW::new(self, 2)
    }
    #[doc = "Bit 3 - Reset selector: u0_axi_cfg0_dec_main"]
    #[inline(always)]
    #[must_use]
    pub fn u0_axi_cfg0_dec_main(&mut self) -> U0AxiCfg0DecMainW<Rst1Spec> {
        U0AxiCfg0DecMainW::new(self, 3)
    }
    #[doc = "Bit 4 - Reset selector: u0_axi_cfg0_dec_main_div"]
    #[inline(always)]
    #[must_use]
    pub fn u0_axi_cfg0_dec_main_div(&mut self) -> U0AxiCfg0DecMainDivW<Rst1Spec> {
        U0AxiCfg0DecMainDivW::new(self, 4)
    }
    #[doc = "Bit 5 - Reset selector: u0_axi_cfg0_dec_hifi4"]
    #[inline(always)]
    #[must_use]
    pub fn u0_axi_cfg0_dec_hifi4(&mut self) -> U0AxiCfg0DecHifi4W<Rst1Spec> {
        U0AxiCfg0DecHifi4W::new(self, 5)
    }
    #[doc = "Bit 6 - Reset selector: u0_ddr_axi"]
    #[inline(always)]
    #[must_use]
    pub fn u0_ddr_axi(&mut self) -> U0DdrAxiW<Rst1Spec> {
        U0DdrAxiW::new(self, 6)
    }
    #[doc = "Bit 7 - Reset selector: u0_ddr_osc"]
    #[inline(always)]
    #[must_use]
    pub fn u0_ddr_osc(&mut self) -> U0DdrOscW<Rst1Spec> {
        U0DdrOscW::new(self, 7)
    }
    #[doc = "Bit 8 - Reset selector: u0_ddr_apb"]
    #[inline(always)]
    #[must_use]
    pub fn u0_ddr_apb(&mut self) -> U0DdrApbW<Rst1Spec> {
        U0DdrApbW::new(self, 8)
    }
    #[doc = "Bit 9 - Reset selector: u0_isp_top"]
    #[inline(always)]
    #[must_use]
    pub fn u0_isp_top(&mut self) -> U0IspTopW<Rst1Spec> {
        U0IspTopW::new(self, 9)
    }
    #[doc = "Bit 10 - Reset selector: u0_isp_axi"]
    #[inline(always)]
    #[must_use]
    pub fn u0_isp_axi(&mut self) -> U0IspAxiW<Rst1Spec> {
        U0IspAxiW::new(self, 10)
    }
    #[doc = "Bit 11 - Reset selector: u0_vout_src"]
    #[inline(always)]
    #[must_use]
    pub fn u0_vout_src(&mut self) -> U0VoutSrcW<Rst1Spec> {
        U0VoutSrcW::new(self, 11)
    }
    #[doc = "Bit 12 - Reset selector: u0_codaj12_axi"]
    #[inline(always)]
    #[must_use]
    pub fn u0_codaj12_axi(&mut self) -> U0Codaj12AxiW<Rst1Spec> {
        U0Codaj12AxiW::new(self, 12)
    }
    #[doc = "Bit 13 - Reset selector: u0_codaj12_core"]
    #[inline(always)]
    #[must_use]
    pub fn u0_codaj12_core(&mut self) -> U0Codaj12CoreW<Rst1Spec> {
        U0Codaj12CoreW::new(self, 13)
    }
    #[doc = "Bit 14 - Reset selector: u0_codaj12_apb"]
    #[inline(always)]
    #[must_use]
    pub fn u0_codaj12_apb(&mut self) -> U0Codaj12ApbW<Rst1Spec> {
        U0Codaj12ApbW::new(self, 14)
    }
    #[doc = "Bit 15 - Reset selector: u0_wave511_axi"]
    #[inline(always)]
    #[must_use]
    pub fn u0_wave511_axi(&mut self) -> U0Wave511AxiW<Rst1Spec> {
        U0Wave511AxiW::new(self, 15)
    }
    #[doc = "Bit 16 - Reset selector: u0_wave511_bpu"]
    #[inline(always)]
    #[must_use]
    pub fn u0_wave511_bpu(&mut self) -> U0Wave511BpuW<Rst1Spec> {
        U0Wave511BpuW::new(self, 16)
    }
    #[doc = "Bit 17 - Reset selector: u0_wave511_vce"]
    #[inline(always)]
    #[must_use]
    pub fn u0_wave511_vce(&mut self) -> U0Wave511VceW<Rst1Spec> {
        U0Wave511VceW::new(self, 17)
    }
    #[doc = "Bit 18 - Reset selector: u0_wave511_apb"]
    #[inline(always)]
    #[must_use]
    pub fn u0_wave511_apb(&mut self) -> U0Wave511ApbW<Rst1Spec> {
        U0Wave511ApbW::new(self, 18)
    }
    #[doc = "Bit 19 - Reset selector: u0_vdec_jpg_arb"]
    #[inline(always)]
    #[must_use]
    pub fn u0_vdec_jpg_arb(&mut self) -> U0VdecJpgArbW<Rst1Spec> {
        U0VdecJpgArbW::new(self, 19)
    }
    #[doc = "Bit 20 - Reset selector: u0_vdec_jpg_arb_main"]
    #[inline(always)]
    #[must_use]
    pub fn u0_vdec_jpg_arb_main(&mut self) -> U0VdecJpgArbMainW<Rst1Spec> {
        U0VdecJpgArbMainW::new(self, 20)
    }
    #[doc = "Bit 21 - Reset selector: u0_aximem_128b_axi"]
    #[inline(always)]
    #[must_use]
    pub fn u0_aximem_128b_axi(&mut self) -> U0Aximem128bAxiW<Rst1Spec> {
        U0Aximem128bAxiW::new(self, 21)
    }
    #[doc = "Bit 22 - Reset selector: u0_wave420l_axi"]
    #[inline(always)]
    #[must_use]
    pub fn u0_wave420l_axi(&mut self) -> U0Wave420lAxiW<Rst1Spec> {
        U0Wave420lAxiW::new(self, 22)
    }
    #[doc = "Bit 23 - Reset selector: u0_wave420l_bpu"]
    #[inline(always)]
    #[must_use]
    pub fn u0_wave420l_bpu(&mut self) -> U0Wave420lBpuW<Rst1Spec> {
        U0Wave420lBpuW::new(self, 23)
    }
    #[doc = "Bit 24 - Reset selector: u0_wave420l_vce"]
    #[inline(always)]
    #[must_use]
    pub fn u0_wave420l_vce(&mut self) -> U0Wave420lVceW<Rst1Spec> {
        U0Wave420lVceW::new(self, 24)
    }
    #[doc = "Bit 25 - Reset selector: u0_wave420l_apb"]
    #[inline(always)]
    #[must_use]
    pub fn u0_wave420l_apb(&mut self) -> U0Wave420lApbW<Rst1Spec> {
        U0Wave420lApbW::new(self, 25)
    }
    #[doc = "Bit 26 - Reset selector: u1_aximem"]
    #[inline(always)]
    #[must_use]
    pub fn u1_aximem(&mut self) -> U1AximemW<Rst1Spec> {
        U1AximemW::new(self, 26)
    }
    #[doc = "Bit 27 - Reset selector: u2_aximem"]
    #[inline(always)]
    #[must_use]
    pub fn u2_aximem(&mut self) -> U2AximemW<Rst1Spec> {
        U2AximemW::new(self, 27)
    }
    #[doc = "Bit 28 - Reset selector: u0_intmem_rom_sram"]
    #[inline(always)]
    #[must_use]
    pub fn u0_intmem_rom_sram(&mut self) -> U0IntmemRomSramW<Rst1Spec> {
        U0IntmemRomSramW::new(self, 28)
    }
    #[doc = "Bit 29 - Reset selector: u0_qspi_ahb"]
    #[inline(always)]
    #[must_use]
    pub fn u0_qspi_ahb(&mut self) -> U0QspiAhbW<Rst1Spec> {
        U0QspiAhbW::new(self, 29)
    }
    #[doc = "Bit 30 - Reset selector: u0_qspi_apb"]
    #[inline(always)]
    #[must_use]
    pub fn u0_qspi_apb(&mut self) -> U0QspiApbW<Rst1Spec> {
        U0QspiApbW::new(self, 30)
    }
    #[doc = "Bit 31 - Reset selector: u0_qspi_ref"]
    #[inline(always)]
    #[must_use]
    pub fn u0_qspi_ref(&mut self) -> U0QspiRefW<Rst1Spec> {
        U0QspiRefW::new(self, 31)
    }
}
#[doc = "RESET 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rst1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rst1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rst1Spec;
impl crate::RegisterSpec for Rst1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rst1::R`](R) reader structure"]
impl crate::Readable for Rst1Spec {}
#[doc = "`write(|w| ..)` method takes [`rst1::W`](W) writer structure"]
impl crate::Writable for Rst1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets rst1 to value 0x07e7_fe00"]
impl crate::Resettable for Rst1Spec {
    const RESET_VALUE: u32 = 0x07e7_fe00;
}
