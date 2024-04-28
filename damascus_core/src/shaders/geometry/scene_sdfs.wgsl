// Copyright 2024 by Owen Bulka.
// All rights reserved.
// This file is released under the "MIT License Agreement".
// Please see the LICENSE file that is included as part of this package.


fn find_nearest_descendant(
    position: vec3<f32>,
    hit_tolerance: f32,
    earliest_ancestor_index: u32,
    family: ptr<function, Primitive>,
) -> f32 {
    // Get the distance to the topmost primitive
    var distance_to_family: f32 = distance_to_textured_primitive(position, family);

    // Check if the topmost primmitive is a bounding volume
    var family_is_bounded: bool = bool((*family).modifiers & BOUNDING_VOLUME);
    // And if we are outside that bounding volume if so
    var out_of_familys_boundary: bool = (
        family_is_bounded && distance_to_family > hit_tolerance
    );

    // If we are inside the bounding volume we don't want the initial distance
    // to be to the boundary, so set it to the maximum distance instead.
    distance_to_family = select(
        distance_to_family,
        _render_parameters.max_distance,
        family_is_bounded && !out_of_familys_boundary,
    );

    // Track the number of descendants that should be, and have been, processed
    var num_descendants_to_process: u32 = (*family).num_descendants;
    // If are outside the boundary we want to return immediately
    // but failing the existing loop break conditions benchmarked faster
    // than adding an additional if statement, or adding
    // `out_of_familys_boundary` to the condition
    var descendants_processed: u32 = select(
        0u,
        num_descendants_to_process,
        out_of_familys_boundary,
    );

    // Track the index of the parent and current child we are processing
    var current_parent_index: u32 = earliest_ancestor_index;
    var child_index: u32 = current_parent_index + 1u + select(
        0u,
        num_descendants_to_process,
        out_of_familys_boundary,
    );

    // Allow us to set the next parent while we are looping rather than
    // searching for it after each level of children
    var next_parent_index: u32 = current_parent_index;
    var searching_for_next_parent: bool = true;

    // Create a primitive to re-use as our child
    var child: Primitive;

    // Process all immediate children breadth first
    // Then step into children and process all grandchildren breadth first
    // continuing until all relatives are processed
    loop {
        // If there are no more direct children
        if child_index > current_parent_index + (*family).num_descendants {
            // If all children & grandchildren have been processed, stop
            if num_descendants_to_process <= descendants_processed {
                break;
            }

            // Otherwise, continue until all grandchildren have been processed.
            // The next parent will either be the one we found at the current
            // depth, or at most the current child index
            current_parent_index = select(
                next_parent_index,
                child_index,
                searching_for_next_parent,
            );
            // Get the next parent and apply the current blended material
            *family = _primitives.primitives[current_parent_index];
            (*family).id = child.id;
            (*family).material = child.material;

            // Update the child index to point to the first child of the
            // new parent
            child_index = current_parent_index;

            // Reset this flag so we can find the next parent
            searching_for_next_parent = true;

            continue;
        }

        // Get and process the child, blending the material and distance
        // in the chosen manner
        child = _primitives.primitives[child_index];
        var distance_to_child: f32 = distance_to_textured_primitive(position, &child);

        var child_is_bounding_volume: bool = bool(child.modifiers & BOUNDING_VOLUME);
        var out_of_childs_boundary: bool = (
            child_is_bounding_volume
            && distance_to_child > hit_tolerance
        );

        // If this child has children record its index to use as the
        // next parent. This ensures the first, deepest child with
        // children is processed first
        var found_next_parent: bool = (
            searching_for_next_parent
            && child.num_descendants > 0u
            && !out_of_childs_boundary
        );
        next_parent_index = select(next_parent_index, child_index, found_next_parent);
        searching_for_next_parent = select(
            searching_for_next_parent,
            false,
            found_next_parent,
        );

        if out_of_childs_boundary {
            // If we are outside the childs boundary use the distance to the
            // boundary in a simple union with our current distance
            // and mark all children as processed
            descendants_processed += child.num_descendants;

            var child_closest: bool = distance_to_child < distance_to_family;
            distance_to_family = select(
                distance_to_family,
                distance_to_child,
                child_closest,
            );
            select_primitive(family, &child, child_closest);
        } else if !child_is_bounding_volume {
            // Otherwise, as long as the child isn't a bounding volume,
            // we can perform the normal blending operation
            distance_to_family = blend_primitives(
                distance_to_family,
                distance_to_child,
                family,
                &child,
            );
        }

        // Increment the counter tracking the number of children
        // processed so far
        descendants_processed++;
        // Skip the descendants of this child, for now
        child_index += child.num_descendants;

        continuing {
            // Continue to the next child
            child_index++;
        }
    }

    return distance_to_family;
}


fn find_nearest_primitive(
    position: vec3<f32>,
    pixel_footprint: f32,
    closest_primitive: ptr<function, Primitive>,
) {
    var distance_to_scene: f32 = _render_parameters.max_distance;
    var primitive: Primitive;
    var primitives_processed = 0u;
    var hit_tolerance: f32 = _render_parameters.hit_tolerance + pixel_footprint;
    while primitives_processed < _scene_parameters.num_primitives {
        primitive = _primitives.primitives[primitives_processed];
        var num_descendants: u32 = primitive.num_descendants;

        var signed_distance_field: f32 = find_nearest_descendant(
            position,
            hit_tolerance,
            primitives_processed,
            &primitive,
        );

        var primitive_is_new_closest: bool = (
            abs(signed_distance_field) < abs(distance_to_scene)
        );
        distance_to_scene = select(
            distance_to_scene,
            signed_distance_field,
            primitive_is_new_closest,
        );
        select_primitive(
            closest_primitive,
            &primitive,
            primitive_is_new_closest,
        );

        // Skip all descendants, they were processed in the
        // `find_nearest_descendant` function
        primitives_processed += num_descendants + 1u;
    }
    // Ensure the number of descendants is that of the closest primitive
    var unmodified_closest_primitive: Primitive = (
        _primitives.primitives[(*closest_primitive).id - 1u]
    );
    (*closest_primitive).num_descendants = unmodified_closest_primitive.num_descendants;
    (*closest_primitive).dimensional_data = unmodified_closest_primitive.dimensional_data;
    (*closest_primitive).transform = unmodified_closest_primitive.transform;
}


fn distance_to_descendants(
    position: vec3<f32>,
    hit_tolerance: f32,
    earliest_ancestor_index: u32,
    family: ptr<function, Primitive>,
) -> f32 {
    // Get the distance to the topmost primitive
    var distance_to_family: f32 = distance_to_primitive(position, family);

    // Check if the topmost primmitive is a bounding volume
    var family_is_bounded: bool = bool((*family).modifiers & BOUNDING_VOLUME);
    // And if we are outside that bounding volume if so
    var out_of_familys_boundary: bool = (
        family_is_bounded && distance_to_family > hit_tolerance
    );

    // If we are inside the bounding volume we don't want the initial distance
    // to be to the boundary, so set it to the maximum distance instead.
    distance_to_family = select(
        distance_to_family,
        _render_parameters.max_distance,
        family_is_bounded && !out_of_familys_boundary,
    );

    // Track the number of descendants that should be, and have been, processed
    var num_descendants_to_process: u32 = (*family).num_descendants;
    // If are outside the boundary we want to return immediately
    // but failing the existing loop break conditions benchmarked faster
    // than adding an additional if statement, or adding
    // `out_of_familys_boundary` to the condition
    var descendants_processed: u32 = select(
        0u,
        num_descendants_to_process,
        out_of_familys_boundary,
    );

    // Track the index of the parent and current child we are processing
    var current_parent_index: u32 = earliest_ancestor_index;
    var child_index: u32 = current_parent_index + 1u + select(
        0u,
        num_descendants_to_process,
        out_of_familys_boundary,
    );

    // Allow us to set the next parent while we are looping rather than
    // searching for it after each level of children
    var next_parent_index: u32 = current_parent_index;
    var searching_for_next_parent: bool = true;

    // Create a primitive to re-use as our child
    var child: Primitive;

    // Process all immediate children breadth first
    // Then step into children and process all grandchildren breadth first
    // continuing until all relatives are processed
    loop {
        // If there are no more direct children
        if child_index > current_parent_index + (*family).num_descendants {
            // If all children & grandchildren have been processed, stop
            if num_descendants_to_process <= descendants_processed {
                break;
            }

            // Otherwise, continue until all grandchildren have been processed.
            // The next parent will either be the one we found at the current
            // depth, or at most the current child index
            current_parent_index = select(
                next_parent_index,
                child_index,
                searching_for_next_parent,
            );
            // Get the next parent and apply the current blended material
            *family = _primitives.primitives[current_parent_index];

            // Update the child index to point to the first child of the
            // new parent
            child_index = current_parent_index;

            // Reset this flag so we can find the next parent
            searching_for_next_parent = true;

            continue;
        }

        // Get and process the child, blending the material and distance
        // in the chosen manner
        child = _primitives.primitives[child_index];
        var distance_to_child: f32 = distance_to_primitive(position, &child);

        var child_is_bounding_volume: bool = bool(child.modifiers & BOUNDING_VOLUME);
        var out_of_childs_boundary: bool = (
            child_is_bounding_volume
            && distance_to_child > hit_tolerance
        );

        // If this child has children record its index to use as the
        // next parent. This ensures the first, deepest child with
        // children is processed first
        var found_next_parent: bool = (
            searching_for_next_parent
            && child.num_descendants > 0u
            && !out_of_childs_boundary
        );
        next_parent_index = select(next_parent_index, child_index, found_next_parent);
        searching_for_next_parent = select(
            searching_for_next_parent,
            false,
            found_next_parent,
        );

        if out_of_childs_boundary {
            // If we are outside the childs boundary use the distance to the
            // boundary in a simple union with our current distance
            // and mark all children as processed
            descendants_processed += child.num_descendants;
            distance_to_family = min(distance_to_family, distance_to_child);
        } else if !child_is_bounding_volume {
            // Otherwise, as long as the child isn't a bounding volume,
            // we can perform the normal blending operation
            distance_to_family = blend_distances(
                distance_to_family,
                distance_to_child,
                family,
            );
        }

        // Increment the counter tracking the number of children
        // processed so far
        descendants_processed++;
        // Skip the descendants of this child, for now
        child_index += child.num_descendants;

        continuing {
            // Continue to the next child
            child_index++;
        }
    }

    return distance_to_family;
}


fn signed_distance_to_scene(
    position: vec3<f32>,
    pixel_footprint: f32,
) -> f32 {
    var distance_to_scene: f32 = _render_parameters.max_distance;
    var primitive: Primitive;
    var primitives_processed = 0u;
    var hit_tolerance: f32 = _render_parameters.hit_tolerance + pixel_footprint;
    while primitives_processed < _scene_parameters.num_primitives {
        primitive = _primitives.primitives[primitives_processed];
        var num_descendants: u32 = primitive.num_descendants;

        var signed_distance_field: f32 = distance_to_descendants(
            position,
            hit_tolerance,
            primitives_processed,
            &primitive,
        );

        var primitive_is_new_closest: bool = (
            abs(signed_distance_field) < abs(distance_to_scene)
        );
        distance_to_scene = select(
            distance_to_scene,
            signed_distance_field,
            primitive_is_new_closest,
        );

        // Skip all descendants, they were processed in the
        // `distance_to_descendants` function
        primitives_processed += num_descendants + 1u;
    }

    return distance_to_scene;
}
