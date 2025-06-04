use tera::{from_value, to_value, Function};

pub struct Progressbar {
    min_value: f64,
    max_value: f64,
    
    empty_char: char,
    shaded_char: char,
    full_char: char,

    char_length: u8,
}

impl Progressbar {
    pub fn builder() -> Self {
        Self::default()
    }

    pub fn min_value(mut self, min_value: f64) -> Self {
        self.min_value = min_value;
        self
    }

    pub fn max_value(mut self, max_value: f64) -> Self {
        self.max_value = max_value;
        self
    }

    pub fn empty_char(mut self, empty_char: char) -> Self {
        self.empty_char = empty_char;
        self
    }

    pub fn shaded_char(mut self, shaded_char: char) -> Self {
        self.shaded_char = shaded_char;
        self
    }

    pub fn full_char(mut self, full_char: char) -> Self {
        self.full_char = full_char;
        self
    }

    pub fn char_length(mut self, char_length: u8) -> Self {
        self.char_length = char_length;
        self
    }
}

impl Default for Progressbar {
    fn default() -> Self {
        Self {
            min_value: 0.0_f64,
            max_value: 100.0_f64,

            empty_char: '.',
            shaded_char: '▒',
            full_char: '█',

            char_length: 16,
        }
    }
}

impl Function for Progressbar {

    fn call(
        &self, 
        args: &std::collections::HashMap<String, tera::Value>
    ) -> tera::Result<tera::Value> {
        
        if let Some(v) = args.get("value") {
            
            // ensure that the value is the correct data type
            if (!v.is_f64() && !v.is_u64()) {
                return Err("wrong data type".into());
            }

            // create our funny little buffer
            let mut buffer = String::new();

            // Turn v into an f64 so we can do math on it
            let val = from_value::<f64>(v.clone())
                .expect("What");

            // We make sure we only show off anything after the min value
            let start_offset = val - self.min_value;

            // we get our actual percentage
            let percentage = start_offset / self.max_value;

            // Add the first few full starting chars to the buffer
            buffer += std::iter::repeat(self.full_char)
                .take((percentage * self.char_length as f64).floor() as usize)
                .collect::<String>()
                .as_str();

            // Add the shaded char if necessary (when >0.5 difference between next empty cell)
            if (percentage * self.char_length as f64).fract() >= 0.5 {
                buffer += &self.shaded_char.to_string();
            }

            // Add the rest of the empty chars
            buffer += std::iter::repeat(self.empty_char)
                .take(((1.0_f64 - percentage) * self.char_length as f64).round() as usize)
                .collect::<String>()
                .as_str();

            // Return the buffer (the completed progress bar)
            return Ok(to_value(buffer).unwrap());

        }

        // If you provide the wrong value you get a swear
        return Err("fuck".into());

    }

}