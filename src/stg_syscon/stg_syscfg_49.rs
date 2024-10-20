#[doc = "Register `stg_syscfg_49` reader"]
pub type R = crate::R<StgSyscfg49Spec>;
#[doc = "Register `stg_syscfg_49` writer"]
pub type W = crate::W<StgSyscfg49Spec>;
#[doc = "Field `u0_pcie_axi4_slvl_awfunc` reader - PCIE AXI4 SLV1 AWFUNC"]
pub type U0PcieAxi4SlvlAwfuncR = crate::FieldReader<u16>;
#[doc = "Field `u0_pcie_axi4_slvl_awfunc` writer - PCIE AXI4 SLV1 AWFUNC"]
pub type U0PcieAxi4SlvlAwfuncW<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
#[doc = "Field `u0_pcie_bus_width_o` reader - PCIE Bus width"]
pub type U0PcieBusWidthOR = crate::FieldReader;
#[doc = "Field `u0_pcie_bypass_codec` reader - PCIE Bypass Codec"]
pub type U0PcieBypassCodecR = crate::BitReader;
#[doc = "Field `u0_pcie_bypass_codec` writer - PCIE Bypass Codec"]
pub type U0PcieBypassCodecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u0_pcie_ckref_src` reader - PCIE Clock Reference Source"]
pub type U0PcieCkrefSrcR = crate::FieldReader;
#[doc = "Field `u0_pcie_ckref_src` writer - PCIE Clock Reference Source"]
pub type U0PcieCkrefSrcW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `u0_pcie_clk_sel` reader - PCIE Clock Select"]
pub type U0PcieClkSelR = crate::FieldReader;
#[doc = "Field `u0_pcie_clk_sel` writer - PCIE Clock Select"]
pub type U0PcieClkSelW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `u0_pcie_clkreq` reader - PCIE Clock Req"]
pub type U0PcieClkreqR = crate::BitReader;
#[doc = "Field `u0_pcie_clkreq` writer - PCIE Clock Req"]
pub type U0PcieClkreqW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:14 - PCIE AXI4 SLV1 AWFUNC"]
    #[inline(always)]
    pub fn u0_pcie_axi4_slvl_awfunc(&self) -> U0PcieAxi4SlvlAwfuncR {
        U0PcieAxi4SlvlAwfuncR::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bits 15:16 - PCIE Bus width"]
    #[inline(always)]
    pub fn u0_pcie_bus_width_o(&self) -> U0PcieBusWidthOR {
        U0PcieBusWidthOR::new(((self.bits >> 15) & 3) as u8)
    }
    #[doc = "Bit 17 - PCIE Bypass Codec"]
    #[inline(always)]
    pub fn u0_pcie_bypass_codec(&self) -> U0PcieBypassCodecR {
        U0PcieBypassCodecR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - PCIE Clock Reference Source"]
    #[inline(always)]
    pub fn u0_pcie_ckref_src(&self) -> U0PcieCkrefSrcR {
        U0PcieCkrefSrcR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - PCIE Clock Select"]
    #[inline(always)]
    pub fn u0_pcie_clk_sel(&self) -> U0PcieClkSelR {
        U0PcieClkSelR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22 - PCIE Clock Req"]
    #[inline(always)]
    pub fn u0_pcie_clkreq(&self) -> U0PcieClkreqR {
        U0PcieClkreqR::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:14 - PCIE AXI4 SLV1 AWFUNC"]
    #[inline(always)]
    #[must_use]
    pub fn u0_pcie_axi4_slvl_awfunc(&mut self) -> U0PcieAxi4SlvlAwfuncW<StgSyscfg49Spec> {
        U0PcieAxi4SlvlAwfuncW::new(self, 0)
    }
    #[doc = "Bit 17 - PCIE Bypass Codec"]
    #[inline(always)]
    #[must_use]
    pub fn u0_pcie_bypass_codec(&mut self) -> U0PcieBypassCodecW<StgSyscfg49Spec> {
        U0PcieBypassCodecW::new(self, 17)
    }
    #[doc = "Bits 18:19 - PCIE Clock Reference Source"]
    #[inline(always)]
    #[must_use]
    pub fn u0_pcie_ckref_src(&mut self) -> U0PcieCkrefSrcW<StgSyscfg49Spec> {
        U0PcieCkrefSrcW::new(self, 18)
    }
    #[doc = "Bits 20:21 - PCIE Clock Select"]
    #[inline(always)]
    #[must_use]
    pub fn u0_pcie_clk_sel(&mut self) -> U0PcieClkSelW<StgSyscfg49Spec> {
        U0PcieClkSelW::new(self, 20)
    }
    #[doc = "Bit 22 - PCIE Clock Req"]
    #[inline(always)]
    #[must_use]
    pub fn u0_pcie_clkreq(&mut self) -> U0PcieClkreqW<StgSyscfg49Spec> {
        U0PcieClkreqW::new(self, 22)
    }
}
#[doc = "STG SYSCONSAIF SYSCFG 196\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`stg_syscfg_49::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`stg_syscfg_49::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StgSyscfg49Spec;
impl crate::RegisterSpec for StgSyscfg49Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stg_syscfg_49::R`](R) reader structure"]
impl crate::Readable for StgSyscfg49Spec {}
#[doc = "`write(|w| ..)` method takes [`stg_syscfg_49::W`](W) writer structure"]
impl crate::Writable for StgSyscfg49Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets stg_syscfg_49 to value 0"]
impl crate::Resettable for StgSyscfg49Spec {
    const RESET_VALUE: u32 = 0;
}
