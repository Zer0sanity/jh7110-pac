#[doc = "Register `chabort` reader"]
pub type R = crate::R<ChabortSpec>;
#[doc = "Register `chabort` writer"]
pub type W = crate::W<ChabortSpec>;
#[doc = "Field `abort0_ch(1-16)` reader - DMAC Channel Abort - 0: no DMAC channel abort request, 1: request DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: read-write"]
pub type Abort0ChR = crate::BitReader;
#[doc = "Field `abort0_ch(1-16)` writer - DMAC Channel Abort - 0: no DMAC channel abort request, 1: request DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: read-write"]
pub type Abort0ChW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `abort_we0_ch(1-16)` reader - DMAC Channel Abort Write-enable - 0: disable write to DMAC channel abort bit, 1: enable write DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: write-only"]
pub type AbortWe0ChR = crate::BitReader;
#[doc = "Field `abort_we0_ch(1-16)` writer - DMAC Channel Abort Write-enable - 0: disable write to DMAC channel abort bit, 1: enable write DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: write-only"]
pub type AbortWe0ChW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `abort1_ch(17-32)` reader - DMAC Channel Abort - 0: no DMAC channel abort request, 1: request DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: read-write"]
pub type Abort1ChR = crate::BitReader;
#[doc = "Field `abort1_ch(17-32)` writer - DMAC Channel Abort - 0: no DMAC channel abort request, 1: request DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: read-write"]
pub type Abort1ChW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `abort_we1_ch(17-32)` reader - DMAC Channel Abort Write-enable - 0: disable write to DMAC channel abort bit, 1: enable write DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: write-only"]
pub type AbortWe1ChR = crate::BitReader;
#[doc = "Field `abort_we1_ch(17-32)` writer - DMAC Channel Abort Write-enable - 0: disable write to DMAC channel abort bit, 1: enable write DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: write-only"]
pub type AbortWe1ChW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "DMAC Channel Abort - 0: no DMAC channel abort request, 1: request DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: read-write"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `abort0_ch1` field"]
    #[inline(always)]
    pub fn abort0_ch(&self, n: u8) -> Abort0ChR {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        Abort0ChR::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "DMAC Channel Abort - 0: no DMAC channel abort request, 1: request DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: read-write"]
    #[inline(always)]
    pub fn abort0_ch_iter(&self) -> impl Iterator<Item = Abort0ChR> + '_ {
        (0..16).map(move |n| Abort0ChR::new(((self.bits >> n) & 1) != 0))
    }
    #[doc = "Bit 0 - DMAC Channel Abort - 0: no DMAC channel abort request, 1: request DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: read-write"]
    #[inline(always)]
    pub fn abort0_ch1(&self) -> Abort0ChR {
        Abort0ChR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMAC Channel Abort - 0: no DMAC channel abort request, 1: request DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: read-write"]
    #[inline(always)]
    pub fn abort0_ch2(&self) -> Abort0ChR {
        Abort0ChR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DMAC Channel Abort - 0: no DMAC channel abort request, 1: request DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: read-write"]
    #[inline(always)]
    pub fn abort0_ch3(&self) -> Abort0ChR {
        Abort0ChR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMAC Channel Abort - 0: no DMAC channel abort request, 1: request DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: read-write"]
    #[inline(always)]
    pub fn abort0_ch4(&self) -> Abort0ChR {
        Abort0ChR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DMAC Channel Abort - 0: no DMAC channel abort request, 1: request DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: read-write"]
    #[inline(always)]
    pub fn abort0_ch5(&self) -> Abort0ChR {
        Abort0ChR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DMAC Channel Abort - 0: no DMAC channel abort request, 1: request DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: read-write"]
    #[inline(always)]
    pub fn abort0_ch6(&self) -> Abort0ChR {
        Abort0ChR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - DMAC Channel Abort - 0: no DMAC channel abort request, 1: request DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: read-write"]
    #[inline(always)]
    pub fn abort0_ch7(&self) -> Abort0ChR {
        Abort0ChR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DMAC Channel Abort - 0: no DMAC channel abort request, 1: request DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: read-write"]
    #[inline(always)]
    pub fn abort0_ch8(&self) -> Abort0ChR {
        Abort0ChR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - DMAC Channel Abort - 0: no DMAC channel abort request, 1: request DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: read-write"]
    #[inline(always)]
    pub fn abort0_ch9(&self) -> Abort0ChR {
        Abort0ChR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - DMAC Channel Abort - 0: no DMAC channel abort request, 1: request DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: read-write"]
    #[inline(always)]
    pub fn abort0_ch10(&self) -> Abort0ChR {
        Abort0ChR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - DMAC Channel Abort - 0: no DMAC channel abort request, 1: request DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: read-write"]
    #[inline(always)]
    pub fn abort0_ch11(&self) -> Abort0ChR {
        Abort0ChR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - DMAC Channel Abort - 0: no DMAC channel abort request, 1: request DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: read-write"]
    #[inline(always)]
    pub fn abort0_ch12(&self) -> Abort0ChR {
        Abort0ChR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - DMAC Channel Abort - 0: no DMAC channel abort request, 1: request DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: read-write"]
    #[inline(always)]
    pub fn abort0_ch13(&self) -> Abort0ChR {
        Abort0ChR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - DMAC Channel Abort - 0: no DMAC channel abort request, 1: request DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: read-write"]
    #[inline(always)]
    pub fn abort0_ch14(&self) -> Abort0ChR {
        Abort0ChR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - DMAC Channel Abort - 0: no DMAC channel abort request, 1: request DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: read-write"]
    #[inline(always)]
    pub fn abort0_ch15(&self) -> Abort0ChR {
        Abort0ChR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - DMAC Channel Abort - 0: no DMAC channel abort request, 1: request DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: read-write"]
    #[inline(always)]
    pub fn abort0_ch16(&self) -> Abort0ChR {
        Abort0ChR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "DMAC Channel Abort Write-enable - 0: disable write to DMAC channel abort bit, 1: enable write DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: write-only"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `abort_we0_ch1` field"]
    #[inline(always)]
    pub fn abort_we0_ch(&self, n: u8) -> AbortWe0ChR {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        AbortWe0ChR::new(((self.bits >> (n + 16)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "DMAC Channel Abort Write-enable - 0: disable write to DMAC channel abort bit, 1: enable write DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: write-only"]
    #[inline(always)]
    pub fn abort_we0_ch_iter(&self) -> impl Iterator<Item = AbortWe0ChR> + '_ {
        (0..16).map(move |n| AbortWe0ChR::new(((self.bits >> (n + 16)) & 1) != 0))
    }
    #[doc = "Bit 16 - DMAC Channel Abort Write-enable - 0: disable write to DMAC channel abort bit, 1: enable write DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: write-only"]
    #[inline(always)]
    pub fn abort_we0_ch1(&self) -> AbortWe0ChR {
        AbortWe0ChR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - DMAC Channel Abort Write-enable - 0: disable write to DMAC channel abort bit, 1: enable write DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: write-only"]
    #[inline(always)]
    pub fn abort_we0_ch2(&self) -> AbortWe0ChR {
        AbortWe0ChR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - DMAC Channel Abort Write-enable - 0: disable write to DMAC channel abort bit, 1: enable write DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: write-only"]
    #[inline(always)]
    pub fn abort_we0_ch3(&self) -> AbortWe0ChR {
        AbortWe0ChR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - DMAC Channel Abort Write-enable - 0: disable write to DMAC channel abort bit, 1: enable write DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: write-only"]
    #[inline(always)]
    pub fn abort_we0_ch4(&self) -> AbortWe0ChR {
        AbortWe0ChR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - DMAC Channel Abort Write-enable - 0: disable write to DMAC channel abort bit, 1: enable write DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: write-only"]
    #[inline(always)]
    pub fn abort_we0_ch5(&self) -> AbortWe0ChR {
        AbortWe0ChR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - DMAC Channel Abort Write-enable - 0: disable write to DMAC channel abort bit, 1: enable write DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: write-only"]
    #[inline(always)]
    pub fn abort_we0_ch6(&self) -> AbortWe0ChR {
        AbortWe0ChR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - DMAC Channel Abort Write-enable - 0: disable write to DMAC channel abort bit, 1: enable write DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: write-only"]
    #[inline(always)]
    pub fn abort_we0_ch7(&self) -> AbortWe0ChR {
        AbortWe0ChR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - DMAC Channel Abort Write-enable - 0: disable write to DMAC channel abort bit, 1: enable write DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: write-only"]
    #[inline(always)]
    pub fn abort_we0_ch8(&self) -> AbortWe0ChR {
        AbortWe0ChR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - DMAC Channel Abort Write-enable - 0: disable write to DMAC channel abort bit, 1: enable write DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: write-only"]
    #[inline(always)]
    pub fn abort_we0_ch9(&self) -> AbortWe0ChR {
        AbortWe0ChR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - DMAC Channel Abort Write-enable - 0: disable write to DMAC channel abort bit, 1: enable write DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: write-only"]
    #[inline(always)]
    pub fn abort_we0_ch10(&self) -> AbortWe0ChR {
        AbortWe0ChR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - DMAC Channel Abort Write-enable - 0: disable write to DMAC channel abort bit, 1: enable write DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: write-only"]
    #[inline(always)]
    pub fn abort_we0_ch11(&self) -> AbortWe0ChR {
        AbortWe0ChR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - DMAC Channel Abort Write-enable - 0: disable write to DMAC channel abort bit, 1: enable write DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: write-only"]
    #[inline(always)]
    pub fn abort_we0_ch12(&self) -> AbortWe0ChR {
        AbortWe0ChR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - DMAC Channel Abort Write-enable - 0: disable write to DMAC channel abort bit, 1: enable write DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: write-only"]
    #[inline(always)]
    pub fn abort_we0_ch13(&self) -> AbortWe0ChR {
        AbortWe0ChR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - DMAC Channel Abort Write-enable - 0: disable write to DMAC channel abort bit, 1: enable write DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: write-only"]
    #[inline(always)]
    pub fn abort_we0_ch14(&self) -> AbortWe0ChR {
        AbortWe0ChR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - DMAC Channel Abort Write-enable - 0: disable write to DMAC channel abort bit, 1: enable write DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: write-only"]
    #[inline(always)]
    pub fn abort_we0_ch15(&self) -> AbortWe0ChR {
        AbortWe0ChR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - DMAC Channel Abort Write-enable - 0: disable write to DMAC channel abort bit, 1: enable write DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: write-only"]
    #[inline(always)]
    pub fn abort_we0_ch16(&self) -> AbortWe0ChR {
        AbortWe0ChR::new(((self.bits >> 31) & 1) != 0)
    }
    #[doc = "DMAC Channel Abort - 0: no DMAC channel abort request, 1: request DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: read-write"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `abort1_ch17` field"]
    #[inline(always)]
    pub fn abort1_ch(&self, n: u8) -> Abort1ChR {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        Abort1ChR::new(((self.bits >> (n + 32)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "DMAC Channel Abort - 0: no DMAC channel abort request, 1: request DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: read-write"]
    #[inline(always)]
    pub fn abort1_ch_iter(&self) -> impl Iterator<Item = Abort1ChR> + '_ {
        (0..16).map(move |n| Abort1ChR::new(((self.bits >> (n + 32)) & 1) != 0))
    }
    #[doc = "Bit 32 - DMAC Channel Abort - 0: no DMAC channel abort request, 1: request DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: read-write"]
    #[inline(always)]
    pub fn abort1_ch17(&self) -> Abort1ChR {
        Abort1ChR::new(((self.bits >> 32) & 1) != 0)
    }
    #[doc = "Bit 33 - DMAC Channel Abort - 0: no DMAC channel abort request, 1: request DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: read-write"]
    #[inline(always)]
    pub fn abort1_ch18(&self) -> Abort1ChR {
        Abort1ChR::new(((self.bits >> 33) & 1) != 0)
    }
    #[doc = "Bit 34 - DMAC Channel Abort - 0: no DMAC channel abort request, 1: request DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: read-write"]
    #[inline(always)]
    pub fn abort1_ch19(&self) -> Abort1ChR {
        Abort1ChR::new(((self.bits >> 34) & 1) != 0)
    }
    #[doc = "Bit 35 - DMAC Channel Abort - 0: no DMAC channel abort request, 1: request DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: read-write"]
    #[inline(always)]
    pub fn abort1_ch20(&self) -> Abort1ChR {
        Abort1ChR::new(((self.bits >> 35) & 1) != 0)
    }
    #[doc = "Bit 36 - DMAC Channel Abort - 0: no DMAC channel abort request, 1: request DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: read-write"]
    #[inline(always)]
    pub fn abort1_ch21(&self) -> Abort1ChR {
        Abort1ChR::new(((self.bits >> 36) & 1) != 0)
    }
    #[doc = "Bit 37 - DMAC Channel Abort - 0: no DMAC channel abort request, 1: request DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: read-write"]
    #[inline(always)]
    pub fn abort1_ch22(&self) -> Abort1ChR {
        Abort1ChR::new(((self.bits >> 37) & 1) != 0)
    }
    #[doc = "Bit 38 - DMAC Channel Abort - 0: no DMAC channel abort request, 1: request DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: read-write"]
    #[inline(always)]
    pub fn abort1_ch23(&self) -> Abort1ChR {
        Abort1ChR::new(((self.bits >> 38) & 1) != 0)
    }
    #[doc = "Bit 39 - DMAC Channel Abort - 0: no DMAC channel abort request, 1: request DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: read-write"]
    #[inline(always)]
    pub fn abort1_ch24(&self) -> Abort1ChR {
        Abort1ChR::new(((self.bits >> 39) & 1) != 0)
    }
    #[doc = "Bit 40 - DMAC Channel Abort - 0: no DMAC channel abort request, 1: request DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: read-write"]
    #[inline(always)]
    pub fn abort1_ch25(&self) -> Abort1ChR {
        Abort1ChR::new(((self.bits >> 40) & 1) != 0)
    }
    #[doc = "Bit 41 - DMAC Channel Abort - 0: no DMAC channel abort request, 1: request DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: read-write"]
    #[inline(always)]
    pub fn abort1_ch26(&self) -> Abort1ChR {
        Abort1ChR::new(((self.bits >> 41) & 1) != 0)
    }
    #[doc = "Bit 42 - DMAC Channel Abort - 0: no DMAC channel abort request, 1: request DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: read-write"]
    #[inline(always)]
    pub fn abort1_ch27(&self) -> Abort1ChR {
        Abort1ChR::new(((self.bits >> 42) & 1) != 0)
    }
    #[doc = "Bit 43 - DMAC Channel Abort - 0: no DMAC channel abort request, 1: request DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: read-write"]
    #[inline(always)]
    pub fn abort1_ch28(&self) -> Abort1ChR {
        Abort1ChR::new(((self.bits >> 43) & 1) != 0)
    }
    #[doc = "Bit 44 - DMAC Channel Abort - 0: no DMAC channel abort request, 1: request DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: read-write"]
    #[inline(always)]
    pub fn abort1_ch29(&self) -> Abort1ChR {
        Abort1ChR::new(((self.bits >> 44) & 1) != 0)
    }
    #[doc = "Bit 45 - DMAC Channel Abort - 0: no DMAC channel abort request, 1: request DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: read-write"]
    #[inline(always)]
    pub fn abort1_ch30(&self) -> Abort1ChR {
        Abort1ChR::new(((self.bits >> 45) & 1) != 0)
    }
    #[doc = "Bit 46 - DMAC Channel Abort - 0: no DMAC channel abort request, 1: request DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: read-write"]
    #[inline(always)]
    pub fn abort1_ch31(&self) -> Abort1ChR {
        Abort1ChR::new(((self.bits >> 46) & 1) != 0)
    }
    #[doc = "Bit 47 - DMAC Channel Abort - 0: no DMAC channel abort request, 1: request DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: read-write"]
    #[inline(always)]
    pub fn abort1_ch32(&self) -> Abort1ChR {
        Abort1ChR::new(((self.bits >> 47) & 1) != 0)
    }
    #[doc = "DMAC Channel Abort Write-enable - 0: disable write to DMAC channel abort bit, 1: enable write DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: write-only"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `abort_we1_ch17` field"]
    #[inline(always)]
    pub fn abort_we1_ch(&self, n: u8) -> AbortWe1ChR {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        AbortWe1ChR::new(((self.bits >> (n + 48)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "DMAC Channel Abort Write-enable - 0: disable write to DMAC channel abort bit, 1: enable write DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: write-only"]
    #[inline(always)]
    pub fn abort_we1_ch_iter(&self) -> impl Iterator<Item = AbortWe1ChR> + '_ {
        (0..16).map(move |n| AbortWe1ChR::new(((self.bits >> (n + 48)) & 1) != 0))
    }
    #[doc = "Bit 48 - DMAC Channel Abort Write-enable - 0: disable write to DMAC channel abort bit, 1: enable write DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: write-only"]
    #[inline(always)]
    pub fn abort_we1_ch17(&self) -> AbortWe1ChR {
        AbortWe1ChR::new(((self.bits >> 48) & 1) != 0)
    }
    #[doc = "Bit 49 - DMAC Channel Abort Write-enable - 0: disable write to DMAC channel abort bit, 1: enable write DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: write-only"]
    #[inline(always)]
    pub fn abort_we1_ch18(&self) -> AbortWe1ChR {
        AbortWe1ChR::new(((self.bits >> 49) & 1) != 0)
    }
    #[doc = "Bit 50 - DMAC Channel Abort Write-enable - 0: disable write to DMAC channel abort bit, 1: enable write DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: write-only"]
    #[inline(always)]
    pub fn abort_we1_ch19(&self) -> AbortWe1ChR {
        AbortWe1ChR::new(((self.bits >> 50) & 1) != 0)
    }
    #[doc = "Bit 51 - DMAC Channel Abort Write-enable - 0: disable write to DMAC channel abort bit, 1: enable write DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: write-only"]
    #[inline(always)]
    pub fn abort_we1_ch20(&self) -> AbortWe1ChR {
        AbortWe1ChR::new(((self.bits >> 51) & 1) != 0)
    }
    #[doc = "Bit 52 - DMAC Channel Abort Write-enable - 0: disable write to DMAC channel abort bit, 1: enable write DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: write-only"]
    #[inline(always)]
    pub fn abort_we1_ch21(&self) -> AbortWe1ChR {
        AbortWe1ChR::new(((self.bits >> 52) & 1) != 0)
    }
    #[doc = "Bit 53 - DMAC Channel Abort Write-enable - 0: disable write to DMAC channel abort bit, 1: enable write DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: write-only"]
    #[inline(always)]
    pub fn abort_we1_ch22(&self) -> AbortWe1ChR {
        AbortWe1ChR::new(((self.bits >> 53) & 1) != 0)
    }
    #[doc = "Bit 54 - DMAC Channel Abort Write-enable - 0: disable write to DMAC channel abort bit, 1: enable write DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: write-only"]
    #[inline(always)]
    pub fn abort_we1_ch23(&self) -> AbortWe1ChR {
        AbortWe1ChR::new(((self.bits >> 54) & 1) != 0)
    }
    #[doc = "Bit 55 - DMAC Channel Abort Write-enable - 0: disable write to DMAC channel abort bit, 1: enable write DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: write-only"]
    #[inline(always)]
    pub fn abort_we1_ch24(&self) -> AbortWe1ChR {
        AbortWe1ChR::new(((self.bits >> 55) & 1) != 0)
    }
    #[doc = "Bit 56 - DMAC Channel Abort Write-enable - 0: disable write to DMAC channel abort bit, 1: enable write DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: write-only"]
    #[inline(always)]
    pub fn abort_we1_ch25(&self) -> AbortWe1ChR {
        AbortWe1ChR::new(((self.bits >> 56) & 1) != 0)
    }
    #[doc = "Bit 57 - DMAC Channel Abort Write-enable - 0: disable write to DMAC channel abort bit, 1: enable write DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: write-only"]
    #[inline(always)]
    pub fn abort_we1_ch26(&self) -> AbortWe1ChR {
        AbortWe1ChR::new(((self.bits >> 57) & 1) != 0)
    }
    #[doc = "Bit 58 - DMAC Channel Abort Write-enable - 0: disable write to DMAC channel abort bit, 1: enable write DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: write-only"]
    #[inline(always)]
    pub fn abort_we1_ch27(&self) -> AbortWe1ChR {
        AbortWe1ChR::new(((self.bits >> 58) & 1) != 0)
    }
    #[doc = "Bit 59 - DMAC Channel Abort Write-enable - 0: disable write to DMAC channel abort bit, 1: enable write DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: write-only"]
    #[inline(always)]
    pub fn abort_we1_ch28(&self) -> AbortWe1ChR {
        AbortWe1ChR::new(((self.bits >> 59) & 1) != 0)
    }
    #[doc = "Bit 60 - DMAC Channel Abort Write-enable - 0: disable write to DMAC channel abort bit, 1: enable write DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: write-only"]
    #[inline(always)]
    pub fn abort_we1_ch29(&self) -> AbortWe1ChR {
        AbortWe1ChR::new(((self.bits >> 60) & 1) != 0)
    }
    #[doc = "Bit 61 - DMAC Channel Abort Write-enable - 0: disable write to DMAC channel abort bit, 1: enable write DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: write-only"]
    #[inline(always)]
    pub fn abort_we1_ch30(&self) -> AbortWe1ChR {
        AbortWe1ChR::new(((self.bits >> 61) & 1) != 0)
    }
    #[doc = "Bit 62 - DMAC Channel Abort Write-enable - 0: disable write to DMAC channel abort bit, 1: enable write DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: write-only"]
    #[inline(always)]
    pub fn abort_we1_ch31(&self) -> AbortWe1ChR {
        AbortWe1ChR::new(((self.bits >> 62) & 1) != 0)
    }
    #[doc = "Bit 63 - DMAC Channel Abort Write-enable - 0: disable write to DMAC channel abort bit, 1: enable write DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: write-only"]
    #[inline(always)]
    pub fn abort_we1_ch32(&self) -> AbortWe1ChR {
        AbortWe1ChR::new(((self.bits >> 63) & 1) != 0)
    }
}
impl W {
    #[doc = "DMAC Channel Abort - 0: no DMAC channel abort request, 1: request DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: read-write"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `abort0_ch1` field"]
    #[inline(always)]
    #[must_use]
    pub fn abort0_ch(&mut self, n: u8) -> Abort0ChW<ChabortSpec> {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        Abort0ChW::new(self, n)
    }
    #[doc = "Bit 0 - DMAC Channel Abort - 0: no DMAC channel abort request, 1: request DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: read-write"]
    #[inline(always)]
    #[must_use]
    pub fn abort0_ch1(&mut self) -> Abort0ChW<ChabortSpec> {
        Abort0ChW::new(self, 0)
    }
    #[doc = "Bit 1 - DMAC Channel Abort - 0: no DMAC channel abort request, 1: request DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: read-write"]
    #[inline(always)]
    #[must_use]
    pub fn abort0_ch2(&mut self) -> Abort0ChW<ChabortSpec> {
        Abort0ChW::new(self, 1)
    }
    #[doc = "Bit 2 - DMAC Channel Abort - 0: no DMAC channel abort request, 1: request DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: read-write"]
    #[inline(always)]
    #[must_use]
    pub fn abort0_ch3(&mut self) -> Abort0ChW<ChabortSpec> {
        Abort0ChW::new(self, 2)
    }
    #[doc = "Bit 3 - DMAC Channel Abort - 0: no DMAC channel abort request, 1: request DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: read-write"]
    #[inline(always)]
    #[must_use]
    pub fn abort0_ch4(&mut self) -> Abort0ChW<ChabortSpec> {
        Abort0ChW::new(self, 3)
    }
    #[doc = "Bit 4 - DMAC Channel Abort - 0: no DMAC channel abort request, 1: request DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: read-write"]
    #[inline(always)]
    #[must_use]
    pub fn abort0_ch5(&mut self) -> Abort0ChW<ChabortSpec> {
        Abort0ChW::new(self, 4)
    }
    #[doc = "Bit 5 - DMAC Channel Abort - 0: no DMAC channel abort request, 1: request DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: read-write"]
    #[inline(always)]
    #[must_use]
    pub fn abort0_ch6(&mut self) -> Abort0ChW<ChabortSpec> {
        Abort0ChW::new(self, 5)
    }
    #[doc = "Bit 6 - DMAC Channel Abort - 0: no DMAC channel abort request, 1: request DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: read-write"]
    #[inline(always)]
    #[must_use]
    pub fn abort0_ch7(&mut self) -> Abort0ChW<ChabortSpec> {
        Abort0ChW::new(self, 6)
    }
    #[doc = "Bit 7 - DMAC Channel Abort - 0: no DMAC channel abort request, 1: request DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: read-write"]
    #[inline(always)]
    #[must_use]
    pub fn abort0_ch8(&mut self) -> Abort0ChW<ChabortSpec> {
        Abort0ChW::new(self, 7)
    }
    #[doc = "Bit 8 - DMAC Channel Abort - 0: no DMAC channel abort request, 1: request DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: read-write"]
    #[inline(always)]
    #[must_use]
    pub fn abort0_ch9(&mut self) -> Abort0ChW<ChabortSpec> {
        Abort0ChW::new(self, 8)
    }
    #[doc = "Bit 9 - DMAC Channel Abort - 0: no DMAC channel abort request, 1: request DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: read-write"]
    #[inline(always)]
    #[must_use]
    pub fn abort0_ch10(&mut self) -> Abort0ChW<ChabortSpec> {
        Abort0ChW::new(self, 9)
    }
    #[doc = "Bit 10 - DMAC Channel Abort - 0: no DMAC channel abort request, 1: request DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: read-write"]
    #[inline(always)]
    #[must_use]
    pub fn abort0_ch11(&mut self) -> Abort0ChW<ChabortSpec> {
        Abort0ChW::new(self, 10)
    }
    #[doc = "Bit 11 - DMAC Channel Abort - 0: no DMAC channel abort request, 1: request DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: read-write"]
    #[inline(always)]
    #[must_use]
    pub fn abort0_ch12(&mut self) -> Abort0ChW<ChabortSpec> {
        Abort0ChW::new(self, 11)
    }
    #[doc = "Bit 12 - DMAC Channel Abort - 0: no DMAC channel abort request, 1: request DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: read-write"]
    #[inline(always)]
    #[must_use]
    pub fn abort0_ch13(&mut self) -> Abort0ChW<ChabortSpec> {
        Abort0ChW::new(self, 12)
    }
    #[doc = "Bit 13 - DMAC Channel Abort - 0: no DMAC channel abort request, 1: request DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: read-write"]
    #[inline(always)]
    #[must_use]
    pub fn abort0_ch14(&mut self) -> Abort0ChW<ChabortSpec> {
        Abort0ChW::new(self, 13)
    }
    #[doc = "Bit 14 - DMAC Channel Abort - 0: no DMAC channel abort request, 1: request DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: read-write"]
    #[inline(always)]
    #[must_use]
    pub fn abort0_ch15(&mut self) -> Abort0ChW<ChabortSpec> {
        Abort0ChW::new(self, 14)
    }
    #[doc = "Bit 15 - DMAC Channel Abort - 0: no DMAC channel abort request, 1: request DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: read-write"]
    #[inline(always)]
    #[must_use]
    pub fn abort0_ch16(&mut self) -> Abort0ChW<ChabortSpec> {
        Abort0ChW::new(self, 15)
    }
    #[doc = "DMAC Channel Abort Write-enable - 0: disable write to DMAC channel abort bit, 1: enable write DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: write-only"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `abort_we0_ch1` field"]
    #[inline(always)]
    #[must_use]
    pub fn abort_we0_ch(&mut self, n: u8) -> AbortWe0ChW<ChabortSpec> {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        AbortWe0ChW::new(self, n + 16)
    }
    #[doc = "Bit 16 - DMAC Channel Abort Write-enable - 0: disable write to DMAC channel abort bit, 1: enable write DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: write-only"]
    #[inline(always)]
    #[must_use]
    pub fn abort_we0_ch1(&mut self) -> AbortWe0ChW<ChabortSpec> {
        AbortWe0ChW::new(self, 16)
    }
    #[doc = "Bit 17 - DMAC Channel Abort Write-enable - 0: disable write to DMAC channel abort bit, 1: enable write DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: write-only"]
    #[inline(always)]
    #[must_use]
    pub fn abort_we0_ch2(&mut self) -> AbortWe0ChW<ChabortSpec> {
        AbortWe0ChW::new(self, 17)
    }
    #[doc = "Bit 18 - DMAC Channel Abort Write-enable - 0: disable write to DMAC channel abort bit, 1: enable write DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: write-only"]
    #[inline(always)]
    #[must_use]
    pub fn abort_we0_ch3(&mut self) -> AbortWe0ChW<ChabortSpec> {
        AbortWe0ChW::new(self, 18)
    }
    #[doc = "Bit 19 - DMAC Channel Abort Write-enable - 0: disable write to DMAC channel abort bit, 1: enable write DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: write-only"]
    #[inline(always)]
    #[must_use]
    pub fn abort_we0_ch4(&mut self) -> AbortWe0ChW<ChabortSpec> {
        AbortWe0ChW::new(self, 19)
    }
    #[doc = "Bit 20 - DMAC Channel Abort Write-enable - 0: disable write to DMAC channel abort bit, 1: enable write DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: write-only"]
    #[inline(always)]
    #[must_use]
    pub fn abort_we0_ch5(&mut self) -> AbortWe0ChW<ChabortSpec> {
        AbortWe0ChW::new(self, 20)
    }
    #[doc = "Bit 21 - DMAC Channel Abort Write-enable - 0: disable write to DMAC channel abort bit, 1: enable write DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: write-only"]
    #[inline(always)]
    #[must_use]
    pub fn abort_we0_ch6(&mut self) -> AbortWe0ChW<ChabortSpec> {
        AbortWe0ChW::new(self, 21)
    }
    #[doc = "Bit 22 - DMAC Channel Abort Write-enable - 0: disable write to DMAC channel abort bit, 1: enable write DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: write-only"]
    #[inline(always)]
    #[must_use]
    pub fn abort_we0_ch7(&mut self) -> AbortWe0ChW<ChabortSpec> {
        AbortWe0ChW::new(self, 22)
    }
    #[doc = "Bit 23 - DMAC Channel Abort Write-enable - 0: disable write to DMAC channel abort bit, 1: enable write DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: write-only"]
    #[inline(always)]
    #[must_use]
    pub fn abort_we0_ch8(&mut self) -> AbortWe0ChW<ChabortSpec> {
        AbortWe0ChW::new(self, 23)
    }
    #[doc = "Bit 24 - DMAC Channel Abort Write-enable - 0: disable write to DMAC channel abort bit, 1: enable write DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: write-only"]
    #[inline(always)]
    #[must_use]
    pub fn abort_we0_ch9(&mut self) -> AbortWe0ChW<ChabortSpec> {
        AbortWe0ChW::new(self, 24)
    }
    #[doc = "Bit 25 - DMAC Channel Abort Write-enable - 0: disable write to DMAC channel abort bit, 1: enable write DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: write-only"]
    #[inline(always)]
    #[must_use]
    pub fn abort_we0_ch10(&mut self) -> AbortWe0ChW<ChabortSpec> {
        AbortWe0ChW::new(self, 25)
    }
    #[doc = "Bit 26 - DMAC Channel Abort Write-enable - 0: disable write to DMAC channel abort bit, 1: enable write DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: write-only"]
    #[inline(always)]
    #[must_use]
    pub fn abort_we0_ch11(&mut self) -> AbortWe0ChW<ChabortSpec> {
        AbortWe0ChW::new(self, 26)
    }
    #[doc = "Bit 27 - DMAC Channel Abort Write-enable - 0: disable write to DMAC channel abort bit, 1: enable write DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: write-only"]
    #[inline(always)]
    #[must_use]
    pub fn abort_we0_ch12(&mut self) -> AbortWe0ChW<ChabortSpec> {
        AbortWe0ChW::new(self, 27)
    }
    #[doc = "Bit 28 - DMAC Channel Abort Write-enable - 0: disable write to DMAC channel abort bit, 1: enable write DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: write-only"]
    #[inline(always)]
    #[must_use]
    pub fn abort_we0_ch13(&mut self) -> AbortWe0ChW<ChabortSpec> {
        AbortWe0ChW::new(self, 28)
    }
    #[doc = "Bit 29 - DMAC Channel Abort Write-enable - 0: disable write to DMAC channel abort bit, 1: enable write DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: write-only"]
    #[inline(always)]
    #[must_use]
    pub fn abort_we0_ch14(&mut self) -> AbortWe0ChW<ChabortSpec> {
        AbortWe0ChW::new(self, 29)
    }
    #[doc = "Bit 30 - DMAC Channel Abort Write-enable - 0: disable write to DMAC channel abort bit, 1: enable write DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: write-only"]
    #[inline(always)]
    #[must_use]
    pub fn abort_we0_ch15(&mut self) -> AbortWe0ChW<ChabortSpec> {
        AbortWe0ChW::new(self, 30)
    }
    #[doc = "Bit 31 - DMAC Channel Abort Write-enable - 0: disable write to DMAC channel abort bit, 1: enable write DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: write-only"]
    #[inline(always)]
    #[must_use]
    pub fn abort_we0_ch16(&mut self) -> AbortWe0ChW<ChabortSpec> {
        AbortWe0ChW::new(self, 31)
    }
    #[doc = "DMAC Channel Abort - 0: no DMAC channel abort request, 1: request DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: read-write"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `abort1_ch17` field"]
    #[inline(always)]
    #[must_use]
    pub fn abort1_ch(&mut self, n: u8) -> Abort1ChW<ChabortSpec> {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        Abort1ChW::new(self, n + 32)
    }
    #[doc = "Bit 32 - DMAC Channel Abort - 0: no DMAC channel abort request, 1: request DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: read-write"]
    #[inline(always)]
    #[must_use]
    pub fn abort1_ch17(&mut self) -> Abort1ChW<ChabortSpec> {
        Abort1ChW::new(self, 32)
    }
    #[doc = "Bit 33 - DMAC Channel Abort - 0: no DMAC channel abort request, 1: request DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: read-write"]
    #[inline(always)]
    #[must_use]
    pub fn abort1_ch18(&mut self) -> Abort1ChW<ChabortSpec> {
        Abort1ChW::new(self, 33)
    }
    #[doc = "Bit 34 - DMAC Channel Abort - 0: no DMAC channel abort request, 1: request DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: read-write"]
    #[inline(always)]
    #[must_use]
    pub fn abort1_ch19(&mut self) -> Abort1ChW<ChabortSpec> {
        Abort1ChW::new(self, 34)
    }
    #[doc = "Bit 35 - DMAC Channel Abort - 0: no DMAC channel abort request, 1: request DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: read-write"]
    #[inline(always)]
    #[must_use]
    pub fn abort1_ch20(&mut self) -> Abort1ChW<ChabortSpec> {
        Abort1ChW::new(self, 35)
    }
    #[doc = "Bit 36 - DMAC Channel Abort - 0: no DMAC channel abort request, 1: request DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: read-write"]
    #[inline(always)]
    #[must_use]
    pub fn abort1_ch21(&mut self) -> Abort1ChW<ChabortSpec> {
        Abort1ChW::new(self, 36)
    }
    #[doc = "Bit 37 - DMAC Channel Abort - 0: no DMAC channel abort request, 1: request DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: read-write"]
    #[inline(always)]
    #[must_use]
    pub fn abort1_ch22(&mut self) -> Abort1ChW<ChabortSpec> {
        Abort1ChW::new(self, 37)
    }
    #[doc = "Bit 38 - DMAC Channel Abort - 0: no DMAC channel abort request, 1: request DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: read-write"]
    #[inline(always)]
    #[must_use]
    pub fn abort1_ch23(&mut self) -> Abort1ChW<ChabortSpec> {
        Abort1ChW::new(self, 38)
    }
    #[doc = "Bit 39 - DMAC Channel Abort - 0: no DMAC channel abort request, 1: request DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: read-write"]
    #[inline(always)]
    #[must_use]
    pub fn abort1_ch24(&mut self) -> Abort1ChW<ChabortSpec> {
        Abort1ChW::new(self, 39)
    }
    #[doc = "Bit 40 - DMAC Channel Abort - 0: no DMAC channel abort request, 1: request DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: read-write"]
    #[inline(always)]
    #[must_use]
    pub fn abort1_ch25(&mut self) -> Abort1ChW<ChabortSpec> {
        Abort1ChW::new(self, 40)
    }
    #[doc = "Bit 41 - DMAC Channel Abort - 0: no DMAC channel abort request, 1: request DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: read-write"]
    #[inline(always)]
    #[must_use]
    pub fn abort1_ch26(&mut self) -> Abort1ChW<ChabortSpec> {
        Abort1ChW::new(self, 41)
    }
    #[doc = "Bit 42 - DMAC Channel Abort - 0: no DMAC channel abort request, 1: request DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: read-write"]
    #[inline(always)]
    #[must_use]
    pub fn abort1_ch27(&mut self) -> Abort1ChW<ChabortSpec> {
        Abort1ChW::new(self, 42)
    }
    #[doc = "Bit 43 - DMAC Channel Abort - 0: no DMAC channel abort request, 1: request DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: read-write"]
    #[inline(always)]
    #[must_use]
    pub fn abort1_ch28(&mut self) -> Abort1ChW<ChabortSpec> {
        Abort1ChW::new(self, 43)
    }
    #[doc = "Bit 44 - DMAC Channel Abort - 0: no DMAC channel abort request, 1: request DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: read-write"]
    #[inline(always)]
    #[must_use]
    pub fn abort1_ch29(&mut self) -> Abort1ChW<ChabortSpec> {
        Abort1ChW::new(self, 44)
    }
    #[doc = "Bit 45 - DMAC Channel Abort - 0: no DMAC channel abort request, 1: request DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: read-write"]
    #[inline(always)]
    #[must_use]
    pub fn abort1_ch30(&mut self) -> Abort1ChW<ChabortSpec> {
        Abort1ChW::new(self, 45)
    }
    #[doc = "Bit 46 - DMAC Channel Abort - 0: no DMAC channel abort request, 1: request DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: read-write"]
    #[inline(always)]
    #[must_use]
    pub fn abort1_ch31(&mut self) -> Abort1ChW<ChabortSpec> {
        Abort1ChW::new(self, 46)
    }
    #[doc = "Bit 47 - DMAC Channel Abort - 0: no DMAC channel abort request, 1: request DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: read-write"]
    #[inline(always)]
    #[must_use]
    pub fn abort1_ch32(&mut self) -> Abort1ChW<ChabortSpec> {
        Abort1ChW::new(self, 47)
    }
    #[doc = "DMAC Channel Abort Write-enable - 0: disable write to DMAC channel abort bit, 1: enable write DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: write-only"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `abort_we1_ch17` field"]
    #[inline(always)]
    #[must_use]
    pub fn abort_we1_ch(&mut self, n: u8) -> AbortWe1ChW<ChabortSpec> {
        #[allow(clippy::no_effect)]
        [(); 16][n as usize];
        AbortWe1ChW::new(self, n + 48)
    }
    #[doc = "Bit 48 - DMAC Channel Abort Write-enable - 0: disable write to DMAC channel abort bit, 1: enable write DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: write-only"]
    #[inline(always)]
    #[must_use]
    pub fn abort_we1_ch17(&mut self) -> AbortWe1ChW<ChabortSpec> {
        AbortWe1ChW::new(self, 48)
    }
    #[doc = "Bit 49 - DMAC Channel Abort Write-enable - 0: disable write to DMAC channel abort bit, 1: enable write DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: write-only"]
    #[inline(always)]
    #[must_use]
    pub fn abort_we1_ch18(&mut self) -> AbortWe1ChW<ChabortSpec> {
        AbortWe1ChW::new(self, 49)
    }
    #[doc = "Bit 50 - DMAC Channel Abort Write-enable - 0: disable write to DMAC channel abort bit, 1: enable write DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: write-only"]
    #[inline(always)]
    #[must_use]
    pub fn abort_we1_ch19(&mut self) -> AbortWe1ChW<ChabortSpec> {
        AbortWe1ChW::new(self, 50)
    }
    #[doc = "Bit 51 - DMAC Channel Abort Write-enable - 0: disable write to DMAC channel abort bit, 1: enable write DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: write-only"]
    #[inline(always)]
    #[must_use]
    pub fn abort_we1_ch20(&mut self) -> AbortWe1ChW<ChabortSpec> {
        AbortWe1ChW::new(self, 51)
    }
    #[doc = "Bit 52 - DMAC Channel Abort Write-enable - 0: disable write to DMAC channel abort bit, 1: enable write DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: write-only"]
    #[inline(always)]
    #[must_use]
    pub fn abort_we1_ch21(&mut self) -> AbortWe1ChW<ChabortSpec> {
        AbortWe1ChW::new(self, 52)
    }
    #[doc = "Bit 53 - DMAC Channel Abort Write-enable - 0: disable write to DMAC channel abort bit, 1: enable write DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: write-only"]
    #[inline(always)]
    #[must_use]
    pub fn abort_we1_ch22(&mut self) -> AbortWe1ChW<ChabortSpec> {
        AbortWe1ChW::new(self, 53)
    }
    #[doc = "Bit 54 - DMAC Channel Abort Write-enable - 0: disable write to DMAC channel abort bit, 1: enable write DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: write-only"]
    #[inline(always)]
    #[must_use]
    pub fn abort_we1_ch23(&mut self) -> AbortWe1ChW<ChabortSpec> {
        AbortWe1ChW::new(self, 54)
    }
    #[doc = "Bit 55 - DMAC Channel Abort Write-enable - 0: disable write to DMAC channel abort bit, 1: enable write DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: write-only"]
    #[inline(always)]
    #[must_use]
    pub fn abort_we1_ch24(&mut self) -> AbortWe1ChW<ChabortSpec> {
        AbortWe1ChW::new(self, 55)
    }
    #[doc = "Bit 56 - DMAC Channel Abort Write-enable - 0: disable write to DMAC channel abort bit, 1: enable write DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: write-only"]
    #[inline(always)]
    #[must_use]
    pub fn abort_we1_ch25(&mut self) -> AbortWe1ChW<ChabortSpec> {
        AbortWe1ChW::new(self, 56)
    }
    #[doc = "Bit 57 - DMAC Channel Abort Write-enable - 0: disable write to DMAC channel abort bit, 1: enable write DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: write-only"]
    #[inline(always)]
    #[must_use]
    pub fn abort_we1_ch26(&mut self) -> AbortWe1ChW<ChabortSpec> {
        AbortWe1ChW::new(self, 57)
    }
    #[doc = "Bit 58 - DMAC Channel Abort Write-enable - 0: disable write to DMAC channel abort bit, 1: enable write DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: write-only"]
    #[inline(always)]
    #[must_use]
    pub fn abort_we1_ch27(&mut self) -> AbortWe1ChW<ChabortSpec> {
        AbortWe1ChW::new(self, 58)
    }
    #[doc = "Bit 59 - DMAC Channel Abort Write-enable - 0: disable write to DMAC channel abort bit, 1: enable write DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: write-only"]
    #[inline(always)]
    #[must_use]
    pub fn abort_we1_ch28(&mut self) -> AbortWe1ChW<ChabortSpec> {
        AbortWe1ChW::new(self, 59)
    }
    #[doc = "Bit 60 - DMAC Channel Abort Write-enable - 0: disable write to DMAC channel abort bit, 1: enable write DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: write-only"]
    #[inline(always)]
    #[must_use]
    pub fn abort_we1_ch29(&mut self) -> AbortWe1ChW<ChabortSpec> {
        AbortWe1ChW::new(self, 60)
    }
    #[doc = "Bit 61 - DMAC Channel Abort Write-enable - 0: disable write to DMAC channel abort bit, 1: enable write DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: write-only"]
    #[inline(always)]
    #[must_use]
    pub fn abort_we1_ch30(&mut self) -> AbortWe1ChW<ChabortSpec> {
        AbortWe1ChW::new(self, 61)
    }
    #[doc = "Bit 62 - DMAC Channel Abort Write-enable - 0: disable write to DMAC channel abort bit, 1: enable write DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: write-only"]
    #[inline(always)]
    #[must_use]
    pub fn abort_we1_ch31(&mut self) -> AbortWe1ChW<ChabortSpec> {
        AbortWe1ChW::new(self, 62)
    }
    #[doc = "Bit 63 - DMAC Channel Abort Write-enable - 0: disable write to DMAC channel abort bit, 1: enable write DMAC channel abort. Memory access depends on DMAX_CH_ABORT_EN configuration setting - 0: read-only, 1: write-only"]
    #[inline(always)]
    #[must_use]
    pub fn abort_we1_ch32(&mut self) -> AbortWe1ChW<ChabortSpec> {
        AbortWe1ChW::new(self, 63)
    }
}
#[doc = "DMAC Channel Abort register contains the DMAC channel abort settings. Only exists when DMAX_NUM_CHANNELS > 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`chabort::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`chabort::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChabortSpec;
impl crate::RegisterSpec for ChabortSpec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`chabort::R`](R) reader structure"]
impl crate::Readable for ChabortSpec {}
#[doc = "`write(|w| ..)` method takes [`chabort::W`](W) writer structure"]
impl crate::Writable for ChabortSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
}
#[doc = "`reset()` method sets chabort to value 0"]
impl crate::Resettable for ChabortSpec {
    const RESET_VALUE: u64 = 0;
}
