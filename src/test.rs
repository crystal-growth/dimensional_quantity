#[cfg(test)]
mod test_dimensions {
    use crate::si::extended::f64::quantities::{
        AmountOfSubstance, Angle, ElectricCurrent, Information, Length, LuminousIntensity, Mass,
        TemperatureInterval, ThermodynamicTemperature, Time,
    };
    use crate::si::extended::f64::quantity::Quantity;
    #[test]
    fn length() {
        let _l: Length = Quantity::<1, 0, 0, 0, 0, 0, 0, 0, 0, 0>::new(1.0);
    }
    #[test]
    fn mass() {
        let _l: Mass = Quantity::<0, 1, 0, 0, 0, 0, 0, 0, 0, 0>::new(1.0);
    }
    #[test]
    fn time() {
        let _l: Time = Quantity::<0, 0, 1, 0, 0, 0, 0, 0, 0, 0>::new(1.0);
    }
    #[test]
    fn electric_current() {
        let _l: ElectricCurrent = Quantity::<0, 0, 0, 1, 0, 0, 0, 0, 0, 0>::new(1.0);
    }
    #[test]
    fn thermodynamic_temperature() {
        let _l: ThermodynamicTemperature = Quantity::<0, 0, 0, 0, 1, 0, 0, 0, 0, 0>::new(1.0);
    }
    #[test]
    fn amount_of_substance() {
        let _l: AmountOfSubstance = Quantity::<0, 0, 0, 0, 0, 1, 0, 0, 0, 0>::new(1.0);
    }
    #[test]
    fn luminous_intensity() {
        let _l: LuminousIntensity = Quantity::<0, 0, 0, 0, 0, 0, 1, 0, 0, 0>::new(1.0);
    }
    #[test]
    fn temperature_interval() {
        let _l: TemperatureInterval = Quantity::<0, 0, 0, 0, 0, 0, 0, 1, 0, 0>::new(1.0);
    }
    #[test]
    fn angle() {
        let _l: Angle = Quantity::<0, 0, 0, 0, 0, 0, 0, 0, 1, 0>::new(1.0);
    }
    #[test]
    fn information() {
        let _l: Information = Quantity::<0, 0, 0, 0, 0, 0, 0, 0, 0, 1>::new(1.0);
    }
}

#[cfg(test)]
mod tests_si_extended_f64 {
    use crate::prefix::metric::f64::MILLI;
    use crate::si::extended::f64::quantities::{
        Acceleration, Area, Capacitance, ElectricCharge, ElectricCurrent, ElectricPermittivity,
        ElectricPotential, Energy, Force, Frequency, HeatCapacity, Length, Mass, Power,
        TemperatureInterval, ThermodynamicTemperature, Velocity,
    };
    use crate::si::extended::f64::{
        constants::{PLANCK_CONSTANT, SPEED_OF_LIGHT_IN_VACUUM, STANDARD_ACCELERATION_OF_GRAVITY},
        units_of_measure::{
            coulomb_constant::COULOMB_CONSTANT,
            length::{METER, MILLIMETER},
        },
    };

    #[test]
    fn test_new_with_unit() {
        let meter = Length::new_with_unit(1.0, METER);
        let millilmeter = Length::new_with_unit(1.0, MILLIMETER);
        assert_eq!(meter * MILLI, millilmeter);
    }

    #[test]
    fn test_get_with_unit() {
        let meter = Length::new_with_unit(1.0, METER);
        let millilmeter = Length::new_with_unit(1.0, MILLIMETER);
        let m = meter.get_with_unit(MILLIMETER);
        let mm = millilmeter.get_with_unit(MILLIMETER);

        assert_eq!(m, 1000.0);
        assert_eq!(mm, 1.0);
    }

    #[test]
    fn test_heat_capacity() {
        let cp = HeatCapacity::new(3.1);
        let dth = TemperatureInterval::new(100.0);
        let _dq: Energy = cp * dth;
    }

    #[test]
    fn test_kinetic_energy() {
        let c: Velocity = SPEED_OF_LIGHT_IN_VACUUM;
        let m: Mass = Mass::new(1.0);
        let _e: Energy = m * c.powi::<2>();
    }

    #[test]
    fn test_potential_energy() {
        let m: Mass = Mass::new(1.0);
        let g: Acceleration = STANDARD_ACCELERATION_OF_GRAVITY;
        let h: Length = Length::new(1.0);
        let _e: Energy = m * g * h;
    }

    #[test]
    fn test_electric_power() {
        let i: ElectricCurrent = ElectricCurrent::new(1.0);
        let v: ElectricPotential = ElectricPotential::new(1.0);

        let _p: Power = i * v;
    }

    #[test]
    fn coulomb_law() {
        let q1: ElectricCharge = ElectricCharge::new(1.0);
        let q2: ElectricCharge = ElectricCharge::new(2.0);
        let r: Length = Length::new(1.0);
        let _f: Force = COULOMB_CONSTANT * q1 * q2 / r.powi::<2>();
    }

    #[test]
    fn photon_energy() {
        let f: Frequency = Frequency::new(1.0);
        let _e: Energy = PLANCK_CONSTANT * f;
    }

    #[test]
    fn flat_capacitor() {
        let a: Area = Area::new(1.0);
        let d: Length = Length::new(1.0);
        let relative_eps = 8.1;
        let absolute_eps = ElectricPermittivity::new(relative_eps);
        let _c: Capacitance = absolute_eps * a / d;
    }

    #[test]
    fn celsius_to_kelvin() {
        let t_c = ThermodynamicTemperature::from_degree_celsius(0.0);
        let t_k = ThermodynamicTemperature::new(273.15);
        assert_eq!(t_c, t_k);
    }
    #[test]
    fn add_temperatures() {
        let t = ThermodynamicTemperature::from_degree_celsius(0.0);
        let ti = TemperatureInterval::new(10.0);
        let _t2 = t + ti;
        let t_square = t * t;
        let ti_square = ti * ti;
        let _two_t_square = ti_square + ti_square;
        let _two_t_square = t_square + t_square;
    }
}

#[cfg(test)]
mod tests_si_extended_f32 {
    use crate::si::extended::f32::quantities::{
        Acceleration, Area, Capacitance, ElectricCharge, ElectricCurrent, ElectricPermittivity,
        ElectricPotential, Energy, Force, Frequency, HeatCapacity, Length, Mass, Power,
        TemperatureInterval, ThermodynamicTemperature, Velocity,
    };
    use crate::si::extended::f32::{
        constants::{PLANCK_CONSTANT, SPEED_OF_LIGHT_IN_VACUUM, STANDARD_ACCELERATION_OF_GRAVITY},
        units_of_measure::coulomb_constant::COULOMB_CONSTANT,
    };

    #[test]
    fn test_heat_capacity() {
        let cp = HeatCapacity::new(3.1);
        let dth = TemperatureInterval::new(100.0);
        let _dq: Energy = cp * dth;
    }

    #[test]
    fn test_kinetic_energy() {
        let c: Velocity = SPEED_OF_LIGHT_IN_VACUUM;
        let m: Mass = Mass::new(1.0);
        let _e: Energy = m * c.powi::<2>();
    }

    #[test]
    fn test_potential_energy() {
        let m: Mass = Mass::new(1.0);
        let g: Acceleration = STANDARD_ACCELERATION_OF_GRAVITY;
        let h: Length = Length::new(1.0);
        let _e: Energy = m * g * h;
    }

    #[test]
    fn test_electric_power() {
        let i: ElectricCurrent = ElectricCurrent::new(1.0);
        let v: ElectricPotential = ElectricPotential::new(1.0);

        let _p: Power = i * v;
    }

    #[test]
    fn coulomb_law() {
        let q1: ElectricCharge = ElectricCharge::new(1.0);
        let q2: ElectricCharge = ElectricCharge::new(2.0);
        let r: Length = Length::new(1.0);
        let _f: Force = COULOMB_CONSTANT * q1 * q2 / r.powi::<2>();
    }

    #[test]
    fn photon_energy() {
        let f: Frequency = Frequency::new(1.0);
        let _e: Energy = PLANCK_CONSTANT * f;
    }

    #[test]
    fn flat_capacitor() {
        let a: Area = Area::new(1.0);
        let d: Length = Length::new(1.0);
        let relative_eps = 8.1;
        let absolute_eps = ElectricPermittivity::new(relative_eps);
        let _c: Capacitance = absolute_eps * a / d;
    }

    #[test]
    fn celsius_to_kelvin() {
        let t_c = ThermodynamicTemperature::from_degree_celsius(0.0);
        let t_k = ThermodynamicTemperature::new(273.15);
        assert_eq!(t_c, t_k);
    }
}

#[cfg(test)]
mod tests_si_isq_f32 {
    use crate::si::isq::f32::quantities::{
        Acceleration, Area, Capacitance, ElectricCharge, ElectricCurrent, ElectricPermittivity,
        ElectricPotential, Energy, Force, Frequency, HeatCapacity, Length, Mass, Power,
        TemperatureInterval, ThermodynamicTemperature, Velocity,
    };
    use crate::si::isq::f32::{
        constants::{PLANCK_CONSTANT, SPEED_OF_LIGHT_IN_VACUUM, STANDARD_ACCELERATION_OF_GRAVITY},
        units_of_measure::coulomb_constant::COULOMB_CONSTANT,
    };

    #[test]
    fn test_heat_capacity() {
        let cp = HeatCapacity::new(3.1);
        let dth = TemperatureInterval::new(100.0);
        let _dq: Energy = cp * dth;
    }

    #[test]
    fn test_kinetic_energy() {
        let c: Velocity = SPEED_OF_LIGHT_IN_VACUUM;
        let m: Mass = Mass::new(1.0);
        let _e: Energy = m * c.powi::<2>();
    }

    #[test]
    fn test_potential_energy() {
        let m: Mass = Mass::new(1.0);
        let g: Acceleration = STANDARD_ACCELERATION_OF_GRAVITY;
        let h: Length = Length::new(1.0);
        let _e: Energy = m * g * h;
    }

    #[test]
    fn test_electric_power() {
        let i: ElectricCurrent = ElectricCurrent::new(1.0);
        let v: ElectricPotential = ElectricPotential::new(1.0);

        let _p: Power = i * v;
    }

    #[test]
    fn coulomb_law() {
        let q1: ElectricCharge = ElectricCharge::new(1.0);
        let q2: ElectricCharge = ElectricCharge::new(2.0);
        let r: Length = Length::new(1.0);
        let _f: Force = COULOMB_CONSTANT * q1 * q2 / r.powi::<2>();
    }

    #[test]
    fn photon_energy() {
        let f: Frequency = Frequency::new(1.0);
        let _e: Energy = PLANCK_CONSTANT * f;
    }

    #[test]
    fn flat_capacitor() {
        let a: Area = Area::new(1.0);
        let d: Length = Length::new(1.0);
        let relative_eps = 8.1;
        let absolute_eps = ElectricPermittivity::new(relative_eps);
        let _c: Capacitance = absolute_eps * a / d;
    }

    #[test]
    fn celsius_to_kelvin() {
        let t_c = ThermodynamicTemperature::from_degree_celsius(0.0);
        let t_k = ThermodynamicTemperature::new(273.15);
        assert_eq!(t_c, t_k);
    }
}

#[cfg(test)]
mod tests_si_isq_f64 {
    use crate::prefix::metric::f64::MILLI;
    use crate::si::isq::f64::quantities::{
        Acceleration, Area, Capacitance, ElectricCharge, ElectricCurrent, ElectricPermittivity,
        ElectricPotential, Energy, Entropy, Force, Frequency, HeatCapacity, Length, Mass, Power,
        TemperatureInterval, ThermodynamicTemperature, Velocity,
    };
    use crate::si::isq::f64::{
        constants::{PLANCK_CONSTANT, SPEED_OF_LIGHT_IN_VACUUM, STANDARD_ACCELERATION_OF_GRAVITY},
        units_of_measure::{
            coulomb_constant::COULOMB_CONSTANT,
            length::{METER, MILLIMETER},
        },
    };

    #[test]
    fn it_works() {
        let cp = HeatCapacity::new(3.1);
        let dth = TemperatureInterval::new(100.0);
        let dq: Energy = cp * dth;
        dbg!(dq);
        let ds = Entropy::new(1.3);
        let t = ThermodynamicTemperature::new(500.0);
        let dq1 = t * ds;
        dbg!(dq1);
        let l = 3.0 * METER;
        println!("{:#?}", l / MILLIMETER);
    }

    #[test]
    fn test_new_with_unit() {
        let meter = Length::new_with_unit(1.0, METER);
        let millilmeter = Length::new_with_unit(1.0, MILLIMETER);
        assert_eq!(meter * MILLI, millilmeter);
    }

    #[test]
    fn test_get_with_unit() {
        let meter = Length::new_with_unit(1.0, METER);
        let millilmeter = Length::new_with_unit(1.0, MILLIMETER);
        let m = meter.get_with_unit(MILLIMETER);
        let mm = millilmeter.get_with_unit(MILLIMETER);

        assert_eq!(m, 1000.0);
        assert_eq!(mm, 1.0);
    }

    #[test]
    fn test_heat_capacity() {
        let cp = HeatCapacity::new(3.1);
        let dth = TemperatureInterval::new(100.0);
        let _dq: Energy = cp * dth;
    }

    #[test]
    fn test_kinetic_energy() {
        let c: Velocity = SPEED_OF_LIGHT_IN_VACUUM;
        let m: Mass = Mass::new(1.0);
        let _e: Energy = m * c.powi::<2>();
    }

    #[test]
    fn test_potential_energy() {
        let m: Mass = Mass::new(1.0);
        let g: Acceleration = STANDARD_ACCELERATION_OF_GRAVITY;
        let h: Length = Length::new(1.0);
        let _e: Energy = m * g * h;
    }

    #[test]
    fn test_electric_power() {
        let i: ElectricCurrent = ElectricCurrent::new(1.0);
        let v: ElectricPotential = ElectricPotential::new(1.0);

        let _p: Power = i * v;
    }

    #[test]
    fn coulomb_law() {
        let q1: ElectricCharge = ElectricCharge::new(1.0);
        let q2: ElectricCharge = ElectricCharge::new(2.0);
        let r: Length = Length::new(1.0);
        let _f: Force = COULOMB_CONSTANT * q1 * q2 / r.powi::<2>();
    }

    #[test]
    fn photon_energy() {
        let f: Frequency = Frequency::new(1.0);
        let _e: Energy = PLANCK_CONSTANT * f;
    }

    #[test]
    fn flat_capacitor() {
        let a: Area = Area::new(1.0);
        let d: Length = Length::new(1.0);
        let relative_eps = 8.1;
        let absolute_eps = ElectricPermittivity::new(relative_eps);
        let _c: Capacitance = absolute_eps * a / d;
    }

    #[test]
    fn celsius_to_kelvin() {
        let t_c = ThermodynamicTemperature::from_degree_celsius(0.0);
        let t_k = ThermodynamicTemperature::new(273.15);
        assert_eq!(t_c, t_k);
    }
    #[test]
    fn add_temperatures() {
        let t = ThermodynamicTemperature::from_degree_celsius(0.0);
        let ti = TemperatureInterval::new(10.0);
        let _t2 = t + ti;
        let t_square = t * t;
        let ti_square = ti * ti;
        let _two_t_square = ti_square + ti_square;
        let _two_t_square = t_square + t_square;
    }
}
