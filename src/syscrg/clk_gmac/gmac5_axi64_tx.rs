#[doc = "Register `gmac5_axi64_tx` reader"]
pub type R = crate::R<Gmac5Axi64TxSpec>;
#[doc = "Register `gmac5_axi64_tx` writer"]
pub type W = crate::W<Gmac5Axi64TxSpec>;
#[doc = "Clock multiplexing selector: clk_gmac1_gtx_clk, clk_gmac1_rmii_rtx\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ClkMuxSel {
    #[doc = "0: Select `clk_gmac1_gtx_clk` as the GMAC5 AXI64 TX clock."]
    ClkGmac1GtxClk = 0,
    #[doc = "1: Select `clk_gmac1_rmii_rtx` as the GMAC5 AXI64 TX clock."]
    ClkGmac1RmiiRtx = 1,
}
impl From<ClkMuxSel> for u8 {
    #[inline(always)]
    fn from(variant: ClkMuxSel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ClkMuxSel {
    type Ux = u8;
}
#[doc = "Field `clk_mux_sel` reader - Clock multiplexing selector: clk_gmac1_gtx_clk, clk_gmac1_rmii_rtx"]
pub type ClkMuxSelR = crate::FieldReader<ClkMuxSel>;
impl ClkMuxSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ClkMuxSel> {
        match self.bits {
            0 => Some(ClkMuxSel::ClkGmac1GtxClk),
            1 => Some(ClkMuxSel::ClkGmac1RmiiRtx),
            _ => None,
        }
    }
    #[doc = "Select `clk_gmac1_gtx_clk` as the GMAC5 AXI64 TX clock."]
    #[inline(always)]
    pub fn is_clk_gmac1_gtx_clk(&self) -> bool {
        *self == ClkMuxSel::ClkGmac1GtxClk
    }
    #[doc = "Select `clk_gmac1_rmii_rtx` as the GMAC5 AXI64 TX clock."]
    #[inline(always)]
    pub fn is_clk_gmac1_rmii_rtx(&self) -> bool {
        *self == ClkMuxSel::ClkGmac1RmiiRtx
    }
}
#[doc = "Field `clk_mux_sel` writer - Clock multiplexing selector: clk_gmac1_gtx_clk, clk_gmac1_rmii_rtx"]
pub type ClkMuxSelW<'a, REG> = crate::FieldWriter<'a, REG, 6, ClkMuxSel>;
impl<'a, REG> ClkMuxSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Select `clk_gmac1_gtx_clk` as the GMAC5 AXI64 TX clock."]
    #[inline(always)]
    pub fn clk_gmac1_gtx_clk(self) -> &'a mut crate::W<REG> {
        self.variant(ClkMuxSel::ClkGmac1GtxClk)
    }
    #[doc = "Select `clk_gmac1_rmii_rtx` as the GMAC5 AXI64 TX clock."]
    #[inline(always)]
    pub fn clk_gmac1_rmii_rtx(self) -> &'a mut crate::W<REG> {
        self.variant(ClkMuxSel::ClkGmac1RmiiRtx)
    }
}
#[doc = "Clock ICG enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClkIcg {
    #[doc = "0: Disable the clock."]
    Disable = 0,
    #[doc = "1: Enable the clock."]
    Enable = 1,
}
impl From<ClkIcg> for bool {
    #[inline(always)]
    fn from(variant: ClkIcg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `clk_icg` reader - Clock ICG enable."]
pub type ClkIcgR = crate::BitReader<ClkIcg>;
impl ClkIcgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ClkIcg {
        match self.bits {
            false => ClkIcg::Disable,
            true => ClkIcg::Enable,
        }
    }
    #[doc = "Disable the clock."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ClkIcg::Disable
    }
    #[doc = "Enable the clock."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ClkIcg::Enable
    }
}
#[doc = "Field `clk_icg` writer - Clock ICG enable."]
pub type ClkIcgW<'a, REG> = crate::BitWriter<'a, REG, ClkIcg>;
impl<'a, REG> ClkIcgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable the clock."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(ClkIcg::Disable)
    }
    #[doc = "Enable the clock."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(ClkIcg::Enable)
    }
}
impl R {
    #[doc = "Bits 24:29 - Clock multiplexing selector: clk_gmac1_gtx_clk, clk_gmac1_rmii_rtx"]
    #[inline(always)]
    pub fn clk_mux_sel(&self) -> ClkMuxSelR {
        ClkMuxSelR::new(((self.bits >> 24) & 0x3f) as u8)
    }
    #[doc = "Bit 31 - Clock ICG enable."]
    #[inline(always)]
    pub fn clk_icg(&self) -> ClkIcgR {
        ClkIcgR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 24:29 - Clock multiplexing selector: clk_gmac1_gtx_clk, clk_gmac1_rmii_rtx"]
    #[inline(always)]
    #[must_use]
    pub fn clk_mux_sel(&mut self) -> ClkMuxSelW<Gmac5Axi64TxSpec> {
        ClkMuxSelW::new(self, 24)
    }
    #[doc = "Bit 31 - Clock ICG enable."]
    #[inline(always)]
    #[must_use]
    pub fn clk_icg(&mut self) -> ClkIcgW<Gmac5Axi64TxSpec> {
        ClkIcgW::new(self, 31)
    }
}
#[doc = "Clock GMAC5 AXI64 TX\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac5_axi64_tx::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac5_axi64_tx::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gmac5Axi64TxSpec;
impl crate::RegisterSpec for Gmac5Axi64TxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gmac5_axi64_tx::R`](R) reader structure"]
impl crate::Readable for Gmac5Axi64TxSpec {}
#[doc = "`write(|w| ..)` method takes [`gmac5_axi64_tx::W`](W) writer structure"]
impl crate::Writable for Gmac5Axi64TxSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets gmac5_axi64_tx to value 0"]
impl crate::Resettable for Gmac5Axi64TxSpec {
    const RESET_VALUE: u32 = 0;
}
