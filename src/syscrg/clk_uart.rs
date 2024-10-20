#[repr(C)]
#[doc = "Clock UART"]
#[doc(alias = "clk_uart")]
pub struct ClkUart {
    uart02: [Uart02; 3],
    uart35: [Uart35; 3],
}
impl ClkUart {
    #[doc = "0x00..0x18 - Clock UART U0-U2"]
    #[inline(always)]
    pub const fn uart02(&self, n: usize) -> &Uart02 {
        &self.uart02[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x18 - Clock UART U0-U2"]
    #[inline(always)]
    pub fn uart02_iter(&self) -> impl Iterator<Item = &Uart02> {
        self.uart02.iter()
    }
    #[doc = "0x00..0x08 - Clock UART U0-U2"]
    #[inline(always)]
    pub const fn uart02_u0(&self) -> &Uart02 {
        self.uart02(0)
    }
    #[doc = "0x08..0x10 - Clock UART U0-U2"]
    #[inline(always)]
    pub const fn uart02_u1(&self) -> &Uart02 {
        self.uart02(1)
    }
    #[doc = "0x10..0x18 - Clock UART U0-U2"]
    #[inline(always)]
    pub const fn uart02_u2(&self) -> &Uart02 {
        self.uart02(2)
    }
    #[doc = "0x18..0x30 - Clock UART U3-U5"]
    #[inline(always)]
    pub const fn uart35(&self, n: usize) -> &Uart35 {
        &self.uart35[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x18..0x30 - Clock UART U3-U5"]
    #[inline(always)]
    pub fn uart35_iter(&self) -> impl Iterator<Item = &Uart35> {
        self.uart35.iter()
    }
    #[doc = "0x18..0x20 - Clock UART U3-U5"]
    #[inline(always)]
    pub const fn uart35_u3(&self) -> &Uart35 {
        self.uart35(0)
    }
    #[doc = "0x20..0x28 - Clock UART U3-U5"]
    #[inline(always)]
    pub const fn uart35_u4(&self) -> &Uart35 {
        self.uart35(1)
    }
    #[doc = "0x28..0x30 - Clock UART U3-U5"]
    #[inline(always)]
    pub const fn uart35_u5(&self) -> &Uart35 {
        self.uart35(2)
    }
}
#[doc = "Clock UART U0-U2"]
pub use self::uart02::Uart02;
#[doc = r"Cluster"]
#[doc = "Clock UART U0-U2"]
pub mod uart02;
#[doc = "Clock UART U3-U5"]
pub use self::uart35::Uart35;
#[doc = r"Cluster"]
#[doc = "Clock UART U3-U5"]
pub mod uart35;
