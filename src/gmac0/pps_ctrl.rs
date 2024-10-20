#[doc = "Register `pps_ctrl[%s]` reader"]
pub type R = crate::R<PpsCtrlSpec>;
#[doc = "Register `pps_ctrl[%s]` writer"]
pub type W = crate::W<PpsCtrlSpec>;
#[doc = "Field `cmd0` reader - MTL PPS CMD"]
pub type Cmd0R = crate::FieldReader;
#[doc = "Field `cmd0` writer - MTL PPS CMD"]
pub type Cmd0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `en0` reader - MTL PPS EN"]
pub type En0R = crate::BitReader;
#[doc = "Field `en0` writer - MTL PPS EN"]
pub type En0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tgtmodsel0` reader - MTL PPS Target Mode Select"]
pub type Tgtmodsel0R = crate::FieldReader;
#[doc = "Field `tgtmodsel0` writer - MTL PPS Target Mode Select"]
pub type Tgtmodsel0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `mcgren0` reader - MTL PPS MCGR Enable"]
pub type Mcgren0R = crate::BitReader;
#[doc = "Field `mcgren0` writer - MTL PPS MCGR Enable"]
pub type Mcgren0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cmd1` reader - MTL PPS CMD"]
pub type Cmd1R = crate::FieldReader;
#[doc = "Field `cmd1` writer - MTL PPS CMD"]
pub type Cmd1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `en1` reader - MTL PPS EN"]
pub type En1R = crate::BitReader;
#[doc = "Field `en1` writer - MTL PPS EN"]
pub type En1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tgtmodsel1` reader - MTL PPS Target Mode Select"]
pub type Tgtmodsel1R = crate::FieldReader;
#[doc = "Field `tgtmodsel1` writer - MTL PPS Target Mode Select"]
pub type Tgtmodsel1W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `mcgren1` reader - MTL PPS MCGR Enable"]
pub type Mcgren1R = crate::BitReader;
#[doc = "Field `mcgren1` writer - MTL PPS MCGR Enable"]
pub type Mcgren1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cmd2` reader - MTL PPS CMD"]
pub type Cmd2R = crate::FieldReader;
#[doc = "Field `cmd2` writer - MTL PPS CMD"]
pub type Cmd2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `en2` reader - MTL PPS EN"]
pub type En2R = crate::BitReader;
#[doc = "Field `en2` writer - MTL PPS EN"]
pub type En2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tgtmodsel2` reader - MTL PPS Target Mode Select"]
pub type Tgtmodsel2R = crate::FieldReader;
#[doc = "Field `tgtmodsel2` writer - MTL PPS Target Mode Select"]
pub type Tgtmodsel2W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `mcgren2` reader - MTL PPS MCGR Enable"]
pub type Mcgren2R = crate::BitReader;
#[doc = "Field `mcgren2` writer - MTL PPS MCGR Enable"]
pub type Mcgren2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cmd3` reader - MTL PPS CMD"]
pub type Cmd3R = crate::FieldReader;
#[doc = "Field `cmd3` writer - MTL PPS CMD"]
pub type Cmd3W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `en3` reader - MTL PPS EN"]
pub type En3R = crate::BitReader;
#[doc = "Field `en3` writer - MTL PPS EN"]
pub type En3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `tgtmodsel3` reader - MTL PPS Target Mode Select"]
pub type Tgtmodsel3R = crate::FieldReader;
#[doc = "Field `tgtmodsel3` writer - MTL PPS Target Mode Select"]
pub type Tgtmodsel3W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `mcgren3` reader - MTL PPS MCGR Enable"]
pub type Mcgren3R = crate::BitReader;
#[doc = "Field `mcgren3` writer - MTL PPS MCGR Enable"]
pub type Mcgren3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - MTL PPS CMD"]
    #[inline(always)]
    pub fn cmd0(&self) -> Cmd0R {
        Cmd0R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - MTL PPS EN"]
    #[inline(always)]
    pub fn en0(&self) -> En0R {
        En0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - MTL PPS Target Mode Select"]
    #[inline(always)]
    pub fn tgtmodsel0(&self) -> Tgtmodsel0R {
        Tgtmodsel0R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - MTL PPS MCGR Enable"]
    #[inline(always)]
    pub fn mcgren0(&self) -> Mcgren0R {
        Mcgren0R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - MTL PPS CMD"]
    #[inline(always)]
    pub fn cmd1(&self) -> Cmd1R {
        Cmd1R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - MTL PPS EN"]
    #[inline(always)]
    pub fn en1(&self) -> En1R {
        En1R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:14 - MTL PPS Target Mode Select"]
    #[inline(always)]
    pub fn tgtmodsel1(&self) -> Tgtmodsel1R {
        Tgtmodsel1R::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15 - MTL PPS MCGR Enable"]
    #[inline(always)]
    pub fn mcgren1(&self) -> Mcgren1R {
        Mcgren1R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:19 - MTL PPS CMD"]
    #[inline(always)]
    pub fn cmd2(&self) -> Cmd2R {
        Cmd2R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - MTL PPS EN"]
    #[inline(always)]
    pub fn en2(&self) -> En2R {
        En2R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:22 - MTL PPS Target Mode Select"]
    #[inline(always)]
    pub fn tgtmodsel2(&self) -> Tgtmodsel2R {
        Tgtmodsel2R::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bit 23 - MTL PPS MCGR Enable"]
    #[inline(always)]
    pub fn mcgren2(&self) -> Mcgren2R {
        Mcgren2R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:27 - MTL PPS CMD"]
    #[inline(always)]
    pub fn cmd3(&self) -> Cmd3R {
        Cmd3R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 28 - MTL PPS EN"]
    #[inline(always)]
    pub fn en3(&self) -> En3R {
        En3R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bits 29:30 - MTL PPS Target Mode Select"]
    #[inline(always)]
    pub fn tgtmodsel3(&self) -> Tgtmodsel3R {
        Tgtmodsel3R::new(((self.bits >> 29) & 3) as u8)
    }
    #[doc = "Bit 31 - MTL PPS MCGR Enable"]
    #[inline(always)]
    pub fn mcgren3(&self) -> Mcgren3R {
        Mcgren3R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - MTL PPS CMD"]
    #[inline(always)]
    #[must_use]
    pub fn cmd0(&mut self) -> Cmd0W<PpsCtrlSpec> {
        Cmd0W::new(self, 0)
    }
    #[doc = "Bit 4 - MTL PPS EN"]
    #[inline(always)]
    #[must_use]
    pub fn en0(&mut self) -> En0W<PpsCtrlSpec> {
        En0W::new(self, 4)
    }
    #[doc = "Bits 5:6 - MTL PPS Target Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn tgtmodsel0(&mut self) -> Tgtmodsel0W<PpsCtrlSpec> {
        Tgtmodsel0W::new(self, 5)
    }
    #[doc = "Bit 7 - MTL PPS MCGR Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mcgren0(&mut self) -> Mcgren0W<PpsCtrlSpec> {
        Mcgren0W::new(self, 7)
    }
    #[doc = "Bits 8:11 - MTL PPS CMD"]
    #[inline(always)]
    #[must_use]
    pub fn cmd1(&mut self) -> Cmd1W<PpsCtrlSpec> {
        Cmd1W::new(self, 8)
    }
    #[doc = "Bit 12 - MTL PPS EN"]
    #[inline(always)]
    #[must_use]
    pub fn en1(&mut self) -> En1W<PpsCtrlSpec> {
        En1W::new(self, 12)
    }
    #[doc = "Bits 13:14 - MTL PPS Target Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn tgtmodsel1(&mut self) -> Tgtmodsel1W<PpsCtrlSpec> {
        Tgtmodsel1W::new(self, 13)
    }
    #[doc = "Bit 15 - MTL PPS MCGR Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mcgren1(&mut self) -> Mcgren1W<PpsCtrlSpec> {
        Mcgren1W::new(self, 15)
    }
    #[doc = "Bits 16:19 - MTL PPS CMD"]
    #[inline(always)]
    #[must_use]
    pub fn cmd2(&mut self) -> Cmd2W<PpsCtrlSpec> {
        Cmd2W::new(self, 16)
    }
    #[doc = "Bit 20 - MTL PPS EN"]
    #[inline(always)]
    #[must_use]
    pub fn en2(&mut self) -> En2W<PpsCtrlSpec> {
        En2W::new(self, 20)
    }
    #[doc = "Bits 21:22 - MTL PPS Target Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn tgtmodsel2(&mut self) -> Tgtmodsel2W<PpsCtrlSpec> {
        Tgtmodsel2W::new(self, 21)
    }
    #[doc = "Bit 23 - MTL PPS MCGR Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mcgren2(&mut self) -> Mcgren2W<PpsCtrlSpec> {
        Mcgren2W::new(self, 23)
    }
    #[doc = "Bits 24:27 - MTL PPS CMD"]
    #[inline(always)]
    #[must_use]
    pub fn cmd3(&mut self) -> Cmd3W<PpsCtrlSpec> {
        Cmd3W::new(self, 24)
    }
    #[doc = "Bit 28 - MTL PPS EN"]
    #[inline(always)]
    #[must_use]
    pub fn en3(&mut self) -> En3W<PpsCtrlSpec> {
        En3W::new(self, 28)
    }
    #[doc = "Bits 29:30 - MTL PPS Target Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn tgtmodsel3(&mut self) -> Tgtmodsel3W<PpsCtrlSpec> {
        Tgtmodsel3W::new(self, 29)
    }
    #[doc = "Bit 31 - MTL PPS MCGR Enable"]
    #[inline(always)]
    #[must_use]
    pub fn mcgren3(&mut self) -> Mcgren3W<PpsCtrlSpec> {
        Mcgren3W::new(self, 31)
    }
}
#[doc = "MTL PPS Control and Status - pps_ctrl0: channel 0-3 control, pps_ctrl1: channel 4-7 control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pps_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pps_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PpsCtrlSpec;
impl crate::RegisterSpec for PpsCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pps_ctrl::R`](R) reader structure"]
impl crate::Readable for PpsCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`pps_ctrl::W`](W) writer structure"]
impl crate::Writable for PpsCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets pps_ctrl[%s]
to value 0"]
impl crate::Resettable for PpsCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
