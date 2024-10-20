#[doc = "Register `tmout` reader"]
pub type R = crate::R<TmoutSpec>;
#[doc = "Register `tmout` writer"]
pub type W = crate::W<TmoutSpec>;
#[doc = "Field `resp` reader - MMC Response Timeout"]
pub type RespR = crate::FieldReader;
#[doc = "Field `resp` writer - MMC Response Timeout"]
pub type RespW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `data` reader - MMC Data Timeout"]
pub type DataR = crate::FieldReader<u32>;
#[doc = "Field `data` writer - MMC Data Timeout"]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:7 - MMC Response Timeout"]
    #[inline(always)]
    pub fn resp(&self) -> RespR {
        RespR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - MMC Data Timeout"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - MMC Response Timeout"]
    #[inline(always)]
    #[must_use]
    pub fn resp(&mut self) -> RespW<TmoutSpec> {
        RespW::new(self, 0)
    }
    #[doc = "Bits 8:31 - MMC Data Timeout"]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DataW<TmoutSpec> {
        DataW::new(self, 8)
    }
}
#[doc = "MMC Timeout\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tmout::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tmout::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TmoutSpec;
impl crate::RegisterSpec for TmoutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tmout::R`](R) reader structure"]
impl crate::Readable for TmoutSpec {}
#[doc = "`write(|w| ..)` method takes [`tmout::W`](W) writer structure"]
impl crate::Writable for TmoutSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets tmout to value 0"]
impl crate::Resettable for TmoutSpec {
    const RESET_VALUE: u32 = 0;
}
