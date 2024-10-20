#[doc = "Register `sys_syscfg24` reader"]
pub type R = crate::R<SysSyscfg24Spec>;
#[doc = "Register `sys_syscfg24` writer"]
pub type W = crate::W<SysSyscfg24Spec>;
#[doc = "Field `tdm1616slot_clkpol` reader - "]
pub type Tdm1616slotClkpolR = crate::BitReader;
#[doc = "Field `tdm1616slot_pcm_ms` reader - "]
pub type Tdm1616slotPcmMsR = crate::BitReader;
#[doc = "Field `u0_trace_mtx_in0_c0` reader - "]
pub type U0TraceMtxIn0C0R = crate::FieldReader;
#[doc = "Field `u0_trace_mtx_in0_c0` writer - "]
pub type U0TraceMtxIn0C0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `u0_trace_mtx_in1_c0` reader - "]
pub type U0TraceMtxIn1C0R = crate::FieldReader;
#[doc = "Field `u0_trace_mtx_in1_c0` writer - "]
pub type U0TraceMtxIn1C0W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `u0_trace_mtx_in0_c1` reader - "]
pub type U0TraceMtxIn0C1R = crate::FieldReader;
#[doc = "Field `u0_trace_mtx_in0_c1` writer - "]
pub type U0TraceMtxIn0C1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `u0_trace_mtx_in1_c1` reader - "]
pub type U0TraceMtxIn1C1R = crate::FieldReader;
#[doc = "Field `u0_trace_mtx_in1_c1` writer - "]
pub type U0TraceMtxIn1C1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `u0_trace_mtx_in0_c2` reader - "]
pub type U0TraceMtxIn0C2R = crate::FieldReader;
#[doc = "Field `u0_trace_mtx_in0_c2` writer - "]
pub type U0TraceMtxIn0C2W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `u0_trace_mtx_in1_c2` reader - "]
pub type U0TraceMtxIn1C2R = crate::FieldReader;
#[doc = "Field `u0_trace_mtx_in1_c2` writer - "]
pub type U0TraceMtxIn1C2W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn tdm1616slot_clkpol(&self) -> Tdm1616slotClkpolR {
        Tdm1616slotClkpolR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn tdm1616slot_pcm_ms(&self) -> Tdm1616slotPcmMsR {
        Tdm1616slotPcmMsR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:6"]
    #[inline(always)]
    pub fn u0_trace_mtx_in0_c0(&self) -> U0TraceMtxIn0C0R {
        U0TraceMtxIn0C0R::new(((self.bits >> 2) & 0x1f) as u8)
    }
    #[doc = "Bits 7:11"]
    #[inline(always)]
    pub fn u0_trace_mtx_in1_c0(&self) -> U0TraceMtxIn1C0R {
        U0TraceMtxIn1C0R::new(((self.bits >> 7) & 0x1f) as u8)
    }
    #[doc = "Bits 12:16"]
    #[inline(always)]
    pub fn u0_trace_mtx_in0_c1(&self) -> U0TraceMtxIn0C1R {
        U0TraceMtxIn0C1R::new(((self.bits >> 12) & 0x1f) as u8)
    }
    #[doc = "Bits 17:21"]
    #[inline(always)]
    pub fn u0_trace_mtx_in1_c1(&self) -> U0TraceMtxIn1C1R {
        U0TraceMtxIn1C1R::new(((self.bits >> 17) & 0x1f) as u8)
    }
    #[doc = "Bits 22:26"]
    #[inline(always)]
    pub fn u0_trace_mtx_in0_c2(&self) -> U0TraceMtxIn0C2R {
        U0TraceMtxIn0C2R::new(((self.bits >> 22) & 0x1f) as u8)
    }
    #[doc = "Bits 27:31"]
    #[inline(always)]
    pub fn u0_trace_mtx_in1_c2(&self) -> U0TraceMtxIn1C2R {
        U0TraceMtxIn1C2R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 2:6"]
    #[inline(always)]
    #[must_use]
    pub fn u0_trace_mtx_in0_c0(&mut self) -> U0TraceMtxIn0C0W<SysSyscfg24Spec> {
        U0TraceMtxIn0C0W::new(self, 2)
    }
    #[doc = "Bits 7:11"]
    #[inline(always)]
    #[must_use]
    pub fn u0_trace_mtx_in1_c0(&mut self) -> U0TraceMtxIn1C0W<SysSyscfg24Spec> {
        U0TraceMtxIn1C0W::new(self, 7)
    }
    #[doc = "Bits 12:16"]
    #[inline(always)]
    #[must_use]
    pub fn u0_trace_mtx_in0_c1(&mut self) -> U0TraceMtxIn0C1W<SysSyscfg24Spec> {
        U0TraceMtxIn0C1W::new(self, 12)
    }
    #[doc = "Bits 17:21"]
    #[inline(always)]
    #[must_use]
    pub fn u0_trace_mtx_in1_c1(&mut self) -> U0TraceMtxIn1C1W<SysSyscfg24Spec> {
        U0TraceMtxIn1C1W::new(self, 17)
    }
    #[doc = "Bits 22:26"]
    #[inline(always)]
    #[must_use]
    pub fn u0_trace_mtx_in0_c2(&mut self) -> U0TraceMtxIn0C2W<SysSyscfg24Spec> {
        U0TraceMtxIn0C2W::new(self, 22)
    }
    #[doc = "Bits 27:31"]
    #[inline(always)]
    #[must_use]
    pub fn u0_trace_mtx_in1_c2(&mut self) -> U0TraceMtxIn1C2W<SysSyscfg24Spec> {
        U0TraceMtxIn1C2W::new(self, 27)
    }
}
#[doc = "SYS SYSCONSAIF SYSCFG 96\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sys_syscfg24::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sys_syscfg24::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SysSyscfg24Spec;
impl crate::RegisterSpec for SysSyscfg24Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_syscfg24::R`](R) reader structure"]
impl crate::Readable for SysSyscfg24Spec {}
#[doc = "`write(|w| ..)` method takes [`sys_syscfg24::W`](W) writer structure"]
impl crate::Writable for SysSyscfg24Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets sys_syscfg24 to value 0"]
impl crate::Resettable for SysSyscfg24Spec {
    const RESET_VALUE: u32 = 0;
}
