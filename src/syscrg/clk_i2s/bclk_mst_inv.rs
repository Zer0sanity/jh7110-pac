#[doc = "Register `bclk_mst_inv` reader"]
pub type R = crate::R<BclkMstInvSpec>;
#[doc = "Register `bclk_mst_inv` writer"]
pub type W = crate::W<BclkMstInvSpec>;
#[doc = "Clock polarity settings.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClkPolarity {
    #[doc = "0: Set the clock polarity to use the clock buffer."]
    Buffer = 0,
    #[doc = "1: Set the clock polarity to use the clock inverter."]
    Inverter = 1,
}
impl From<ClkPolarity> for bool {
    #[inline(always)]
    fn from(variant: ClkPolarity) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `clk_polarity` reader - Clock polarity settings."]
pub type ClkPolarityR = crate::BitReader<ClkPolarity>;
impl ClkPolarityR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ClkPolarity {
        match self.bits {
            false => ClkPolarity::Buffer,
            true => ClkPolarity::Inverter,
        }
    }
    #[doc = "Set the clock polarity to use the clock buffer."]
    #[inline(always)]
    pub fn is_buffer(&self) -> bool {
        *self == ClkPolarity::Buffer
    }
    #[doc = "Set the clock polarity to use the clock inverter."]
    #[inline(always)]
    pub fn is_inverter(&self) -> bool {
        *self == ClkPolarity::Inverter
    }
}
#[doc = "Field `clk_polarity` writer - Clock polarity settings."]
pub type ClkPolarityW<'a, REG> = crate::BitWriter<'a, REG, ClkPolarity>;
impl<'a, REG> ClkPolarityW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Set the clock polarity to use the clock buffer."]
    #[inline(always)]
    pub fn buffer(self) -> &'a mut crate::W<REG> {
        self.variant(ClkPolarity::Buffer)
    }
    #[doc = "Set the clock polarity to use the clock inverter."]
    #[inline(always)]
    pub fn inverter(self) -> &'a mut crate::W<REG> {
        self.variant(ClkPolarity::Inverter)
    }
}
impl R {
    #[doc = "Bit 30 - Clock polarity settings."]
    #[inline(always)]
    pub fn clk_polarity(&self) -> ClkPolarityR {
        ClkPolarityR::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 30 - Clock polarity settings."]
    #[inline(always)]
    #[must_use]
    pub fn clk_polarity(&mut self) -> ClkPolarityW<BclkMstInvSpec> {
        ClkPolarityW::new(self, 30)
    }
}
#[doc = "U0 Clock I2S BCLK MST Inverter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bclk_mst_inv::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bclk_mst_inv::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BclkMstInvSpec;
impl crate::RegisterSpec for BclkMstInvSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bclk_mst_inv::R`](R) reader structure"]
impl crate::Readable for BclkMstInvSpec {}
#[doc = "`write(|w| ..)` method takes [`bclk_mst_inv::W`](W) writer structure"]
impl crate::Writable for BclkMstInvSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets bclk_mst_inv to value 0x4000_0000"]
impl crate::Resettable for BclkMstInvSpec {
    const RESET_VALUE: u32 = 0x4000_0000;
}