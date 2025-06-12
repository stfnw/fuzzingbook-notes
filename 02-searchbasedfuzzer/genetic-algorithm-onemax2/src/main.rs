// SPDX-FileCopyrightText: 2025 stfnw
// SPDX-License-Identifier: MIT

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
        // Selection: Sort decreasing by fitness and select best individuals
        // (here: by elitism).
        population
            .0
            .sort_by_key(|ind| std::cmp::Reverse(ind.fitness()));
        let mut new_population = population.0[0..population_size / 2].to_vec();

        while new_population.len() < population_size {
            // Crossover.
            let (parent1, parent2) = select_parents(rng, &population);
            let mut child = crossover(rng, &parent1, &parent2);

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

/// Select two random individuals from a population.
fn select_parents(rng: &mut rng::Rng, population: &Population) -> (Individual, Individual) {
    let parent1 = rng.choice(&population.0);
    let parent2 = rng.choice(&population.0);
    (parent1.clone(), parent2.clone())
}

/// One-point crossover between individual vectors.
fn crossover(rng: &mut rng::Rng, parent1: &Individual, parent2: &Individual) -> Individual {
    assert!(parent1.genome.len() == parent2.genome.len());
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
}

/// Randomly flip a bit according to the mutation rate.
fn mutate(rng: &mut rng::Rng, individual: &mut Individual, mutation_rate: f64) {
    for gene in &mut individual.genome {
        if rng.f64() < mutation_rate {
            *gene = !*gene; // Flip the gene
        }
    }
}
