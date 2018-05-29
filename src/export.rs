// should support csv, database and return for piping into other applications

struct Glucose {
    time: String,
    value: f32
}

struct Basal_injection {
    time: String,
    units: f32
}

struct Bolus_injection {
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
