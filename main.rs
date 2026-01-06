fn find_short(s: &str) -> u32 
{
  let words: Vec<&str> = s.split_whitespace().collect();
  let lens: Vec<u32> = words
                        .iter()
                        .map(|str_slc| str_slc.len() as u32)
                        .collect();
            
    return *lens.iter().min().unwrap();
}
