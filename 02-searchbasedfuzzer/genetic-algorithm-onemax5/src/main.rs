// SPDX-FileCopyrightText: 2025 Original python code: Distributed Evolutionary Algorithms in Python (DEAP) https://github.com/DEAP/deap
// SPDX-FileCopyrightText: 2025 Rust translation and adaptation: stfnw
//
// SPDX-License-Identifier: LGPL-3.0-only

mod rng;

fn main() {
    let mut rng = rng::Rng::seeded(42);
    let individual = genetic_algorithm(&mut rng);
    println!("Best Individual: {}", individual);
}

/// Run the genetic algorithm and return the best evolved individual.
fn genetic_algorithm(rng: &mut rng::Rng) -> Individual {
    /* Constants for the algorithm. ******************************************/
    let population_size: usize = 300;
    let genome_size: usize = 400;
    let select_tournament_size = 3;
    let crossover_rate: f64 = 0.5;
    let mutation_rate: f64 = 0.2;
    let mutation_rate_bitflip: f64 = 0.005;

    // Here we can for example run either a fixed number of generations, or
    // until the fitness value hits a maximum value that is "good enough".
    let generations: usize = 1000;
    // In this case the maximum possible fitness is the genome size.
    let good_enough_fitness: f64 = genome_size as f64;
    /*************************************************************************/

    // Generate new population of random individuals.
    let mut population = Population::new(rng, population_size, genome_size);

    for generation in 0..generations {
        // Selection.
        let mut new_population = select(rng, &population, population_size, select_tournament_size);

        // Crossover.
        for chunk in new_population.0.chunks_mut(2) {
            if let [parent1, parent2] = chunk {
                if rng.f64() < crossover_rate {
                    crossover(rng, parent1, parent2);
                }
            }
        }

        // Mutation.
        for mutant in new_population.0.iter_mut() {
            if rng.f64() < mutation_rate {
                mutate(rng, mutant, mutation_rate_bitflip);
            }
        }

        // Replace population with next generation / new population.
        population = new_population;

        // Print status.
        let best_fitness = population.0[0].fitness;
        println!(
            "Generation {:4}: Best Fitness = {}",
            generation, best_fitness
        );

        if best_fitness >= good_enough_fitness {
            break;
        }
    }

    // Return best individual.
    population.0[0].clone()
}

struct Population(Vec<Individual>);

impl Population {
    fn new(rng: &mut rng::Rng, population_size: usize, genome_size: usize) -> Self {
        Self(
            (0..population_size)
                .map(|_| Individual::new(rng, genome_size))
                .collect(),
        )
    }
}

#[derive(Debug, Clone)]
struct Individual {
    genome: Vec<bool>,
    fitness: f64,
}

impl std::fmt::Display for Individual {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for &bit in self.genome.iter() {
            write!(f, "{}", bit as usize)?;
        }
        Ok(())
    }
}

impl Individual {
    fn new(rng: &mut rng::Rng, genome_size: usize) -> Self {
        let genome: Vec<_> = (0..genome_size).map(|_| rng.bool()).collect();
        let fitness = Self::fitness(&genome);
        Individual { genome, fitness }
    }

    fn fitness(genome: &[bool]) -> f64 {
        genome.iter().filter(|&&gene| gene).count() as f64
    }
}

/// Select k random individuals from a population by tournament selection.
fn select(rng: &mut rng::Rng, population: &Population, k: usize, tournsize: usize) -> Population {
    let mut new_population = Vec::with_capacity(k);

    while new_population.len() < k {
        let choices: Vec<_> = (0..tournsize).map(|_| rng.choice(&population.0)).collect();
        new_population.push(
            choices
                .into_iter()
                .max_by(|a, b| a.fitness.partial_cmp(&b.fitness).unwrap())
                .unwrap()
                .clone(),
        );
    }

    Population(new_population)
}

/// One-point crossover between individual vectors.
fn crossover(rng: &mut rng::Rng, parent1: &mut Individual, parent2: &mut Individual) {
    assert!(parent1.genome.len() == parent2.genome.len());

    let genome_size = parent1.genome.len();
    let point = rng.range(1, genome_size as u64) as usize;

    // Swap bits before crossover point.
    for i in 0..point {
        (parent1.genome[i], parent2.genome[i]) = (parent2.genome[i], parent1.genome[i]);
    }

    // Recompute fitness after modification

    parent1.fitness = Individual::fitness(&parent1.genome);
    parent2.fitness = Individual::fitness(&parent2.genome);
}

/// Randomly flip a bit according to the mutation rate.
fn mutate(rng: &mut rng::Rng, individual: &mut Individual, mutation_rate_bitflip: f64) {
    for gene in &mut individual.genome {
        if rng.f64() < mutation_rate_bitflip {
            *gene = !*gene; // Flip the gene
        }
    }
}
