pub struct DateTime {
    time: f64
}

pub enum TimeUnit {
    Second,
    Minute,
    Hour,
    Day,
    Month,
    Year
}

impl DateTime {
    pub fn new () -> Self {
        Self {
            time: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs_f64()
        }
    }
    pub fn time(&self) -> f64 {
        self.time
    }
    pub fn nano(&self) -> u128 {
        (self.time * 10_f64.powi(6)) as u128
    }
    pub fn millis(&self) -> u128 {
        (self.time * 10_f64.powi(3)) as u128
    }
    pub fn second(&self) -> u8 {
        (self.time % 60.) as u8
    }
    pub fn minute(&self) -> u8 {
        (self.time / 60. % 60.) as u8
    }
    // this is UTC! no time zone conversion done. can't do that without C code i don't want to get into or a dependency
    pub fn hour(&self) -> u8 {
        (self.time / 60. / 60. % 24.) as u8
    }
    pub fn day(&self) -> u8 {
        let (day, _, _) = self.date();
        day
    }
    pub fn month(&self) -> u8 {
        let (_, month, _) = self.date();
        month
    }
    pub fn year(&self) -> u16 {
        let (_, _, year) = self.date();
        year
    }
    fn is_leap(&self, year: u16) -> bool {
        (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
    }
    // this was an absolute pain to figure out
    pub fn date(&self) -> (u8, u8, u16) {
        // so this is days since epoch
        let mut day = (self.time / 60. / 60. / 24.) as u16;
        // years at epoch
        let mut year: u16 = 1970;
        
        // have to go through each year (cos leap year!) and subtract the days
        // this pinpoints year
        loop {
            let days_in_year = if self.is_leap(year) { 366 } else { 365 };
            if day < days_in_year { break; }
            day -= days_in_year;
            year += 1;
        }
        
        let mut days: [u16; 12] = [
            31, // january
            28, // february
            31, // march
            30, // april
            31, // may
            30, // june
            31, // july
            31, // august
            30, // september
            31, // october
            30, // november
            31  // december
        ];
        if self.is_leap(year) {
            days[1] = 29; // leap year! again.
        }
        // go through each month and subtract the days
        // this pinpoints the month and day
        for i in 0..days.len() {
            let month_day = days[i];
            if day < month_day {
                // (day, month, year)
                return ((day + 1) as u8, (i + 1) as u8, year);
            }
            else {
                day -= month_day;
            }
        }
        // just assume epoch
        // (day, month, year)
        (0, 0, 1970)
    }
    /*
    pub fn add(&mut self, unit: TimeUnit, val: i64) -> &mut Self {
        match unit {
            TimeUnit::Second => {
                self.time += val as f64;
            }
            TimeUnit::Minute => {
                self.time += (val * 60) as f64;
            }
            TimeUnit::Hour => {
                self.time += (val * 60 * 60) as f64;
            }
            // i hate loathe deplore detest you.
            // why couldn't self.time += (val * 60 * 60 * 24) as f64; have worked
            TimeUnit::Day => {
                self.time += (val * 60 * 60 * 24) as f64;
            }
            // yuck yuck muck
            TimeUnit::Month => {
                let mut month = 0;
                let mut days: [u16; 12] = [
                    31, // january
                    28, // february
                    31, // march
                    30, // april
                    31, // may
                    30, // june
                    31, // july
                    31, // august
                    30, // september
                    31, // october
                    30, // november
                    31  // december
                ];
                
                let mut cache_year = 0;
                while month < val.abs() {
                    if cache_year != self.year() {
                        days[1] = if self.is_leap(self.year()) { 29 } else if self.is_leap(cache_year) { 28 } else { 28 };
                        cache_year = self.year();
                    }
                    //println!("1. (!)\t{}\t{}\t{}\t{}", self.month() as i16 % 12, if val.signum() < 0 { -1 } else { 0 }, self.month() as i16 % 12 + if val.signum() < 0 { -1 } else { 0 }, ((self.month() as i16 % 12 + if val.signum() < 0 { -1 } else { 0 }) % 12) as u16);
                    let mut index = (self.month() % 12) as u16;
                    if val.signum() < 0 {
                        let mut temp = index as i16;
                        temp -= 1;
                        if temp < 0 {
                            temp += 12;
                        }
                        index = temp as u16;
                    }
                    // println!("2. (@)\t{}\t{}\t{}\t{}", index, self.month(), days[index as usize], self.year());
                    self.time += val.signum() as f64 * days[index as usize] as f64 * 60. * 60. * 24.;
                    // println!("4. ($)\t{}\t{}\t{}\t{}", index, self.month(), days[index as usize], self.year());
                    month += 1;
                }
            }
            // less yuck but still muck
            TimeUnit::Year => {
                // self.add(TimeUnit::Month, val * 12);
                let mut year = 0;
                while year < val.abs() {
                    println!("5. (%)\t{}\t{}\t{}\t{}\t{}", self.year(), self.is_leap(self.year()), year, self.day(), val.signum());
                    self.time += val.signum() as f64 * (if self.is_leap(self.year()) { 366. * 60. * 60. * 24. } else { 365. * 60. * 60. * 24. });
                    year += 1;
                }
            }
        }
        self
    }
    */
    
    pub fn add(&mut self, unit: TimeUnit, val: i64) -> &mut Self {
        let val_f64 = val as f64;
        let sign = val.signum() as f64;
        
        match unit {
            TimeUnit::Second => self.time += val_f64,
            TimeUnit::Minute => self.time += val_f64 * 60.0,
            TimeUnit::Hour => self.time += val_f64 * 3600.0,
            TimeUnit::Day => self.time += val_f64 * 86400.0,
            TimeUnit::Month => {
                let mut month_counter = 0;
                let days_in_months: [f64; 12] = [31.0, 28.0, 31.0, 30.0, 31.0, 30.0, 31.0, 31.0, 30.0, 31.0, 30.0, 31.0];
                
                while month_counter < val.abs() {
                    let current_month_0_11 = (self.month() - 1) as usize;
                    let mut year_check = self.year();
                    let mut index_check = current_month_0_11;
    
                    if sign < 0.0 {
                        if index_check == 0 {
                            index_check = 11;
                            year_check -= 1;
                        } else {
                            index_check -= 1;
                        }
                    }
    
                    let days = if index_check == 1 && self.is_leap(year_check) { 29.0 } else { days_in_months[index_check] };
                    self.time += sign * days * 86400.0;
                    month_counter += 1;
                }
            }
            TimeUnit::Year => {
                let mut year_counter = 0;
                while year_counter < val.abs() {
                    let days = if self.is_leap(self.year()) { 366.0 } else { 365.0 };
                    self.time += sign * days * 86400.0;
                    year_counter += 1;
                }
            }
        }
        self
    }
    /*
    pub fn floor(&mut self, unit: TimeUnit)
    pub fn round(&mut self, unit: TimeUnit)
    pub fn ceil(&mut self, unit: TimeUnit)
    */
}
