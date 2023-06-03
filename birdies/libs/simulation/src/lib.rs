use lib_genetic_algorithm as ga;
use lib_neural_network as nn;

use nalgebra as na;
use rand::{Rng, RngCore};
use std::f32::consts::*;

use std::f32::consts::FRAC_PI_2;

/// Minimum speed of a bird.
///
/// Keeping it above zero prevents birds from getting stuck in one place.
const SPEED_MIN: f32 = 0.001;
const GENERATION_LENGTH: usize = 2500;
/// Maximum speed of a bird.
///
/// Keeping it "sane" prevents birds from accelerating up to infinity,
/// which makes the simulation... unrealistic :-)
const SPEED_MAX: f32 = 0.005;

/// Speed acceleration; determines how much the brain can affect bird's
/// speed during one step.
///
/// Assuming our bird is currently flying with speed=0.5, when the brain
/// yells "stop flying!", a SPEED_ACCEL of:
///
/// - 0.1 = makes it take 5 steps ("5 seconds") for the bird to actually
///         slow down to SPEED_MIN,
///
/// - 0.5 = makes it take 1 step for the bird to slow down to SPEED_MIN.
///
/// This improves simulation faithfulness, because - as in real life -
/// it's not possible to increase speed from 1km/h to 50km/h in one
/// instant, even if your brain very much wants to.
const SPEED_ACCEL: f32 = 0.2;

/// Ditto, but for rotation:
///
/// - 2 * PI = it takes one step for the bird to do a 360° rotation,
/// - PI = it takes two steps for the bird to do a 360° rotation,
///
/// I've chosen PI/2, because - as our motto goes - this value seems
/// to play nice.
const ROTATION_ACCEL: f32 = FRAC_PI_2;
const FOV_RANGE: f32 = 0.25;
const FOV_ANGLE: f32 = PI + FRAC_PI_4;
const CELLS: usize = 9;
pub struct Simulation {
    world: World,
    ga: ga::GeneticAlghoritm<ga::RouletteWheelSelection>,
    age: usize,
}
pub struct AnimalIndividual {
    fitness: f32,
    chromosome: ga::Chromosome,
}
pub struct World {
    animals: Vec<Animal>,
    foods: Vec<Food>,
}
pub struct Brain {
    nn: nn::Network,
}

pub struct Eye {
    fov_range: f32,
    fov_angle: f32,
    cells: usize,
}
pub struct Animal {
    position: na::Point2<f32>,
    rotation: na::Rotation2<f32>,
    speed: f32,
    eye: Eye,
    brain: Brain,
    satiation: usize,
}
pub struct Food {
    position: na::Point2<f32>,
}

impl Simulation {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        let world = World::random(rng);

        let ga = ga::GeneticAlghoritm::new(
            ga::RouletteWheelSelection,
            ga::UniformCrossover,
            ga::GaussianMutation::new(0.01, 0.3),
            // ---------------------- ^--^ -^-^
            // | Chosen with a bit of experimentation.
            // |
            // | Higher values can make the simulation more chaotic,
            // | which - a bit counterintuitively - might allow for
            // | it to discover *better* solutions; but the trade-off
            // | is that higher values might also cause current, good
            // | enough solutions to be discarded.
            // ---
        );

        Self { world, ga, age: 0 }
    }

    pub fn step(&mut self, rng: &mut dyn RngCore) -> Option<ga::Statistics> {
        self.process_collisions(rng);
        self.process_brains();
        self.process_movements();
        self.age += 1;

        if self.age > GENERATION_LENGTH {
            Some(self.evolve(rng))
        } else {
            None
        }
    }

    pub fn train(&mut self, rng: &mut dyn RngCore) -> ga::Statistics {
        loop {
            if let Some(summary) = self.step(rng) {
                return summary;
            }
        }
    }

    fn evolve(&mut self, rng: &mut dyn RngCore) -> ga::Statistics {
        self.age = 0;

        // Step 1: Prepare birdies to be sent into the genetic algorithm
        let current_population: Vec<_> = self
            .world
            .animals
            .iter()
            .map(AnimalIndividual::from_animal)
            .collect();

        // Step 2: Evolve birdies
        let (evolved_population, stats) = self.ga.evolve(rng, &current_population);

        // Step 3: Bring birdies back from the genetic algorithm
        self.world.animals = evolved_population
            .into_iter()
            .map(|individual| individual.into_animal(rng))
            .collect();

        // Step 4: Restart foods
        //
        // (this is not strictly necessary, but it allows to easily spot
        // when the evolution happens - so it's more of a UI thing.)
        for food in &mut self.world.foods {
            food.position = rng.gen();
        }

        stats
    }

    fn process_brains(&mut self) {
        for animal in &mut self.world.animals {
            let vision =
                animal
                    .eye
                    .process_vision(animal.position, animal.rotation, &self.world.foods);

            let response = animal.brain.nn.propagate(vision);

            // ---
            // | Limits number to given range.
            // -------------------- v---v
            let speed = response[0].clamp(-SPEED_ACCEL, SPEED_ACCEL);

            let rotation = response[1].clamp(-ROTATION_ACCEL, ROTATION_ACCEL);

            // Our speed & rotation here are *relative* - that is: when
            // they are equal to zero, what the brain says is "keep
            // flying as you are now", not "stop flying".
            //
            // Both values being relative is crucial, because our bird's
            // brain doesn't know its own speed and rotation*, meaning
            // that it fundamentally cannot return absolute values.
            //
            // * they'd have to be provided as separate inputs to the
            //   neural network, which would make the evolution process
            //   waaay longer, if even possible.

            animal.speed = (animal.speed + speed).clamp(SPEED_MIN, SPEED_MAX);

            animal.rotation = na::Rotation2::new(animal.rotation.angle() + rotation);

            // (btw, there is no need for ROTATION_MIN or ROTATION_MAX,
            // because rotation automatically wraps from 2*PI back to 0 -
            // we've already witnessed that when we were testing eyes,
            // inside `mod different_rotations { ... }`.)
        }
    }

    fn process_collisions(&mut self, rng: &mut dyn RngCore) {
        for animal in &mut self.world.animals {
            for food in &mut self.world.foods {
                let distance = na::distance(&animal.position, &food.position);

                if distance <= 0.01 {
                    animal.satiation += 1;
                    food.position = rng.gen();
                }
            }
        }
    }

    fn process_movements(&mut self) {
        for animal in &mut self.world.animals {
            animal.position += animal.rotation * na::Vector2::new(0.0, animal.speed);

            animal.position.x = na::wrap(animal.position.x, 0.0, 1.0);
            animal.position.y = na::wrap(animal.position.y, 0.0, 1.0);
        }
    }
    pub fn world(&self) -> &World {
        &self.world
    }
}

impl World {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        let animals = (0..40).map(|_| Animal::random(rng)).collect();

        let foods = (0..60).map(|_| Food::random(rng)).collect();

        // ^ Our algorithm allows for animals and foods to overlap, so
        // | it's hardly ideal - but good enough for our purposes.
        // |
        // | A more complex solution could be based off of e.g.
        // | Poisson disk sampling:
        // |
        // | https://en.wikipedia.org/wiki/Supersampling
        // ---

        Self { animals, foods }
    }

    pub fn animals(&self) -> &[Animal] {
        &self.animals
    }

    pub fn foods(&self) -> &[Food] {
        &self.foods
    }
}

impl Animal {
    fn from_chromosome(chromosome: ga::Chromosome, rng: &mut dyn RngCore) -> Self {
        let eye = Eye::default();
        let brain = Brain::from_chromosome(chromosome, &eye);

        Self::new(eye, brain, rng)
    }

    pub fn random(rng: &mut dyn RngCore) -> Self {
        let eye = Eye::default();
        let brain = Brain::random(rng, &eye);

        Self::new(eye, brain, rng)
    }

    fn as_chromosome(&self) -> ga::Chromosome {
        // We evolve only our birds' brains, but technically there's no
        // reason not to simulate e.g. physical properties such as size.
        //
        // If that was to happen, this function could be adjusted to
        // return a longer chromosome that encodes not only the brain,
        // but also, say, birdie's color.

        self.brain.as_chromosome()
    }

    fn new(eye: Eye, brain: Brain, rng: &mut dyn RngCore) -> Self {
        Self {
            position: rng.gen(),
            rotation: rng.gen(),
            speed: 0.002,
            eye,
            brain,
            satiation: 0,
        }
    }

    pub fn position(&self) -> na::Point2<f32> {
        // ------------------ ^
        // | No need to return a reference, because na::Point2 is Copy.
        // |
        // | (meaning: it's so small that cloning it is cheaper than
        // | messing with references.)
        // |
        // | Of course you don't have to memorize which types are Copy
        // | and which aren't - if you accidentally return a reference
        // | to a type that's Copy, rust-clippy will point it out and
        // | suggest a change :-)
        // ---

        self.position
    }

    pub fn rotation(&self) -> na::Rotation2<f32> {
        self.rotation
    }
}

impl Food {
    pub fn random(rng: &mut dyn RngCore) -> Self {
        Self {
            position: rng.gen(),
        }
    }

    pub fn position(&self) -> na::Point2<f32> {
        self.position
    }
}

impl Eye {
    // FOV_RANGE, FOV_ANGLE & CELLS are the values we'll use during
    // simulation - but being able to create an arbitrary eye will
    // come handy during the testing:
    fn new(fov_range: f32, fov_angle: f32, cells: usize) -> Self {
        assert!(fov_range > 0.0);
        assert!(fov_angle > 0.0);
        assert!(cells > 0);

        Self {
            fov_range,
            fov_angle,
            cells,
        }
    }

    pub fn cells(&self) -> usize {
        self.cells
    }

    pub fn process_vision(
        &self,
        position: na::Point2<f32>,
        rotation: na::Rotation2<f32>,
        foods: &[Food],
    ) -> Vec<f32> {
        let mut cells = vec![0.0; self.cells];
        for food in foods {
            let vec = food.position - position;
            let dist = vec.norm();

            if dist >= self.fov_range {
                continue;
            }

            // in disguise.)
            let angle = na::Rotation2::rotation_between(&na::Vector2::y(), &vec).angle();

            // Because our bird is *also* rotated, we have to include its
            // rotation too:
            let angle = angle - rotation.angle();

            // Rotation is wrapping (from -PI to PI), that is:
            //
            //   = angle of 2*PI
            //   = angle of PI    (because 2*PI >= PI)
            //   = angle of 0     (          PI >= PI)
            //                    (           0 < PI)
            //
            //  angle of 2*PI + PI/2
            //  = angle of 1*PI + PI/2  (because 2*PI + PI/2 >= PI)
            //  = angle of PI/2         (          PI + PI/2 >= PI)
            //                          (               PI/2 < PI)
            //
            //  angle of -2.5*PI
            //  = angle of -1.5*PI  (because -2.5*PI <= -PI)
            //  = angle of -0.5*PI  (        -1.5*PI <= -PI)
            //                      (        -0.5*PI > -PI)
            //
            // Intuitively:
            //
            // - when you rotate yourself twice around the axis, it's the
            //   same as if you rotated once, as if you've never rotated
            //   at all.
            //
            //   (your bony labyrinth might have a different opinion tho.)
            //
            // - when you rotate by 90° and then by 360°, it's the same
            //   as if you rotated only by 90° (*or* by 270°, just in the
            //   opposite direction).
            let angle = na::wrap(angle, -PI, PI);

            // If current angle is outside our birdie's field of view, jump
            // to the next food
            if angle < -self.fov_angle / 2.0 || angle > self.fov_angle / 2.0 {
                continue;
            }

            // - an angle of self.fov_angle means "the ending of the FOV".
            let angle = angle + self.fov_angle / 2.0;

            // Since this angle is now in range <0,FOV_ANGLE>, by dividing it by
            // FOV_ANGLE, we transform it to range <0,1>.
            //
            // The value we get can be treated as a percentage, that is:
            //
            // - 0.2 = the food is seen by the "20%-th" eye cell
            //         (practically: it's a bit to the left)
            //
            // - 0.5 = the food is seen by the "50%-th" eye cell
            //         (practically: it's in front of our birdie)
            //
            // - 0.8 = the food is seen by the "80%-th" eye cell
            //         (practically: it's a bit to the right)
            let cell = angle / self.fov_angle;

            // With cell in range <0,1>, by multiplying it by the number of
            // cells we get range <0,CELLS> - this corresponds to the actual
            // cell index inside our `cells` array.
            //
            // Say, we've got 8 eye cells:
            // - 0.2 * 8 = 20% * 8 = 1.6 ~= 1 = second cell (indexing from 0!)
            // - 0.5 * 8 = 50% * 8 = 4.0 ~= 4 = fifth cell
            // - 0.8 * 8 = 80% * 8 = 6.4 ~= 6 = seventh cell
            let cell = cell * (self.cells as f32);

            // Our `cell` is of type `f32` - before we're able to use it to
            // index an array, we have to convert it to `usize`.
            //
            // We're also doing `.min()` to cover an extreme edge case: for
            // cell=1.0 (which corresponds to a food being maximally to the
            // right side of our birdie), we'd get `cell` of `cells.len()`,
            // which is one element *beyond* what the `cells` array contains
            // (its range is <0, cells.len()-1>).
            //
            // Being honest, I've only caught this thanks to unit tests we'll
            // write in a moment, so if you consider my explanation
            // insufficient (pretty fair!), please feel free to drop the
            // `.min()` part later and see which tests fail - and why :-)
            let cell = (cell as usize).min(cells.len() - 1);
            let energy = (self.fov_range - dist) / self.fov_range;

            cells[cell] += energy;
        }
        cells
    }
}

impl Default for Eye {
    fn default() -> Self {
        Self::new(FOV_RANGE, FOV_ANGLE, CELLS)
    }
}

impl AnimalIndividual {
    pub fn from_animal(animal: &Animal) -> Self {
        Self {
            fitness: animal.satiation as f32,
            chromosome: animal.as_chromosome(),
        }
    }

    pub fn into_animal(self, rng: &mut dyn RngCore) -> Animal {
        Animal::from_chromosome(self.chromosome, rng)
    }
}

impl ga::Individual for AnimalIndividual {
    fn create(chromosome: ga::Chromosome) -> Self {
        Self {
            fitness: 0.0,
            chromosome,
        }
    }

    fn chromosome(&self) -> &ga::Chromosome {
        &self.chromosome
    }

    fn fitness(&self) -> f32 {
        self.fitness
    }
}

impl Brain {
    fn from_chromosome(chromosome: ga::Chromosome, eye: &Eye) -> Self {
        Self {
            nn: nn::Network::from_weights(&Self::topology(eye), chromosome),
        }
    }
    pub fn random(rng: &mut dyn RngCore, eye: &Eye) -> Self {
        Self {
            nn: nn::Network::random(rng, &Self::topology(eye)),
        }
    }

    fn as_chromosome(&self) -> ga::Chromosome {
        self.nn.weights().collect()
    }

    fn topology(eye: &Eye) -> [nn::LayerTopology; 3] {
        [
            nn::LayerTopology {
                neurons: eye.cells(),
            },
            nn::LayerTopology {
                neurons: 2 * eye.cells(),
            },
            nn::LayerTopology { neurons: 2 },
        ]
    }
}
