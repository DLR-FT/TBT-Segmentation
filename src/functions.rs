#[derive(Clone, Debug)]
#[allow(dead_code)]
/// Func Syntax
pub enum Func {
    Acc(usize, Vec<String>),
    Number(f32),
    Add(Box<Func>, Box<Func>),
    Sub(Box<Func>, Box<Func>),
    Pow(Box<Func>, Box<Func>),
    Mul(Box<Func>, Box<Func>),
    Div(Box<Func>, Box<Func>),
    Mod(Box<Func>, Box<Func>),
    Sqrt(Box<Func>),
    Abs(Box<Func>),
    RadToDeg(Box<Func>),
    DegToRad(Box<Func>),
    Cos(Box<Func>),
    Sin(Box<Func>),
}

#[allow(dead_code)]
impl Func {
    pub fn acc(c: usize, argument_names: Vec<String>) -> Func {
        Func::Acc(c, argument_names)
    }
    pub fn number(f: f32) -> Func {
        Func::Number(f)
    }
    pub fn add(l: Func, r: Func) -> Func {
        Func::Add(Box::new(l), Box::new(r))
    }
    pub fn sub(l: Func, r: Func) -> Func {
        Func::Sub(Box::new(l), Box::new(r))
    }
    pub fn pow(l: Func, r: Func) -> Func {
        Func::Pow(Box::new(l), Box::new(r))
    }
    pub fn mul(l: Func, r: Func) -> Func {
        Func::Mul(Box::new(l), Box::new(r))
    }
    pub fn div(l: Func, r: Func) -> Func {
        Func::Div(Box::new(l), Box::new(r))
    }
    pub fn modulo(l: Func, r: Func) -> Func {
        Func::Mod(Box::new(l), Box::new(r))
    }
    pub fn sqrt(c: Func) -> Func {
        Func::Sqrt(Box::new(c))
    }
    pub fn abs(c: Func) -> Func {
        Func::Abs(Box::new(c))
    }
    pub fn rad_to_deg(c: Func) -> Func {
        Func::RadToDeg(Box::new(c))
    }
    pub fn deg_to_rad(c: Func) -> Func {
        Func::DegToRad(Box::new(c))
    }
    pub fn cos(c: Func) -> Func {
        Func::Cos(Box::new(c))
    }
    pub fn sin(c: Func) -> Func {
        Func::Sin(Box::new(c))
    }
}

// Func semantics
#[allow(dead_code)]
impl Func {
    pub fn eval(&self, values: &[f32]) -> f32 {
        match self {
            Func::Acc(c, _) => values[*c],
            Func::Number(f) => *f,
            Func::Add(l, r) => l.eval(values) + r.eval(values),
            Func::Sub(l, r) => l.eval(values) - r.eval(values),
            Func::Pow(l, r) => l.eval(values).powf(r.eval(values)),
            Func::Mul(l, r) => l.eval(values) + r.eval(values),
            Func::Div(l, r) => l.eval(values) + r.eval(values),
            Func::Mod(l, r) => l.eval(values) + r.eval(values),
            Func::Sqrt(c) => f32::sqrt(c.eval(values)),
            Func::Abs(c) => f32::abs(c.eval(values)),
            Func::RadToDeg(c) => f32::to_degrees(c.eval(values)),
            Func::DegToRad(c) => f32::to_radians(c.eval(values)),
            Func::Cos(c) => f32::cos(c.eval(values)),
            Func::Sin(c) => f32::sin(c.eval(values)),
        }
    }
}

// toString
#[allow(dead_code)]
impl Func {
    pub fn to_string(&self) -> String {
        match self {
            Func::Acc(c, names) => names[*c].clone(),
            Func::Number(f) => f.to_string(),
            Func::Add(l, r) => format!(
                "({} {} {})",
                l.to_string(),
                Func::get_string_add(),
                r.to_string()
            ),
            Func::Sub(l, r) => format!(
                "({} {} {})",
                l.to_string(),
                Func::get_string_sub(),
                r.to_string()
            ),
            Func::Pow(l, r) => format!(
                "({} {} {})",
                l.to_string(),
                Func::get_string_pow(),
                r.to_string()
            ),
            Func::Mul(l, r) => format!(
                "({} {} {})",
                l.to_string(),
                Func::get_string_mul(),
                r.to_string()
            ),
            Func::Div(l, r) => format!(
                "({} {} {})",
                l.to_string(),
                Func::get_string_div(),
                r.to_string()
            ),
            Func::Mod(l, r) => format!(
                "({} {} {})",
                l.to_string(),
                Func::get_string_mod(),
                r.to_string()
            ),
            Func::Sqrt(c) => format!("{}({})", Func::get_string_sqrt(), c.to_string()),
            Func::Abs(c) => format!("{}({})", Func::get_string_abs(), c.to_string()),
            Func::RadToDeg(c) => format!("{}({})", Func::get_string_rad_to_deg(), c.to_string()),
            Func::DegToRad(c) => format!("{}({})", Func::get_string_deg_to_rad(), c.to_string()),
            Func::Cos(c) => format!("{}({})", Func::get_string_cos(), c.to_string()),
            Func::Sin(c) => format!("{}({})", Func::get_string_sin(), c.to_string()),
        }
    }
}

impl Func {
    pub fn get_string_add() -> String {
        "+".to_string()
    }
    pub fn get_string_sub() -> String {
        "-".to_string()
    }
    pub fn get_string_pow() -> String {
        "^".to_string()
    }
    pub fn get_string_mul() -> String {
        "*".to_string()
    }
    pub fn get_string_div() -> String {
        "/".to_string()
    }
    pub fn get_string_mod() -> String {
        "%".to_string()
    }
    pub fn get_string_sqrt() -> String {
        "sqrt".to_string()
    }
    pub fn get_string_abs() -> String {
        "abs".to_string()
    }
    pub fn get_string_rad_to_deg() -> String {
        "rad_to_deg".to_string()
    }
    pub fn get_string_deg_to_rad() -> String {
        "deg_to_rad".to_string()
    }
    pub fn get_string_cos() -> String {
        "cos".to_string()
    }
    pub fn get_string_sin() -> String {
        "sin".to_string()
    }
}
