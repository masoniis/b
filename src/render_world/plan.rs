/*
The app should extract a mutable reference of the game world and the render world should have a resource that wraps the reference.


The extract phase has exclusive mutable access to the game world,but after that we pass the resource back and then the game world
can resume it's simulation while this thread worries about rendering.

The extract phase must properly insert all resources needed for rendering.

We use arc pointers in our mesh handles, and do a copy of all the handles to ensure they stay in memory. ARc reference are immutable
so during the extraction phase while the render world has exclusive access it creates a "time capsule" copy of the game world. This will
ensure a clean secure architecture.

Schedule phases:

Extract
Prepare
Queue
Render
Cleanup

*/
