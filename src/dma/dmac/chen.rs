#[doc = "Register `chen` reader"]
pub type R = crate::R<ChenSpec>;
#[doc = "Register `chen` writer"]
pub type W = crate::W<ChenSpec>;
#[doc = "Field `en_ch(1-8)` reader - DMAC Channel Enable - 0: disable DMAC channel, 1: enable DMAC channel"]
pub type EnChR = crate::BitReader;
#[doc = "Field `en_ch(1-8)` writer - DMAC Channel Enable - 0: disable DMAC channel, 1: enable DMAC channel"]
pub type EnChW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `en_we_ch(1-8)` writer - DMAC Channel Enable Write-enable - 0: disable write to DMAC channel enable bit, 1: enable write to DMAC channel enable bit"]
pub type EnWeChW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `susp_ch(1-8)` reader - DMAC Channel Suspend - 0: no DMAC channel suspend request, 1: DMAC channel suspend request"]
pub type SuspChR = crate::BitReader;
#[doc = "Field `susp_ch(1-8)` writer - DMAC Channel Suspend - 0: no DMAC channel suspend request, 1: DMAC channel suspend request"]
pub type SuspChW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `susp_we_ch(1-8)` writer - DMAC Channel Suspend Write-enable - 0: disable write to DMAC channel suspend bit, 1: enable write to DMAC channel suspend bit"]
pub type SuspWeChW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `abort_ch(1-8)` reader - DMAC Channel Abort - 0: no DMAC channel abort request, 1: request DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: read-write"]
pub type AbortChR = crate::BitReader;
#[doc = "Field `abort_ch(1-8)` writer - DMAC Channel Abort - 0: no DMAC channel abort request, 1: request DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: read-write"]
pub type AbortChW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `abort_we_ch(1-8)` reader - DMAC Channel Abort Write-enable - 0: disable write to DMAC channel abort bit, 1: enable write DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: write-only"]
pub type AbortWeChR = crate::BitReader;
#[doc = "Field `abort_we_ch(1-8)` writer - DMAC Channel Abort Write-enable - 0: disable write to DMAC channel abort bit, 1: enable write DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: write-only"]
pub type AbortWeChW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "DMAC Channel Enable - 0: disable DMAC channel, 1: enable DMAC channel"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `en_ch1` field"]
    #[inline(always)]
    pub fn en_ch(&self, n: u8) -> EnChR {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        EnChR::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "DMAC Channel Enable - 0: disable DMAC channel, 1: enable DMAC channel"]
    #[inline(always)]
    pub fn en_ch_iter(&self) -> impl Iterator<Item = EnChR> + '_ {
        (0..8).map(move |n| EnChR::new(((self.bits >> n) & 1) != 0))
    }
    #[doc = "Bit 0 - DMAC Channel Enable - 0: disable DMAC channel, 1: enable DMAC channel"]
    #[inline(always)]
    pub fn en_ch1(&self) -> EnChR {
        EnChR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMAC Channel Enable - 0: disable DMAC channel, 1: enable DMAC channel"]
    #[inline(always)]
    pub fn en_ch2(&self) -> EnChR {
        EnChR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DMAC Channel Enable - 0: disable DMAC channel, 1: enable DMAC channel"]
    #[inline(always)]
    pub fn en_ch3(&self) -> EnChR {
        EnChR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMAC Channel Enable - 0: disable DMAC channel, 1: enable DMAC channel"]
    #[inline(always)]
    pub fn en_ch4(&self) -> EnChR {
        EnChR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DMAC Channel Enable - 0: disable DMAC channel, 1: enable DMAC channel"]
    #[inline(always)]
    pub fn en_ch5(&self) -> EnChR {
        EnChR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DMAC Channel Enable - 0: disable DMAC channel, 1: enable DMAC channel"]
    #[inline(always)]
    pub fn en_ch6(&self) -> EnChR {
        EnChR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - DMAC Channel Enable - 0: disable DMAC channel, 1: enable DMAC channel"]
    #[inline(always)]
    pub fn en_ch7(&self) -> EnChR {
        EnChR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DMAC Channel Enable - 0: disable DMAC channel, 1: enable DMAC channel"]
    #[inline(always)]
    pub fn en_ch8(&self) -> EnChR {
        EnChR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "DMAC Channel Suspend - 0: no DMAC channel suspend request, 1: DMAC channel suspend request"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `susp_ch1` field"]
    #[inline(always)]
    pub fn susp_ch(&self, n: u8) -> SuspChR {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        SuspChR::new(((self.bits >> (n + 16)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "DMAC Channel Suspend - 0: no DMAC channel suspend request, 1: DMAC channel suspend request"]
    #[inline(always)]
    pub fn susp_ch_iter(&self) -> impl Iterator<Item = SuspChR> + '_ {
        (0..8).map(move |n| SuspChR::new(((self.bits >> (n + 16)) & 1) != 0))
    }
    #[doc = "Bit 16 - DMAC Channel Suspend - 0: no DMAC channel suspend request, 1: DMAC channel suspend request"]
    #[inline(always)]
    pub fn susp_ch1(&self) -> SuspChR {
        SuspChR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - DMAC Channel Suspend - 0: no DMAC channel suspend request, 1: DMAC channel suspend request"]
    #[inline(always)]
    pub fn susp_ch2(&self) -> SuspChR {
        SuspChR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - DMAC Channel Suspend - 0: no DMAC channel suspend request, 1: DMAC channel suspend request"]
    #[inline(always)]
    pub fn susp_ch3(&self) -> SuspChR {
        SuspChR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - DMAC Channel Suspend - 0: no DMAC channel suspend request, 1: DMAC channel suspend request"]
    #[inline(always)]
    pub fn susp_ch4(&self) -> SuspChR {
        SuspChR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - DMAC Channel Suspend - 0: no DMAC channel suspend request, 1: DMAC channel suspend request"]
    #[inline(always)]
    pub fn susp_ch5(&self) -> SuspChR {
        SuspChR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - DMAC Channel Suspend - 0: no DMAC channel suspend request, 1: DMAC channel suspend request"]
    #[inline(always)]
    pub fn susp_ch6(&self) -> SuspChR {
        SuspChR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - DMAC Channel Suspend - 0: no DMAC channel suspend request, 1: DMAC channel suspend request"]
    #[inline(always)]
    pub fn susp_ch7(&self) -> SuspChR {
        SuspChR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - DMAC Channel Suspend - 0: no DMAC channel suspend request, 1: DMAC channel suspend request"]
    #[inline(always)]
    pub fn susp_ch8(&self) -> SuspChR {
        SuspChR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "DMAC Channel Abort - 0: no DMAC channel abort request, 1: request DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: read-write"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `abort_ch1` field"]
    #[inline(always)]
    pub fn abort_ch(&self, n: u8) -> AbortChR {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        AbortChR::new(((self.bits >> (n + 32)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "DMAC Channel Abort - 0: no DMAC channel abort request, 1: request DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: read-write"]
    #[inline(always)]
    pub fn abort_ch_iter(&self) -> impl Iterator<Item = AbortChR> + '_ {
        (0..8).map(move |n| AbortChR::new(((self.bits >> (n + 32)) & 1) != 0))
    }
    #[doc = "Bit 32 - DMAC Channel Abort - 0: no DMAC channel abort request, 1: request DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: read-write"]
    #[inline(always)]
    pub fn abort_ch1(&self) -> AbortChR {
        AbortChR::new(((self.bits >> 32) & 1) != 0)
    }
    #[doc = "Bit 33 - DMAC Channel Abort - 0: no DMAC channel abort request, 1: request DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: read-write"]
    #[inline(always)]
    pub fn abort_ch2(&self) -> AbortChR {
        AbortChR::new(((self.bits >> 33) & 1) != 0)
    }
    #[doc = "Bit 34 - DMAC Channel Abort - 0: no DMAC channel abort request, 1: request DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: read-write"]
    #[inline(always)]
    pub fn abort_ch3(&self) -> AbortChR {
        AbortChR::new(((self.bits >> 34) & 1) != 0)
    }
    #[doc = "Bit 35 - DMAC Channel Abort - 0: no DMAC channel abort request, 1: request DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: read-write"]
    #[inline(always)]
    pub fn abort_ch4(&self) -> AbortChR {
        AbortChR::new(((self.bits >> 35) & 1) != 0)
    }
    #[doc = "Bit 36 - DMAC Channel Abort - 0: no DMAC channel abort request, 1: request DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: read-write"]
    #[inline(always)]
    pub fn abort_ch5(&self) -> AbortChR {
        AbortChR::new(((self.bits >> 36) & 1) != 0)
    }
    #[doc = "Bit 37 - DMAC Channel Abort - 0: no DMAC channel abort request, 1: request DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: read-write"]
    #[inline(always)]
    pub fn abort_ch6(&self) -> AbortChR {
        AbortChR::new(((self.bits >> 37) & 1) != 0)
    }
    #[doc = "Bit 38 - DMAC Channel Abort - 0: no DMAC channel abort request, 1: request DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: read-write"]
    #[inline(always)]
    pub fn abort_ch7(&self) -> AbortChR {
        AbortChR::new(((self.bits >> 38) & 1) != 0)
    }
    #[doc = "Bit 39 - DMAC Channel Abort - 0: no DMAC channel abort request, 1: request DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: read-write"]
    #[inline(always)]
    pub fn abort_ch8(&self) -> AbortChR {
        AbortChR::new(((self.bits >> 39) & 1) != 0)
    }
    #[doc = "DMAC Channel Abort Write-enable - 0: disable write to DMAC channel abort bit, 1: enable write DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: write-only"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `abort_we_ch1` field"]
    #[inline(always)]
    pub fn abort_we_ch(&self, n: u8) -> AbortWeChR {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        AbortWeChR::new(((self.bits >> (n + 40)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "DMAC Channel Abort Write-enable - 0: disable write to DMAC channel abort bit, 1: enable write DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: write-only"]
    #[inline(always)]
    pub fn abort_we_ch_iter(&self) -> impl Iterator<Item = AbortWeChR> + '_ {
        (0..8).map(move |n| AbortWeChR::new(((self.bits >> (n + 40)) & 1) != 0))
    }
    #[doc = "Bit 40 - DMAC Channel Abort Write-enable - 0: disable write to DMAC channel abort bit, 1: enable write DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: write-only"]
    #[inline(always)]
    pub fn abort_we_ch1(&self) -> AbortWeChR {
        AbortWeChR::new(((self.bits >> 40) & 1) != 0)
    }
    #[doc = "Bit 41 - DMAC Channel Abort Write-enable - 0: disable write to DMAC channel abort bit, 1: enable write DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: write-only"]
    #[inline(always)]
    pub fn abort_we_ch2(&self) -> AbortWeChR {
        AbortWeChR::new(((self.bits >> 41) & 1) != 0)
    }
    #[doc = "Bit 42 - DMAC Channel Abort Write-enable - 0: disable write to DMAC channel abort bit, 1: enable write DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: write-only"]
    #[inline(always)]
    pub fn abort_we_ch3(&self) -> AbortWeChR {
        AbortWeChR::new(((self.bits >> 42) & 1) != 0)
    }
    #[doc = "Bit 43 - DMAC Channel Abort Write-enable - 0: disable write to DMAC channel abort bit, 1: enable write DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: write-only"]
    #[inline(always)]
    pub fn abort_we_ch4(&self) -> AbortWeChR {
        AbortWeChR::new(((self.bits >> 43) & 1) != 0)
    }
    #[doc = "Bit 44 - DMAC Channel Abort Write-enable - 0: disable write to DMAC channel abort bit, 1: enable write DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: write-only"]
    #[inline(always)]
    pub fn abort_we_ch5(&self) -> AbortWeChR {
        AbortWeChR::new(((self.bits >> 44) & 1) != 0)
    }
    #[doc = "Bit 45 - DMAC Channel Abort Write-enable - 0: disable write to DMAC channel abort bit, 1: enable write DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: write-only"]
    #[inline(always)]
    pub fn abort_we_ch6(&self) -> AbortWeChR {
        AbortWeChR::new(((self.bits >> 45) & 1) != 0)
    }
    #[doc = "Bit 46 - DMAC Channel Abort Write-enable - 0: disable write to DMAC channel abort bit, 1: enable write DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: write-only"]
    #[inline(always)]
    pub fn abort_we_ch7(&self) -> AbortWeChR {
        AbortWeChR::new(((self.bits >> 46) & 1) != 0)
    }
    #[doc = "Bit 47 - DMAC Channel Abort Write-enable - 0: disable write to DMAC channel abort bit, 1: enable write DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: write-only"]
    #[inline(always)]
    pub fn abort_we_ch8(&self) -> AbortWeChR {
        AbortWeChR::new(((self.bits >> 47) & 1) != 0)
    }
}
impl W {
    #[doc = "DMAC Channel Enable - 0: disable DMAC channel, 1: enable DMAC channel"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `en_ch1` field"]
    #[inline(always)]
    #[must_use]
    pub fn en_ch(&mut self, n: u8) -> EnChW<ChenSpec> {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        EnChW::new(self, n)
    }
    #[doc = "Bit 0 - DMAC Channel Enable - 0: disable DMAC channel, 1: enable DMAC channel"]
    #[inline(always)]
    #[must_use]
    pub fn en_ch1(&mut self) -> EnChW<ChenSpec> {
        EnChW::new(self, 0)
    }
    #[doc = "Bit 1 - DMAC Channel Enable - 0: disable DMAC channel, 1: enable DMAC channel"]
    #[inline(always)]
    #[must_use]
    pub fn en_ch2(&mut self) -> EnChW<ChenSpec> {
        EnChW::new(self, 1)
    }
    #[doc = "Bit 2 - DMAC Channel Enable - 0: disable DMAC channel, 1: enable DMAC channel"]
    #[inline(always)]
    #[must_use]
    pub fn en_ch3(&mut self) -> EnChW<ChenSpec> {
        EnChW::new(self, 2)
    }
    #[doc = "Bit 3 - DMAC Channel Enable - 0: disable DMAC channel, 1: enable DMAC channel"]
    #[inline(always)]
    #[must_use]
    pub fn en_ch4(&mut self) -> EnChW<ChenSpec> {
        EnChW::new(self, 3)
    }
    #[doc = "Bit 4 - DMAC Channel Enable - 0: disable DMAC channel, 1: enable DMAC channel"]
    #[inline(always)]
    #[must_use]
    pub fn en_ch5(&mut self) -> EnChW<ChenSpec> {
        EnChW::new(self, 4)
    }
    #[doc = "Bit 5 - DMAC Channel Enable - 0: disable DMAC channel, 1: enable DMAC channel"]
    #[inline(always)]
    #[must_use]
    pub fn en_ch6(&mut self) -> EnChW<ChenSpec> {
        EnChW::new(self, 5)
    }
    #[doc = "Bit 6 - DMAC Channel Enable - 0: disable DMAC channel, 1: enable DMAC channel"]
    #[inline(always)]
    #[must_use]
    pub fn en_ch7(&mut self) -> EnChW<ChenSpec> {
        EnChW::new(self, 6)
    }
    #[doc = "Bit 7 - DMAC Channel Enable - 0: disable DMAC channel, 1: enable DMAC channel"]
    #[inline(always)]
    #[must_use]
    pub fn en_ch8(&mut self) -> EnChW<ChenSpec> {
        EnChW::new(self, 7)
    }
    #[doc = "DMAC Channel Enable Write-enable - 0: disable write to DMAC channel enable bit, 1: enable write to DMAC channel enable bit"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `en_we_ch1` field"]
    #[inline(always)]
    #[must_use]
    pub fn en_we_ch(&mut self, n: u8) -> EnWeChW<ChenSpec> {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        EnWeChW::new(self, n + 8)
    }
    #[doc = "Bit 8 - DMAC Channel Enable Write-enable - 0: disable write to DMAC channel enable bit, 1: enable write to DMAC channel enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn en_we_ch1(&mut self) -> EnWeChW<ChenSpec> {
        EnWeChW::new(self, 8)
    }
    #[doc = "Bit 9 - DMAC Channel Enable Write-enable - 0: disable write to DMAC channel enable bit, 1: enable write to DMAC channel enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn en_we_ch2(&mut self) -> EnWeChW<ChenSpec> {
        EnWeChW::new(self, 9)
    }
    #[doc = "Bit 10 - DMAC Channel Enable Write-enable - 0: disable write to DMAC channel enable bit, 1: enable write to DMAC channel enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn en_we_ch3(&mut self) -> EnWeChW<ChenSpec> {
        EnWeChW::new(self, 10)
    }
    #[doc = "Bit 11 - DMAC Channel Enable Write-enable - 0: disable write to DMAC channel enable bit, 1: enable write to DMAC channel enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn en_we_ch4(&mut self) -> EnWeChW<ChenSpec> {
        EnWeChW::new(self, 11)
    }
    #[doc = "Bit 12 - DMAC Channel Enable Write-enable - 0: disable write to DMAC channel enable bit, 1: enable write to DMAC channel enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn en_we_ch5(&mut self) -> EnWeChW<ChenSpec> {
        EnWeChW::new(self, 12)
    }
    #[doc = "Bit 13 - DMAC Channel Enable Write-enable - 0: disable write to DMAC channel enable bit, 1: enable write to DMAC channel enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn en_we_ch6(&mut self) -> EnWeChW<ChenSpec> {
        EnWeChW::new(self, 13)
    }
    #[doc = "Bit 14 - DMAC Channel Enable Write-enable - 0: disable write to DMAC channel enable bit, 1: enable write to DMAC channel enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn en_we_ch7(&mut self) -> EnWeChW<ChenSpec> {
        EnWeChW::new(self, 14)
    }
    #[doc = "Bit 15 - DMAC Channel Enable Write-enable - 0: disable write to DMAC channel enable bit, 1: enable write to DMAC channel enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn en_we_ch8(&mut self) -> EnWeChW<ChenSpec> {
        EnWeChW::new(self, 15)
    }
    #[doc = "DMAC Channel Suspend - 0: no DMAC channel suspend request, 1: DMAC channel suspend request"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `susp_ch1` field"]
    #[inline(always)]
    #[must_use]
    pub fn susp_ch(&mut self, n: u8) -> SuspChW<ChenSpec> {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        SuspChW::new(self, n + 16)
    }
    #[doc = "Bit 16 - DMAC Channel Suspend - 0: no DMAC channel suspend request, 1: DMAC channel suspend request"]
    #[inline(always)]
    #[must_use]
    pub fn susp_ch1(&mut self) -> SuspChW<ChenSpec> {
        SuspChW::new(self, 16)
    }
    #[doc = "Bit 17 - DMAC Channel Suspend - 0: no DMAC channel suspend request, 1: DMAC channel suspend request"]
    #[inline(always)]
    #[must_use]
    pub fn susp_ch2(&mut self) -> SuspChW<ChenSpec> {
        SuspChW::new(self, 17)
    }
    #[doc = "Bit 18 - DMAC Channel Suspend - 0: no DMAC channel suspend request, 1: DMAC channel suspend request"]
    #[inline(always)]
    #[must_use]
    pub fn susp_ch3(&mut self) -> SuspChW<ChenSpec> {
        SuspChW::new(self, 18)
    }
    #[doc = "Bit 19 - DMAC Channel Suspend - 0: no DMAC channel suspend request, 1: DMAC channel suspend request"]
    #[inline(always)]
    #[must_use]
    pub fn susp_ch4(&mut self) -> SuspChW<ChenSpec> {
        SuspChW::new(self, 19)
    }
    #[doc = "Bit 20 - DMAC Channel Suspend - 0: no DMAC channel suspend request, 1: DMAC channel suspend request"]
    #[inline(always)]
    #[must_use]
    pub fn susp_ch5(&mut self) -> SuspChW<ChenSpec> {
        SuspChW::new(self, 20)
    }
    #[doc = "Bit 21 - DMAC Channel Suspend - 0: no DMAC channel suspend request, 1: DMAC channel suspend request"]
    #[inline(always)]
    #[must_use]
    pub fn susp_ch6(&mut self) -> SuspChW<ChenSpec> {
        SuspChW::new(self, 21)
    }
    #[doc = "Bit 22 - DMAC Channel Suspend - 0: no DMAC channel suspend request, 1: DMAC channel suspend request"]
    #[inline(always)]
    #[must_use]
    pub fn susp_ch7(&mut self) -> SuspChW<ChenSpec> {
        SuspChW::new(self, 22)
    }
    #[doc = "Bit 23 - DMAC Channel Suspend - 0: no DMAC channel suspend request, 1: DMAC channel suspend request"]
    #[inline(always)]
    #[must_use]
    pub fn susp_ch8(&mut self) -> SuspChW<ChenSpec> {
        SuspChW::new(self, 23)
    }
    #[doc = "DMAC Channel Suspend Write-enable - 0: disable write to DMAC channel suspend bit, 1: enable write to DMAC channel suspend bit"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `susp_we_ch1` field"]
    #[inline(always)]
    #[must_use]
    pub fn susp_we_ch(&mut self, n: u8) -> SuspWeChW<ChenSpec> {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        SuspWeChW::new(self, n + 24)
    }
    #[doc = "Bit 24 - DMAC Channel Suspend Write-enable - 0: disable write to DMAC channel suspend bit, 1: enable write to DMAC channel suspend bit"]
    #[inline(always)]
    #[must_use]
    pub fn susp_we_ch1(&mut self) -> SuspWeChW<ChenSpec> {
        SuspWeChW::new(self, 24)
    }
    #[doc = "Bit 25 - DMAC Channel Suspend Write-enable - 0: disable write to DMAC channel suspend bit, 1: enable write to DMAC channel suspend bit"]
    #[inline(always)]
    #[must_use]
    pub fn susp_we_ch2(&mut self) -> SuspWeChW<ChenSpec> {
        SuspWeChW::new(self, 25)
    }
    #[doc = "Bit 26 - DMAC Channel Suspend Write-enable - 0: disable write to DMAC channel suspend bit, 1: enable write to DMAC channel suspend bit"]
    #[inline(always)]
    #[must_use]
    pub fn susp_we_ch3(&mut self) -> SuspWeChW<ChenSpec> {
        SuspWeChW::new(self, 26)
    }
    #[doc = "Bit 27 - DMAC Channel Suspend Write-enable - 0: disable write to DMAC channel suspend bit, 1: enable write to DMAC channel suspend bit"]
    #[inline(always)]
    #[must_use]
    pub fn susp_we_ch4(&mut self) -> SuspWeChW<ChenSpec> {
        SuspWeChW::new(self, 27)
    }
    #[doc = "Bit 28 - DMAC Channel Suspend Write-enable - 0: disable write to DMAC channel suspend bit, 1: enable write to DMAC channel suspend bit"]
    #[inline(always)]
    #[must_use]
    pub fn susp_we_ch5(&mut self) -> SuspWeChW<ChenSpec> {
        SuspWeChW::new(self, 28)
    }
    #[doc = "Bit 29 - DMAC Channel Suspend Write-enable - 0: disable write to DMAC channel suspend bit, 1: enable write to DMAC channel suspend bit"]
    #[inline(always)]
    #[must_use]
    pub fn susp_we_ch6(&mut self) -> SuspWeChW<ChenSpec> {
        SuspWeChW::new(self, 29)
    }
    #[doc = "Bit 30 - DMAC Channel Suspend Write-enable - 0: disable write to DMAC channel suspend bit, 1: enable write to DMAC channel suspend bit"]
    #[inline(always)]
    #[must_use]
    pub fn susp_we_ch7(&mut self) -> SuspWeChW<ChenSpec> {
        SuspWeChW::new(self, 30)
    }
    #[doc = "Bit 31 - DMAC Channel Suspend Write-enable - 0: disable write to DMAC channel suspend bit, 1: enable write to DMAC channel suspend bit"]
    #[inline(always)]
    #[must_use]
    pub fn susp_we_ch8(&mut self) -> SuspWeChW<ChenSpec> {
        SuspWeChW::new(self, 31)
    }
    #[doc = "DMAC Channel Abort - 0: no DMAC channel abort request, 1: request DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: read-write"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `abort_ch1` field"]
    #[inline(always)]
    #[must_use]
    pub fn abort_ch(&mut self, n: u8) -> AbortChW<ChenSpec> {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        AbortChW::new(self, n + 32)
    }
    #[doc = "Bit 32 - DMAC Channel Abort - 0: no DMAC channel abort request, 1: request DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: read-write"]
    #[inline(always)]
    #[must_use]
    pub fn abort_ch1(&mut self) -> AbortChW<ChenSpec> {
        AbortChW::new(self, 32)
    }
    #[doc = "Bit 33 - DMAC Channel Abort - 0: no DMAC channel abort request, 1: request DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: read-write"]
    #[inline(always)]
    #[must_use]
    pub fn abort_ch2(&mut self) -> AbortChW<ChenSpec> {
        AbortChW::new(self, 33)
    }
    #[doc = "Bit 34 - DMAC Channel Abort - 0: no DMAC channel abort request, 1: request DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: read-write"]
    #[inline(always)]
    #[must_use]
    pub fn abort_ch3(&mut self) -> AbortChW<ChenSpec> {
        AbortChW::new(self, 34)
    }
    #[doc = "Bit 35 - DMAC Channel Abort - 0: no DMAC channel abort request, 1: request DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: read-write"]
    #[inline(always)]
    #[must_use]
    pub fn abort_ch4(&mut self) -> AbortChW<ChenSpec> {
        AbortChW::new(self, 35)
    }
    #[doc = "Bit 36 - DMAC Channel Abort - 0: no DMAC channel abort request, 1: request DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: read-write"]
    #[inline(always)]
    #[must_use]
    pub fn abort_ch5(&mut self) -> AbortChW<ChenSpec> {
        AbortChW::new(self, 36)
    }
    #[doc = "Bit 37 - DMAC Channel Abort - 0: no DMAC channel abort request, 1: request DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: read-write"]
    #[inline(always)]
    #[must_use]
    pub fn abort_ch6(&mut self) -> AbortChW<ChenSpec> {
        AbortChW::new(self, 37)
    }
    #[doc = "Bit 38 - DMAC Channel Abort - 0: no DMAC channel abort request, 1: request DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: read-write"]
    #[inline(always)]
    #[must_use]
    pub fn abort_ch7(&mut self) -> AbortChW<ChenSpec> {
        AbortChW::new(self, 38)
    }
    #[doc = "Bit 39 - DMAC Channel Abort - 0: no DMAC channel abort request, 1: request DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: read-write"]
    #[inline(always)]
    #[must_use]
    pub fn abort_ch8(&mut self) -> AbortChW<ChenSpec> {
        AbortChW::new(self, 39)
    }
    #[doc = "DMAC Channel Abort Write-enable - 0: disable write to DMAC channel abort bit, 1: enable write DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: write-only"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `abort_we_ch1` field"]
    #[inline(always)]
    #[must_use]
    pub fn abort_we_ch(&mut self, n: u8) -> AbortWeChW<ChenSpec> {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        AbortWeChW::new(self, n + 40)
    }
    #[doc = "Bit 40 - DMAC Channel Abort Write-enable - 0: disable write to DMAC channel abort bit, 1: enable write DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: write-only"]
    #[inline(always)]
    #[must_use]
    pub fn abort_we_ch1(&mut self) -> AbortWeChW<ChenSpec> {
        AbortWeChW::new(self, 40)
    }
    #[doc = "Bit 41 - DMAC Channel Abort Write-enable - 0: disable write to DMAC channel abort bit, 1: enable write DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: write-only"]
    #[inline(always)]
    #[must_use]
    pub fn abort_we_ch2(&mut self) -> AbortWeChW<ChenSpec> {
        AbortWeChW::new(self, 41)
    }
    #[doc = "Bit 42 - DMAC Channel Abort Write-enable - 0: disable write to DMAC channel abort bit, 1: enable write DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: write-only"]
    #[inline(always)]
    #[must_use]
    pub fn abort_we_ch3(&mut self) -> AbortWeChW<ChenSpec> {
        AbortWeChW::new(self, 42)
    }
    #[doc = "Bit 43 - DMAC Channel Abort Write-enable - 0: disable write to DMAC channel abort bit, 1: enable write DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: write-only"]
    #[inline(always)]
    #[must_use]
    pub fn abort_we_ch4(&mut self) -> AbortWeChW<ChenSpec> {
        AbortWeChW::new(self, 43)
    }
    #[doc = "Bit 44 - DMAC Channel Abort Write-enable - 0: disable write to DMAC channel abort bit, 1: enable write DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: write-only"]
    #[inline(always)]
    #[must_use]
    pub fn abort_we_ch5(&mut self) -> AbortWeChW<ChenSpec> {
        AbortWeChW::new(self, 44)
    }
    #[doc = "Bit 45 - DMAC Channel Abort Write-enable - 0: disable write to DMAC channel abort bit, 1: enable write DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: write-only"]
    #[inline(always)]
    #[must_use]
    pub fn abort_we_ch6(&mut self) -> AbortWeChW<ChenSpec> {
        AbortWeChW::new(self, 45)
    }
    #[doc = "Bit 46 - DMAC Channel Abort Write-enable - 0: disable write to DMAC channel abort bit, 1: enable write DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: write-only"]
    #[inline(always)]
    #[must_use]
    pub fn abort_we_ch7(&mut self) -> AbortWeChW<ChenSpec> {
        AbortWeChW::new(self, 46)
    }
    #[doc = "Bit 47 - DMAC Channel Abort Write-enable - 0: disable write to DMAC channel abort bit, 1: enable write DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: write-only"]
    #[inline(always)]
    #[must_use]
    pub fn abort_we_ch8(&mut self) -> AbortWeChW<ChenSpec> {
        AbortWeChW::new(self, 47)
    }
}
#[doc = "DMAC Channel Enable register contains the DMAC channel enable settings. Only exists when DMAX_NUM_CHANNELS &lt;= 8.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chen::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chen::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChenSpec;
impl crate::RegisterSpec for ChenSpec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`chen::R`](R) reader structure"]
impl crate::Readable for ChenSpec {}
#[doc = "`write(|w| ..)` method takes [`chen::W`](W) writer structure"]
impl crate::Writable for ChenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
}
#[doc = "`reset()` method sets chen to value 0"]
impl crate::Resettable for ChenSpec {
    const RESET_VALUE: u64 = 0;
}
