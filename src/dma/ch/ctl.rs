#[doc = "Register `ctl` reader"]
pub type R = crate::R<CtlSpec>;
#[doc = "Register `ctl` writer"]
pub type W = crate::W<CtlSpec>;
#[doc = "Field `sms` reader - Source Master Select - 0: AXI Master 1, 1: AXI Master 2"]
pub type SmsR = crate::BitReader;
#[doc = "Field `sms` writer - Source Master Select - 0: AXI Master 1, 1: AXI Master 2"]
pub type SmsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dms` reader - Destination Master Select - 0: AXI Master 1, 1: AXI Master 2"]
pub type DmsR = crate::BitReader;
#[doc = "Field `dms` writer - Destination Master Select - 0: AXI Master 1, 1: AXI Master 2"]
pub type DmsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sinc` reader - Source address increment - 0: increment, 1: no change. Indicates whether to increment the address on every transfer."]
pub type SincR = crate::BitReader;
#[doc = "Field `sinc` writer - Source address increment - 0: increment, 1: no change. Indicates whether to increment the address on every transfer."]
pub type SincW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dinc` reader - Destination address increment - 0: increment, 1: no change. Indicates whether to increment the address on every transfer."]
pub type DincR = crate::BitReader;
#[doc = "Field `dinc` writer - Destination address increment - 0: increment, 1: no change. Indicates whether to increment the address on every transfer."]
pub type DincW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tr_width(_src,_dst)` reader - Transfer width - 0: 8-bits, 1: 16-bits, 2: 32-bits, 3: 64-bits, 4: 128-bits, 5: 256-bits, 6: 512-bits."]
pub type TrWidthR = crate::FieldReader;
#[doc = "Field `tr_width(_src,_dst)` writer - Transfer width - 0: 8-bits, 1: 16-bits, 2: 32-bits, 3: 64-bits, 4: 128-bits, 5: 256-bits, 6: 512-bits."]
pub type TrWidthW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `msize(_src,_dst)` reader - Burst transaction length - 0: 1 data item, 1: 4 data items, 2: 8 data items, 3: 16 data items, 4: 32 data items, 5: 64 data items, 6: 128 data items, 7: 256 data items, 8: 512 data items, 9: 1024 data items."]
pub type MsizeR = crate::FieldReader;
#[doc = "Field `msize(_src,_dst)` writer - Burst transaction length - 0: 1 data item, 1: 4 data items, 2: 8 data items, 3: 16 data items, 4: 32 data items, 5: 64 data items, 6: 128 data items, 7: 256 data items, 8: 512 data items, 9: 1024 data items."]
pub type MsizeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `cache(_ar,_aw)` reader - AXI cache signal"]
pub type CacheR = crate::FieldReader;
#[doc = "Field `cache(_ar,_aw)` writer - AXI cache signal"]
pub type CacheW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `non_posted_last_write_en` reader - Non-posted Last Write Enable - 0: posted writes can be used throughout the block transfer, 1: posted writes can be used up to the last write, the last write must be non-posted."]
pub type NonPostedLastWriteEnR = crate::BitReader;
#[doc = "Field `non_posted_last_write_en` writer - Non-posted Last Write Enable - 0: posted writes can be used throughout the block transfer, 1: posted writes can be used up to the last write, the last write must be non-posted."]
pub type NonPostedLastWriteEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `prot(_ar,_aw)` reader - AXI prot signal"]
pub type ProtR = crate::FieldReader;
#[doc = "Field `prot(_ar,_aw)` writer - AXI prot signal"]
pub type ProtW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `arlen_en` reader - Source burst length enable - 0: disable, 1: enable"]
pub type ArlenEnR = crate::BitReader;
#[doc = "Field `arlen_en` writer - Source burst length enable - 0: disable, 1: enable"]
pub type ArlenEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `arlen` reader - Source burst length"]
pub type ArlenR = crate::FieldReader;
#[doc = "Field `arlen` writer - Source burst length"]
pub type ArlenW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `awlen_en` reader - Destination burst length enable - 0: disable, 1: enable"]
pub type AwlenEnR = crate::BitReader;
#[doc = "Field `awlen_en` writer - Destination burst length enable - 0: disable, 1: enable"]
pub type AwlenEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `awlen` reader - Destination burst length"]
pub type AwlenR = crate::FieldReader;
#[doc = "Field `awlen` writer - Destination burst length"]
pub type AwlenW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `stat_en(_src,_dst)` reader - Status enable"]
pub type StatEnR = crate::BitReader;
#[doc = "Field `stat_en(_src,_dst)` writer - Status enable"]
pub type StatEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ioc_block_tr` reader - Interrupt-on-completion block transfer - 0: disable, 1: enable"]
pub type IocBlockTrR = crate::BitReader;
#[doc = "Field `ioc_block_tr` writer - Interrupt-on-completion block transfer - 0: disable, 1: enable"]
pub type IocBlockTrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `shadow_or_lli(_last,_valid)` reader - Shadow or Linked List Item - 0: not last/valid, 1: last/valid"]
pub type ShadowOrLliR = crate::BitReader;
#[doc = "Field `shadow_or_lli(_last,_valid)` writer - Shadow or Linked List Item - 0: not last/valid, 1: last/valid"]
pub type ShadowOrLliW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Source Master Select - 0: AXI Master 1, 1: AXI Master 2"]
    #[inline(always)]
    pub fn sms(&self) -> SmsR {
        SmsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Destination Master Select - 0: AXI Master 1, 1: AXI Master 2"]
    #[inline(always)]
    pub fn dms(&self) -> DmsR {
        DmsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Source address increment - 0: increment, 1: no change. Indicates whether to increment the address on every transfer."]
    #[inline(always)]
    pub fn sinc(&self) -> SincR {
        SincR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Destination address increment - 0: increment, 1: no change. Indicates whether to increment the address on every transfer."]
    #[inline(always)]
    pub fn dinc(&self) -> DincR {
        DincR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Transfer width - 0: 8-bits, 1: 16-bits, 2: 32-bits, 3: 64-bits, 4: 128-bits, 5: 256-bits, 6: 512-bits."]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `tr_width_src` field"]
    #[inline(always)]
    pub fn tr_width(&self, n: u8) -> TrWidthR {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        TrWidthR::new(((self.bits >> (n * 3 + 8)) & 7) as u8)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Transfer width - 0: 8-bits, 1: 16-bits, 2: 32-bits, 3: 64-bits, 4: 128-bits, 5: 256-bits, 6: 512-bits."]
    #[inline(always)]
    pub fn tr_width_iter(&self) -> impl Iterator<Item = TrWidthR> + '_ {
        (0..2).map(move |n| TrWidthR::new(((self.bits >> (n * 3 + 8)) & 7) as u8))
    }
    #[doc = "Bits 8:10 - Transfer width - 0: 8-bits, 1: 16-bits, 2: 32-bits, 3: 64-bits, 4: 128-bits, 5: 256-bits, 6: 512-bits."]
    #[inline(always)]
    pub fn tr_width_src(&self) -> TrWidthR {
        TrWidthR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:13 - Transfer width - 0: 8-bits, 1: 16-bits, 2: 32-bits, 3: 64-bits, 4: 128-bits, 5: 256-bits, 6: 512-bits."]
    #[inline(always)]
    pub fn tr_width_dst(&self) -> TrWidthR {
        TrWidthR::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Burst transaction length - 0: 1 data item, 1: 4 data items, 2: 8 data items, 3: 16 data items, 4: 32 data items, 5: 64 data items, 6: 128 data items, 7: 256 data items, 8: 512 data items, 9: 1024 data items."]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `msize_src` field"]
    #[inline(always)]
    pub fn msize(&self, n: u8) -> MsizeR {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        MsizeR::new(((self.bits >> (n * 4 + 14)) & 0x0f) as u8)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Burst transaction length - 0: 1 data item, 1: 4 data items, 2: 8 data items, 3: 16 data items, 4: 32 data items, 5: 64 data items, 6: 128 data items, 7: 256 data items, 8: 512 data items, 9: 1024 data items."]
    #[inline(always)]
    pub fn msize_iter(&self) -> impl Iterator<Item = MsizeR> + '_ {
        (0..2).map(move |n| MsizeR::new(((self.bits >> (n * 4 + 14)) & 0x0f) as u8))
    }
    #[doc = "Bits 14:17 - Burst transaction length - 0: 1 data item, 1: 4 data items, 2: 8 data items, 3: 16 data items, 4: 32 data items, 5: 64 data items, 6: 128 data items, 7: 256 data items, 8: 512 data items, 9: 1024 data items."]
    #[inline(always)]
    pub fn msize_src(&self) -> MsizeR {
        MsizeR::new(((self.bits >> 14) & 0x0f) as u8)
    }
    #[doc = "Bits 18:21 - Burst transaction length - 0: 1 data item, 1: 4 data items, 2: 8 data items, 3: 16 data items, 4: 32 data items, 5: 64 data items, 6: 128 data items, 7: 256 data items, 8: 512 data items, 9: 1024 data items."]
    #[inline(always)]
    pub fn msize_dst(&self) -> MsizeR {
        MsizeR::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "AXI cache signal"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `cache_ar` field"]
    #[inline(always)]
    pub fn cache(&self, n: u8) -> CacheR {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        CacheR::new(((self.bits >> (n * 4 + 22)) & 0x0f) as u8)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "AXI cache signal"]
    #[inline(always)]
    pub fn cache_iter(&self) -> impl Iterator<Item = CacheR> + '_ {
        (0..2).map(move |n| CacheR::new(((self.bits >> (n * 4 + 22)) & 0x0f) as u8))
    }
    #[doc = "Bits 22:25 - AXI cache signal"]
    #[inline(always)]
    pub fn cache_ar(&self) -> CacheR {
        CacheR::new(((self.bits >> 22) & 0x0f) as u8)
    }
    #[doc = "Bits 26:29 - AXI cache signal"]
    #[inline(always)]
    pub fn cache_aw(&self) -> CacheR {
        CacheR::new(((self.bits >> 26) & 0x0f) as u8)
    }
    #[doc = "Bit 30 - Non-posted Last Write Enable - 0: posted writes can be used throughout the block transfer, 1: posted writes can be used up to the last write, the last write must be non-posted."]
    #[inline(always)]
    pub fn non_posted_last_write_en(&self) -> NonPostedLastWriteEnR {
        NonPostedLastWriteEnR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "AXI prot signal"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `prot_ar` field"]
    #[inline(always)]
    pub fn prot(&self, n: u8) -> ProtR {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        ProtR::new(((self.bits >> (n * 3 + 32)) & 7) as u8)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "AXI prot signal"]
    #[inline(always)]
    pub fn prot_iter(&self) -> impl Iterator<Item = ProtR> + '_ {
        (0..2).map(move |n| ProtR::new(((self.bits >> (n * 3 + 32)) & 7) as u8))
    }
    #[doc = "Bits 32:34 - AXI prot signal"]
    #[inline(always)]
    pub fn prot_ar(&self) -> ProtR {
        ProtR::new(((self.bits >> 32) & 7) as u8)
    }
    #[doc = "Bits 35:37 - AXI prot signal"]
    #[inline(always)]
    pub fn prot_aw(&self) -> ProtR {
        ProtR::new(((self.bits >> 35) & 7) as u8)
    }
    #[doc = "Bit 38 - Source burst length enable - 0: disable, 1: enable"]
    #[inline(always)]
    pub fn arlen_en(&self) -> ArlenEnR {
        ArlenEnR::new(((self.bits >> 38) & 1) != 0)
    }
    #[doc = "Bits 39:46 - Source burst length"]
    #[inline(always)]
    pub fn arlen(&self) -> ArlenR {
        ArlenR::new(((self.bits >> 39) & 0xff) as u8)
    }
    #[doc = "Bit 47 - Destination burst length enable - 0: disable, 1: enable"]
    #[inline(always)]
    pub fn awlen_en(&self) -> AwlenEnR {
        AwlenEnR::new(((self.bits >> 47) & 1) != 0)
    }
    #[doc = "Bits 48:55 - Destination burst length"]
    #[inline(always)]
    pub fn awlen(&self) -> AwlenR {
        AwlenR::new(((self.bits >> 48) & 0xff) as u8)
    }
    #[doc = "Status enable"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `stat_en_src` field"]
    #[inline(always)]
    pub fn stat_en(&self, n: u8) -> StatEnR {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        StatEnR::new(((self.bits >> (n + 56)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Status enable"]
    #[inline(always)]
    pub fn stat_en_iter(&self) -> impl Iterator<Item = StatEnR> + '_ {
        (0..2).map(move |n| StatEnR::new(((self.bits >> (n + 56)) & 1) != 0))
    }
    #[doc = "Bit 56 - Status enable"]
    #[inline(always)]
    pub fn stat_en_src(&self) -> StatEnR {
        StatEnR::new(((self.bits >> 56) & 1) != 0)
    }
    #[doc = "Bit 57 - Status enable"]
    #[inline(always)]
    pub fn stat_en_dst(&self) -> StatEnR {
        StatEnR::new(((self.bits >> 57) & 1) != 0)
    }
    #[doc = "Bit 58 - Interrupt-on-completion block transfer - 0: disable, 1: enable"]
    #[inline(always)]
    pub fn ioc_block_tr(&self) -> IocBlockTrR {
        IocBlockTrR::new(((self.bits >> 58) & 1) != 0)
    }
    #[doc = "Shadow or Linked List Item - 0: not last/valid, 1: last/valid"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `shadow_or_lli_last` field"]
    #[inline(always)]
    pub fn shadow_or_lli(&self, n: u8) -> ShadowOrLliR {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        ShadowOrLliR::new(((self.bits >> (n + 62)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Shadow or Linked List Item - 0: not last/valid, 1: last/valid"]
    #[inline(always)]
    pub fn shadow_or_lli_iter(&self) -> impl Iterator<Item = ShadowOrLliR> + '_ {
        (0..2).map(move |n| ShadowOrLliR::new(((self.bits >> (n + 62)) & 1) != 0))
    }
    #[doc = "Bit 62 - Shadow or Linked List Item - 0: not last/valid, 1: last/valid"]
    #[inline(always)]
    pub fn shadow_or_lli_last(&self) -> ShadowOrLliR {
        ShadowOrLliR::new(((self.bits >> 62) & 1) != 0)
    }
    #[doc = "Bit 63 - Shadow or Linked List Item - 0: not last/valid, 1: last/valid"]
    #[inline(always)]
    pub fn shadow_or_lli_valid(&self) -> ShadowOrLliR {
        ShadowOrLliR::new(((self.bits >> 63) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Source Master Select - 0: AXI Master 1, 1: AXI Master 2"]
    #[inline(always)]
    #[must_use]
    pub fn sms(&mut self) -> SmsW<CtlSpec> {
        SmsW::new(self, 0)
    }
    #[doc = "Bit 2 - Destination Master Select - 0: AXI Master 1, 1: AXI Master 2"]
    #[inline(always)]
    #[must_use]
    pub fn dms(&mut self) -> DmsW<CtlSpec> {
        DmsW::new(self, 2)
    }
    #[doc = "Bit 4 - Source address increment - 0: increment, 1: no change. Indicates whether to increment the address on every transfer."]
    #[inline(always)]
    #[must_use]
    pub fn sinc(&mut self) -> SincW<CtlSpec> {
        SincW::new(self, 4)
    }
    #[doc = "Bit 6 - Destination address increment - 0: increment, 1: no change. Indicates whether to increment the address on every transfer."]
    #[inline(always)]
    #[must_use]
    pub fn dinc(&mut self) -> DincW<CtlSpec> {
        DincW::new(self, 6)
    }
    #[doc = "Transfer width - 0: 8-bits, 1: 16-bits, 2: 32-bits, 3: 64-bits, 4: 128-bits, 5: 256-bits, 6: 512-bits."]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `tr_width_src` field"]
    #[inline(always)]
    #[must_use]
    pub fn tr_width(&mut self, n: u8) -> TrWidthW<CtlSpec> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        TrWidthW::new(self, n * 3 + 8)
    }
    #[doc = "Bits 8:10 - Transfer width - 0: 8-bits, 1: 16-bits, 2: 32-bits, 3: 64-bits, 4: 128-bits, 5: 256-bits, 6: 512-bits."]
    #[inline(always)]
    #[must_use]
    pub fn tr_width_src(&mut self) -> TrWidthW<CtlSpec> {
        TrWidthW::new(self, 8)
    }
    #[doc = "Bits 11:13 - Transfer width - 0: 8-bits, 1: 16-bits, 2: 32-bits, 3: 64-bits, 4: 128-bits, 5: 256-bits, 6: 512-bits."]
    #[inline(always)]
    #[must_use]
    pub fn tr_width_dst(&mut self) -> TrWidthW<CtlSpec> {
        TrWidthW::new(self, 11)
    }
    #[doc = "Burst transaction length - 0: 1 data item, 1: 4 data items, 2: 8 data items, 3: 16 data items, 4: 32 data items, 5: 64 data items, 6: 128 data items, 7: 256 data items, 8: 512 data items, 9: 1024 data items."]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `msize_src` field"]
    #[inline(always)]
    #[must_use]
    pub fn msize(&mut self, n: u8) -> MsizeW<CtlSpec> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        MsizeW::new(self, n * 4 + 14)
    }
    #[doc = "Bits 14:17 - Burst transaction length - 0: 1 data item, 1: 4 data items, 2: 8 data items, 3: 16 data items, 4: 32 data items, 5: 64 data items, 6: 128 data items, 7: 256 data items, 8: 512 data items, 9: 1024 data items."]
    #[inline(always)]
    #[must_use]
    pub fn msize_src(&mut self) -> MsizeW<CtlSpec> {
        MsizeW::new(self, 14)
    }
    #[doc = "Bits 18:21 - Burst transaction length - 0: 1 data item, 1: 4 data items, 2: 8 data items, 3: 16 data items, 4: 32 data items, 5: 64 data items, 6: 128 data items, 7: 256 data items, 8: 512 data items, 9: 1024 data items."]
    #[inline(always)]
    #[must_use]
    pub fn msize_dst(&mut self) -> MsizeW<CtlSpec> {
        MsizeW::new(self, 18)
    }
    #[doc = "AXI cache signal"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `cache_ar` field"]
    #[inline(always)]
    #[must_use]
    pub fn cache(&mut self, n: u8) -> CacheW<CtlSpec> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        CacheW::new(self, n * 4 + 22)
    }
    #[doc = "Bits 22:25 - AXI cache signal"]
    #[inline(always)]
    #[must_use]
    pub fn cache_ar(&mut self) -> CacheW<CtlSpec> {
        CacheW::new(self, 22)
    }
    #[doc = "Bits 26:29 - AXI cache signal"]
    #[inline(always)]
    #[must_use]
    pub fn cache_aw(&mut self) -> CacheW<CtlSpec> {
        CacheW::new(self, 26)
    }
    #[doc = "Bit 30 - Non-posted Last Write Enable - 0: posted writes can be used throughout the block transfer, 1: posted writes can be used up to the last write, the last write must be non-posted."]
    #[inline(always)]
    #[must_use]
    pub fn non_posted_last_write_en(&mut self) -> NonPostedLastWriteEnW<CtlSpec> {
        NonPostedLastWriteEnW::new(self, 30)
    }
    #[doc = "AXI prot signal"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `prot_ar` field"]
    #[inline(always)]
    #[must_use]
    pub fn prot(&mut self, n: u8) -> ProtW<CtlSpec> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        ProtW::new(self, n * 3 + 32)
    }
    #[doc = "Bits 32:34 - AXI prot signal"]
    #[inline(always)]
    #[must_use]
    pub fn prot_ar(&mut self) -> ProtW<CtlSpec> {
        ProtW::new(self, 32)
    }
    #[doc = "Bits 35:37 - AXI prot signal"]
    #[inline(always)]
    #[must_use]
    pub fn prot_aw(&mut self) -> ProtW<CtlSpec> {
        ProtW::new(self, 35)
    }
    #[doc = "Bit 38 - Source burst length enable - 0: disable, 1: enable"]
    #[inline(always)]
    #[must_use]
    pub fn arlen_en(&mut self) -> ArlenEnW<CtlSpec> {
        ArlenEnW::new(self, 38)
    }
    #[doc = "Bits 39:46 - Source burst length"]
    #[inline(always)]
    #[must_use]
    pub fn arlen(&mut self) -> ArlenW<CtlSpec> {
        ArlenW::new(self, 39)
    }
    #[doc = "Bit 47 - Destination burst length enable - 0: disable, 1: enable"]
    #[inline(always)]
    #[must_use]
    pub fn awlen_en(&mut self) -> AwlenEnW<CtlSpec> {
        AwlenEnW::new(self, 47)
    }
    #[doc = "Bits 48:55 - Destination burst length"]
    #[inline(always)]
    #[must_use]
    pub fn awlen(&mut self) -> AwlenW<CtlSpec> {
        AwlenW::new(self, 48)
    }
    #[doc = "Status enable"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `stat_en_src` field"]
    #[inline(always)]
    #[must_use]
    pub fn stat_en(&mut self, n: u8) -> StatEnW<CtlSpec> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        StatEnW::new(self, n + 56)
    }
    #[doc = "Bit 56 - Status enable"]
    #[inline(always)]
    #[must_use]
    pub fn stat_en_src(&mut self) -> StatEnW<CtlSpec> {
        StatEnW::new(self, 56)
    }
    #[doc = "Bit 57 - Status enable"]
    #[inline(always)]
    #[must_use]
    pub fn stat_en_dst(&mut self) -> StatEnW<CtlSpec> {
        StatEnW::new(self, 57)
    }
    #[doc = "Bit 58 - Interrupt-on-completion block transfer - 0: disable, 1: enable"]
    #[inline(always)]
    #[must_use]
    pub fn ioc_block_tr(&mut self) -> IocBlockTrW<CtlSpec> {
        IocBlockTrW::new(self, 58)
    }
    #[doc = "Shadow or Linked List Item - 0: not last/valid, 1: last/valid"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `shadow_or_lli_last` field"]
    #[inline(always)]
    #[must_use]
    pub fn shadow_or_lli(&mut self, n: u8) -> ShadowOrLliW<CtlSpec> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        ShadowOrLliW::new(self, n + 62)
    }
    #[doc = "Bit 62 - Shadow or Linked List Item - 0: not last/valid, 1: last/valid"]
    #[inline(always)]
    #[must_use]
    pub fn shadow_or_lli_last(&mut self) -> ShadowOrLliW<CtlSpec> {
        ShadowOrLliW::new(self, 62)
    }
    #[doc = "Bit 63 - Shadow or Linked List Item - 0: not last/valid, 1: last/valid"]
    #[inline(always)]
    #[must_use]
    pub fn shadow_or_lli_valid(&mut self) -> ShadowOrLliW<CtlSpec> {
        ShadowOrLliW::new(self, 63)
    }
}
#[doc = "DMAC Channel Control.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtlSpec;
impl crate::RegisterSpec for CtlSpec {
    type Ux = u64;
}
#[doc = "`read()` method returns [`ctl::R`](R) reader structure"]
impl crate::Readable for CtlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctl::W`](W) writer structure"]
impl crate::Writable for CtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u64 = 0;
}
#[doc = "`reset()` method sets ctl to value 0"]
impl crate::Resettable for CtlSpec {
    const RESET_VALUE: u64 = 0;
}
