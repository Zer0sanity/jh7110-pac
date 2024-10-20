#[doc = "Register `sts` reader"]
pub type R = crate::R<StsSpec>;
#[doc = "Register `sts` writer"]
pub type W = crate::W<StsSpec>;
#[doc = "Field `con` reader - SS connection interrupt."]
pub type ConR = crate::BitReader;
#[doc = "Field `con` writer - SS connection interrupt."]
pub type ConW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dis` reader - SS disconnection interrupt."]
pub type DisR = crate::BitReader;
#[doc = "Field `dis` writer - SS disconnection interrupt."]
pub type DisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `uwres` reader - SS warm reset interrupt."]
pub type UwresR = crate::BitReader;
#[doc = "Field `uwres` writer - SS warm reset interrupt."]
pub type UwresW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `uhres` reader - SS hot reset interrupt."]
pub type UhresR = crate::BitReader;
#[doc = "Field `uhres` writer - SS hot reset interrupt."]
pub type UhresW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u3ent` reader - SS link U3 state enter interrupt - suspend."]
pub type U3entR = crate::BitReader;
#[doc = "Field `u3ent` writer - SS link U3 state enter interrupt - suspend."]
pub type U3entW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u3ext` reader - SS link U3 state exit interrupt - wakeup."]
pub type U3extR = crate::BitReader;
#[doc = "Field `u3ext` writer - SS link U3 state exit interrupt - wakeup."]
pub type U3extW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u2ent` reader - SS link U2 state enter interrupt."]
pub type U2entR = crate::BitReader;
#[doc = "Field `u2ent` writer - SS link U2 state enter interrupt."]
pub type U2entW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u2ext` reader - SS link U2 state exit interrupt."]
pub type U2extR = crate::BitReader;
#[doc = "Field `u2ext` writer - SS link U2 state exit interrupt."]
pub type U2extW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u1ent` reader - SS link U1 state enter interrupt."]
pub type U1entR = crate::BitReader;
#[doc = "Field `u1ent` writer - SS link U1 state enter interrupt."]
pub type U1entW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u1ext` reader - SS link U1 state exit interrupt."]
pub type U1extR = crate::BitReader;
#[doc = "Field `u1ext` writer - SS link U1 state exit interrupt."]
pub type U1extW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `itp` reader - ITP/SOF packet detected interrupt."]
pub type ItpR = crate::BitReader;
#[doc = "Field `itp` writer - ITP/SOF packet detected interrupt."]
pub type ItpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `wake` reader - Wakeup interrupt."]
pub type WakeR = crate::BitReader;
#[doc = "Field `wake` writer - Wakeup interrupt."]
pub type WakeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `spkt` reader - Send Custom Packet interrupt."]
pub type SpktR = crate::BitReader;
#[doc = "Field `spkt` writer - Send Custom Packet interrupt."]
pub type SpktW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `con2` reader - FS/HS mode connection interrupt."]
pub type Con2R = crate::BitReader;
#[doc = "Field `con2` writer - FS/HS mode connection interrupt."]
pub type Con2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `dis2` reader - FS/HS mode disconnection interrupt."]
pub type Dis2R = crate::BitReader;
#[doc = "Field `dis2` writer - FS/HS mode disconnection interrupt."]
pub type Dis2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `u2res` reader - USB reset interrupt - FS/HS mode."]
pub type U2resR = crate::BitReader;
#[doc = "Field `u2res` writer - USB reset interrupt - FS/HS mode."]
pub type U2resW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `l2ent` reader - LPM L2 state enter interrupt."]
pub type L2entR = crate::BitReader;
#[doc = "Field `l2ent` writer - LPM L2 state enter interrupt."]
pub type L2entW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `l2ext` reader - LPM L2 state exit interrupt."]
pub type L2extR = crate::BitReader;
#[doc = "Field `l2ext` writer - LPM L2 state exit interrupt."]
pub type L2extW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `l1ent` reader - LPM L1 state enter interrupt."]
pub type L1entR = crate::BitReader;
#[doc = "Field `l1ent` writer - LPM L1 state enter interrupt."]
pub type L1entW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `l1ext` reader - LPM L1 state exit interrupt."]
pub type L1extR = crate::BitReader;
#[doc = "Field `l1ext` writer - LPM L1 state exit interrupt."]
pub type L1extW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `cfgres` reader - Configuration reset interrupt."]
pub type CfgresR = crate::BitReader;
#[doc = "Field `cfgres` writer - Configuration reset interrupt."]
pub type CfgresW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `uwress` reader - Start of the USB SS warm reset interrupt."]
pub type UwressR = crate::BitReader;
#[doc = "Field `uwress` writer - Start of the USB SS warm reset interrupt."]
pub type UwressW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `uwrese` reader - End of the USB SS warm reset interrupt."]
pub type UwreseR = crate::BitReader;
#[doc = "Field `uwrese` writer - End of the USB SS warm reset interrupt."]
pub type UwreseW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SS connection interrupt."]
    #[inline(always)]
    pub fn con(&self) -> ConR {
        ConR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SS disconnection interrupt."]
    #[inline(always)]
    pub fn dis(&self) -> DisR {
        DisR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SS warm reset interrupt."]
    #[inline(always)]
    pub fn uwres(&self) -> UwresR {
        UwresR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SS hot reset interrupt."]
    #[inline(always)]
    pub fn uhres(&self) -> UhresR {
        UhresR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SS link U3 state enter interrupt - suspend."]
    #[inline(always)]
    pub fn u3ent(&self) -> U3entR {
        U3entR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SS link U3 state exit interrupt - wakeup."]
    #[inline(always)]
    pub fn u3ext(&self) -> U3extR {
        U3extR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SS link U2 state enter interrupt."]
    #[inline(always)]
    pub fn u2ent(&self) -> U2entR {
        U2entR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SS link U2 state exit interrupt."]
    #[inline(always)]
    pub fn u2ext(&self) -> U2extR {
        U2extR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - SS link U1 state enter interrupt."]
    #[inline(always)]
    pub fn u1ent(&self) -> U1entR {
        U1entR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SS link U1 state exit interrupt."]
    #[inline(always)]
    pub fn u1ext(&self) -> U1extR {
        U1extR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - ITP/SOF packet detected interrupt."]
    #[inline(always)]
    pub fn itp(&self) -> ItpR {
        ItpR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Wakeup interrupt."]
    #[inline(always)]
    pub fn wake(&self) -> WakeR {
        WakeR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Send Custom Packet interrupt."]
    #[inline(always)]
    pub fn spkt(&self) -> SpktR {
        SpktR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - FS/HS mode connection interrupt."]
    #[inline(always)]
    pub fn con2(&self) -> Con2R {
        Con2R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - FS/HS mode disconnection interrupt."]
    #[inline(always)]
    pub fn dis2(&self) -> Dis2R {
        Dis2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - USB reset interrupt - FS/HS mode."]
    #[inline(always)]
    pub fn u2res(&self) -> U2resR {
        U2resR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - LPM L2 state enter interrupt."]
    #[inline(always)]
    pub fn l2ent(&self) -> L2entR {
        L2entR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - LPM L2 state exit interrupt."]
    #[inline(always)]
    pub fn l2ext(&self) -> L2extR {
        L2extR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 24 - LPM L1 state enter interrupt."]
    #[inline(always)]
    pub fn l1ent(&self) -> L1entR {
        L1entR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - LPM L1 state exit interrupt."]
    #[inline(always)]
    pub fn l1ext(&self) -> L1extR {
        L1extR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Configuration reset interrupt."]
    #[inline(always)]
    pub fn cfgres(&self) -> CfgresR {
        CfgresR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - Start of the USB SS warm reset interrupt."]
    #[inline(always)]
    pub fn uwress(&self) -> UwressR {
        UwressR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - End of the USB SS warm reset interrupt."]
    #[inline(always)]
    pub fn uwrese(&self) -> UwreseR {
        UwreseR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SS connection interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn con(&mut self) -> ConW<StsSpec> {
        ConW::new(self, 0)
    }
    #[doc = "Bit 1 - SS disconnection interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn dis(&mut self) -> DisW<StsSpec> {
        DisW::new(self, 1)
    }
    #[doc = "Bit 2 - SS warm reset interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn uwres(&mut self) -> UwresW<StsSpec> {
        UwresW::new(self, 2)
    }
    #[doc = "Bit 3 - SS hot reset interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn uhres(&mut self) -> UhresW<StsSpec> {
        UhresW::new(self, 3)
    }
    #[doc = "Bit 4 - SS link U3 state enter interrupt - suspend."]
    #[inline(always)]
    #[must_use]
    pub fn u3ent(&mut self) -> U3entW<StsSpec> {
        U3entW::new(self, 4)
    }
    #[doc = "Bit 5 - SS link U3 state exit interrupt - wakeup."]
    #[inline(always)]
    #[must_use]
    pub fn u3ext(&mut self) -> U3extW<StsSpec> {
        U3extW::new(self, 5)
    }
    #[doc = "Bit 6 - SS link U2 state enter interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn u2ent(&mut self) -> U2entW<StsSpec> {
        U2entW::new(self, 6)
    }
    #[doc = "Bit 7 - SS link U2 state exit interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn u2ext(&mut self) -> U2extW<StsSpec> {
        U2extW::new(self, 7)
    }
    #[doc = "Bit 8 - SS link U1 state enter interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn u1ent(&mut self) -> U1entW<StsSpec> {
        U1entW::new(self, 8)
    }
    #[doc = "Bit 9 - SS link U1 state exit interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn u1ext(&mut self) -> U1extW<StsSpec> {
        U1extW::new(self, 9)
    }
    #[doc = "Bit 10 - ITP/SOF packet detected interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn itp(&mut self) -> ItpW<StsSpec> {
        ItpW::new(self, 10)
    }
    #[doc = "Bit 11 - Wakeup interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn wake(&mut self) -> WakeW<StsSpec> {
        WakeW::new(self, 11)
    }
    #[doc = "Bit 12 - Send Custom Packet interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn spkt(&mut self) -> SpktW<StsSpec> {
        SpktW::new(self, 12)
    }
    #[doc = "Bit 16 - FS/HS mode connection interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn con2(&mut self) -> Con2W<StsSpec> {
        Con2W::new(self, 16)
    }
    #[doc = "Bit 17 - FS/HS mode disconnection interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn dis2(&mut self) -> Dis2W<StsSpec> {
        Dis2W::new(self, 17)
    }
    #[doc = "Bit 18 - USB reset interrupt - FS/HS mode."]
    #[inline(always)]
    #[must_use]
    pub fn u2res(&mut self) -> U2resW<StsSpec> {
        U2resW::new(self, 18)
    }
    #[doc = "Bit 20 - LPM L2 state enter interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn l2ent(&mut self) -> L2entW<StsSpec> {
        L2entW::new(self, 20)
    }
    #[doc = "Bit 21 - LPM L2 state exit interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn l2ext(&mut self) -> L2extW<StsSpec> {
        L2extW::new(self, 21)
    }
    #[doc = "Bit 24 - LPM L1 state enter interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn l1ent(&mut self) -> L1entW<StsSpec> {
        L1entW::new(self, 24)
    }
    #[doc = "Bit 25 - LPM L1 state exit interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn l1ext(&mut self) -> L1extW<StsSpec> {
        L1extW::new(self, 25)
    }
    #[doc = "Bit 26 - Configuration reset interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn cfgres(&mut self) -> CfgresW<StsSpec> {
        CfgresW::new(self, 26)
    }
    #[doc = "Bit 28 - Start of the USB SS warm reset interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn uwress(&mut self) -> UwressW<StsSpec> {
        UwressW::new(self, 28)
    }
    #[doc = "Bit 29 - End of the USB SS warm reset interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn uwrese(&mut self) -> UwreseW<StsSpec> {
        UwreseW::new(self, 29)
    }
}
#[doc = "Global Interrupt Status.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sts::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sts::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StsSpec;
impl crate::RegisterSpec for StsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sts::R`](R) reader structure"]
impl crate::Readable for StsSpec {}
#[doc = "`write(|w| ..)` method takes [`sts::W`](W) writer structure"]
impl crate::Writable for StsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets sts to value 0"]
impl crate::Resettable for StsSpec {
    const RESET_VALUE: u32 = 0;
}
