// Made by: Luis Eduardo Rozante de Freitas Pereira
// github.com/LuisEduardoR

extern crate rand;
use std::io;
use rand::Rng;

// The range of the function: x,y E [-RANGE, RANGE].
const RANGE : f64 = 30.0;
// The number by wich mutation will be multiplied/divided when population is not improving.
const MUTATION_CHANGE_RATE : f64 = 1.25;
// The number of genrations without improvement to start changing the mutation value.
const GENS_TO_CHANGE_MUTATION : usize = 8;
// Max precision when using really low mutation multiplier.
const MAX_MUTATION_PRECISION : f64 = 0.000000000000000000000000000000001;

// Struct with the characteristics of a pop.
struct Pop {
    x : f64,
    y : f64,
}

// Implement pop functions. 
impl Pop {

    // Creates a new pop with randomized values.
    fn new () -> Pop {

        let pop_x : f64 = rand::thread_rng().gen();
        let pop_x = -RANGE + (2.0 * pop_x * RANGE);

        let pop_y : f64 = rand::thread_rng().gen();
        let pop_y = -RANGE + (2.0 * pop_y * RANGE);

        let pop = Pop {x : pop_x, y : pop_y};
        return pop;

    }

    // Function that will be used to calculate the fitness of this.

    // Can be visualized by using this commands on gnuplot:
    // gnuplot: f(x,y) = x**2 + x**2*y + y**2 + (128*sin((x+y)/4))**2
    // set xrange [-RANGE, RANGE]
    // set xrange [-RANGE, RANGE]
    // splot f(x,y)

    // Remember to change RANGE to the RANGE const of the program.
    fn get_fitness (&self) -> f64 {

        return self.x*self.x + self.x*self.x*self.y + self.y*self.y + (128.0*(((self.x+self.y)/4.0).sin())).powf(2.0);

    }

    // Generates a new pop from a crossover between other two.
    fn crossover (cur : &Pop, other : &Pop) -> Pop {

        let mut res_pop = Pop::new();

        res_pop.x = (cur.x + other.x)/2.0;
        res_pop.y = (cur.y + other.y)/2.0;

        return res_pop;

    }

    // Mutates this pop.
    fn mutate (&mut self, mut_mult : f64) {

        let mutation_x : f64 = rand::thread_rng().gen();
        let mutation_x = mut_mult * (2.0 * (0.5 - mutation_x));

        let mutation_y : f64 = rand::thread_rng().gen();
        let mutation_y = mut_mult * (2.0 * (0.5 - mutation_y));

        self.x += mutation_x;
        self.y += mutation_y;

        // Guarantees mutation will not generate out of range values.
        if self.x < -RANGE { self.x = -RANGE; }
        else if self.x > RANGE { self.x = RANGE; }

        if self.y < -RANGE { self.y = -RANGE; }
        else if self.y > RANGE { self.y = RANGE; }

    }
}

// Struct to store necessary information about the best pop.
struct Best {
    id_in_vector : usize,
    creation_gen : usize,
    fit : f64,
}

fn main () {

    // Receives the population size.
    println!("Enter population size: ");
    let mut population_size = String::new();
    // Receive as text.
	io::stdin().read_line(&mut population_size).expect("Failed to read!");
    // Converts to u32.
    let population_size : u32 = population_size.trim().parse().expect("Invalid input!");
    
    // Receives the desired amount of generations.
    println!("Enter amount of generations: ");
    let mut gen_amount = String::new();
	io::stdin().read_line(&mut gen_amount).expect("Failed to read!");
    let gen_amount : usize = gen_amount.trim().parse().expect("Invalid input!");

    // Receives the mutation multiplier.
    println!("Enter mutation multiplier: ");
    let mut initial_mutation = String::new();
	io::stdin().read_line(&mut initial_mutation).expect("Failed to read!");
    let initial_mutation : f64 = initial_mutation.trim().parse().expect("Invalid input!");

    // Creates the starting population.
    let mut population = Vec::new();

    for _n in 0.. population_size {

        // Create a new pop with a random x,y E [-RANGE, RANGE].
        let mut new_pop = Pop::new();
        population.push(new_pop);

    }

    // Stores current generation.
    let mut cur_gen : usize = 1;
        // Store the generation average to be displayed.
    let mut gen_average : f64;

    // Stores information about the best pop.
    let mut best = Best { id_in_vector : 0, creation_gen: 0, fit : -0x3f3f3f3f as f64};

    // The current value of the mutation.
    let mut cur_mutation : f64 = initial_mutation;
    // Stores if mutation should be divided instead of multiplied when being modfied. 
    let mut invert_mutation_state : bool = false;
    // Stores the max and min mutation value during execution to be displayed.
    let mut max_mut : f64 = -0x3f3f3f3f as f64;
    let mut min_mut : f64 = 0x3f3f3f3f as f64; 

    // Executes while the desired number of generations hasn't been achieved.
    while cur_gen <= gen_amount {

        // Reset the gen average counter.
        gen_average = 0.0;

        if cur_gen != 1 {

            // If the best has been the same for GENS_TO_CHANGE_MUTATION generations, attempts to modify mutation multiplier.
            if (cur_gen - best.creation_gen) % (GENS_TO_CHANGE_MUTATION + 1) == GENS_TO_CHANGE_MUTATION {

                // Updates min and  max mutation to be displayed later.
                if cur_mutation > max_mut {
                    max_mut = cur_mutation;
                }
                if cur_mutation < min_mut {
                    min_mut = cur_mutation;
                }

                // Resets the mutation and inverts the method from multiplication to division or the other
                // way around if either the mutation is bigger than the function range or smaller than the
                // max desired precision. 
                if cur_mutation > 2.0 * RANGE || cur_mutation < MAX_MUTATION_PRECISION { 
                    invert_mutation_state = !invert_mutation_state;
                    cur_mutation = initial_mutation;
                }

                // Stores if mutation should be divided instead of multiplied when being modfied. 
                if invert_mutation_state {
                    cur_mutation /= MUTATION_CHANGE_RATE;
                } else {
                    cur_mutation *= MUTATION_CHANGE_RATE;
                }
                
            }

            // Handles crossing and mutation for generations after the first.
            for n in 0.. population_size {

                // Never modifies the best.
                if n as usize != best.id_in_vector {

                    // Crosses current pop with the best and substitute it.
                    population[n as usize] = Pop::crossover(&population[n as usize], &population[best.id_in_vector]);
                    // Mutates the pop.
                    population[n as usize].mutate(cur_mutation);

                }                
            }
        }

        // Calculates the average fitness of this generation and updates the best.
        for n in 0.. population_size {

            let pop_fit = population[n as usize].get_fitness();
            
            gen_average += pop_fit;

            if pop_fit > best.fit {

                best = Best {id_in_vector : n as usize, creation_gen : cur_gen, fit : pop_fit};

                cur_mutation = initial_mutation;

            }
        }        

        // Gets the final average.
        gen_average /= population_size as f64;

        // Prints relevant data.
        println!("> Generation {}:", cur_gen);
        println!("Average fitness of the gen: {}", gen_average);
        println!("Best: {} ({},{}) with a fit of {}", best.id_in_vector + 1, population[best.id_in_vector].x, population[best.id_in_vector].y, best.fit);
        println!("Best since gen {}!", best.creation_gen);
        println!("Best one for {} generation(s)!", (cur_gen - best.creation_gen));
        println!("Current mutation: {}", cur_mutation);

        if max_mut != -0x3f3f3f3f as f64 && min_mut != 0x3f3f3f3f as f64 {
            println!("Min mutation: {} \nMax mutation: {}\n", min_mut, max_mut);
        }

        // Increases generation counter.
        cur_gen += 1;
    }
}