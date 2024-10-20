#[doc = "Register `int_status` reader"]
pub type R = crate::R<IntStatusSpec>;
#[doc = "Register `int_status` writer"]
pub type W = crate::W<IntStatusSpec>;
#[doc = "Field `queue(0-31)` reader - MTL Interrupt Queue"]
pub type QueueR = crate::BitReader;
#[doc = "Field `queue(0-31)` writer - MTL Interrupt Queue"]
pub type QueueW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "MTL Interrupt Queue"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `queue0` field"]
    #[inline(always)]
    pub fn queue(&self, n: u8) -> QueueR {
        #[allow(clippy::no_effect)]
        [(); 32][n as usize];
        QueueR::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "MTL Interrupt Queue"]
    #[inline(always)]
    pub fn queue_iter(&self) -> impl Iterator<Item = QueueR> + '_ {
        (0..32).map(move |n| QueueR::new(((self.bits >> n) & 1) != 0))
    }
    #[doc = "Bit 0 - MTL Interrupt Queue"]
    #[inline(always)]
    pub fn queue0(&self) -> QueueR {
        QueueR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MTL Interrupt Queue"]
    #[inline(always)]
    pub fn queue1(&self) -> QueueR {
        QueueR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - MTL Interrupt Queue"]
    #[inline(always)]
    pub fn queue2(&self) -> QueueR {
        QueueR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - MTL Interrupt Queue"]
    #[inline(always)]
    pub fn queue3(&self) -> QueueR {
        QueueR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - MTL Interrupt Queue"]
    #[inline(always)]
    pub fn queue4(&self) -> QueueR {
        QueueR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - MTL Interrupt Queue"]
    #[inline(always)]
    pub fn queue5(&self) -> QueueR {
        QueueR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - MTL Interrupt Queue"]
    #[inline(always)]
    pub fn queue6(&self) -> QueueR {
        QueueR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - MTL Interrupt Queue"]
    #[inline(always)]
    pub fn queue7(&self) -> QueueR {
        QueueR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - MTL Interrupt Queue"]
    #[inline(always)]
    pub fn queue8(&self) -> QueueR {
        QueueR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - MTL Interrupt Queue"]
    #[inline(always)]
    pub fn queue9(&self) -> QueueR {
        QueueR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - MTL Interrupt Queue"]
    #[inline(always)]
    pub fn queue10(&self) -> QueueR {
        QueueR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - MTL Interrupt Queue"]
    #[inline(always)]
    pub fn queue11(&self) -> QueueR {
        QueueR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - MTL Interrupt Queue"]
    #[inline(always)]
    pub fn queue12(&self) -> QueueR {
        QueueR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - MTL Interrupt Queue"]
    #[inline(always)]
    pub fn queue13(&self) -> QueueR {
        QueueR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - MTL Interrupt Queue"]
    #[inline(always)]
    pub fn queue14(&self) -> QueueR {
        QueueR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - MTL Interrupt Queue"]
    #[inline(always)]
    pub fn queue15(&self) -> QueueR {
        QueueR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - MTL Interrupt Queue"]
    #[inline(always)]
    pub fn queue16(&self) -> QueueR {
        QueueR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - MTL Interrupt Queue"]
    #[inline(always)]
    pub fn queue17(&self) -> QueueR {
        QueueR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - MTL Interrupt Queue"]
    #[inline(always)]
    pub fn queue18(&self) -> QueueR {
        QueueR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - MTL Interrupt Queue"]
    #[inline(always)]
    pub fn queue19(&self) -> QueueR {
        QueueR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - MTL Interrupt Queue"]
    #[inline(always)]
    pub fn queue20(&self) -> QueueR {
        QueueR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - MTL Interrupt Queue"]
    #[inline(always)]
    pub fn queue21(&self) -> QueueR {
        QueueR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - MTL Interrupt Queue"]
    #[inline(always)]
    pub fn queue22(&self) -> QueueR {
        QueueR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - MTL Interrupt Queue"]
    #[inline(always)]
    pub fn queue23(&self) -> QueueR {
        QueueR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - MTL Interrupt Queue"]
    #[inline(always)]
    pub fn queue24(&self) -> QueueR {
        QueueR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - MTL Interrupt Queue"]
    #[inline(always)]
    pub fn queue25(&self) -> QueueR {
        QueueR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - MTL Interrupt Queue"]
    #[inline(always)]
    pub fn queue26(&self) -> QueueR {
        QueueR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - MTL Interrupt Queue"]
    #[inline(always)]
    pub fn queue27(&self) -> QueueR {
        QueueR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - MTL Interrupt Queue"]
    #[inline(always)]
    pub fn queue28(&self) -> QueueR {
        QueueR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - MTL Interrupt Queue"]
    #[inline(always)]
    pub fn queue29(&self) -> QueueR {
        QueueR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - MTL Interrupt Queue"]
    #[inline(always)]
    pub fn queue30(&self) -> QueueR {
        QueueR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - MTL Interrupt Queue"]
    #[inline(always)]
    pub fn queue31(&self) -> QueueR {
        QueueR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "MTL Interrupt Queue"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `queue0` field"]
    #[inline(always)]
    #[must_use]
    pub fn queue(&mut self, n: u8) -> QueueW<IntStatusSpec> {
        #[allow(clippy::no_effect)]
        [(); 32][n as usize];
        QueueW::new(self, n)
    }
    #[doc = "Bit 0 - MTL Interrupt Queue"]
    #[inline(always)]
    #[must_use]
    pub fn queue0(&mut self) -> QueueW<IntStatusSpec> {
        QueueW::new(self, 0)
    }
    #[doc = "Bit 1 - MTL Interrupt Queue"]
    #[inline(always)]
    #[must_use]
    pub fn queue1(&mut self) -> QueueW<IntStatusSpec> {
        QueueW::new(self, 1)
    }
    #[doc = "Bit 2 - MTL Interrupt Queue"]
    #[inline(always)]
    #[must_use]
    pub fn queue2(&mut self) -> QueueW<IntStatusSpec> {
        QueueW::new(self, 2)
    }
    #[doc = "Bit 3 - MTL Interrupt Queue"]
    #[inline(always)]
    #[must_use]
    pub fn queue3(&mut self) -> QueueW<IntStatusSpec> {
        QueueW::new(self, 3)
    }
    #[doc = "Bit 4 - MTL Interrupt Queue"]
    #[inline(always)]
    #[must_use]
    pub fn queue4(&mut self) -> QueueW<IntStatusSpec> {
        QueueW::new(self, 4)
    }
    #[doc = "Bit 5 - MTL Interrupt Queue"]
    #[inline(always)]
    #[must_use]
    pub fn queue5(&mut self) -> QueueW<IntStatusSpec> {
        QueueW::new(self, 5)
    }
    #[doc = "Bit 6 - MTL Interrupt Queue"]
    #[inline(always)]
    #[must_use]
    pub fn queue6(&mut self) -> QueueW<IntStatusSpec> {
        QueueW::new(self, 6)
    }
    #[doc = "Bit 7 - MTL Interrupt Queue"]
    #[inline(always)]
    #[must_use]
    pub fn queue7(&mut self) -> QueueW<IntStatusSpec> {
        QueueW::new(self, 7)
    }
    #[doc = "Bit 8 - MTL Interrupt Queue"]
    #[inline(always)]
    #[must_use]
    pub fn queue8(&mut self) -> QueueW<IntStatusSpec> {
        QueueW::new(self, 8)
    }
    #[doc = "Bit 9 - MTL Interrupt Queue"]
    #[inline(always)]
    #[must_use]
    pub fn queue9(&mut self) -> QueueW<IntStatusSpec> {
        QueueW::new(self, 9)
    }
    #[doc = "Bit 10 - MTL Interrupt Queue"]
    #[inline(always)]
    #[must_use]
    pub fn queue10(&mut self) -> QueueW<IntStatusSpec> {
        QueueW::new(self, 10)
    }
    #[doc = "Bit 11 - MTL Interrupt Queue"]
    #[inline(always)]
    #[must_use]
    pub fn queue11(&mut self) -> QueueW<IntStatusSpec> {
        QueueW::new(self, 11)
    }
    #[doc = "Bit 12 - MTL Interrupt Queue"]
    #[inline(always)]
    #[must_use]
    pub fn queue12(&mut self) -> QueueW<IntStatusSpec> {
        QueueW::new(self, 12)
    }
    #[doc = "Bit 13 - MTL Interrupt Queue"]
    #[inline(always)]
    #[must_use]
    pub fn queue13(&mut self) -> QueueW<IntStatusSpec> {
        QueueW::new(self, 13)
    }
    #[doc = "Bit 14 - MTL Interrupt Queue"]
    #[inline(always)]
    #[must_use]
    pub fn queue14(&mut self) -> QueueW<IntStatusSpec> {
        QueueW::new(self, 14)
    }
    #[doc = "Bit 15 - MTL Interrupt Queue"]
    #[inline(always)]
    #[must_use]
    pub fn queue15(&mut self) -> QueueW<IntStatusSpec> {
        QueueW::new(self, 15)
    }
    #[doc = "Bit 16 - MTL Interrupt Queue"]
    #[inline(always)]
    #[must_use]
    pub fn queue16(&mut self) -> QueueW<IntStatusSpec> {
        QueueW::new(self, 16)
    }
    #[doc = "Bit 17 - MTL Interrupt Queue"]
    #[inline(always)]
    #[must_use]
    pub fn queue17(&mut self) -> QueueW<IntStatusSpec> {
        QueueW::new(self, 17)
    }
    #[doc = "Bit 18 - MTL Interrupt Queue"]
    #[inline(always)]
    #[must_use]
    pub fn queue18(&mut self) -> QueueW<IntStatusSpec> {
        QueueW::new(self, 18)
    }
    #[doc = "Bit 19 - MTL Interrupt Queue"]
    #[inline(always)]
    #[must_use]
    pub fn queue19(&mut self) -> QueueW<IntStatusSpec> {
        QueueW::new(self, 19)
    }
    #[doc = "Bit 20 - MTL Interrupt Queue"]
    #[inline(always)]
    #[must_use]
    pub fn queue20(&mut self) -> QueueW<IntStatusSpec> {
        QueueW::new(self, 20)
    }
    #[doc = "Bit 21 - MTL Interrupt Queue"]
    #[inline(always)]
    #[must_use]
    pub fn queue21(&mut self) -> QueueW<IntStatusSpec> {
        QueueW::new(self, 21)
    }
    #[doc = "Bit 22 - MTL Interrupt Queue"]
    #[inline(always)]
    #[must_use]
    pub fn queue22(&mut self) -> QueueW<IntStatusSpec> {
        QueueW::new(self, 22)
    }
    #[doc = "Bit 23 - MTL Interrupt Queue"]
    #[inline(always)]
    #[must_use]
    pub fn queue23(&mut self) -> QueueW<IntStatusSpec> {
        QueueW::new(self, 23)
    }
    #[doc = "Bit 24 - MTL Interrupt Queue"]
    #[inline(always)]
    #[must_use]
    pub fn queue24(&mut self) -> QueueW<IntStatusSpec> {
        QueueW::new(self, 24)
    }
    #[doc = "Bit 25 - MTL Interrupt Queue"]
    #[inline(always)]
    #[must_use]
    pub fn queue25(&mut self) -> QueueW<IntStatusSpec> {
        QueueW::new(self, 25)
    }
    #[doc = "Bit 26 - MTL Interrupt Queue"]
    #[inline(always)]
    #[must_use]
    pub fn queue26(&mut self) -> QueueW<IntStatusSpec> {
        QueueW::new(self, 26)
    }
    #[doc = "Bit 27 - MTL Interrupt Queue"]
    #[inline(always)]
    #[must_use]
    pub fn queue27(&mut self) -> QueueW<IntStatusSpec> {
        QueueW::new(self, 27)
    }
    #[doc = "Bit 28 - MTL Interrupt Queue"]
    #[inline(always)]
    #[must_use]
    pub fn queue28(&mut self) -> QueueW<IntStatusSpec> {
        QueueW::new(self, 28)
    }
    #[doc = "Bit 29 - MTL Interrupt Queue"]
    #[inline(always)]
    #[must_use]
    pub fn queue29(&mut self) -> QueueW<IntStatusSpec> {
        QueueW::new(self, 29)
    }
    #[doc = "Bit 30 - MTL Interrupt Queue"]
    #[inline(always)]
    #[must_use]
    pub fn queue30(&mut self) -> QueueW<IntStatusSpec> {
        QueueW::new(self, 30)
    }
    #[doc = "Bit 31 - MTL Interrupt Queue"]
    #[inline(always)]
    #[must_use]
    pub fn queue31(&mut self) -> QueueW<IntStatusSpec> {
        QueueW::new(self, 31)
    }
}
#[doc = "MTL Interrupt Status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntStatusSpec;
impl crate::RegisterSpec for IntStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_status::R`](R) reader structure"]
impl crate::Readable for IntStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`int_status::W`](W) writer structure"]
impl crate::Writable for IntStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets int_status to value 0"]
impl crate::Resettable for IntStatusSpec {
    const RESET_VALUE: u32 = 0;
}
