#[doc = "Register `ext_config` reader"]
pub type R = crate::R<ExtConfigSpec>;
#[doc = "Register `ext_config` writer"]
pub type W = crate::W<ExtConfigSpec>;
#[doc = "Field `hdsms` reader - HDSMS"]
pub type HdsmsR = crate::FieldReader;
#[doc = "Field `hdsms` writer - HDSMS"]
pub type HdsmsW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `eipg_en` reader - EIPG Enable"]
pub type EipgEnR = crate::BitReader;
#[doc = "Field `eipg_en` writer - EIPG Enable"]
pub type EipgEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `eipg` reader - EIPG"]
pub type EipgR = crate::FieldReader;
#[doc = "Field `eipg` writer - EIPG"]
pub type EipgW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 20:22 - HDSMS"]
    #[inline(always)]
    pub fn hdsms(&self) -> HdsmsR {
        HdsmsR::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 24 - EIPG Enable"]
    #[inline(always)]
    pub fn eipg_en(&self) -> EipgEnR {
        EipgEnR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:29 - EIPG"]
    #[inline(always)]
    pub fn eipg(&self) -> EipgR {
        EipgR::new(((self.bits >> 25) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 20:22 - HDSMS"]
    #[inline(always)]
    #[must_use]
    pub fn hdsms(&mut self) -> HdsmsW<ExtConfigSpec> {
        HdsmsW::new(self, 20)
    }
    #[doc = "Bit 24 - EIPG Enable"]
    #[inline(always)]
    #[must_use]
    pub fn eipg_en(&mut self) -> EipgEnW<ExtConfigSpec> {
        EipgEnW::new(self, 24)
    }
    #[doc = "Bits 25:29 - EIPG"]
    #[inline(always)]
    #[must_use]
    pub fn eipg(&mut self) -> EipgW<ExtConfigSpec> {
        EipgW::new(self, 25)
    }
}
#[doc = "MAC Extended Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ext_config::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ext_config::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ExtConfigSpec;
impl crate::RegisterSpec for ExtConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ext_config::R`](R) reader structure"]
impl crate::Readable for ExtConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`ext_config::W`](W) writer structure"]
impl crate::Writable for ExtConfigSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ext_config to value 0"]
impl crate::Resettable for ExtConfigSpec {
    const RESET_VALUE: u32 = 0;
}
