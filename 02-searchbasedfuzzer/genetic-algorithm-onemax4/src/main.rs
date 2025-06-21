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
    let population_size: usize = 100;
    let genome_size: usize = 200;
    let crossover_rate: f64 = 0.5;
    let mutation_rate: f64 = 0.01;

    // Here we can for example run either a fixed number of generations, or
    // until the fitness value hits a maximum value that is "good enough".
    let generations: usize = 1000;
    // In this case the maximum possible fitness is the genome size.
    let good_enough_fitness: usize = genome_size;
    /*************************************************************************/

    // Generate new population of random individuals.
    let mut population = Population::new(rng, population_size, genome_size);

    for generation in 0..generations {
        // Selection: fitter individuals are chosen with higher probability.
        let fitnesses: Vec<_> = population
            .0
            .iter()
            .map(|ind| ind.fitness() as f64)
            .collect();

        let mut new_population = Vec::with_capacity(population_size);

        while new_population.len() < population_size {
            // Selection.
            let parent1 = select(rng, &population, &fitnesses);
            let parent2 = select(rng, &population, &fitnesses);

            // Crossover.
            let mut child = crossover(rng, &parent1, &parent2, crossover_rate);

            // Mutation.
            mutate(rng, &mut child, mutation_rate);

            new_population.push(child);
        }

        // Replace population with next generation / new population.
        population = Population(new_population);

        // Print status.
        let best_fitness = population.0[0].fitness();
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

// TODO cache fitness as Option<f64>
#[derive(Debug, Clone)]
struct Individual {
    genome: Vec<bool>,
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
        let genome = (0..genome_size).map(|_| rng.bool()).collect();
        Individual { genome }
    }

    fn fitness(&self) -> usize {
        self.genome.iter().filter(|&&gene| gene).count()
    }
}

fn select(rng: &mut rng::Rng, population: &Population, fitnesses: &[f64]) -> Individual {
    rng.choice_w(&population.0, fitnesses).clone()
}

/// One-point crossover between individual vectors.
fn crossover(
    rng: &mut rng::Rng,
    parent1: &Individual,
    parent2: &Individual,
    crossover_rate: f64,
) -> Individual {
    assert!(parent1.genome.len() == parent2.genome.len());

    if rng.f64() < crossover_rate {
        // Actually do crossover.
        let genome_size = parent1.genome.len();
        let point = rng.int(genome_size as u64) as usize;

        let mut child_chromosome = Vec::with_capacity(genome_size);
        for i in 0..genome_size {
            if i < point {
                child_chromosome.push(parent1.genome[i]);
            } else {
                child_chromosome.push(parent2.genome[i]);
            }
        }

        Individual {
            genome: child_chromosome,
        }
    } else {
        // Just return parent1 unmodified.
        parent1.clone()
    }
}

/// Randomly flip a bit according to the mutation rate.
fn mutate(rng: &mut rng::Rng, individual: &mut Individual, mutation_rate: f64) {
    for gene in &mut individual.genome {
        if rng.f64() < mutation_rate {
            *gene = !*gene; // Flip the gene
        }
    }
}
