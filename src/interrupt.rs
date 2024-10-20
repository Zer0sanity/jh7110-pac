#[doc = r"Enumeration of all the interrupts."]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Interrupt {
    #[doc = "1 - L2PM"]
    L2PM = 1,
    #[doc = "2 - L2PM1"]
    L2PM1 = 2,
    #[doc = "3 - L2PM2"]
    L2PM2 = 3,
    #[doc = "4 - L2PM3"]
    L2PM3 = 4,
    #[doc = "5 - ETH_LPI0"]
    ETH_LPI0 = 5,
    #[doc = "6 - ETH_WAKE_IRQ0"]
    ETH_WAKE_IRQ0 = 6,
    #[doc = "7 - MACIRQ0"]
    MACIRQ0 = 7,
    #[doc = "25 - QSPI0"]
    QSPI0 = 25,
    #[doc = "28 - CRYPTO"]
    CRYPTO = 28,
    #[doc = "29 - SDMA"]
    SDMA = 29,
    #[doc = "30 - TRNG"]
    TRNG = 30,
    #[doc = "32 - UART0"]
    UART0 = 32,
    #[doc = "33 - UART1"]
    UART1 = 33,
    #[doc = "34 - UART2"]
    UART2 = 34,
    #[doc = "35 - I2C0"]
    I2C0 = 35,
    #[doc = "36 - I2C1"]
    I2C1 = 36,
    #[doc = "37 - I2C2"]
    I2C2 = 37,
    #[doc = "38 - SPI0"]
    SPI0 = 38,
    #[doc = "39 - SPI1"]
    SPI1 = 39,
    #[doc = "40 - SPI2"]
    SPI2 = 40,
    #[doc = "45 - UART3"]
    UART3 = 45,
    #[doc = "46 - UART4"]
    UART4 = 46,
    #[doc = "47 - UART5"]
    UART5 = 47,
    #[doc = "48 - I2C3"]
    I2C3 = 48,
    #[doc = "49 - I2C4"]
    I2C4 = 49,
    #[doc = "50 - I2C5"]
    I2C5 = 50,
    #[doc = "51 - I2C6"]
    I2C6 = 51,
    #[doc = "52 - SPI3"]
    SPI3 = 52,
    #[doc = "53 - SPI4"]
    SPI4 = 53,
    #[doc = "54 - SPI5"]
    SPI5 = 54,
    #[doc = "55 - SPI6"]
    SPI6 = 55,
    #[doc = "73 - DMA"]
    DMA = 73,
    #[doc = "74 - MMC0"]
    MMC0 = 74,
    #[doc = "75 - MMC1"]
    MMC1 = 75,
    #[doc = "76 - ETH_LPI1"]
    ETH_LPI1 = 76,
    #[doc = "77 - ETH_WAKE_IRQ1"]
    ETH_WAKE_IRQ1 = 77,
    #[doc = "78 - MACIRQ1"]
    MACIRQ1 = 78,
    #[doc = "85 - AON_IOMUX"]
    AON_IOMUX = 85,
    #[doc = "86 - SYS_IOMUX"]
    SYS_IOMUX = 86,
    #[doc = "100 - HOST0"]
    HOST0 = 100,
    #[doc = "108 - PERIPHERAL0"]
    PERIPHERAL0 = 108,
    #[doc = "110 - OTG0"]
    OTG0 = 110,
    #[doc = "111 - PMU"]
    PMU = 111,
}
#[doc = r" TryFromInterruptError"]
#[derive(Debug, Copy, Clone)]
pub struct TryFromInterruptError(());
impl Interrupt {
    #[doc = r" Attempt to convert a given value into an `Interrupt`"]
    #[inline]
    pub fn try_from(value: u8) -> Result<Self, TryFromInterruptError> {
        match value {
            1 => Ok(Interrupt::L2PM),
            2 => Ok(Interrupt::L2PM1),
            3 => Ok(Interrupt::L2PM2),
            4 => Ok(Interrupt::L2PM3),
            5 => Ok(Interrupt::ETH_LPI0),
            6 => Ok(Interrupt::ETH_WAKE_IRQ0),
            7 => Ok(Interrupt::MACIRQ0),
            25 => Ok(Interrupt::QSPI0),
            28 => Ok(Interrupt::CRYPTO),
            29 => Ok(Interrupt::SDMA),
            30 => Ok(Interrupt::TRNG),
            32 => Ok(Interrupt::UART0),
            33 => Ok(Interrupt::UART1),
            34 => Ok(Interrupt::UART2),
            35 => Ok(Interrupt::I2C0),
            36 => Ok(Interrupt::I2C1),
            37 => Ok(Interrupt::I2C2),
            38 => Ok(Interrupt::SPI0),
            39 => Ok(Interrupt::SPI1),
            40 => Ok(Interrupt::SPI2),
            45 => Ok(Interrupt::UART3),
            46 => Ok(Interrupt::UART4),
            47 => Ok(Interrupt::UART5),
            48 => Ok(Interrupt::I2C3),
            49 => Ok(Interrupt::I2C4),
            50 => Ok(Interrupt::I2C5),
            51 => Ok(Interrupt::I2C6),
            52 => Ok(Interrupt::SPI3),
            53 => Ok(Interrupt::SPI4),
            54 => Ok(Interrupt::SPI5),
            55 => Ok(Interrupt::SPI6),
            73 => Ok(Interrupt::DMA),
            74 => Ok(Interrupt::MMC0),
            75 => Ok(Interrupt::MMC1),
            76 => Ok(Interrupt::ETH_LPI1),
            77 => Ok(Interrupt::ETH_WAKE_IRQ1),
            78 => Ok(Interrupt::MACIRQ1),
            85 => Ok(Interrupt::AON_IOMUX),
            86 => Ok(Interrupt::SYS_IOMUX),
            100 => Ok(Interrupt::HOST0),
            108 => Ok(Interrupt::PERIPHERAL0),
            110 => Ok(Interrupt::OTG0),
            111 => Ok(Interrupt::PMU),
            _ => Err(TryFromInterruptError(())),
        }
    }
}
#[cfg(feature = "rt")]
#[macro_export]
#[doc = r" Assigns a handler to an interrupt"]
#[doc = r""]
#[doc = r" This macro takes two arguments: the name of an interrupt and the path to the"]
#[doc = r" function that will be used as the handler of that interrupt. That function"]
#[doc = r" must have signature `fn()`."]
#[doc = r""]
#[doc = r" Optionally, a third argument may be used to declare interrupt local data."]
#[doc = r" The handler will have exclusive access to these *local* variables on each"]
#[doc = r" invocation. If the third argument is used then the signature of the handler"]
#[doc = r" function must be `fn(&mut $NAME::Locals)` where `$NAME` is the first argument"]
#[doc = r" passed to the macro."]
#[doc = r""]
#[doc = r" # Example"]
#[doc = r""]
#[doc = r" ``` ignore"]
#[doc = r" interrupt!(TIM2, periodic);"]
#[doc = r""]
#[doc = r" fn periodic() {"]
#[doc = r#"     print!(".");"#]
#[doc = r" }"]
#[doc = r""]
#[doc = r" interrupt!(TIM3, tick, locals: {"]
#[doc = r"     tick: bool = false;"]
#[doc = r" });"]
#[doc = r""]
#[doc = r" fn tick(locals: &mut TIM3::Locals) {"]
#[doc = r"     locals.tick = !locals.tick;"]
#[doc = r""]
#[doc = r"     if locals.tick {"]
#[doc = r#"         println!("Tick");"#]
#[doc = r"     } else {"]
#[doc = r#"         println!("Tock");"#]
#[doc = r"     }"]
#[doc = r" }"]
#[doc = r" ```"]
macro_rules ! interrupt { ($ NAME : ident , $ path : path , locals : { $ ($ lvar : ident : $ lty : ty = $ lval : expr ;) * }) => { # [allow (non_snake_case)] mod $ NAME { pub struct Locals { $ (pub $ lvar : $ lty ,) * } } # [allow (non_snake_case)] # [no_mangle] pub extern "C" fn $ NAME () { let _ = $ crate :: interrupt :: Interrupt :: $ NAME ; static mut LOCALS : self :: $ NAME :: Locals = self :: $ NAME :: Locals { $ ($ lvar : $ lval ,) * } ; let f : fn (& mut self :: $ NAME :: Locals) = $ path ; f (unsafe { & mut LOCALS }) ; } } ; ($ NAME : ident , $ path : path) => { # [allow (non_snake_case)] # [no_mangle] pub extern "C" fn $ NAME () { let _ = $ crate :: interrupt :: Interrupt :: $ NAME ; let f : fn () = $ path ; f () ; } } }
