# Design document Graph Algorithm Visualizer
We want to be able to observe the execution of a graph algorithm.

## Requirements:

### GUI requirements:

- Interface to easily add/remove nodes
- Interface to easily connect two separated nodes (add an edge)
- Ability to update individually or in bulk edges to be directed/undirected
- Will display the process execution and alAso the relevant variables involved (e.g. the distances array)
- Import/export graphs (json, yaml, csv)

### Scalability requirements
- Working for graphs with codeforces constraints (n <= 1e5)

## Proposed solution:

Separation of concerns between the GUI and the Graph state. During the execution of an algorithm (with certain speed) the clock tick will trigger an update of the graph (based on the current execution status) on the GUI. In order to do this we need two things:
- Updating the view from a graph input (potential optimizations, spike here: caching)
- Gather snapshots mid-execution (nodes, edges, current_node, current_edge, distances, etc)


## Future improvements [P1]
- Add a naive step-by-step replay
- Optimize the step-by-step to be memory efficient
