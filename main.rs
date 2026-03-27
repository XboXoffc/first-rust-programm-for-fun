const BEER_IN_BOX_COUNT: usize = 2;

fn main(){
    let mut beer_box = [0.33; BEER_IN_BOX_COUNT];
    let mut beer:f32 = 0.0;
    let slurp:f32 = 0.2;
    println!("у вас {} литров пива", beer);

    loop {
        if beer < slurp {
            (beer_box, beer) = get_beer(beer_box, beer);
        };

        if beer != 0.0 {
            beer = drink(beer, slurp);
        }
        else {
            println!("пива больше нигде нету(((");
            break;
        };

        if beer > 0.0{
            println!("вы хотите выпить еще {} литров пива", slurp);
        };
    };
}

fn get_beer(mut beer_box: [f32; BEER_IN_BOX_COUNT], mut beer:f32) -> ([f32; BEER_IN_BOX_COUNT], f32){

    for i in 0..beer_box.len() {

        if beer_box[i] != 0.0{

            beer = beer_box[i];
            println!("вы берете {} литров пива", beer_box[i]);

            beer_box[i] = 0.0;
            if beer_box[beer_box.len()-1] == 0.0 {
                println!("вы достали последнюю банку пива")
            };

            break;
        };
    };

    return (beer_box, beer);
}

fn drink(mut beer: f32, slurp: f32) -> f32{
    if beer > slurp {
        beer -= slurp;
        println!("вы глотнули {} литров пива, у вас теперь {} литров пива", slurp, beer);
    }
    else{
        println!("У вас недостаточно пива((\nосталось где то {} литров, но вы допиваете без удовольствия(((", beer);
        beer = 0.0
    };
    return beer;
}