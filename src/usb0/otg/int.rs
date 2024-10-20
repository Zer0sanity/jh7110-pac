#[doc = "Register `int[%s]` reader"]
pub type R = crate::R<IntSpec>;
#[doc = "Register `int[%s]` writer"]
pub type W = crate::W<IntSpec>;
#[doc = "Field `id_change` reader - USB3 OTG ID change interrupt."]
pub type IdChangeR = crate::BitReader;
#[doc = "Field `id_change` writer - USB3 OTG ID change interrupt."]
pub type IdChangeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `vbusvalid(_rise,_fall)` reader - USB3 OTG VBUS valid change detected interrupt - 0: rise, 1: fall."]
pub type VbusvalidR = crate::BitReader;
#[doc = "Field `vbusvalid(_rise,_fall)` writer - USB3 OTG VBUS valid change detected interrupt - 0: rise, 1: fall."]
pub type VbusvalidW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - USB3 OTG ID change interrupt."]
    #[inline(always)]
    pub fn id_change(&self) -> IdChangeR {
        IdChangeR::new((self.bits & 1) != 0)
    }
    #[doc = "USB3 OTG VBUS valid change detected interrupt - 0: rise, 1: fall."]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `vbusvalid_rise` field"]
    #[inline(always)]
    pub fn vbusvalid(&self, n: u8) -> VbusvalidR {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        VbusvalidR::new(((self.bits >> (n + 4)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "USB3 OTG VBUS valid change detected interrupt - 0: rise, 1: fall."]
    #[inline(always)]
    pub fn vbusvalid_iter(&self) -> impl Iterator<Item = VbusvalidR> + '_ {
        (0..2).map(move |n| VbusvalidR::new(((self.bits >> (n + 4)) & 1) != 0))
    }
    #[doc = "Bit 4 - USB3 OTG VBUS valid change detected interrupt - 0: rise, 1: fall."]
    #[inline(always)]
    pub fn vbusvalid_rise(&self) -> VbusvalidR {
        VbusvalidR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - USB3 OTG VBUS valid change detected interrupt - 0: rise, 1: fall."]
    #[inline(always)]
    pub fn vbusvalid_fall(&self) -> VbusvalidR {
        VbusvalidR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USB3 OTG ID change interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn id_change(&mut self) -> IdChangeW<IntSpec> {
        IdChangeW::new(self, 0)
    }
    #[doc = "USB3 OTG VBUS valid change detected interrupt - 0: rise, 1: fall."]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `vbusvalid_rise` field"]
    #[inline(always)]
    #[must_use]
    pub fn vbusvalid(&mut self, n: u8) -> VbusvalidW<IntSpec> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        VbusvalidW::new(self, n + 4)
    }
    #[doc = "Bit 4 - USB3 OTG VBUS valid change detected interrupt - 0: rise, 1: fall."]
    #[inline(always)]
    #[must_use]
    pub fn vbusvalid_rise(&mut self) -> VbusvalidW<IntSpec> {
        VbusvalidW::new(self, 4)
    }
    #[doc = "Bit 5 - USB3 OTG VBUS valid change detected interrupt - 0: rise, 1: fall."]
    #[inline(always)]
    #[must_use]
    pub fn vbusvalid_fall(&mut self) -> VbusvalidW<IntSpec> {
        VbusvalidW::new(self, 5)
    }
}
#[doc = "USB3 OTG interrupt registers - 0: enable, 1: vector status. Write `1` to interrupt vector fields to clear the status.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntSpec;
impl crate::RegisterSpec for IntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int::R`](R) reader structure"]
impl crate::Readable for IntSpec {}
#[doc = "`write(|w| ..)` method takes [`int::W`](W) writer structure"]
impl crate::Writable for IntSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets int[%s]
to value 0"]
impl crate::Resettable for IntSpec {
    const RESET_VALUE: u32 = 0;
}
