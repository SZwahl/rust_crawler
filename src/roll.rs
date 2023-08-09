use rand::Rng;

pub struct Roll {
    //size: u32,
    pub total: u32,
    pub individuals: String,
}

pub fn roll(input: &str) -> Roll {
    //split and get components
    let parts: Vec<&str> = input.split('d').collect();
    
    let num_dice: u32 = parts[0].trim().parse()
                                .expect("Bad roll format");
                        
    let dice_type: u32 = parts[1].trim().parse()
                                .expect("Bad roll format");

    //Gather up totals
    let mut total = 0;
    let mut indi_rolls: String = "".to_string();

    for _element in 0..num_dice
    {
        let this_roll = rand::thread_rng().gen_range(1..=dice_type);
        total += this_roll;
        indi_rolls = indi_rolls + this_roll.to_string().as_str() + " ";
    }

    //Package result
    let result: Roll = Roll {
        //size: num_dice,
        total: total,
        individuals: indi_rolls,
    };

    return result;
}