#[doc = "Register `pmt` reader"]
pub type R = crate::R<PmtSpec>;
#[doc = "Register `pmt` writer"]
pub type W = crate::W<PmtSpec>;
#[doc = "PMT Power Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum PowerEvent {
    #[doc = "2147483648: Pointer reset"]
    PointerReset = 2147483648,
    #[doc = "512: Global Unicast"]
    GlobalUnicast = 512,
    #[doc = "64: Wake-up Receive Frame"]
    WakeUpRxFrame = 64,
    #[doc = "32: Magic Frame"]
    MagicFrame = 32,
    #[doc = "4: Wake-up Frame Enable"]
    WakeUpFrameEn = 4,
    #[doc = "2: Magic Packet Enable"]
    MagicPktEn = 2,
    #[doc = "1: Power Down"]
    PowerDown = 1,
}
impl From<PowerEvent> for u32 {
    #[inline(always)]
    fn from(variant: PowerEvent) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PowerEvent {
    type Ux = u32;
}
#[doc = "Field `power_event` reader - PMT Power Event"]
pub type PowerEventR = crate::FieldReader<PowerEvent>;
impl PowerEventR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PowerEvent> {
        match self.bits {
            2147483648 => Some(PowerEvent::PointerReset),
            512 => Some(PowerEvent::GlobalUnicast),
            64 => Some(PowerEvent::WakeUpRxFrame),
            32 => Some(PowerEvent::MagicFrame),
            4 => Some(PowerEvent::WakeUpFrameEn),
            2 => Some(PowerEvent::MagicPktEn),
            1 => Some(PowerEvent::PowerDown),
            _ => None,
        }
    }
    #[doc = "Pointer reset"]
    #[inline(always)]
    pub fn is_pointer_reset(&self) -> bool {
        *self == PowerEvent::PointerReset
    }
    #[doc = "Global Unicast"]
    #[inline(always)]
    pub fn is_global_unicast(&self) -> bool {
        *self == PowerEvent::GlobalUnicast
    }
    #[doc = "Wake-up Receive Frame"]
    #[inline(always)]
    pub fn is_wake_up_rx_frame(&self) -> bool {
        *self == PowerEvent::WakeUpRxFrame
    }
    #[doc = "Magic Frame"]
    #[inline(always)]
    pub fn is_magic_frame(&self) -> bool {
        *self == PowerEvent::MagicFrame
    }
    #[doc = "Wake-up Frame Enable"]
    #[inline(always)]
    pub fn is_wake_up_frame_en(&self) -> bool {
        *self == PowerEvent::WakeUpFrameEn
    }
    #[doc = "Magic Packet Enable"]
    #[inline(always)]
    pub fn is_magic_pkt_en(&self) -> bool {
        *self == PowerEvent::MagicPktEn
    }
    #[doc = "Power Down"]
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        *self == PowerEvent::PowerDown
    }
}
#[doc = "Field `power_event` writer - PMT Power Event"]
pub type PowerEventW<'a, REG> = crate::FieldWriter<'a, REG, 32, PowerEvent>;
impl<'a, REG> PowerEventW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u32>,
{
    #[doc = "Pointer reset"]
    #[inline(always)]
    pub fn pointer_reset(self) -> &'a mut crate::W<REG> {
        self.variant(PowerEvent::PointerReset)
    }
    #[doc = "Global Unicast"]
    #[inline(always)]
    pub fn global_unicast(self) -> &'a mut crate::W<REG> {
        self.variant(PowerEvent::GlobalUnicast)
    }
    #[doc = "Wake-up Receive Frame"]
    #[inline(always)]
    pub fn wake_up_rx_frame(self) -> &'a mut crate::W<REG> {
        self.variant(PowerEvent::WakeUpRxFrame)
    }
    #[doc = "Magic Frame"]
    #[inline(always)]
    pub fn magic_frame(self) -> &'a mut crate::W<REG> {
        self.variant(PowerEvent::MagicFrame)
    }
    #[doc = "Wake-up Frame Enable"]
    #[inline(always)]
    pub fn wake_up_frame_en(self) -> &'a mut crate::W<REG> {
        self.variant(PowerEvent::WakeUpFrameEn)
    }
    #[doc = "Magic Packet Enable"]
    #[inline(always)]
    pub fn magic_pkt_en(self) -> &'a mut crate::W<REG> {
        self.variant(PowerEvent::MagicPktEn)
    }
    #[doc = "Power Down"]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut crate::W<REG> {
        self.variant(PowerEvent::PowerDown)
    }
}
impl R {
    #[doc = "Bits 0:31 - PMT Power Event"]
    #[inline(always)]
    pub fn power_event(&self) -> PowerEventR {
        PowerEventR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - PMT Power Event"]
    #[inline(always)]
    #[must_use]
    pub fn power_event(&mut self) -> PowerEventW<PmtSpec> {
        PowerEventW::new(self, 0)
    }
}
#[doc = "PMT Control and Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmtSpec;
impl crate::RegisterSpec for PmtSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmt::R`](R) reader structure"]
impl crate::Readable for PmtSpec {}
#[doc = "`write(|w| ..)` method takes [`pmt::W`](W) writer structure"]
impl crate::Writable for PmtSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets pmt to value 0"]
impl crate::Resettable for PmtSpec {
    const RESET_VALUE: u32 = 0;
}
