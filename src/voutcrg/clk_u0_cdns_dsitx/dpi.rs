#[doc = "Register `dpi` reader"]
pub type R = crate::R<DpiSpec>;
#[doc = "Register `dpi` writer"]
pub type W = crate::W<DpiSpec>;
#[doc = "Clock multiplexing selector: clk_dc8200_pix0, clk_hdmitx0_pixelclk\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ClkMuxSel {
    #[doc = "0: Select `clk_dc8200_pix0` as the U0 Cadence DSI Transmit DPI clock."]
    ClkDc8200Pix0 = 0,
    #[doc = "1: Select `clk_hdmitx0_pixelclk` as the U0 Cadence DSI Transmit DPI clock."]
    ClkHdmitx0Pixelclk = 1,
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
#[doc = "Field `clk_mux_sel` reader - Clock multiplexing selector: clk_dc8200_pix0, clk_hdmitx0_pixelclk"]
pub type ClkMuxSelR = crate::FieldReader<ClkMuxSel>;
impl ClkMuxSelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ClkMuxSel> {
        match self.bits {
            0 => Some(ClkMuxSel::ClkDc8200Pix0),
            1 => Some(ClkMuxSel::ClkHdmitx0Pixelclk),
            _ => None,
        }
    }
    #[doc = "Select `clk_dc8200_pix0` as the U0 Cadence DSI Transmit DPI clock."]
    #[inline(always)]
    pub fn is_clk_dc8200_pix0(&self) -> bool {
        *self == ClkMuxSel::ClkDc8200Pix0
    }
    #[doc = "Select `clk_hdmitx0_pixelclk` as the U0 Cadence DSI Transmit DPI clock."]
    #[inline(always)]
    pub fn is_clk_hdmitx0_pixelclk(&self) -> bool {
        *self == ClkMuxSel::ClkHdmitx0Pixelclk
    }
}
#[doc = "Field `clk_mux_sel` writer - Clock multiplexing selector: clk_dc8200_pix0, clk_hdmitx0_pixelclk"]
pub type ClkMuxSelW<'a, REG> = crate::FieldWriter<'a, REG, 6, ClkMuxSel>;
impl<'a, REG> ClkMuxSelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Select `clk_dc8200_pix0` as the U0 Cadence DSI Transmit DPI clock."]
    #[inline(always)]
    pub fn clk_dc8200_pix0(self) -> &'a mut crate::W<REG> {
        self.variant(ClkMuxSel::ClkDc8200Pix0)
    }
    #[doc = "Select `clk_hdmitx0_pixelclk` as the U0 Cadence DSI Transmit DPI clock."]
    #[inline(always)]
    pub fn clk_hdmitx0_pixelclk(self) -> &'a mut crate::W<REG> {
        self.variant(ClkMuxSel::ClkHdmitx0Pixelclk)
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
    #[doc = "Bits 24:29 - Clock multiplexing selector: clk_dc8200_pix0, clk_hdmitx0_pixelclk"]
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
    #[doc = "Bits 24:29 - Clock multiplexing selector: clk_dc8200_pix0, clk_hdmitx0_pixelclk"]
    #[inline(always)]
    #[must_use]
    pub fn clk_mux_sel(&mut self) -> ClkMuxSelW<DpiSpec> {
        ClkMuxSelW::new(self, 24)
    }
    #[doc = "Bit 31 - Clock ICG enable."]
    #[inline(always)]
    #[must_use]
    pub fn clk_icg(&mut self) -> ClkIcgW<DpiSpec> {
        ClkIcgW::new(self, 31)
    }
}
#[doc = "Clock U0 Cadence DSI Transmit DPI\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dpi::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dpi::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DpiSpec;
impl crate::RegisterSpec for DpiSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dpi::R`](R) reader structure"]
impl crate::Readable for DpiSpec {}
#[doc = "`write(|w| ..)` method takes [`dpi::W`](W) writer structure"]
impl crate::Writable for DpiSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets dpi to value 0"]
impl crate::Resettable for DpiSpec {
    const RESET_VALUE: u32 = 0;
}
