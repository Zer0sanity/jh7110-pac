#[doc = "Register `over` reader"]
pub type R = crate::R<OverSpec>;
#[doc = "Register `over` writer"]
pub type W = crate::W<OverSpec>;
#[doc = "Field `idpullup` reader - USB3 OTG override ID pullup pin."]
pub type IdpullupR = crate::BitReader;
#[doc = "Field `idpullup` writer - USB3 OTG override ID pullup pin."]
pub type IdpullupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "USB3 OTG override session valid select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SessionValidSelect {
    #[doc = "0: VBUS session valid select"]
    Vbus = 0,
    #[doc = "1: SES session valid select"]
    Ses = 1,
}
impl From<SessionValidSelect> for bool {
    #[inline(always)]
    fn from(variant: SessionValidSelect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `session_valid_select` reader - USB3 OTG override session valid select."]
pub type SessionValidSelectR = crate::BitReader<SessionValidSelect>;
impl SessionValidSelectR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SessionValidSelect {
        match self.bits {
            false => SessionValidSelect::Vbus,
            true => SessionValidSelect::Ses,
        }
    }
    #[doc = "VBUS session valid select"]
    #[inline(always)]
    pub fn is_vbus(&self) -> bool {
        *self == SessionValidSelect::Vbus
    }
    #[doc = "SES session valid select"]
    #[inline(always)]
    pub fn is_ses(&self) -> bool {
        *self == SessionValidSelect::Ses
    }
}
#[doc = "Field `session_valid_select` writer - USB3 OTG override session valid select."]
pub type SessionValidSelectW<'a, REG> = crate::BitWriter<'a, REG, SessionValidSelect>;
impl<'a, REG> SessionValidSelectW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "VBUS session valid select"]
    #[inline(always)]
    pub fn vbus(self) -> &'a mut crate::W<REG> {
        self.variant(SessionValidSelect::Vbus)
    }
    #[doc = "SES session valid select"]
    #[inline(always)]
    pub fn ses(self) -> &'a mut crate::W<REG> {
        self.variant(SessionValidSelect::Ses)
    }
}
impl R {
    #[doc = "Bit 0 - USB3 OTG override ID pullup pin."]
    #[inline(always)]
    pub fn idpullup(&self) -> IdpullupR {
        IdpullupR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 10 - USB3 OTG override session valid select."]
    #[inline(always)]
    pub fn session_valid_select(&self) -> SessionValidSelectR {
        SessionValidSelectR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USB3 OTG override ID pullup pin."]
    #[inline(always)]
    #[must_use]
    pub fn idpullup(&mut self) -> IdpullupW<OverSpec> {
        IdpullupW::new(self, 0)
    }
    #[doc = "Bit 10 - USB3 OTG override session valid select."]
    #[inline(always)]
    #[must_use]
    pub fn session_valid_select(&mut self) -> SessionValidSelectW<OverSpec> {
        SessionValidSelectW::new(self, 10)
    }
}
#[doc = "USB3 OTG override.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`over::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`over::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OverSpec;
impl crate::RegisterSpec for OverSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`over::R`](R) reader structure"]
impl crate::Readable for OverSpec {}
#[doc = "`write(|w| ..)` method takes [`over::W`](W) writer structure"]
impl crate::Writable for OverSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets over to value 0"]
impl crate::Resettable for OverSpec {
    const RESET_VALUE: u32 = 0;
}
