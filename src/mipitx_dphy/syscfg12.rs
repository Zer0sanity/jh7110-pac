#[doc = "Register `syscfg12` reader"]
pub type R = crate::R<Syscfg12Spec>;
#[doc = "Register `syscfg12` writer"]
pub type W = crate::W<Syscfg12Spec>;
#[doc = "Field `scfg_dphy_src_sel` reader - SCFG DPHY Source Select: u0_mipitx_dphy_SCFG_dphy_src_sel"]
pub type ScfgDphySrcSelR = crate::BitReader;
#[doc = "Field `scfg_dphy_src_sel` writer - SCFG DPHY Source Select: u0_mipitx_dphy_SCFG_dphy_src_sel"]
pub type ScfgDphySrcSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `scfg_dsi_txready_esc_sel` reader - SCFG DSI TX Ready Escape Select: u0_mipitx_dphy_SCFG_dsi_txready_esc_sel"]
pub type ScfgDsiTxreadyEscSelR = crate::FieldReader;
#[doc = "Field `scfg_dsi_txready_esc_sel` writer - SCFG DSI TX Ready Escape Select: u0_mipitx_dphy_SCFG_dsi_txready_esc_sel"]
pub type ScfgDsiTxreadyEscSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `scfg_ppi_c_ready_sel` reader - SCFG PPI C Ready Select: u0_mipitx_dphy_SCFG_ppi_c_ready_sel"]
pub type ScfgPpiCReadySelR = crate::FieldReader;
#[doc = "Field `scfg_ppi_c_ready_sel` writer - SCFG PPI C Ready Select: u0_mipitx_dphy_SCFG_ppi_c_ready_sel"]
pub type ScfgPpiCReadySelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `vcontrol` reader - VCONTROL: u0_mipitx_dphy_VCONTROL"]
pub type VcontrolR = crate::FieldReader;
#[doc = "Field `vcontrol` writer - VCONTROL: u0_mipitx_dphy_VCONTROL"]
pub type VcontrolW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bit 0 - SCFG DPHY Source Select: u0_mipitx_dphy_SCFG_dphy_src_sel"]
    #[inline(always)]
    pub fn scfg_dphy_src_sel(&self) -> ScfgDphySrcSelR {
        ScfgDphySrcSelR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - SCFG DSI TX Ready Escape Select: u0_mipitx_dphy_SCFG_dsi_txready_esc_sel"]
    #[inline(always)]
    pub fn scfg_dsi_txready_esc_sel(&self) -> ScfgDsiTxreadyEscSelR {
        ScfgDsiTxreadyEscSelR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 3:4 - SCFG PPI C Ready Select: u0_mipitx_dphy_SCFG_ppi_c_ready_sel"]
    #[inline(always)]
    pub fn scfg_ppi_c_ready_sel(&self) -> ScfgPpiCReadySelR {
        ScfgPpiCReadySelR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:9 - VCONTROL: u0_mipitx_dphy_VCONTROL"]
    #[inline(always)]
    pub fn vcontrol(&self) -> VcontrolR {
        VcontrolR::new(((self.bits >> 5) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - SCFG DPHY Source Select: u0_mipitx_dphy_SCFG_dphy_src_sel"]
    #[inline(always)]
    #[must_use]
    pub fn scfg_dphy_src_sel(&mut self) -> ScfgDphySrcSelW<Syscfg12Spec> {
        ScfgDphySrcSelW::new(self, 0)
    }
    #[doc = "Bits 1:2 - SCFG DSI TX Ready Escape Select: u0_mipitx_dphy_SCFG_dsi_txready_esc_sel"]
    #[inline(always)]
    #[must_use]
    pub fn scfg_dsi_txready_esc_sel(&mut self) -> ScfgDsiTxreadyEscSelW<Syscfg12Spec> {
        ScfgDsiTxreadyEscSelW::new(self, 1)
    }
    #[doc = "Bits 3:4 - SCFG PPI C Ready Select: u0_mipitx_dphy_SCFG_ppi_c_ready_sel"]
    #[inline(always)]
    #[must_use]
    pub fn scfg_ppi_c_ready_sel(&mut self) -> ScfgPpiCReadySelW<Syscfg12Spec> {
        ScfgPpiCReadySelW::new(self, 3)
    }
    #[doc = "Bits 5:9 - VCONTROL: u0_mipitx_dphy_VCONTROL"]
    #[inline(always)]
    #[must_use]
    pub fn vcontrol(&mut self) -> VcontrolW<Syscfg12Spec> {
        VcontrolW::new(self, 5)
    }
}
#[doc = "MIPITX DPHY SYSCFG 12: mipitx_apbifsaif_syscfg_48\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg12::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`syscfg12::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Syscfg12Spec;
impl crate::RegisterSpec for Syscfg12Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syscfg12::R`](R) reader structure"]
impl crate::Readable for Syscfg12Spec {}
#[doc = "`write(|w| ..)` method takes [`syscfg12::W`](W) writer structure"]
impl crate::Writable for Syscfg12Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets syscfg12 to value 0"]
impl crate::Resettable for Syscfg12Spec {
    const RESET_VALUE: u32 = 0;
}
