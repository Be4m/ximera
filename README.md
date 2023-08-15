# ximera
Visual simulator of chemical reactions written in Rust.

# Feature List

## Camera
- Have a functioning camera which locks onto the atom present in the scene, or if there are multiple,
  averages out their positions and locks onto that point.

- The camera should be capable of zooming-in and out.

## Atoms
- Have multiple modes of visual representation, one of them being low-poly ico-spherical meshes which work as
  a brief summary of the atom, that tell it's boundaries in 3D space and show relations with other atoms. This
  could be useful when inspecting large number of atoms or complex molecules.
  Other types of visual representation should go up in complexity, in their way including Bohr's model and eventually
  acurrate visualisation of Shrödinger's equations.

- Drag functionality. The user should be able to interact with atoms. One of the ways
  they could do that is by dragging the atoms around or into a desirable position using their mouse. Of course, that would require nice lagging-behind
  effect giving shaders :D.

- The atoms should interact with each other, or more properly said: react with one another. More on this in the [ReactionEngine](##reactionengine) section.

## ReactionEngine
- Break points. When reactions are taking place, the user can activate break point functionality, where the reaction engine
  will "break down" isolated reaction or multiple reactions step-by-step.

- Slow-mo. The speed at which reactions are occurring should be adjustable.

- Much more interesting stuff should be thought of here...
  
# RoadMap
- [X] Design basic software structure.
