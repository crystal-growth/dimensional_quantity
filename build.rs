use num_traits::Float;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::io::{BufRead, BufReader, Write};
use std::str::FromStr;
use std::{
    collections::BTreeMap,
    env,
    fs::{self, File},
    path::Path,
};

#[derive(Deserialize, Serialize, Debug, Copy, Clone, PartialEq)]
struct Dimensions {
    length: i64,
    mass: i64,
    time: i64,
    electric_current: i64,
    thermodynamic_temperature: i64,
    amount_of_substance: i64,
    luminous_intensity: i64,
    angle: i64,
    temperature_interval: i64,
    information: i64,
}
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]

struct Quantity {
    name: String,
    symbol: String,
    snake_case_name: String,
    short_dim_formula: String,
    long_dim_formula: String,
    units_formula: String,
    dimensions: Dimensions,
    units: BTreeMap<String, Unit>,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
struct Unit {
    multiplier: String,
    symbol: String,
    singular: String,
    plural: String,
}

fn load_quantities_from_dir(dir: &Path) -> Vec<Quantity> {
    let mut quantities = Vec::<Quantity>::new();

    for file in fs::read_dir(dir).unwrap() {
        let path = file.unwrap().path();
        let extension = path.extension();

        if let Some(extension) = extension {
            if extension.eq("toml") {
                let q = fs::read_to_string(&path).unwrap();
                let q = toml::de::from_str(&q).unwrap();
                quantities.push(q);
            }
        }
    }

    quantities
}

fn quantity_from_dimension(dim: &str) -> Option<String> {
    let quantity = match dim {
        "" => "Ratio",
        "m" => "Length",
        "kg" => "Mass",
        "J T^-1" => "MagneticMoment",
        "kg mol^-1" => "MolarMass",
        "J" => "Energy",
        "J s" => "Action",
        "J Hz^-1" => "Action",
        "s" => "Time",
        "m s^-2" => "Acceleration",
        "ohm" => "ElectricalResistance",
        "m^3 mol^-1" => "MolarVolume",
        "C" => "ElectricCharge",
        "kg m s^-1" => "Momentum",
        "F m^-1" => "ElectricPermittivity",
        "m s^-1" => "Velocity",
        "C kg^-1" => "ElectricChargeToMassRatio",
        "m^-3" => "VolumetricNumberDensity",
        "Pa" => "Pressure",
        "K" => "ThermodynamicTemperature",
        "C mol^-1" => "MolarElectricCharge",
        "W m^-2 K^-4" => "StefanBoltzmannConstant",
        "mol^-1" => "ReciprocalAmountOfSubstance",
        "m^3 kg^-1 s^-2" => "NewtonianConstantOfGravitation",
        "N" => "Force",
        "N A^-2" => "MagneticPermeability",
        "A" => "ElectricCurrent",
        "Hz" => "Frequency",
        "J K^-1" => "Entropy",
        "S" => "ElectricalConductance",
        "m^2" => "Area",
        "m^-1" => "ReciprocalLength",
        "C m^2" => "ElectricQuadrupoleMoment",
        "C m" => "ElectricDipoleMoment",
        "J mol^-1 K^-1" => "MolarEntropy",
        "V m^-1" => "ElectricField",
        "C m^-3" => "ElectricChargeVolumetricDensity",
        "V" => "ElectricPotential",
        "T" => "MagneticFluxDensity",
        "Wb" => "MagneticFlux",
        "m^2 s^-1" => "Circulation",
        "m K" => "WienWavelengthDisplacementLawConstant",
        "Hz K^-1" => "WienFrequencyDisplacementLawConstant",
        "W m^2" => "FirstRadiationConstant",
        "W m^2 sr^-1" => "FirstRadiationConstantForSpectralRadiance",
        "C^3 m^3 J^-2" => "FirstHyperpolarizability",
        "C^4 m^4 J^-3" => "SecondHyperpolarizability",
        "J Hz^-1 mol^-1" => "MolarAction",
        "V m^-2" => "ElectricFieldGradient",
        "lm W^-1" => "LuminousEfficacy",
        "Hz V^-1" => "JosephsonConstant",
        "J T^-2" => "Magnetizability",
        "C^2 m^2 J^-1" => "ElectricPolarizability",
        _ => "UNKNOWN",
    };
    match quantity {
        "UNKNOWN" => None,
        _ => Some(quantity.to_string()),
    }
}

enum SystemOfQuantities {
    SiIsq,
    SiExtended,
}

fn generate_quantities(qs: &[Quantity], path: &Path, system_of_quantities: SystemOfQuantities) {
    let mut output = File::create(path).unwrap();
    for q in qs.iter() {
        let Quantity {
            name,
            symbol,
            short_dim_formula,
            long_dim_formula,
            units_formula,
            ..
        } = q;

        let Dimensions {
            length,
            mass,
            time,
            electric_current,
            thermodynamic_temperature,
            amount_of_substance,
            luminous_intensity,
            angle,
            temperature_interval,
            information,
        } = q.dimensions;
        writeln!(
            output,
            "/// {name}, {short_dim_formula}, \\[{units_formula}\\];\n/// {long_dim_formula}"
        )
        .unwrap();
        match system_of_quantities {
            SystemOfQuantities::SiIsq => {
                let temperature = thermodynamic_temperature + temperature_interval;
                writeln!(
                output,
                "pub type {symbol} = Quantity<{length}, {mass}, {time}, {electric_current}, {temperature} , {amount_of_substance}, {luminous_intensity}>;").unwrap()
            }
            SystemOfQuantities::SiExtended => {
                writeln!(
                output,
                "pub type {symbol} = Quantity<{length}, {mass}, {time}, {electric_current}, {thermodynamic_temperature} , {amount_of_substance}, {luminous_intensity}, {temperature_interval},  {angle}, {information}>;")
            .unwrap();
            }
        }
    }
}

fn generate_codata_constants<Storage: Float + FromStr + Display>(codata: &Path, out_dir: &Path) {
    let codata = File::open(codata).unwrap();
    let codata = BufReader::new(codata);
    let storage_type = std::any::type_name::<Storage>();
    let of = out_dir.join(format!("codata2018_{storage_type}.rs"));
    let of = Path::new(&of);
    let mut of = File::create(of).unwrap();

    for line in codata.lines() {
        let line = line.unwrap();
        let name: String = line[..60]
            .trim()
            .replace(' ', "_")
            .replace('.', "")
            .replace('-', "_")
            .replace([',', '(', ')'], "")
            .replace('/', "_PER_")
            .to_uppercase();
        let name_pretty = line[..60]
            .trim()
            .replace(" mag.", " magnetic")
            .replace(" mom.", " moment");
        let value = line[60..85].trim().replace(' ', "_").replace("...", "");
        let unit = line[110..].trim();

        let quantity = quantity_from_dimension(unit);
        if let Some(quantity) = quantity {
            let value = if value.contains('.') {
                value
            } else {
                format!("{value}.0")
            };

            let value_numeric = Storage::from_str(&value.trim().replace('_', ""));
            if let Ok(value_numeric) = value_numeric {
                if !(value_numeric.is_finite() && value_numeric.is_normal()) {
                    continue;
                }
            } else {
                continue;
            }

            let unit_pretty = if unit.is_empty() {
                "dimensionless".to_string()
            } else {
                unit.replace(' ', "⋅")
                    .replace("^-1", "<sup>-1</sup>")
                    .replace("^-2", "<sup>-2</sup>")
                    .replace("^-3", "<sup>-3</sup>")
                    .replace("^-4", "<sup>-4</sup>")
                    .replace("^2", "<sup>2</sup>")
                    .replace("^3", "<sup>3</sup>")
                    .replace("^4", "<sup>4</sup>")
            };
            let value_pretty = if let Some((mantissa, order)) = value.split_once("_e") {
                format!("{mantissa}⋅10<sup>{order}</sup>")
            } else {
                value.clone()
            };

            writeln!(
                of,
                "/// {name_pretty}, {value_pretty} \\[{unit_pretty}\\]"
            )
            .unwrap();
            writeln!(
                of,
                "pub const {name}: {quantity} = {quantity}::new({value});"
            )
            .unwrap();
        } else {
            // println!("cargo:warning=Unknown unit {unit}");
        };
    }
}

fn generate_units<Storage: Float + FromStr + Display>(
    qs: &[Quantity],
    path: &Path,
    system_of_quantities: SystemOfQuantities,
) {
    let mut output = File::create(path).unwrap();
    let storage_type = std::any::type_name::<Storage>();

    for q in qs.iter() {
        let Quantity {
            symbol: q_symbol,
            snake_case_name,
            name,
            ..
        } = q;
        writeln!(output, "/// {name} units of measure").unwrap();
        writeln!(output, "pub mod {snake_case_name} {{").unwrap();
        let system_module_name = match system_of_quantities {
            SystemOfQuantities::SiIsq => "isq",
            SystemOfQuantities::SiExtended => "extended",
        };
        writeln!(
            output,
            "use crate::si::{system_module_name}::{storage_type}::quantities::{q_symbol};"
        )
        .unwrap();

        for (unit_name, u) in q.units.iter() {
            let Unit {
                multiplier,
                symbol,
                singular,
                ..
            } = u;
            if multiplier.contains(',') {
                let brackets: &[_] = &['(', ')'];
                let multiplier = multiplier.trim().trim_matches(brackets);
                let (mul, add) = multiplier.split_once(',').unwrap();

                let unit_lower = unit_name.to_ascii_lowercase();
                let getter_setter = format!(
                    "
                impl {q_symbol} {{
                    /// Conversion from {singular}, \\[{symbol}\\]
                    pub fn from_{unit_lower}(x: {storage_type}) -> {q_symbol} {{
                                {q_symbol}::new((x + {add}) * {mul})
                        }}
                    /// Conversion to {unit_lower}
                    pub fn to_{unit_lower}(&self)  -> {storage_type} {{
                        let x = self.get_with_si_unit();
                        (x  - {add}) / {mul}
                    }}
                }}"
                );
                writeln!(output, "{getter_setter}").unwrap();
            } else {
                let multiplier = &multiplier.trim();
                if let Ok(x) = Storage::from_str(&multiplier.replace('_', "")) {
                    if x.is_finite() && x.is_normal() {
                        writeln!(output, "/// {singular}, \\[{symbol}\\]").unwrap();
                        writeln!(
                            output,
                            "pub const {unit_name}: {q_symbol} = {q_symbol}::new({multiplier});"
                        )
                        .unwrap();
                    }
                } else {
                    println!(
                        "cargo:warning=Error parsing multiplier {multiplier} from {}",
                        path.display()
                    );
                    // writeln!(
                    //     output,
                    //     "pub const {unit_name}: {q_symbol} = {q_symbol}::new({multiplier});"
                    // )
                    // .unwrap();
                }
            }
        }

        writeln!(output, "}}").unwrap();
    }
}

fn main() {
    println!("cargo:rerun-if-changed=src/si/quantities_definition/");
    println!("cargo:rerun-if-changed=src/si/constants_definition/");

    let data_dir = Path::new("src/si/quantities_definition");
    let quantities = load_quantities_from_dir(data_dir);

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let quantity_si_extended_path = Path::new(&out_dir).join("quantities_si_extended.rs");
    let quantity_si_isq_path = Path::new(&out_dir).join("quantities_si_isq.rs");
    generate_quantities(
        &quantities,
        &quantity_si_extended_path,
        SystemOfQuantities::SiExtended,
    );
    generate_quantities(
        &quantities,
        &quantity_si_isq_path,
        SystemOfQuantities::SiIsq,
    );
    let units_si_extended_f64_path = Path::new(&out_dir).join("units_si_extended_f64.rs");
    let units_si_extended_f32_path = Path::new(&out_dir).join("units_si_extended_f32.rs");
    let units_si_isq_f64_path = Path::new(&out_dir).join("units_si_isq_f64.rs");
    let units_si_isq_f32_path = Path::new(&out_dir).join("units_si_isq_f32.rs");
    generate_units::<f64>(
        &quantities,
        &units_si_extended_f64_path,
        SystemOfQuantities::SiExtended,
    );
    generate_units::<f32>(
        &quantities,
        &units_si_extended_f32_path,
        SystemOfQuantities::SiExtended,
    );
    generate_units::<f64>(
        &quantities,
        &units_si_isq_f64_path,
        SystemOfQuantities::SiIsq,
    );
    generate_units::<f32>(
        &quantities,
        &units_si_isq_f32_path,
        SystemOfQuantities::SiIsq,
    );
    let codata_src_path = Path::new("src/si/constants_definition/codata2018.txt");
    let out_dir_path = Path::new(&out_dir);
    generate_codata_constants::<f64>(codata_src_path, out_dir_path);
    generate_codata_constants::<f32>(codata_src_path, out_dir_path);
}
