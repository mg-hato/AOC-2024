# Day 15 - Boxes and Robot

## Common Logic

To solve both parts we make `box_prediction_model.rs` with the idea of scale. In part 1 and 2, the scale used is 1 and 2, respectively. The rest is the same.

Firstly, we take the input map and we scale it up (if scale is 1, effectively nothing happens, if scale is 2, then boxes, walls, empty spaces are of width 2, robot does not scale up).

The map is captured in a `map_state.rs`. The map state describes three things:
- Boxes and their left-most positions (if the box is of width 2, we capture the left-most field of the box)
- Non-wall space in a mapping format `P -> Option<ID>`, where for a given position `p` we associate it with whether it is occuppied by a box - in which case it is associated with `Some(id)` where `id` is ID of the box - or otherwise `None` if it is empty space unoccupied by a box. Note that if there exists a mapping for position `p` whether it is a box-occupied or not, that means it is a non-wall field.
- Robot current position.

We apply transforms on the map state to get next state. Transformation is effectively trying to move the robot in a certain direciton. That is done in the following stages:
1. We work out what boxes are "affected" by the move. We start with the robot and what is in front of it. If it moves a box, we add all the positions in fron of that box w.r.t. direction of movement, and kind of do it again in a loop until we run out of new boxes to check for. That gives us a list of boxes that will be updated.
2. Next step, is the move possible? Can the robot be moved i.e. is not going to go into a wall plus can all the boxes be moved? If so, we move onto the next step.
3. Generate the next map-state. Update the robot's next position and all the affected boxes are to be moved by unit-movement corresponding to the direction. That needs to be reflected in the box positions plus the non-wall space mapping.

Lastly, once we apply all the transformations to the initial map state in a fold-like manner, we calculate GPS score sum as described in the problem statement (using safe-sums etc)

## Part 1

Scale is 1.

## Part 2

Scale is 2.

## Time

Part 1: 87 ms

Part 2: 94 ms