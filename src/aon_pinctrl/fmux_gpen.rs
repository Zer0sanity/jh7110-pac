#[doc = "Register `fmux_gpen` reader"]
pub type R = crate::R<FmuxGpenSpec>;
#[doc = "Register `fmux_gpen` writer"]
pub type W = crate::W<FmuxGpenSpec>;
#[doc = "Field `gpen0` reader - Enable GPIO IRQ function."]
pub type Gpen0R = crate::BitReader;
#[doc = "Field `gpen0` writer - Enable GPIO IRQ function."]
pub type Gpen0W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable GPIO IRQ function."]
    #[inline(always)]
    pub fn gpen0(&self) -> Gpen0R {
        Gpen0R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable GPIO IRQ function."]
    #[inline(always)]
    #[must_use]
    pub fn gpen0(&mut self) -> Gpen0W<FmuxGpenSpec> {
        Gpen0W::new(self, 0)
    }
}
#[doc = "Enable always-on GPIO IRQ function.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fmux_gpen::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fmux_gpen::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FmuxGpenSpec;
impl crate::RegisterSpec for FmuxGpenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fmux_gpen::R`](R) reader structure"]
impl crate::Readable for FmuxGpenSpec {}
#[doc = "`write(|w| ..)` method takes [`fmux_gpen::W`](W) writer structure"]
impl crate::Writable for FmuxGpenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets fmux_gpen to value 0"]
impl crate::Resettable for FmuxGpenSpec {
    const RESET_VALUE: u32 = 0;
}
