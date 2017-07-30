extern crate rand;
use std::collections::HashMap;
use std::vec::Vec;
use lindenmayer::rand::distributions::IndependentSample;


pub struct LSystem
{
    iteration : u32,
    current_iteration_derivation : String,
    production_rules : HashMap<String,Vec<(f32, String)>>,
}

impl LSystem
{
    pub fn new(start_symbol : &'static str, production_rules : HashMap<&'static str, Vec<(f32,&'static str)>>) -> LSystem
    {
        let mut tmp = HashMap::new();
        for (key, value) in production_rules.iter()
        {
            let mut tmp_vec = Vec::new();
            for temp_val in value.iter()
            {
                tmp_vec.push( (temp_val.0, String::from(temp_val.1)) );
            }
            tmp.insert(key.to_string(), tmp_vec);
        }
        LSystem{iteration : 0, current_iteration_derivation : String::from(start_symbol), production_rules : tmp}
    }
}

impl Iterator for LSystem
{
    type Item = String;
    fn next(&mut self) -> Option<String>
    {
        let mut out = String::new();
        let distr = rand::distributions::Range::new(0f32,1f32);
        let mut rng = rand::thread_rng(); 
        for production_elem in self.current_iteration_derivation.chars()
        {
            if production_elem.is_lowercase() || production_elem == '[' || production_elem == ']' {out = format!("{}{}", out, production_elem); continue;}
            let mut derivation = self.production_rules.get(&production_elem.to_string()).unwrap().clone();
            let random_number = distr.ind_sample(&mut rng);     
            let mut chosen_index = 0;
            //println!("random number is: {}",random_number);

            for index in 0..derivation.len()
            {
                let derived_production_elem = derivation[index].clone();
                //println!("currently looking at production: {:?}", derived_production_elem);
                if random_number <= derived_production_elem.0 {/*println!("I have chosen: {:?}",derived_production_elem);*/ chosen_index = index; break;}
                
                for next_index in index+1..derivation.len()
                {
                    //println!("Vor Erhöhung {:?}", derivation);
                    derivation[next_index].0 += (derived_production_elem.0 as f32) / ((derivation.len()-1) as f32);
                    //println!("Nach Erhöhung {:?}", derivation);
                }
            }

            out = format!("{}{}",out,derivation[chosen_index].1);

        }
        self.current_iteration_derivation = out.clone();
        Some(out)
    }
}
