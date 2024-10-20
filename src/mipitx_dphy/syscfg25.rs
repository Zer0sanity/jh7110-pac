#[doc = "Register `syscfg25` reader"]
pub type R = crate::R<Syscfg25Spec>;
#[doc = "Register `syscfg25` writer"]
pub type W = crate::W<Syscfg25Spec>;
#[doc = "Field `dbg1_mux_dout` reader - DBG1 Mux DOUT: u0_mipitx_dphy_dbg1_mux_dout"]
pub type Dbg1MuxDoutR = crate::FieldReader;
#[doc = "Field `dbg1_mux_sel` reader - DBG1 Mux Select: u0_mipitx_dphy_dbg1_mux_sel"]
pub type Dbg1MuxSelR = crate::FieldReader;
#[doc = "Field `dbg1_mux_sel` writer - DBG1 Mux Select: u0_mipitx_dphy_dbg1_mux_sel"]
pub type Dbg1MuxSelW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `dbg2_mux_dout` reader - DBG2 Mux DOUT: u0_mipitx_dphy_dbg2_mux_dout"]
pub type Dbg2MuxDoutR = crate::FieldReader;
#[doc = "Field `dbg2_mux_sel` reader - DBG2 Mux Select: u0_mipitx_dphy_dbg2_mux_sel"]
pub type Dbg2MuxSelR = crate::FieldReader;
#[doc = "Field `dbg2_mux_sel` writer - DBG2 Mux Select: u0_mipitx_dphy_dbg2_mux_sel"]
pub type Dbg2MuxSelW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `refclk_in_sel` reader - Reference Clock In Select: u0_mipitx_dphy_refclk_in_sel"]
pub type RefclkInSelR = crate::FieldReader;
#[doc = "Field `refclk_in_sel` writer - Reference Clock In Select: u0_mipitx_dphy_refclk_in_sel"]
pub type RefclkInSelW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `resetb` reader - Reset B: u0_mipitx_dphy_resetb"]
pub type ResetbR = crate::BitReader;
#[doc = "Field `resetb` writer - Reset B: u0_mipitx_dphy_resetb"]
pub type ResetbW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - DBG1 Mux DOUT: u0_mipitx_dphy_dbg1_mux_dout"]
    #[inline(always)]
    pub fn dbg1_mux_dout(&self) -> Dbg1MuxDoutR {
        Dbg1MuxDoutR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:12 - DBG1 Mux Select: u0_mipitx_dphy_dbg1_mux_sel"]
    #[inline(always)]
    pub fn dbg1_mux_sel(&self) -> Dbg1MuxSelR {
        Dbg1MuxSelR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 13:20 - DBG2 Mux DOUT: u0_mipitx_dphy_dbg2_mux_dout"]
    #[inline(always)]
    pub fn dbg2_mux_dout(&self) -> Dbg2MuxDoutR {
        Dbg2MuxDoutR::new(((self.bits >> 13) & 0xff) as u8)
    }
    #[doc = "Bits 21:25 - DBG2 Mux Select: u0_mipitx_dphy_dbg2_mux_sel"]
    #[inline(always)]
    pub fn dbg2_mux_sel(&self) -> Dbg2MuxSelR {
        Dbg2MuxSelR::new(((self.bits >> 21) & 0x1f) as u8)
    }
    #[doc = "Bits 26:28 - Reference Clock In Select: u0_mipitx_dphy_refclk_in_sel"]
    #[inline(always)]
    pub fn refclk_in_sel(&self) -> RefclkInSelR {
        RefclkInSelR::new(((self.bits >> 26) & 7) as u8)
    }
    #[doc = "Bit 29 - Reset B: u0_mipitx_dphy_resetb"]
    #[inline(always)]
    pub fn resetb(&self) -> ResetbR {
        ResetbR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 8:12 - DBG1 Mux Select: u0_mipitx_dphy_dbg1_mux_sel"]
    #[inline(always)]
    #[must_use]
    pub fn dbg1_mux_sel(&mut self) -> Dbg1MuxSelW<Syscfg25Spec> {
        Dbg1MuxSelW::new(self, 8)
    }
    #[doc = "Bits 21:25 - DBG2 Mux Select: u0_mipitx_dphy_dbg2_mux_sel"]
    #[inline(always)]
    #[must_use]
    pub fn dbg2_mux_sel(&mut self) -> Dbg2MuxSelW<Syscfg25Spec> {
        Dbg2MuxSelW::new(self, 21)
    }
    #[doc = "Bits 26:28 - Reference Clock In Select: u0_mipitx_dphy_refclk_in_sel"]
    #[inline(always)]
    #[must_use]
    pub fn refclk_in_sel(&mut self) -> RefclkInSelW<Syscfg25Spec> {
        RefclkInSelW::new(self, 26)
    }
    #[doc = "Bit 29 - Reset B: u0_mipitx_dphy_resetb"]
    #[inline(always)]
    #[must_use]
    pub fn resetb(&mut self) -> ResetbW<Syscfg25Spec> {
        ResetbW::new(self, 29)
    }
}
#[doc = "MIPITX DPHY SYSCFG 25: mipitx_apbifsaif_syscfg_100\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`syscfg25::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`syscfg25::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Syscfg25Spec;
impl crate::RegisterSpec for Syscfg25Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syscfg25::R`](R) reader structure"]
impl crate::Readable for Syscfg25Spec {}
#[doc = "`write(|w| ..)` method takes [`syscfg25::W`](W) writer structure"]
impl crate::Writable for Syscfg25Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets syscfg25 to value 0"]
impl crate::Resettable for Syscfg25Spec {
    const RESET_VALUE: u32 = 0;
}
