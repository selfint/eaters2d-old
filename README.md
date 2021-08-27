# eaters2d

DNA-like evolution of 2d eating creatures.

## Parts

1. ECS - Using the bevy engine
2. DNA - Each sequence of 'genes' will correspond to a component that an entity can have

## Flow

Creatures evolve using a standard genetic algorithm that modifies their 'DNA'.

Systems from the ECS framework then act on all entities with respect to their components.

Creatures can 'unlock' new capabilities by creating a new sequence in their DNA that
an as-of-yet unused system acts on.

## Example

COMPONENTS:

- aa: vision
- bb: fins
- cc: random signal passed to fins
- cd: vision input signals pass to fins

```
Creature DNA: aabbcc
```

The creature has eyes and fins, but no connection between them.

Then a mutation occurs, the last `c` switches to an `d`.

```
Creature DNA: aabbcd
```

Now the creature's vision affects the creature's fins.
