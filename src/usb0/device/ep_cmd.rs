#[doc = "Register `ep_cmd` reader"]
pub type R = crate::R<EpCmdSpec>;
#[doc = "Register `ep_cmd` writer"]
pub type W = crate::W<EpCmdSpec>;
#[doc = "Field `eprst` reader - Endpoint reset."]
pub type EprstR = crate::BitReader;
#[doc = "Field `eprst` writer - Endpoint reset."]
pub type EprstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `stall(_set,_clear)` reader - Endpoint STALL set/clear."]
pub type StallR = crate::BitReader;
#[doc = "Field `stall(_set,_clear)` writer - Endpoint STALL set/clear."]
pub type StallW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `erdy` reader - Send ERDY TP."]
pub type ErdyR = crate::BitReader;
#[doc = "Field `erdy` writer - Send ERDY TP."]
pub type ErdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `req_cmpl` reader - Request complete."]
pub type ReqCmplR = crate::BitReader;
#[doc = "Field `req_cmpl` writer - Request complete."]
pub type ReqCmplW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `drdy` reader - Transfer descriptor ready."]
pub type DrdyR = crate::BitReader;
#[doc = "Field `drdy` writer - Transfer descriptor ready."]
pub type DrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dflush` reader - Data flush."]
pub type DflushR = crate::BitReader;
#[doc = "Field `dflush` writer - Data flush."]
pub type DflushW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `stdl` reader - Transfer Descriptor Length write - only for SS mode BULK mode endpoints, removed in `DEV_VER_V3`."]
pub type StdlR = crate::BitReader;
#[doc = "Field `stdl` writer - Transfer Descriptor Length write - only for SS mode BULK mode endpoints, removed in `DEV_VER_V3`."]
pub type StdlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tdl` reader - Transfer Descriptor Length - only for SS mode BULK mode endpoints, removed in `DEV_VER_V3`."]
pub type TdlR = crate::FieldReader;
#[doc = "Field `tdl` writer - Transfer Descriptor Length - only for SS mode BULK mode endpoints, removed in `DEV_VER_V3`."]
pub type TdlW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `erdy_sid` reader - ERDY Stream ID value - used in SS mode."]
pub type ErdySidR = crate::FieldReader<u16>;
#[doc = "Field `erdy_sid` writer - ERDY Stream ID value - used in SS mode."]
pub type ErdySidW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - Endpoint reset."]
    #[inline(always)]
    pub fn eprst(&self) -> EprstR {
        EprstR::new((self.bits & 1) != 0)
    }
    #[doc = "Endpoint STALL set/clear."]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `stall_set` field"]
    #[inline(always)]
    pub fn stall(&self, n: u8) -> StallR {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        StallR::new(((self.bits >> (n + 1)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Endpoint STALL set/clear."]
    #[inline(always)]
    pub fn stall_iter(&self) -> impl Iterator<Item = StallR> + '_ {
        (0..2).map(move |n| StallR::new(((self.bits >> (n + 1)) & 1) != 0))
    }
    #[doc = "Bit 1 - Endpoint STALL set/clear."]
    #[inline(always)]
    pub fn stall_set(&self) -> StallR {
        StallR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Endpoint STALL set/clear."]
    #[inline(always)]
    pub fn stall_clear(&self) -> StallR {
        StallR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Send ERDY TP."]
    #[inline(always)]
    pub fn erdy(&self) -> ErdyR {
        ErdyR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Request complete."]
    #[inline(always)]
    pub fn req_cmpl(&self) -> ReqCmplR {
        ReqCmplR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transfer descriptor ready."]
    #[inline(always)]
    pub fn drdy(&self) -> DrdyR {
        DrdyR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Data flush."]
    #[inline(always)]
    pub fn dflush(&self) -> DflushR {
        DflushR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Transfer Descriptor Length write - only for SS mode BULK mode endpoints, removed in `DEV_VER_V3`."]
    #[inline(always)]
    pub fn stdl(&self) -> StdlR {
        StdlR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:15 - Transfer Descriptor Length - only for SS mode BULK mode endpoints, removed in `DEV_VER_V3`."]
    #[inline(always)]
    pub fn tdl(&self) -> TdlR {
        TdlR::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bits 16:31 - ERDY Stream ID value - used in SS mode."]
    #[inline(always)]
    pub fn erdy_sid(&self) -> ErdySidR {
        ErdySidR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Endpoint reset."]
    #[inline(always)]
    #[must_use]
    pub fn eprst(&mut self) -> EprstW<EpCmdSpec> {
        EprstW::new(self, 0)
    }
    #[doc = "Endpoint STALL set/clear."]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `stall_set` field"]
    #[inline(always)]
    #[must_use]
    pub fn stall(&mut self, n: u8) -> StallW<EpCmdSpec> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        StallW::new(self, n + 1)
    }
    #[doc = "Bit 1 - Endpoint STALL set/clear."]
    #[inline(always)]
    #[must_use]
    pub fn stall_set(&mut self) -> StallW<EpCmdSpec> {
        StallW::new(self, 1)
    }
    #[doc = "Bit 2 - Endpoint STALL set/clear."]
    #[inline(always)]
    #[must_use]
    pub fn stall_clear(&mut self) -> StallW<EpCmdSpec> {
        StallW::new(self, 2)
    }
    #[doc = "Bit 3 - Send ERDY TP."]
    #[inline(always)]
    #[must_use]
    pub fn erdy(&mut self) -> ErdyW<EpCmdSpec> {
        ErdyW::new(self, 3)
    }
    #[doc = "Bit 5 - Request complete."]
    #[inline(always)]
    #[must_use]
    pub fn req_cmpl(&mut self) -> ReqCmplW<EpCmdSpec> {
        ReqCmplW::new(self, 5)
    }
    #[doc = "Bit 6 - Transfer descriptor ready."]
    #[inline(always)]
    #[must_use]
    pub fn drdy(&mut self) -> DrdyW<EpCmdSpec> {
        DrdyW::new(self, 6)
    }
    #[doc = "Bit 7 - Data flush."]
    #[inline(always)]
    #[must_use]
    pub fn dflush(&mut self) -> DflushW<EpCmdSpec> {
        DflushW::new(self, 7)
    }
    #[doc = "Bit 8 - Transfer Descriptor Length write - only for SS mode BULK mode endpoints, removed in `DEV_VER_V3`."]
    #[inline(always)]
    #[must_use]
    pub fn stdl(&mut self) -> StdlW<EpCmdSpec> {
        StdlW::new(self, 8)
    }
    #[doc = "Bits 9:15 - Transfer Descriptor Length - only for SS mode BULK mode endpoints, removed in `DEV_VER_V3`."]
    #[inline(always)]
    #[must_use]
    pub fn tdl(&mut self) -> TdlW<EpCmdSpec> {
        TdlW::new(self, 9)
    }
    #[doc = "Bits 16:31 - ERDY Stream ID value - used in SS mode."]
    #[inline(always)]
    #[must_use]
    pub fn erdy_sid(&mut self) -> ErdySidW<EpCmdSpec> {
        ErdySidW::new(self, 16)
    }
}
#[doc = "USB3 Endpoint command.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ep_cmd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ep_cmd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EpCmdSpec;
impl crate::RegisterSpec for EpCmdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ep_cmd::R`](R) reader structure"]
impl crate::Readable for EpCmdSpec {}
#[doc = "`write(|w| ..)` method takes [`ep_cmd::W`](W) writer structure"]
impl crate::Writable for EpCmdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ep_cmd to value 0"]
impl crate::Resettable for EpCmdSpec {
    const RESET_VALUE: u32 = 0;
}
