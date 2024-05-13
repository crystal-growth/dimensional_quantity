#[cfg(test)]
mod test_dimensions {
    use crate::si::extended::f64::quantities::{
        AmountOfSubstance, Angle, ElectricCurrent, Information, Length, LuminousIntensity, Mass,
        Ratio, SolidAngle, TemperatureInterval, ThermodynamicTemperature, Time,
    };
    use crate::si::extended::f64::quantity::Quantity;
    #[test]
    fn ratio() {
        let _: Ratio = Quantity::<0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0>::new(1.0);
    }
    #[test]
    fn length() {
        let _l: Length = Quantity::<1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0>::new(1.0);
    }
    #[test]
    fn mass() {
        let _l: Mass = Quantity::<0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0>::new(1.0);
    }
    #[test]
    fn time() {
        let _l: Time = Quantity::<0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0>::new(1.0);
    }
    #[test]
    fn electric_current() {
        let _l: ElectricCurrent = Quantity::<0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0>::new(1.0);
    }
    #[test]
    fn thermodynamic_temperature() {
        let _l: ThermodynamicTemperature = Quantity::<0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0>::new(1.0);
    }
    #[test]
    fn amount_of_substance() {
        let _l: AmountOfSubstance = Quantity::<0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0>::new(1.0);
    }
    #[test]
    fn luminous_intensity() {
        let _l: LuminousIntensity = Quantity::<0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0>::new(1.0);
    }
    #[test]
    fn temperature_interval() {
        let _l: TemperatureInterval = Quantity::<0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0>::new(1.0);
    }
    #[test]
    fn angle() {
        let _l: Angle = Quantity::<0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0>::new(1.0);
    }

    #[test]
    fn solid_angle() {
        let _l: SolidAngle = Quantity::<0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0>::new(1.0);
    }
    #[test]
    fn information() {
        let _l: Information = Quantity::<0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1>::new(1.0);
    }
}

#[cfg(test)]
mod tests_operations_si_extended_f64 {
    use crate::si::extended::f64::quantities::{
        Area, Length, Ratio, Volume, VolumetricNumberDensity,
    };
    #[test]

    fn powi_3() {
        let v = Volume::new(1000.0);
        let l: Length = Length::new(10.0);
        let v_2: Volume = l.powi::<3>();
        assert_eq!(v, v_2)
    }

    #[test]

    fn sqrt() {
        let a: Area = Area::new(100.0);
        let l = Length::new(10.0);
        let l_2: Length = a.sqrt();
        assert_eq!(l, l_2);
    }

    #[test]
    fn cbrt() {
        let vol: Volume = Volume::new(1000.0);
        let len_1: Length = vol.cbrt();
        let len_2: Length = Length::new(10.0);
        assert_eq!(len_1, len_2);
    }
    #[test]
    fn mul_div_scalar() {
        let vol: Volume = Volume::new(1000.0);
        let v1 = vol * 1000.0;
        assert_eq!(v1, Volume::new(1000000.0));
        let v2 = 1000.0 * vol;
        assert_eq!(v2, Volume::new(1000000.0));
        let v3 = vol / 1000.0;
        assert_eq!(v3, Volume::new(1.0));
        let v4 = 1000.0 / vol;
        assert_eq!(v4, VolumetricNumberDensity::new(1.0));
    }
    #[test]
    fn into() {
        let r: Ratio = 20.0.into();
        let _f: f64 = r.get_with_si_unit();
    }
    #[test]
    fn equality_and_comparizon() {
        let v1: Volume = Volume::new(1000.0);
        let v2: Volume = Volume::new(1000.0);
        let v3: Volume = Volume::new(1.0);
        assert_eq!(v1, v2);
        assert_eq!(v1, v1);
        assert_ne!(v1, v3);
        assert!((v1 >= v2) == true);
        assert!((v1 <= v2) == true);
        assert!((v1 > v3) == true);
        assert!((v1 < v2) == false);
        assert!((v1 < v3) == false);
    }
}

#[cfg(test)]
mod tests_si_extended_f64 {
    use crate::prefix::metric::f64::MILLI;
    use crate::si::extended::f64::quantities::{
        Acceleration, Area, Capacitance, ElectricCharge, ElectricCurrent, ElectricPermittivity,
        ElectricPotential, Energy, Force, Frequency, HeatCapacity, Length, Mass, Power, Pressure,
        TemperatureInterval, ThermalPressureCoefficient, ThermodynamicTemperature, Velocity,
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
    #[test]
    fn test_thermal_pressure_coefficient() {
        let dp = Pressure::new(1.0);
        let dt = TemperatureInterval::new(1.0);
        let dp_over_dt: ThermalPressureCoefficient = dp / dt;

        assert_eq!(dp_over_dt, dp / dt);
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
        ElectricPotential, Energy, Force, Frequency, HeatCapacity, Length, Mass, Power,
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
