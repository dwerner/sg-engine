# something good...

- physics
- network
- audio (supercollider?)
- rendering (vulkan)

## TODO:

- TDD
- rendering start (vulkan)
- networking
- concurrency model
- simulation
- sim replication
- * with futures

## Stuff

### Idea: arbitrary model support through observables (a framework for simulation state sync?)

#### Synchronization model

  http://gafferongames.com/networked-physics/snapshot-compression/

  https://github.com/rygorous/gaffer_net/blob/master/main.cpp

  Target bandwidth range vs Actual bandwidth use

  - separate sync mechanisms for separate properties
  - prioritization of sync for different properties
  - adaptive sync methodology
  - express changing values as rate of change for interp
  - trans Simuation migration of objects
  - support simulation-level persistence (binary file, and maybe redis?)
  - property bounding (limit range, define quantization)
  - custom property serialization traits (e.g. quaternion's 'smallest three')
  - delta compression - send only what has changed
  - arbitrary precision types (like varint)
  - desync handling
  
  Knobs:
  - snapshot send rate (per second)
  - packet size
  - interpolation between snapshots in buffer
  - size of snapshot buffer
  - extrapolation of velocities, linear and angular
  - protocol (tcp/udp) - udp send/ack_send
  - data compression (none, zlib, compress)

  Detections:
  - snapshot length in bytes
  - bandwidth
  - latency
  - packet loss

  Deterministic Lock-step
  Snapshots and Interpolation (send all state)
  State synchronization

  p2p vs client/server

  Gameworld = [ x x x x x x ] ==> [ x x x x x y ] === [ 6x, x->y ]

  more scrap:
  		- sync priority (level of detail for syncs)
  			- near points of interest (high-low) etc


  'Object model'

  SimSync
   \-> Schedule
  	  \-> Object
  		  \-> Object
  			  \-> ...

