# Suna Solvo

A project to assist my attempts to make a paraboloid out of cardboard or other readily available objects.

## Technologies

* Rust

## Background

A paraboloid mirror reflects light to a single point - perfect for a solar oven! There are some excellent videos available demonstrating how to make a paraboloid mirror, but they require material and equipment which is very hard to acquire. Can I find a way to make a paraboloid mirror out of cardboard and tinfoil, or other readily available material?

Note that for a solar oven, my paraboloid mirror does not have to collect the heat of the sun to a single point. A small volume is acceptable, because the paraboloid mirror can be used with a cooking pot.

## Methods

### Cone Rings

Approximate a paraboloid by a series of cone slices. The first "cone" is actually a flat plane, and the "cone slice" is a circle.

`
 |     |
  \   /
   `-'
`

### Extend a Parabola to a Plane

Approximate a paraboloid by a series of planes sections. Each plane section is cut to a computed `V` shape. If the `V` shapes are created correctly, when they are joined along the edges, the centre of each individual plane section should be pulled up to follow a parabola.

`
 \  |  /
  \ | /
   \|/
 ---\---
   /|\
  / | \
 /  |  \
`

