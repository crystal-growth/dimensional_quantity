//! Constants for metric and binary prefixes

/// Constants for metric prefixes
pub mod metric {
    pub mod f64 {
        //! Constants for metric prefixes, f64 precision

        /// 10<sup>24</sup>
        pub const YOTTA: f64 = 1.0_E24;
        /// 10<sup>21</sup>
        pub const ZETTA: f64 = 1.0_E21;
        /// 10<sup>18</sup>
        pub const EXA: f64 = 1.0_E18;
        /// 10<sup>15</sup>
        pub const PETA: f64 = 1.0_E15;
        /// 10<sup>12</sup>
        pub const TERA: f64 = 1.0_E12;
        /// 10<sup>9</sup>
        pub const GIGA: f64 = 1.0_E9;
        /// 10<sup>6</sup>
        pub const MEGA: f64 = 1.0_E6;
        /// 10<sup>3</sup>
        pub const KILO: f64 = 1.0_E3;
        /// 10<sup>2</sup>
        pub const HECTO: f64 = 1.0_E2;
        /// 10<sup>1</sup>
        pub const DECA: f64 = 1.0_E1;

        /// 10<sup>-1</sup>
        pub const DECI: f64 = 1.0_E-1;
        /// 10<sup>-2</sup>
        pub const CENTI: f64 = 1.0_E-2;
        /// 10<sup>-3</sup>
        pub const MILLI: f64 = 1.0_E-3;
        /// 10<sup>-6</sup>
        pub const MICRO: f64 = 1.0_E-6;
        /// 10<sup>-9</sup>
        pub const NANO: f64 = 1.0_E-9;
        /// 10<sup>-12</sup>
        pub const PICO: f64 = 1.0_E-12;
        /// 10<sup>-15</sup>
        pub const FEMTO: f64 = 1.0_E-15;
        /// 10<sup>-18</sup>
        pub const ATTO: f64 = 1.0_E-18;
        /// 10<sup>-21</sup>
        pub const ZEPTO: f64 = 1.0_E-21;
        /// 10<sup>-24</sup>
        pub const YOCTO: f64 = 1.0_E-24;
    }
    pub mod f32 {
        //! Constants for metric prefixes, f32 precision

        /// 10<sup>24</sup>
        pub const YOTTA: f32 = 1.0_E24;
        /// 10<sup>21</sup>
        pub const ZETTA: f32 = 1.0_E21;
        /// 10<sup>18</sup>
        pub const EXA: f32 = 1.0_E18;
        /// 10<sup>15</sup>
        pub const PETA: f32 = 1.0_E15;
        /// 10<sup>12</sup>
        pub const TERA: f32 = 1.0_E12;
        /// 10<sup>9</sup>
        pub const GIGA: f32 = 1.0_E9;
        /// 10<sup>6</sup>
        pub const MEGA: f32 = 1.0_E6;
        /// 10<sup>3</sup>
        pub const KILO: f32 = 1.0_E3;
        /// 10<sup>2</sup>
        pub const HECTO: f32 = 1.0_E2;
        /// 10<sup>1</sup>
        pub const DECA: f32 = 1.0_E1;

        /// 10<sup>-1</sup>
        pub const DECI: f32 = 1.0_E-1;
        /// 10<sup>-2</sup>
        pub const CENTI: f32 = 1.0_E-2;
        /// 10<sup>-3</sup>
        pub const MILLI: f32 = 1.0_E-3;
        /// 10<sup>-6</sup>
        pub const MICRO: f32 = 1.0_E-6;
        /// 10<sup>-9</sup>
        pub const NANO: f32 = 1.0_E-9;
        /// 10<sup>-12</sup>
        pub const PICO: f32 = 1.0_E-12;
        /// 10<sup>-15</sup>
        pub const FEMTO: f32 = 1.0_E-15;
        /// 10<sup>-18</sup>
        pub const ATTO: f32 = 1.0_E-18;
        /// 10<sup>-21</sup>
        pub const ZEPTO: f32 = 1.0_E-21;
        /// 10<sup>-24</sup>
        pub const YOCTO: f32 = 1.0_E-24;
    }
}
/// Constants for binary prefixes
pub mod binary {
    /// Constants for binary prefixes, f64 precision
    pub mod f64 {
        /// 2<sup>80</sup>
        pub const YOBI: f64 = 1024.0 * 1024.0 * 1024.0 * 1024.0 * 1024.0 * 1024.0 * 1024.0 * 1024.0;
        /// 2<sup>70</sup>
        pub const ZEBI: f64 = 1024.0 * 1024.0 * 1024.0 * 1024.0 * 1024.0 * 1024.0 * 1024.0;
        /// 2<sup>60</sup>
        pub const EXBI: f64 = 1024.0 * 1024.0 * 1024.0 * 1024.0 * 1024.0 * 1024.0;
        /// 2<sup>50</sup>
        pub const PEBI: f64 = 1024.0 * 1024.0 * 1024.0 * 1024.0 * 1024.0;
        /// 2<sup>40</sup>
        pub const TEBI: f64 = 1024.0 * 1024.0 * 1024.0 * 1024.0;
        /// 2<sup>30</sup>
        pub const GIBI: f64 = 1024.0 * 1024.0 * 1024.0;
        /// 2<sup>20</sup>
        pub const MEBI: f64 = 1024.0 * 1024.0;
        /// 2<sup>10</sup>
        pub const KIBI: f64 = 1024.0;
    }
    /// Constants for binary prefixes, 32 precision
    pub mod f32 {
        /// 2<sup>80</sup>
        pub const YOBI: f32 = 1024.0 * 1024.0 * 1024.0 * 1024.0 * 1024.0 * 1024.0 * 1024.0 * 1024.0;
        /// 2<sup>70</sup>
        pub const ZEBI: f32 = 1024.0 * 1024.0 * 1024.0 * 1024.0 * 1024.0 * 1024.0 * 1024.0;
        /// 2<sup>60</sup>
        pub const EXBI: f32 = 1024.0 * 1024.0 * 1024.0 * 1024.0 * 1024.0 * 1024.0;
        /// 2<sup>50</sup>
        pub const PEBI: f32 = 1024.0 * 1024.0 * 1024.0 * 1024.0 * 1024.0;
        /// 2<sup>40</sup>
        pub const TEBI: f32 = 1024.0 * 1024.0 * 1024.0 * 1024.0;
        /// 2<sup>30</sup>
        pub const GIBI: f32 = 1024.0 * 1024.0 * 1024.0;
        /// 2<sup>20</sup>
        pub const MEBI: f32 = 1024.0 * 1024.0;
        /// 2<sup>10</sup>
        pub const KIBI: f32 = 1024.0;
    }
}
