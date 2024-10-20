#[doc = "Register `gmac1_gtxc` reader"]
pub type R = crate::R<Gmac1GtxcSpec>;
#[doc = "Register `gmac1_gtxc` writer"]
pub type W = crate::W<Gmac1GtxcSpec>;
#[doc = "Field `dly_chain_sel` reader - Selector delay chain stage number, totally 32 stages, -50 ps each stage. The register value indicates the delay chain stage number. For example, diy_chain_sel=1 means to delay 1 stage."]
pub type DlyChainSelR = crate::FieldReader<u32>;
#[doc = "Field `dly_chain_sel` writer - Selector delay chain stage number, totally 32 stages, -50 ps each stage. The register value indicates the delay chain stage number. For example, diy_chain_sel=1 means to delay 1 stage."]
pub type DlyChainSelW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - Selector delay chain stage number, totally 32 stages, -50 ps each stage. The register value indicates the delay chain stage number. For example, diy_chain_sel=1 means to delay 1 stage."]
    #[inline(always)]
    pub fn dly_chain_sel(&self) -> DlyChainSelR {
        DlyChainSelR::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - Selector delay chain stage number, totally 32 stages, -50 ps each stage. The register value indicates the delay chain stage number. For example, diy_chain_sel=1 means to delay 1 stage."]
    #[inline(always)]
    #[must_use]
    pub fn dly_chain_sel(&mut self) -> DlyChainSelW<Gmac1GtxcSpec> {
        DlyChainSelW::new(self, 0)
    }
}
#[doc = "Clock GMAC1 GTXC\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac1_gtxc::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac1_gtxc::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gmac1GtxcSpec;
impl crate::RegisterSpec for Gmac1GtxcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gmac1_gtxc::R`](R) reader structure"]
impl crate::Readable for Gmac1GtxcSpec {}
#[doc = "`write(|w| ..)` method takes [`gmac1_gtxc::W`](W) writer structure"]
impl crate::Writable for Gmac1GtxcSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets gmac1_gtxc to value 0"]
impl crate::Resettable for Gmac1GtxcSpec {
    const RESET_VALUE: u32 = 0;
}
