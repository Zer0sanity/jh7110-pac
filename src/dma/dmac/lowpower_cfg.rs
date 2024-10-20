#[doc = "Register `lowpower_cfg` reader"]
pub type R = crate::R<LowpowerCfgSpec>;
#[doc = "Register `lowpower_cfg` writer"]
pub type W = crate::W<LowpowerCfgSpec>;
#[doc = "Field `clsp_en(_gbl,_chnl,_sbiu,_mxif)` reader - DMAC Context Sensitive Low Power feature enable - 0: disable, 1: enable. GBL: Global, CHNL: Channel, SBIU: SBIU, MXI: AXI Master Interface"]
pub type ClspEnR = crate::BitReader;
#[doc = "Field `clsp_en(_gbl,_chnl,_sbiu,_mxif)` writer - DMAC Context Sensitive Low Power feature enable - 0: disable, 1: enable. GBL: Global, CHNL: Channel, SBIU: SBIU, MXI: AXI Master Interface"]
pub type ClspEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `lpdly(_glch,_sbiu,_mxif)` reader - DMAC Low Power Delay counter. GLCH: Global and DMA channel, SBIU: SBIU, MXI: AXI Master Interface"]
pub type LpdlyR = crate::FieldReader;
#[doc = "Field `lpdly(_glch,_sbiu,_mxif)` writer - DMAC Low Power Delay counter. GLCH: Global and DMA channel, SBIU: SBIU, MXI: AXI Master Interface"]
pub type LpdlyW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "DMAC Context Sensitive Low Power feature enable - 0: disable, 1: enable. GBL: Global, CHNL: Channel, SBIU: SBIU, MXI: AXI Master Interface"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `clsp_en_gbl` field"]
    #[inline(always)]
    pub fn clsp_en(&self, n: u8) -> ClspEnR {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        ClspEnR::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "DMAC Context Sensitive Low Power feature enable - 0: disable, 1: enable. GBL: Global, CHNL: Channel, SBIU: SBIU, MXI: AXI Master Interface"]
    #[inline(always)]
    pub fn clsp_en_iter(&self) -> impl Iterator<Item = ClspEnR> + '_ {
        (0..4).map(move |n| ClspEnR::new(((self.bits >> n) & 1) != 0))
    }
    #[doc = "Bit 0 - DMAC Context Sensitive Low Power feature enable - 0: disable, 1: enable. GBL: Global, CHNL: Channel, SBIU: SBIU, MXI: AXI Master Interface"]
    #[inline(always)]
    pub fn clsp_en_gbl(&self) -> ClspEnR {
        ClspEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMAC Context Sensitive Low Power feature enable - 0: disable, 1: enable. GBL: Global, CHNL: Channel, SBIU: SBIU, MXI: AXI Master Interface"]
    #[inline(always)]
    pub fn clsp_en_chnl(&self) -> ClspEnR {
        ClspEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DMAC Context Sensitive Low Power feature enable - 0: disable, 1: enable. GBL: Global, CHNL: Channel, SBIU: SBIU, MXI: AXI Master Interface"]
    #[inline(always)]
    pub fn clsp_en_sbiu(&self) -> ClspEnR {
        ClspEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMAC Context Sensitive Low Power feature enable - 0: disable, 1: enable. GBL: Global, CHNL: Channel, SBIU: SBIU, MXI: AXI Master Interface"]
    #[inline(always)]
    pub fn clsp_en_mxif(&self) -> ClspEnR {
        ClspEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "DMAC Low Power Delay counter. GLCH: Global and DMA channel, SBIU: SBIU, MXI: AXI Master Interface"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `lpdly_glch` field"]
    #[inline(always)]
    pub fn lpdly(&self, n: u8) -> LpdlyR {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        LpdlyR::new(((self.bits >> (n * 8 + 32)) & 0xff) as u8)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "DMAC Low Power Delay counter. GLCH: Global and DMA channel, SBIU: SBIU, MXI: AXI Master Interface"]
    #[inline(always)]
    pub fn lpdly_iter(&self) -> impl Iterator<Item = LpdlyR> + '_ {
        (0..3).map(move |n| LpdlyR::new(((self.bits >> (n * 8 + 32)) & 0xff) as u8))
    }
    #[doc = "Bits 32:39 - DMAC Low Power Delay counter. GLCH: Global and DMA channel, SBIU: SBIU, MXI: AXI Master Interface"]
    #[inline(always)]
    pub fn lpdly_glch(&self) -> LpdlyR {
        LpdlyR::new(((self.bits >> 32) & 0xff) as u8)
    }
    #[doc = "Bits 40:47 - DMAC Low Power Delay counter. GLCH: Global and DMA channel, SBIU: SBIU, MXI: AXI Master Interface"]
    #[inline(always)]
    pub fn lpdly_sbiu(&self) -> LpdlyR {
        LpdlyR::new(((self.bits >> 40) & 0xff) as u8)
    }
    #[doc = "Bits 48:55 - DMAC Low Power Delay counter. GLCH: Global and DMA channel, SBIU: SBIU, MXI: AXI Master Interface"]
    #[inline(always)]
    pub fn lpdly_mxif(&self) -> LpdlyR {
        LpdlyR::new(((self.bits >> 48) & 0xff) as u8)
    }
}
impl W {
    #[doc = "DMAC Context Sensitive Low Power feature enable - 0: disable, 1: enable. GBL: Global, CHNL: Channel, SBIU: SBIU, MXI: AXI Master Interface"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `clsp_en_gbl` field"]
    #[inline(always)]
    #[must_use]
    pub fn clsp_en(&mut self, n: u8) -> ClspEnW<LowpowerCfgSpec> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        ClspEnW::new(self, n)
    }
    #[doc = "Bit 0 - DMAC Context Sensitive Low Power feature enable - 0: disable, 1: enable. GBL: Global, CHNL: Channel, SBIU: SBIU, MXI: AXI Master Interface"]
    #[inline(always)]
    #[must_use]
    pub fn clsp_en_gbl(&mut self) -> ClspEnW<LowpowerCfgSpec> {
        ClspEnW::new(self, 0)
    }
    #[doc = "Bit 1 - DMAC Context Sensitive Low Power feature enable - 0: disable, 1: enable. GBL: Global, CHNL: Channel, SBIU: SBIU, MXI: AXI Master Interface"]
    #[inline(always)]
    #[must_use]
    pub fn clsp_en_chnl(&mut self) -> ClspEnW<LowpowerCfgSpec> {
        ClspEnW::new(self, 1)
    }
    #[doc = "Bit 2 - DMAC Context Sensitive Low Power feature enable - 0: disable, 1: enable. GBL: Global, CHNL: Channel, SBIU: SBIU, MXI: AXI Master Interface"]
    #[inline(always)]
    #[must_use]
    pub fn clsp_en_sbiu(&mut self) -> ClspEnW<LowpowerCfgSpec> {
        ClspEnW::new(self, 2)
    }
    #[doc = "Bit 3 - DMAC Context Sensitive Low Power feature enable - 0: disable, 1: enable. GBL: Global, CHNL: Channel, SBIU: SBIU, MXI: AXI Master Interface"]
    #[inline(always)]
    #[must_use]
    pub fn clsp_en_mxif(&mut self) -> ClspEnW<LowpowerCfgSpec> {
        ClspEnW::new(self, 3)
    }
    #[doc = "DMAC Low Power Delay counter. GLCH: Global and DMA channel, SBIU: SBIU, MXI: AXI Master Interface"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `lpdly_glch` field"]
    #[inline(always)]
    #[must_use]
    pub fn lpdly(&mut self, n: u8) -> LpdlyW<LowpowerCfgSpec> {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        LpdlyW::new(self, n * 8 + 32)
    }
    #[doc = "Bits 32:39 - DMAC Low Power Delay counter. GLCH: Global and DMA channel, SBIU: SBIU, MXI: AXI Master Interface"]
    #[inline(always)]
    #[must_use]
    pub fn lpdly_glch(&mut self) -> LpdlyW<LowpowerCfgSpec> {
        LpdlyW::new(self, 32)
    }
    #[doc = "Bits 40:47 - DMAC Low Power Delay counter. GLCH: Global and DMA channel, SBIU: SBIU, MXI: AXI Master Interface"]
    #[inline(always)]
    #[must_use]
    pub fn lpdly_sbiu(&mut self) -> LpdlyW<LowpowerCfgSpec> {
        LpdlyW::new(self, 40)
    }
    #[doc = "Bits 48:55 - DMAC Low Power Delay counter. GLCH: Global and DMA channel, SBIU: SBIU, MXI: AXI Master Interface"]
    #[inline(always)]
    #[must_use]
    pub fn lpdly_mxif(&mut self) -> LpdlyW<LowpowerCfgSpec> {
        LpdlyW::new(self, 48)
    }
}
#[doc = "DMAC Low Power Configuration register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lowpower_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lowpower_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LowpowerCfgSpec;
impl crate::RegisterSpec for LowpowerCfgSpec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`lowpower_cfg::R`](R) reader structure"]
impl crate::Readable for LowpowerCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`lowpower_cfg::W`](W) writer structure"]
impl crate::Writable for LowpowerCfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
}
#[doc = "`reset()` method sets lowpower_cfg to value 0"]
impl crate::Resettable for LowpowerCfgSpec {
    const RESET_VALUE: u64 = 0;
}
