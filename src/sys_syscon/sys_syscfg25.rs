#[doc = "Register `sys_syscfg25` reader"]
pub type R = crate::R<SysSyscfg25Spec>;
#[doc = "Register `sys_syscfg25` writer"]
pub type W = crate::W<SysSyscfg25Spec>;
#[doc = "Field `u0_trace_mtx_scfg_in0_c3` reader - "]
pub type U0TraceMtxScfgIn0C3R = crate::FieldReader;
#[doc = "Field `u0_trace_mtx_scfg_in0_c3` writer - "]
pub type U0TraceMtxScfgIn0C3W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `u0_trace_mtx_scfg_in1_c3` reader - "]
pub type U0TraceMtxScfgIn1C3R = crate::FieldReader;
#[doc = "Field `u0_trace_mtx_scfg_in1_c3` writer - "]
pub type U0TraceMtxScfgIn1C3W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `u0_trace_mtx_scfg_in0_c4` reader - "]
pub type U0TraceMtxScfgIn0C4R = crate::FieldReader;
#[doc = "Field `u0_trace_mtx_scfg_in0_c4` writer - "]
pub type U0TraceMtxScfgIn0C4W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `u0_trace_mtx_scfg_in1_c4` reader - "]
pub type U0TraceMtxScfgIn1C4R = crate::FieldReader;
#[doc = "Field `u0_trace_mtx_scfg_in1_c4` writer - "]
pub type U0TraceMtxScfgIn1C4W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `u0_cease_from_tile0` reader - "]
pub type U0CeaseFromTile0R = crate::BitReader;
#[doc = "Field `u0_cease_from_tile1` reader - "]
pub type U0CeaseFromTile1R = crate::BitReader;
#[doc = "Field `u0_cease_from_tile2` reader - "]
pub type U0CeaseFromTile2R = crate::BitReader;
#[doc = "Field `u0_cease_from_tile3` reader - "]
pub type U0CeaseFromTile3R = crate::BitReader;
#[doc = "Field `u0_cease_from_tile4` reader - "]
pub type U0CeaseFromTile4R = crate::BitReader;
#[doc = "Field `u0_halt_from_tile0` reader - "]
pub type U0HaltFromTile0R = crate::BitReader;
#[doc = "Field `u0_halt_from_tile1` reader - "]
pub type U0HaltFromTile1R = crate::BitReader;
#[doc = "Field `u0_halt_from_tile2` reader - "]
pub type U0HaltFromTile2R = crate::BitReader;
#[doc = "Field `u0_halt_from_tile3` reader - "]
pub type U0HaltFromTile3R = crate::BitReader;
#[doc = "Field `u0_halt_from_tile4` reader - "]
pub type U0HaltFromTile4R = crate::BitReader;
impl R {
    #[doc = "Bits 2:6"]
    #[inline(always)]
    pub fn u0_trace_mtx_scfg_in0_c3(&self) -> U0TraceMtxScfgIn0C3R {
        U0TraceMtxScfgIn0C3R::new(((self.bits >> 2) & 0x1f) as u8)
    }
    #[doc = "Bits 7:11"]
    #[inline(always)]
    pub fn u0_trace_mtx_scfg_in1_c3(&self) -> U0TraceMtxScfgIn1C3R {
        U0TraceMtxScfgIn1C3R::new(((self.bits >> 7) & 0x1f) as u8)
    }
    #[doc = "Bits 12:16"]
    #[inline(always)]
    pub fn u0_trace_mtx_scfg_in0_c4(&self) -> U0TraceMtxScfgIn0C4R {
        U0TraceMtxScfgIn0C4R::new(((self.bits >> 12) & 0x1f) as u8)
    }
    #[doc = "Bits 17:21"]
    #[inline(always)]
    pub fn u0_trace_mtx_scfg_in1_c4(&self) -> U0TraceMtxScfgIn1C4R {
        U0TraceMtxScfgIn1C4R::new(((self.bits >> 17) & 0x1f) as u8)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn u0_cease_from_tile0(&self) -> U0CeaseFromTile0R {
        U0CeaseFromTile0R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn u0_cease_from_tile1(&self) -> U0CeaseFromTile1R {
        U0CeaseFromTile1R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn u0_cease_from_tile2(&self) -> U0CeaseFromTile2R {
        U0CeaseFromTile2R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn u0_cease_from_tile3(&self) -> U0CeaseFromTile3R {
        U0CeaseFromTile3R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn u0_cease_from_tile4(&self) -> U0CeaseFromTile4R {
        U0CeaseFromTile4R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn u0_halt_from_tile0(&self) -> U0HaltFromTile0R {
        U0HaltFromTile0R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn u0_halt_from_tile1(&self) -> U0HaltFromTile1R {
        U0HaltFromTile1R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn u0_halt_from_tile2(&self) -> U0HaltFromTile2R {
        U0HaltFromTile2R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn u0_halt_from_tile3(&self) -> U0HaltFromTile3R {
        U0HaltFromTile3R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn u0_halt_from_tile4(&self) -> U0HaltFromTile4R {
        U0HaltFromTile4R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 2:6"]
    #[inline(always)]
    #[must_use]
    pub fn u0_trace_mtx_scfg_in0_c3(&mut self) -> U0TraceMtxScfgIn0C3W<SysSyscfg25Spec> {
        U0TraceMtxScfgIn0C3W::new(self, 2)
    }
    #[doc = "Bits 7:11"]
    #[inline(always)]
    #[must_use]
    pub fn u0_trace_mtx_scfg_in1_c3(&mut self) -> U0TraceMtxScfgIn1C3W<SysSyscfg25Spec> {
        U0TraceMtxScfgIn1C3W::new(self, 7)
    }
    #[doc = "Bits 12:16"]
    #[inline(always)]
    #[must_use]
    pub fn u0_trace_mtx_scfg_in0_c4(&mut self) -> U0TraceMtxScfgIn0C4W<SysSyscfg25Spec> {
        U0TraceMtxScfgIn0C4W::new(self, 12)
    }
    #[doc = "Bits 17:21"]
    #[inline(always)]
    #[must_use]
    pub fn u0_trace_mtx_scfg_in1_c4(&mut self) -> U0TraceMtxScfgIn1C4W<SysSyscfg25Spec> {
        U0TraceMtxScfgIn1C4W::new(self, 17)
    }
}
#[doc = "SYS SYSCONSAIF SYSCFG 96\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_syscfg25::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_syscfg25::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysSyscfg25Spec;
impl crate::RegisterSpec for SysSyscfg25Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_syscfg25::R`](R) reader structure"]
impl crate::Readable for SysSyscfg25Spec {}
#[doc = "`write(|w| ..)` method takes [`sys_syscfg25::W`](W) writer structure"]
impl crate::Writable for SysSyscfg25Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets sys_syscfg25 to value 0"]
impl crate::Resettable for SysSyscfg25Spec {
    const RESET_VALUE: u32 = 0;
}
