// should support csv, database and return for piping into other applications

struct Glucose {
    time: String,
    value: f32
}

struct BasalInjection {
    time: String,
    units: f32
}

struct BolusInjection {
    time: String,
    units: f32
}

struct Carbs {
    time: String,
    grams: i32
}

struct Exercise {
    time: String
}

struct Medication {
    time: String
}

pub fn export_data() {
    println!("export");
}

pub fn export_to_file(file: String) {
    // should have default filename and location
    // should append if existing
}

pub fn export_to_sql(db: String) {
    // should create database if not existing
}
