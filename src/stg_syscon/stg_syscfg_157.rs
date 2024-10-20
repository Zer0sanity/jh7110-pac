#[doc = "Register `stg_syscfg_157` reader"]
pub type R = crate::R<StgSyscfg157Spec>;
#[doc = "Register `stg_syscfg_157` writer"]
pub type W = crate::W<StgSyscfg157Spec>;
#[doc = "Field `u1_pcie_axi4_slvl_awfunc` reader - PCIE AXI4 SLV1 AWFUNC"]
pub type U1PcieAxi4SlvlAwfuncR = crate::FieldReader<u16>;
#[doc = "Field `u1_pcie_axi4_slvl_awfunc` writer - PCIE AXI4 SLV1 AWFUNC"]
pub type U1PcieAxi4SlvlAwfuncW<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
#[doc = "Field `u1_pcie_bus_width_o` reader - PCIE Bus width"]
pub type U1PcieBusWidthOR = crate::FieldReader;
#[doc = "Field `u1_pcie_bypass_codec` reader - PCIE Bypass Codec"]
pub type U1PcieBypassCodecR = crate::BitReader;
#[doc = "Field `u1_pcie_bypass_codec` writer - PCIE Bypass Codec"]
pub type U1PcieBypassCodecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u1_pcie_ckref_src` reader - PCIE Clock Reference Source"]
pub type U1PcieCkrefSrcR = crate::FieldReader;
#[doc = "Field `u1_pcie_ckref_src` writer - PCIE Clock Reference Source"]
pub type U1PcieCkrefSrcW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `u1_pcie_clk_sel` reader - PCIE Clock Select"]
pub type U1PcieClkSelR = crate::FieldReader;
#[doc = "Field `u1_pcie_clk_sel` writer - PCIE Clock Select"]
pub type U1PcieClkSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `u1_pcie_clkreq` reader - PCIE Clock Req"]
pub type U1PcieClkreqR = crate::BitReader;
#[doc = "Field `u1_pcie_clkreq` writer - PCIE Clock Req"]
pub type U1PcieClkreqW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:14 - PCIE AXI4 SLV1 AWFUNC"]
    #[inline(always)]
    pub fn u1_pcie_axi4_slvl_awfunc(&self) -> U1PcieAxi4SlvlAwfuncR {
        U1PcieAxi4SlvlAwfuncR::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bits 15:16 - PCIE Bus width"]
    #[inline(always)]
    pub fn u1_pcie_bus_width_o(&self) -> U1PcieBusWidthOR {
        U1PcieBusWidthOR::new(((self.bits >> 15) & 3) as u8)
    }
    #[doc = "Bit 17 - PCIE Bypass Codec"]
    #[inline(always)]
    pub fn u1_pcie_bypass_codec(&self) -> U1PcieBypassCodecR {
        U1PcieBypassCodecR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - PCIE Clock Reference Source"]
    #[inline(always)]
    pub fn u1_pcie_ckref_src(&self) -> U1PcieCkrefSrcR {
        U1PcieCkrefSrcR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - PCIE Clock Select"]
    #[inline(always)]
    pub fn u1_pcie_clk_sel(&self) -> U1PcieClkSelR {
        U1PcieClkSelR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22 - PCIE Clock Req"]
    #[inline(always)]
    pub fn u1_pcie_clkreq(&self) -> U1PcieClkreqR {
        U1PcieClkreqR::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:14 - PCIE AXI4 SLV1 AWFUNC"]
    #[inline(always)]
    #[must_use]
    pub fn u1_pcie_axi4_slvl_awfunc(&mut self) -> U1PcieAxi4SlvlAwfuncW<StgSyscfg157Spec> {
        U1PcieAxi4SlvlAwfuncW::new(self, 0)
    }
    #[doc = "Bit 17 - PCIE Bypass Codec"]
    #[inline(always)]
    #[must_use]
    pub fn u1_pcie_bypass_codec(&mut self) -> U1PcieBypassCodecW<StgSyscfg157Spec> {
        U1PcieBypassCodecW::new(self, 17)
    }
    #[doc = "Bits 18:19 - PCIE Clock Reference Source"]
    #[inline(always)]
    #[must_use]
    pub fn u1_pcie_ckref_src(&mut self) -> U1PcieCkrefSrcW<StgSyscfg157Spec> {
        U1PcieCkrefSrcW::new(self, 18)
    }
    #[doc = "Bits 20:21 - PCIE Clock Select"]
    #[inline(always)]
    #[must_use]
    pub fn u1_pcie_clk_sel(&mut self) -> U1PcieClkSelW<StgSyscfg157Spec> {
        U1PcieClkSelW::new(self, 20)
    }
    #[doc = "Bit 22 - PCIE Clock Req"]
    #[inline(always)]
    #[must_use]
    pub fn u1_pcie_clkreq(&mut self) -> U1PcieClkreqW<StgSyscfg157Spec> {
        U1PcieClkreqW::new(self, 22)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 628\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_157::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stg_syscfg_157::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg157Spec;
impl crate::RegisterSpec for StgSyscfg157Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_157::R`](R) reader structure"]
impl crate::Readable for StgSyscfg157Spec {}
#[doc = "`write(|w| ..)` method takes [`stg_syscfg_157::W`](W) writer structure"]
impl crate::Writable for StgSyscfg157Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets stg_syscfg_157 to value 0"]
impl crate::Resettable for StgSyscfg157Spec {
    const RESET_VALUE: u32 = 0;
}
