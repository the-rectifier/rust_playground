use itertools::Itertools;
use petgraph::algo::toposort;
use petgraph::dot::Dot;
use petgraph::graph::{Graph, NodeIndex};
use rand::Rng;
use std::collections::HashMap;
use std::env;

fn create_deps(jobs: &Vec<u32>) -> Vec<Vec<u32>> {
    let mut deps: Vec<Vec<u32>> = Vec::with_capacity(jobs.len());

    // dep job 0
    deps.push(vec![]);
    // dep job 1
    deps.push(vec![0]);
    // dep job 2
    deps.push(vec![0]);
    // dep job 3
    deps.push(vec![0, 2]);
    // dep job 4
    deps.push(vec![1]);
    // dep job 5
    deps.push(vec![2, 4]);
    // dep job 6
    deps.push(vec![3]);
    // dep job 7
    deps.push(vec![5]);
    // dep job 8
    deps.push(vec![4]);
    // dep job 9
    deps.push(vec![6, 7, 8]);
    // dep job 10
    deps.push(vec![]);

    deps
}

fn create_graph(jobs: &Vec<u32>, deps: &Vec<Vec<u32>>) -> Graph<u32, ()> {
    // Create new directed graph
    let mut g: Graph<u32, ()> = Graph::new();

    // Add all nodes from jobs
    for job in jobs {
        g.add_node(*job);
    }

    // For each node create directed edges based on their dependencies
    for job in jobs {
        let dep = match deps.get(*job as usize) {
            Some(x) => x,
            None => panic!(),
        };

        if !dep.is_empty() {
            for d in dep {
                g.add_edge(
                    NodeIndex::new(*d as usize),
                    NodeIndex::new(*job as usize),
                    (),
                );
            }
        }
    }
    // println!("{}", g.is_directed());
    g
}

fn get_time(topo_g: Vec<u32>, times: Vec<u32>, deps: Vec<Vec<u32>>) -> HashMap<u32, u32> {
    // Create a hashmap to store the times
    let mut d: HashMap<u32, u32> = HashMap::new();

    // Iterate over the jobs
    for i in topo_g {
        // Get the dependencies for each job
        let deps = deps.get(i as usize).expect("Invalid Index!");
        // println!("Job {} has deps: {:?}", i, deps);
        // If the list of dependencies is empty insert the time for the current job
        // Since it can be scheduled to run at any given moment
        if deps.is_empty() {
            d.insert(i, *times.get(i as usize).expect("Couldn't get time!"));
        } else {
            // If not empty get the time for the dependable job
            // And find the maximum time from its dependencies
            let job_time = *times.get(i as usize).expect("Couldn't get time!");
            let max_dep = *deps
                .iter()
                .map(|x| d.get(x).expect("Unable to get time!"))
                .max()
                .expect("Unable to find max!");
            // println!("Job {} and max dep: {}", i, max_dep);
            // Update hashmap with final time for the current job Including Dependencies
            d.insert(i, job_time + max_dep);
        }
    }
    d
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // Create a RNG instance
    let mut rng = rand::thread_rng();
    // Create 11 jobs
    let jobs: Vec<u32> = (0..=10).collect();
    // Create the dependencies
    // Shold be hardcoded to avoid Cycles
    let deps = create_deps(&jobs);
    // Create the graph
    let g = create_graph(&jobs, &deps);
    // Create random times for each job
    let times: Vec<u32> = (0..jobs.len()).map(|_| rng.gen_range(1..9)).collect();
    // let times = vec![8, 4, 5, 5, 3, 8, 5, 6, 4, 5, 4];
    // Topo Sort the graph and extract the sorted graph
    let topo_g: Vec<u32> = match toposort(&g, None) {
        Ok(x) => x.iter().map(|x| x.index() as u32).collect(),
        Err(_) => panic!("Cycle detected"),
    };

    println!("Time for each Job: {:?}", times);
    // println!("{:?}", Dot::new(&g));
    println!("Topo Sort for graph: {:?}", topo_g);
    let times_jobs = get_time(topo_g, times, deps);

    // Print results
    for key in times_jobs.keys().sorted() {
        println!(
            "Job {} needs {} units of time",
            key,
            times_jobs.get(key).unwrap()
        );
    }

    if args.len() > 1 && args[1].as_str() == "-G" {
        println!(
            "\nTo visualize the Graph\nPaste this into http://viz-js.com:\n\n{}",
            format!("{:?}", Dot::new(&g)).replace("()", ""),
        );
    }
}
