## Notes and Ideas, Links

https://www.youtube.com/watch?v=c1H92b_uLdU

- physics
- network
- audio (supercollider?)
- rendering (vulkan)

## TODO:

- networking
- concurrency model
- simulation
- sim replication
- with futures


## TODOs and notes on networking

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

### Knobs:
  - snapshot send rate (per second)
  - packet size
  - interpolation between snapshots in buffer
  - size of snapshot buffer
  - extrapolation of velocities, linear and angular
  - protocol (tcp/udp) - udp send/ack_send
  - data compression (none, zlib, compress)
