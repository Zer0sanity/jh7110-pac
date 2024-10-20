#[doc = "Register `l3l4_ctrl` reader"]
pub type R = crate::R<L3l4CtrlSpec>;
#[doc = "Register `l3l4_ctrl` writer"]
pub type W = crate::W<L3l4CtrlSpec>;
#[doc = "Field `l3pen` reader - L3 Filter PEN"]
pub type L3penR = crate::BitReader;
#[doc = "Field `l3pen` writer - L3 Filter PEN"]
pub type L3penW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `l3sam` reader - L3 Filter SAM"]
pub type L3samR = crate::BitReader;
#[doc = "Field `l3sam` writer - L3 Filter SAM"]
pub type L3samW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `l3saim` reader - L3 Filter SAIM"]
pub type L3saimR = crate::BitReader;
#[doc = "Field `l3saim` writer - L3 Filter SAIM"]
pub type L3saimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `l3dam` reader - L3 Filter DAM"]
pub type L3damR = crate::BitReader;
#[doc = "Field `l3dam` writer - L3 Filter DAM"]
pub type L3damW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `l3daim` reader - L3 Filter DAIM"]
pub type L3daimR = crate::BitReader;
#[doc = "Field `l3daim` writer - L3 Filter DAIM"]
pub type L3daimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `l4pen` reader - L4 Filter PEN"]
pub type L4penR = crate::BitReader;
#[doc = "Field `l4pen` writer - L4 Filter PEN"]
pub type L4penW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `l4spm` reader - L4 Filter SPM"]
pub type L4spmR = crate::BitReader;
#[doc = "Field `l4spm` writer - L4 Filter SPM"]
pub type L4spmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `l4spim` reader - L4 Filter SPIM"]
pub type L4spimR = crate::BitReader;
#[doc = "Field `l4spim` writer - L4 Filter SPIM"]
pub type L4spimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `l4dpm` reader - L4 Filter DPM"]
pub type L4dpmR = crate::BitReader;
#[doc = "Field `l4dpm` writer - L4 Filter DPM"]
pub type L4dpmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `l4dpim` reader - L4 Filter DPIM"]
pub type L4dpimR = crate::BitReader;
#[doc = "Field `l4dpim` writer - L4 Filter DPIM"]
pub type L4dpimW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - L3 Filter PEN"]
    #[inline(always)]
    pub fn l3pen(&self) -> L3penR {
        L3penR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - L3 Filter SAM"]
    #[inline(always)]
    pub fn l3sam(&self) -> L3samR {
        L3samR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - L3 Filter SAIM"]
    #[inline(always)]
    pub fn l3saim(&self) -> L3saimR {
        L3saimR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - L3 Filter DAM"]
    #[inline(always)]
    pub fn l3dam(&self) -> L3damR {
        L3damR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - L3 Filter DAIM"]
    #[inline(always)]
    pub fn l3daim(&self) -> L3daimR {
        L3daimR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 16 - L4 Filter PEN"]
    #[inline(always)]
    pub fn l4pen(&self) -> L4penR {
        L4penR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - L4 Filter SPM"]
    #[inline(always)]
    pub fn l4spm(&self) -> L4spmR {
        L4spmR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - L4 Filter SPIM"]
    #[inline(always)]
    pub fn l4spim(&self) -> L4spimR {
        L4spimR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - L4 Filter DPM"]
    #[inline(always)]
    pub fn l4dpm(&self) -> L4dpmR {
        L4dpmR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - L4 Filter DPIM"]
    #[inline(always)]
    pub fn l4dpim(&self) -> L4dpimR {
        L4dpimR::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - L3 Filter PEN"]
    #[inline(always)]
    #[must_use]
    pub fn l3pen(&mut self) -> L3penW<L3l4CtrlSpec> {
        L3penW::new(self, 0)
    }
    #[doc = "Bit 2 - L3 Filter SAM"]
    #[inline(always)]
    #[must_use]
    pub fn l3sam(&mut self) -> L3samW<L3l4CtrlSpec> {
        L3samW::new(self, 2)
    }
    #[doc = "Bit 3 - L3 Filter SAIM"]
    #[inline(always)]
    #[must_use]
    pub fn l3saim(&mut self) -> L3saimW<L3l4CtrlSpec> {
        L3saimW::new(self, 3)
    }
    #[doc = "Bit 4 - L3 Filter DAM"]
    #[inline(always)]
    #[must_use]
    pub fn l3dam(&mut self) -> L3damW<L3l4CtrlSpec> {
        L3damW::new(self, 4)
    }
    #[doc = "Bit 5 - L3 Filter DAIM"]
    #[inline(always)]
    #[must_use]
    pub fn l3daim(&mut self) -> L3daimW<L3l4CtrlSpec> {
        L3daimW::new(self, 5)
    }
    #[doc = "Bit 16 - L4 Filter PEN"]
    #[inline(always)]
    #[must_use]
    pub fn l4pen(&mut self) -> L4penW<L3l4CtrlSpec> {
        L4penW::new(self, 16)
    }
    #[doc = "Bit 18 - L4 Filter SPM"]
    #[inline(always)]
    #[must_use]
    pub fn l4spm(&mut self) -> L4spmW<L3l4CtrlSpec> {
        L4spmW::new(self, 18)
    }
    #[doc = "Bit 19 - L4 Filter SPIM"]
    #[inline(always)]
    #[must_use]
    pub fn l4spim(&mut self) -> L4spimW<L3l4CtrlSpec> {
        L4spimW::new(self, 19)
    }
    #[doc = "Bit 20 - L4 Filter DPM"]
    #[inline(always)]
    #[must_use]
    pub fn l4dpm(&mut self) -> L4dpmW<L3l4CtrlSpec> {
        L4dpmW::new(self, 20)
    }
    #[doc = "Bit 21 - L4 Filter DPIM"]
    #[inline(always)]
    #[must_use]
    pub fn l4dpim(&mut self) -> L4dpimW<L3l4CtrlSpec> {
        L4dpimW::new(self, 21)
    }
}
#[doc = "L3/L4 Filter Control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l3l4_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l3l4_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L3l4CtrlSpec;
impl crate::RegisterSpec for L3l4CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l3l4_ctrl::R`](R) reader structure"]
impl crate::Readable for L3l4CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`l3l4_ctrl::W`](W) writer structure"]
impl crate::Writable for L3l4CtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets l3l4_ctrl to value 0"]
impl crate::Resettable for L3l4CtrlSpec {
    const RESET_VALUE: u32 = 0;
}
