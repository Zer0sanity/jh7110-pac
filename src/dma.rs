#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    dmac: Dmac,
    ch: [Ch; 32],
}
impl RegisterBlock {
    #[doc = "0x00..0x100 - DesignWare DMAC registers"]
    #[inline(always)]
    pub const fn dmac(&self) -> &Dmac {
        &self.dmac
    }
    #[doc = "0x100..0x2100 - DesignWare DMAC Channel registers"]
    #[inline(always)]
    pub const fn ch(&self, n: usize) -> &Ch {
        &self.ch[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x100..0x2100 - DesignWare DMAC Channel registers"]
    #[inline(always)]
    pub fn ch_iter(&self) -> impl Iterator<Item = &Ch> {
        self.ch.iter()
    }
    #[doc = "0x100..0x200 - DesignWare DMAC Channel registers"]
    #[inline(always)]
    pub const fn ch1(&self) -> &Ch {
        self.ch(0)
    }
    #[doc = "0x200..0x300 - DesignWare DMAC Channel registers"]
    #[inline(always)]
    pub const fn ch2(&self) -> &Ch {
        self.ch(1)
    }
    #[doc = "0x300..0x400 - DesignWare DMAC Channel registers"]
    #[inline(always)]
    pub const fn ch3(&self) -> &Ch {
        self.ch(2)
    }
    #[doc = "0x400..0x500 - DesignWare DMAC Channel registers"]
    #[inline(always)]
    pub const fn ch4(&self) -> &Ch {
        self.ch(3)
    }
    #[doc = "0x500..0x600 - DesignWare DMAC Channel registers"]
    #[inline(always)]
    pub const fn ch5(&self) -> &Ch {
        self.ch(4)
    }
    #[doc = "0x600..0x700 - DesignWare DMAC Channel registers"]
    #[inline(always)]
    pub const fn ch6(&self) -> &Ch {
        self.ch(5)
    }
    #[doc = "0x700..0x800 - DesignWare DMAC Channel registers"]
    #[inline(always)]
    pub const fn ch7(&self) -> &Ch {
        self.ch(6)
    }
    #[doc = "0x800..0x900 - DesignWare DMAC Channel registers"]
    #[inline(always)]
    pub const fn ch8(&self) -> &Ch {
        self.ch(7)
    }
    #[doc = "0x900..0xa00 - DesignWare DMAC Channel registers"]
    #[inline(always)]
    pub const fn ch9(&self) -> &Ch {
        self.ch(8)
    }
    #[doc = "0xa00..0xb00 - DesignWare DMAC Channel registers"]
    #[inline(always)]
    pub const fn ch10(&self) -> &Ch {
        self.ch(9)
    }
    #[doc = "0xb00..0xc00 - DesignWare DMAC Channel registers"]
    #[inline(always)]
    pub const fn ch11(&self) -> &Ch {
        self.ch(10)
    }
    #[doc = "0xc00..0xd00 - DesignWare DMAC Channel registers"]
    #[inline(always)]
    pub const fn ch12(&self) -> &Ch {
        self.ch(11)
    }
    #[doc = "0xd00..0xe00 - DesignWare DMAC Channel registers"]
    #[inline(always)]
    pub const fn ch13(&self) -> &Ch {
        self.ch(12)
    }
    #[doc = "0xe00..0xf00 - DesignWare DMAC Channel registers"]
    #[inline(always)]
    pub const fn ch14(&self) -> &Ch {
        self.ch(13)
    }
    #[doc = "0xf00..0x1000 - DesignWare DMAC Channel registers"]
    #[inline(always)]
    pub const fn ch15(&self) -> &Ch {
        self.ch(14)
    }
    #[doc = "0x1000..0x1100 - DesignWare DMAC Channel registers"]
    #[inline(always)]
    pub const fn ch16(&self) -> &Ch {
        self.ch(15)
    }
    #[doc = "0x1100..0x1200 - DesignWare DMAC Channel registers"]
    #[inline(always)]
    pub const fn ch17(&self) -> &Ch {
        self.ch(16)
    }
    #[doc = "0x1200..0x1300 - DesignWare DMAC Channel registers"]
    #[inline(always)]
    pub const fn ch18(&self) -> &Ch {
        self.ch(17)
    }
    #[doc = "0x1300..0x1400 - DesignWare DMAC Channel registers"]
    #[inline(always)]
    pub const fn ch19(&self) -> &Ch {
        self.ch(18)
    }
    #[doc = "0x1400..0x1500 - DesignWare DMAC Channel registers"]
    #[inline(always)]
    pub const fn ch20(&self) -> &Ch {
        self.ch(19)
    }
    #[doc = "0x1500..0x1600 - DesignWare DMAC Channel registers"]
    #[inline(always)]
    pub const fn ch21(&self) -> &Ch {
        self.ch(20)
    }
    #[doc = "0x1600..0x1700 - DesignWare DMAC Channel registers"]
    #[inline(always)]
    pub const fn ch22(&self) -> &Ch {
        self.ch(21)
    }
    #[doc = "0x1700..0x1800 - DesignWare DMAC Channel registers"]
    #[inline(always)]
    pub const fn ch23(&self) -> &Ch {
        self.ch(22)
    }
    #[doc = "0x1800..0x1900 - DesignWare DMAC Channel registers"]
    #[inline(always)]
    pub const fn ch24(&self) -> &Ch {
        self.ch(23)
    }
    #[doc = "0x1900..0x1a00 - DesignWare DMAC Channel registers"]
    #[inline(always)]
    pub const fn ch25(&self) -> &Ch {
        self.ch(24)
    }
    #[doc = "0x1a00..0x1b00 - DesignWare DMAC Channel registers"]
    #[inline(always)]
    pub const fn ch26(&self) -> &Ch {
        self.ch(25)
    }
    #[doc = "0x1b00..0x1c00 - DesignWare DMAC Channel registers"]
    #[inline(always)]
    pub const fn ch27(&self) -> &Ch {
        self.ch(26)
    }
    #[doc = "0x1c00..0x1d00 - DesignWare DMAC Channel registers"]
    #[inline(always)]
    pub const fn ch28(&self) -> &Ch {
        self.ch(27)
    }
    #[doc = "0x1d00..0x1e00 - DesignWare DMAC Channel registers"]
    #[inline(always)]
    pub const fn ch29(&self) -> &Ch {
        self.ch(28)
    }
    #[doc = "0x1e00..0x1f00 - DesignWare DMAC Channel registers"]
    #[inline(always)]
    pub const fn ch30(&self) -> &Ch {
        self.ch(29)
    }
    #[doc = "0x1f00..0x2000 - DesignWare DMAC Channel registers"]
    #[inline(always)]
    pub const fn ch31(&self) -> &Ch {
        self.ch(30)
    }
    #[doc = "0x2000..0x2100 - DesignWare DMAC Channel registers"]
    #[inline(always)]
    pub const fn ch32(&self) -> &Ch {
        self.ch(31)
    }
}
#[doc = "DesignWare DMAC registers"]
pub use self::dmac::Dmac;
#[doc = r"Cluster"]
#[doc = "DesignWare DMAC registers"]
pub mod dmac;
#[doc = "DesignWare DMAC Channel registers"]
pub use self::ch::Ch;
#[doc = r"Cluster"]
#[doc = "DesignWare DMAC Channel registers"]
pub mod ch;
