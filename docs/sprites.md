Sprites
=======

# How to stack sprites?

Sometimes it will be useful to stack multiple sprites on top of each other. Spawn them separate or with a hierarchy can be done, but the important part is the translation value in the Transform; it has to be different in one of the two or more sprites. Like one has to be 1.0 and the other 2.0 for the second to be on top of the first.

# How to combine sprites?

Can be done with a hierarchy in which the parent is an SpatialBundle and the children are Sprites. The children will inherit the Transform from the parent, that can be useful to reduce redundant configurations.
