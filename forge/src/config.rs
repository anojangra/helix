pub static STATEMENTS: [&str; 2] = ["<stmnt>", "<stmnt>::<code>"];

pub static STRATEGIES: [&str; 10] = [
	"hhv:<ticker>:<param>",
	"llv:<ticker>:<param>",
	"conupdays:<ticker>:<param>",
	"condowndays:<ticker>:<param>",
	"gapup:<ticker>:<param>",
	"gapdown:<ticker>:<param>",
	"stdeva:<ticker>:<param>",
	"stdevb:<ticker>:<param>",
	"stdevd:<ticker>:<param>",
	"stdevf:<ticker>:<param>",
];

pub fn strategies_length() -> i32 {
	STRATEGIES.len() as i32
}

pub fn statements_length() -> i32 {
	STATEMENTS.len() as i32
}

// Mutate probably of dna applied after crossover
pub static MUTATE_PROB: f32 = 0.20;

// The number of fittest chromosomes to evolve
pub static FITTEST: usize = 500;

pub static POPULATION_SIZE: i32 = 10000;
