#[doc = "Register `rst[%s]` reader"]
pub type R = crate::R<RstSpec>;
#[doc = "Register `rst[%s]` writer"]
pub type W = crate::W<RstSpec>;
#[doc = "Field `rstn_u0_dc8200_axi` reader - 0: De-assert reset, 1: Assert reset"]
pub type RstnU0Dc8200AxiR = crate::BitReader;
#[doc = "Field `rstn_u0_dc8200_axi` writer - 0: De-assert reset, 1: Assert reset"]
pub type RstnU0Dc8200AxiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rstn_u0_dc8200_ahb` reader - 0: De-assert reset, 1: Assert reset"]
pub type RstnU0Dc8200AhbR = crate::BitReader;
#[doc = "Field `rstn_u0_dc8200_ahb` writer - 0: De-assert reset, 1: Assert reset"]
pub type RstnU0Dc8200AhbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rstn_u0_dc8200_core` reader - 0: De-assert reset, 1: Assert reset"]
pub type RstnU0Dc8200CoreR = crate::BitReader;
#[doc = "Field `rstn_u0_dc8200_core` writer - 0: De-assert reset, 1: Assert reset"]
pub type RstnU0Dc8200CoreW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rstn_u0_cdns_dsitx_dpi` reader - 0: De-assert reset, 1: Assert reset"]
pub type RstnU0CdnsDsitxDpiR = crate::BitReader;
#[doc = "Field `rstn_u0_cdns_dsitx_dpi` writer - 0: De-assert reset, 1: Assert reset"]
pub type RstnU0CdnsDsitxDpiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rstn_u0_cdns_dsitx_apb` reader - 0: De-assert reset, 1: Assert reset"]
pub type RstnU0CdnsDsitxApbR = crate::BitReader;
#[doc = "Field `rstn_u0_cdns_dsitx_apb` writer - 0: De-assert reset, 1: Assert reset"]
pub type RstnU0CdnsDsitxApbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rstn_u0_cdns_dsitx_rxesc` reader - 0: De-assert reset, 1: Assert reset"]
pub type RstnU0CdnsDsitxRxescR = crate::BitReader;
#[doc = "Field `rstn_u0_cdns_dsitx_rxesc` writer - 0: De-assert reset, 1: Assert reset"]
pub type RstnU0CdnsDsitxRxescW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rstn_u0_cdns_dsitx_sys` reader - 0: De-assert reset, 1: Assert reset"]
pub type RstnU0CdnsDsitxSysR = crate::BitReader;
#[doc = "Field `rstn_u0_cdns_dsitx_sys` writer - 0: De-assert reset, 1: Assert reset"]
pub type RstnU0CdnsDsitxSysW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rstn_u0_cdns_dsitx_txbytehs` reader - 0: De-assert reset, 1: Assert reset"]
pub type RstnU0CdnsDsitxTxbytehsR = crate::BitReader;
#[doc = "Field `rstn_u0_cdns_dsitx_txbytehs` writer - 0: De-assert reset, 1: Assert reset"]
pub type RstnU0CdnsDsitxTxbytehsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rstn_u0_cdns_dsitx_txesc` reader - 0: De-assert reset, 1: Assert reset"]
pub type RstnU0CdnsDsitxTxescR = crate::BitReader;
#[doc = "Field `rstn_u0_cdns_dsitx_txesc` writer - 0: De-assert reset, 1: Assert reset"]
pub type RstnU0CdnsDsitxTxescW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rstn_u0_hdmi_tx_hdmi` reader - 0: De-assert reset, 1: Assert reset"]
pub type RstnU0HdmiTxHdmiR = crate::BitReader;
#[doc = "Field `rstn_u0_hdmi_tx_hdmi` writer - 0: De-assert reset, 1: Assert reset"]
pub type RstnU0HdmiTxHdmiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rstn_u0_mipitx_dphy_sys` reader - 0: De-assert reset, 1: Assert reset"]
pub type RstnU0MipitxDphySysR = crate::BitReader;
#[doc = "Field `rstn_u0_mipitx_dphy_sys` writer - 0: De-assert reset, 1: Assert reset"]
pub type RstnU0MipitxDphySysW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `rstn_u0_mipitx_dphy_txbytehs` reader - 0: De-assert reset, 1: Assert reset"]
pub type RstnU0MipitxDphyTxbytehsR = crate::BitReader;
#[doc = "Field `rstn_u0_mipitx_dphy_txbytehs` writer - 0: De-assert reset, 1: Assert reset"]
pub type RstnU0MipitxDphyTxbytehsW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0: De-assert reset, 1: Assert reset"]
    #[inline(always)]
    pub fn rstn_u0_dc8200_axi(&self) -> RstnU0Dc8200AxiR {
        RstnU0Dc8200AxiR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 0: De-assert reset, 1: Assert reset"]
    #[inline(always)]
    pub fn rstn_u0_dc8200_ahb(&self) -> RstnU0Dc8200AhbR {
        RstnU0Dc8200AhbR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 0: De-assert reset, 1: Assert reset"]
    #[inline(always)]
    pub fn rstn_u0_dc8200_core(&self) -> RstnU0Dc8200CoreR {
        RstnU0Dc8200CoreR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 0: De-assert reset, 1: Assert reset"]
    #[inline(always)]
    pub fn rstn_u0_cdns_dsitx_dpi(&self) -> RstnU0CdnsDsitxDpiR {
        RstnU0CdnsDsitxDpiR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 0: De-assert reset, 1: Assert reset"]
    #[inline(always)]
    pub fn rstn_u0_cdns_dsitx_apb(&self) -> RstnU0CdnsDsitxApbR {
        RstnU0CdnsDsitxApbR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 0: De-assert reset, 1: Assert reset"]
    #[inline(always)]
    pub fn rstn_u0_cdns_dsitx_rxesc(&self) -> RstnU0CdnsDsitxRxescR {
        RstnU0CdnsDsitxRxescR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 0: De-assert reset, 1: Assert reset"]
    #[inline(always)]
    pub fn rstn_u0_cdns_dsitx_sys(&self) -> RstnU0CdnsDsitxSysR {
        RstnU0CdnsDsitxSysR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 0: De-assert reset, 1: Assert reset"]
    #[inline(always)]
    pub fn rstn_u0_cdns_dsitx_txbytehs(&self) -> RstnU0CdnsDsitxTxbytehsR {
        RstnU0CdnsDsitxTxbytehsR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 0: De-assert reset, 1: Assert reset"]
    #[inline(always)]
    pub fn rstn_u0_cdns_dsitx_txesc(&self) -> RstnU0CdnsDsitxTxescR {
        RstnU0CdnsDsitxTxescR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 0: De-assert reset, 1: Assert reset"]
    #[inline(always)]
    pub fn rstn_u0_hdmi_tx_hdmi(&self) -> RstnU0HdmiTxHdmiR {
        RstnU0HdmiTxHdmiR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 0: De-assert reset, 1: Assert reset"]
    #[inline(always)]
    pub fn rstn_u0_mipitx_dphy_sys(&self) -> RstnU0MipitxDphySysR {
        RstnU0MipitxDphySysR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 0: De-assert reset, 1: Assert reset"]
    #[inline(always)]
    pub fn rstn_u0_mipitx_dphy_txbytehs(&self) -> RstnU0MipitxDphyTxbytehsR {
        RstnU0MipitxDphyTxbytehsR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0: De-assert reset, 1: Assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u0_dc8200_axi(&mut self) -> RstnU0Dc8200AxiW<RstSpec> {
        RstnU0Dc8200AxiW::new(self, 0)
    }
    #[doc = "Bit 1 - 0: De-assert reset, 1: Assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u0_dc8200_ahb(&mut self) -> RstnU0Dc8200AhbW<RstSpec> {
        RstnU0Dc8200AhbW::new(self, 1)
    }
    #[doc = "Bit 2 - 0: De-assert reset, 1: Assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u0_dc8200_core(&mut self) -> RstnU0Dc8200CoreW<RstSpec> {
        RstnU0Dc8200CoreW::new(self, 2)
    }
    #[doc = "Bit 3 - 0: De-assert reset, 1: Assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u0_cdns_dsitx_dpi(&mut self) -> RstnU0CdnsDsitxDpiW<RstSpec> {
        RstnU0CdnsDsitxDpiW::new(self, 3)
    }
    #[doc = "Bit 4 - 0: De-assert reset, 1: Assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u0_cdns_dsitx_apb(&mut self) -> RstnU0CdnsDsitxApbW<RstSpec> {
        RstnU0CdnsDsitxApbW::new(self, 4)
    }
    #[doc = "Bit 5 - 0: De-assert reset, 1: Assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u0_cdns_dsitx_rxesc(&mut self) -> RstnU0CdnsDsitxRxescW<RstSpec> {
        RstnU0CdnsDsitxRxescW::new(self, 5)
    }
    #[doc = "Bit 6 - 0: De-assert reset, 1: Assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u0_cdns_dsitx_sys(&mut self) -> RstnU0CdnsDsitxSysW<RstSpec> {
        RstnU0CdnsDsitxSysW::new(self, 6)
    }
    #[doc = "Bit 7 - 0: De-assert reset, 1: Assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u0_cdns_dsitx_txbytehs(&mut self) -> RstnU0CdnsDsitxTxbytehsW<RstSpec> {
        RstnU0CdnsDsitxTxbytehsW::new(self, 7)
    }
    #[doc = "Bit 8 - 0: De-assert reset, 1: Assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u0_cdns_dsitx_txesc(&mut self) -> RstnU0CdnsDsitxTxescW<RstSpec> {
        RstnU0CdnsDsitxTxescW::new(self, 8)
    }
    #[doc = "Bit 9 - 0: De-assert reset, 1: Assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u0_hdmi_tx_hdmi(&mut self) -> RstnU0HdmiTxHdmiW<RstSpec> {
        RstnU0HdmiTxHdmiW::new(self, 9)
    }
    #[doc = "Bit 10 - 0: De-assert reset, 1: Assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u0_mipitx_dphy_sys(&mut self) -> RstnU0MipitxDphySysW<RstSpec> {
        RstnU0MipitxDphySysW::new(self, 10)
    }
    #[doc = "Bit 11 - 0: De-assert reset, 1: Assert reset"]
    #[inline(always)]
    #[must_use]
    pub fn rstn_u0_mipitx_dphy_txbytehs(&mut self) -> RstnU0MipitxDphyTxbytehsW<RstSpec> {
        RstnU0MipitxDphyTxbytehsW::new(self, 11)
    }
}
#[doc = "VOUT CRG RESET register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rst::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rst::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
to value 0x0fff"]
impl crate::Resettable for RstSpec {
    const RESET_VALUE: u32 = 0x0fff;
}
