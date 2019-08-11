# RandR-PhysicsEngine
Wasm library of a fast physicsEngine

The physics engine is implemented by Rust and a .wasm libaray provides the efficency in Javascript envrionment.

## Roadmap   
- [x] Stage 0: Proof of concept: Three bodies demo    **【Completed】 in 2019-04**   
- [ ] Stage 1: Collision enabled for convex polygon   **【In progress ..】**   
- [ ] Stage 2: Rotation enabled for convex polygon    **【Planned】**   
- [ ] Stage 3: Speed Optimization ## Planned      

Possible optimization options:   
1. Vectorized all shapes internally.
2. Add grid to a world
3. [maybe] multi-thread support

## Feature in consideration:
1. True curve support
2. GPU support



## Run the dome

You need `npm` to run the demo:   
```
git clone git@github.com:lirhtc/RandR-PhysicsEngine.git
cd RandR-PhysicsEngine/demo
npm i
npm start
```

Then you will see a new window with two buttons. Input the number of particles you want and hit `Go demo`.
2000 particles might be the up-limit for most PCs. 


