#[doc = "Register `cr` reader"]
pub type R = crate::R<CrSpec>;
#[doc = "Register `cr` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `start` reader - "]
pub type StartR = crate::BitReader;
#[doc = "Field `start` writer - "]
pub type StartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `aes_dma_en` reader - "]
pub type AesDmaEnR = crate::BitReader;
#[doc = "Field `aes_dma_en` writer - "]
pub type AesDmaEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `des_dma_en` reader - "]
pub type DesDmaEnR = crate::BitReader;
#[doc = "Field `des_dma_en` writer - "]
pub type DesDmaEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `sha_dma_en` reader - "]
pub type ShaDmaEnR = crate::BitReader;
#[doc = "Field `sha_dma_en` writer - "]
pub type ShaDmaEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `alg_done` reader - "]
pub type AlgDoneR = crate::BitReader;
#[doc = "Field `alg_done` writer - "]
pub type AlgDoneW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `clear` reader - "]
pub type ClearR = crate::BitReader;
#[doc = "Field `clear` writer - "]
pub type ClearW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn start(&self) -> StartR {
        StartR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn aes_dma_en(&self) -> AesDmaEnR {
        AesDmaEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn des_dma_en(&self) -> DesDmaEnR {
        DesDmaEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn sha_dma_en(&self) -> ShaDmaEnR {
        ShaDmaEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn alg_done(&self) -> AlgDoneR {
        AlgDoneR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn clear(&self) -> ClearR {
        ClearR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> StartW<CrSpec> {
        StartW::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn aes_dma_en(&mut self) -> AesDmaEnW<CrSpec> {
        AesDmaEnW::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn des_dma_en(&mut self) -> DesDmaEnW<CrSpec> {
        DesDmaEnW::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn sha_dma_en(&mut self) -> ShaDmaEnW<CrSpec> {
        ShaDmaEnW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn alg_done(&mut self) -> AlgDoneW<CrSpec> {
        AlgDoneW::new(self, 4)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn clear(&mut self) -> ClearW<CrSpec> {
        ClearW::new(self, 8)
    }
}
#[doc = "JH7110 Crypto Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrSpec;
impl crate::RegisterSpec for CrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CrSpec {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets cr to value 0"]
impl crate::Resettable for CrSpec {
    const RESET_VALUE: u32 = 0;
}
