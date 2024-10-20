#[doc = "Register `clk_u0_vin_p_axiwr` reader"]
pub type R = crate::R<ClkU0VinPAxiwrSpec>;
#[doc = "Register `clk_u0_vin_p_axiwr` writer"]
pub type W = crate::W<ClkU0VinPAxiwrSpec>;
#[doc = "Clock multiplexing selector: clk_mipi_rx0_pxl, clk_dvp_inv\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ClkMuxSel {
    #[doc = "0: Select `clk_mipi_rx0_pxl` as the U0 Video In P AXIWR clock."]
    ClkMipiRx0Pxl = 0,
    #[doc = "1: Select `clk_dvp_inv` as the U0 Video In P AXIWR clock."]
    ClkDvpInv = 1,
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
#[doc = "Field `clk_mux_sel` reader - Clock multiplexing selector: clk_mipi_rx0_pxl, clk_dvp_inv"]
pub type ClkMuxSelR = crate::FieldReader<ClkMuxSel>;
impl ClkMuxSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ClkMuxSel> {
        match self.bits {
            0 => Some(ClkMuxSel::ClkMipiRx0Pxl),
            1 => Some(ClkMuxSel::ClkDvpInv),
            _ => None,
        }
    }
    #[doc = "Select `clk_mipi_rx0_pxl` as the U0 Video In P AXIWR clock."]
    #[inline(always)]
    pub fn is_clk_mipi_rx0_pxl(&self) -> bool {
        *self == ClkMuxSel::ClkMipiRx0Pxl
    }
    #[doc = "Select `clk_dvp_inv` as the U0 Video In P AXIWR clock."]
    #[inline(always)]
    pub fn is_clk_dvp_inv(&self) -> bool {
        *self == ClkMuxSel::ClkDvpInv
    }
}
#[doc = "Field `clk_mux_sel` writer - Clock multiplexing selector: clk_mipi_rx0_pxl, clk_dvp_inv"]
pub type ClkMuxSelW<'a, REG> = crate::FieldWriter<'a, REG, 6, ClkMuxSel>;
impl<'a, REG> ClkMuxSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Select `clk_mipi_rx0_pxl` as the U0 Video In P AXIWR clock."]
    #[inline(always)]
    pub fn clk_mipi_rx0_pxl(self) -> &'a mut crate::W<REG> {
        self.variant(ClkMuxSel::ClkMipiRx0Pxl)
    }
    #[doc = "Select `clk_dvp_inv` as the U0 Video In P AXIWR clock."]
    #[inline(always)]
    pub fn clk_dvp_inv(self) -> &'a mut crate::W<REG> {
        self.variant(ClkMuxSel::ClkDvpInv)
    }
}
impl R {
    #[doc = "Bits 24:29 - Clock multiplexing selector: clk_mipi_rx0_pxl, clk_dvp_inv"]
    #[inline(always)]
    pub fn clk_mux_sel(&self) -> ClkMuxSelR {
        ClkMuxSelR::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 24:29 - Clock multiplexing selector: clk_mipi_rx0_pxl, clk_dvp_inv"]
    #[inline(always)]
    #[must_use]
    pub fn clk_mux_sel(&mut self) -> ClkMuxSelW<ClkU0VinPAxiwrSpec> {
        ClkMuxSelW::new(self, 24)
    }
}
#[doc = "Clock U0 Video In P AXIWR\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_u0_vin_p_axiwr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_u0_vin_p_axiwr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkU0VinPAxiwrSpec;
impl crate::RegisterSpec for ClkU0VinPAxiwrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_u0_vin_p_axiwr::R`](R) reader structure"]
impl crate::Readable for ClkU0VinPAxiwrSpec {}
#[doc = "`write(|w| ..)` method takes [`clk_u0_vin_p_axiwr::W`](W) writer structure"]
impl crate::Writable for ClkU0VinPAxiwrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets clk_u0_vin_p_axiwr to value 0"]
impl crate::Resettable for ClkU0VinPAxiwrSpec {
    const RESET_VALUE: u32 = 0;
}
