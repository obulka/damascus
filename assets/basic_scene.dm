graph:
  nodes:
  - value: null
    version: 0
  - value:
      id:
        idx: 1
        version: 3
      label: camera
      inputs:
      - - focal_length
        - idx: 19
          version: 3
      - - focal_distance
        - idx: 18
          version: 3
      - - f_stop
        - idx: 17
          version: 3
      - - horizontal_aperture
        - idx: 16
          version: 3
      - - near_plane
        - idx: 15
          version: 3
      - - far_plane
        - idx: 14
          version: 3
      - - world_matrix
        - idx: 13
          version: 3
      - - enable_depth_of_field
        - idx: 12
          version: 3
      outputs:
      - - out
        - idx: 1
          version: 3
      user_data:
        template: Camera
    version: 3
  - value:
      id:
        idx: 2
        version: 3
      label: axis
      inputs:
      - - axis
        - idx: 23
          version: 3
      - - translate
        - idx: 22
          version: 3
      - - rotate
        - idx: 21
          version: 3
      - - uniform_scale
        - idx: 20
          version: 3
      outputs:
      - - out
        - idx: 2
          version: 3
      user_data:
        template: Axis
    version: 3
  - value:
      id:
        idx: 3
        version: 3
      label: axis
      inputs:
      - - axis
        - idx: 27
          version: 3
      - - translate
        - idx: 26
          version: 3
      - - rotate
        - idx: 25
          version: 3
      - - uniform_scale
        - idx: 24
          version: 3
      outputs:
      - - out
        - idx: 3
          version: 3
      user_data:
        template: Axis
    version: 3
  - value:
      id:
        idx: 4
        version: 1
      label: scene
      inputs:
      - - render_camera
        - idx: 11
          version: 3
      - - primitives
        - idx: 10
          version: 3
      - - lights
        - idx: 9
          version: 3
      - - atmosphere
        - idx: 8
          version: 3
      outputs:
      - - out
        - idx: 4
          version: 1
      user_data:
        template: Scene
    version: 1
  - value:
      id:
        idx: 5
        version: 1
      label: ray marcher
      inputs:
      - - scene
        - idx: 7
          version: 3
      - - max_distance
        - idx: 6
          version: 3
      - - max_ray_steps
        - idx: 5
          version: 3
      - - max_bounces
        - idx: 4
          version: 3
      - - hit_tolerance
        - idx: 3
          version: 3
      - - shadow_bias
        - idx: 2
          version: 3
      - - max_brightness
        - idx: 1
          version: 3
      - - seeds
        - idx: 28
          version: 1
      - - dynamic_level_of_detail
        - idx: 29
          version: 1
      - - equiangular_samples
        - idx: 30
          version: 1
      - - max_light_sampling_bounces
        - idx: 31
          version: 1
      - - sample_atmosphere
        - idx: 32
          version: 1
      - - light_sampling_bias
        - idx: 33
          version: 1
      - - secondary_sampling
        - idx: 34
          version: 1
      - - output_aov
        - idx: 35
          version: 1
      - - latlong
        - idx: 36
          version: 1
      outputs:
      - - out
        - idx: 5
          version: 1
      user_data:
        template: RayMarcher
    version: 1
  - value:
      id:
        idx: 6
        version: 1
      label: primitive
      inputs:
      - - siblings
        - idx: 37
          version: 1
      - - children
        - idx: 38
          version: 1
      - - material
        - idx: 39
          version: 1
      - - shape
        - idx: 40
          version: 1
      - - radius
        - idx: 41
          version: 1
      - - radii
        - idx: 42
          version: 1
      - - height
        - idx: 43
          version: 1
      - - hollow_radius
        - idx: 44
          version: 1
      - - hollow_height
        - idx: 45
          version: 1
      - - solid_angle
        - idx: 46
          version: 1
      - - width
        - idx: 47
          version: 1
      - - depth
        - idx: 48
          version: 1
      - - thickness
        - idx: 49
          version: 1
      - - corner_radius
        - idx: 50
          version: 1
      - - base
        - idx: 51
          version: 1
      - - normal
        - idx: 52
          version: 1
      - - negative_height
        - idx: 53
          version: 1
      - - positive_height
        - idx: 54
          version: 1
      - - angle
        - idx: 55
          version: 1
      - - lower_radius
        - idx: 56
          version: 1
      - - upper_radius
        - idx: 57
          version: 1
      - - ring_radius
        - idx: 58
          version: 1
      - - tube_radius
        - idx: 59
          version: 1
      - - cap_angle
        - idx: 60
          version: 1
      - - radial_extent
        - idx: 61
          version: 1
      - - power
        - idx: 62
          version: 1
      - - iterations
        - idx: 63
          version: 1
      - - max_square_radius
        - idx: 64
          version: 1
      - - scale
        - idx: 65
          version: 1
      - - min_square_radius
        - idx: 66
          version: 1
      - - folding_limit
        - idx: 67
          version: 1
      - - world_matrix
        - idx: 68
          version: 1
      - - edge_radius
        - idx: 69
          version: 1
      - - repetition
        - idx: 70
          version: 1
      - - negative_repetitions
        - idx: 71
          version: 1
      - - positive_repetitions
        - idx: 72
          version: 1
      - - spacing
        - idx: 73
          version: 1
      - - bounding_volume
        - idx: 74
          version: 1
      - - blend_type
        - idx: 75
          version: 1
      - - blend_strength
        - idx: 76
          version: 1
      - - mirror
        - idx: 77
          version: 1
      - - hollow
        - idx: 78
          version: 1
      - - wall_thickness
        - idx: 79
          version: 1
      - - elongate
        - idx: 80
          version: 1
      - - elongation
        - idx: 81
          version: 1
      outputs:
      - - out
        - idx: 6
          version: 1
      user_data:
        template: Primitive
    version: 1
  - value:
      id:
        idx: 7
        version: 1
      label: light
      inputs:
      - - lights
        - idx: 82
          version: 1
      - - world_matrix
        - idx: 83
          version: 1
      - - light_type
        - idx: 84
          version: 1
      - - direction
        - idx: 85
          version: 1
      - - position
        - idx: 86
          version: 1
      - - iterations
        - idx: 87
          version: 1
      - - intensity
        - idx: 88
          version: 1
      - - falloff
        - idx: 89
          version: 1
      - - colour
        - idx: 90
          version: 1
      - - shadow_hardness
        - idx: 91
          version: 1
      - - soften_shadows
        - idx: 92
          version: 1
      outputs:
      - - out
        - idx: 7
          version: 1
      user_data:
        template: Light
    version: 1
  - value:
      id:
        idx: 8
        version: 1
      label: material
      inputs:
      - - diffuse_colour
        - idx: 93
          version: 1
      - - diffuse_colour_texture
        - idx: 94
          version: 1
      - - specular_probability
        - idx: 95
          version: 1
      - - specular_probability_texture
        - idx: 96
          version: 1
      - - specular_roughness
        - idx: 97
          version: 1
      - - specular_roughness_texture
        - idx: 98
          version: 1
      - - specular_colour
        - idx: 99
          version: 1
      - - specular_colour_texture
        - idx: 100
          version: 1
      - - transmissive_probability
        - idx: 101
          version: 1
      - - transmissive_probability_texture
        - idx: 102
          version: 1
      - - transmissive_roughness
        - idx: 103
          version: 1
      - - transmissive_roughness_texture
        - idx: 104
          version: 1
      - - extinction_coefficient
        - idx: 105
          version: 1
      - - transmissive_colour
        - idx: 106
          version: 1
      - - transmissive_colour_texture
        - idx: 107
          version: 1
      - - emissive_intensity
        - idx: 108
          version: 1
      - - emissive_colour
        - idx: 109
          version: 1
      - - emissive_colour_texture
        - idx: 110
          version: 1
      - - refractive_index
        - idx: 111
          version: 1
      - - refractive_index_texture
        - idx: 112
          version: 1
      - - scattering_coefficient
        - idx: 113
          version: 1
      - - scattering_colour
        - idx: 114
          version: 1
      - - scattering_colour_texture
        - idx: 115
          version: 1
      outputs:
      - - out
        - idx: 8
          version: 1
      user_data:
        template: Material
    version: 1
  - value:
      id:
        idx: 9
        version: 1
      label: procedural texture
      inputs:
      - - texture_type
        - idx: 116
          version: 1
      - - scale
        - idx: 117
          version: 1
      - - black_point
        - idx: 118
          version: 1
      - - white_point
        - idx: 119
          version: 1
      - - lift
        - idx: 120
          version: 1
      - - gain
        - idx: 121
          version: 1
      - - octaves
        - idx: 122
          version: 1
      - - lacunarity
        - idx: 123
          version: 1
      - - amplitude_gain
        - idx: 124
          version: 1
      - - gamma
        - idx: 125
          version: 1
      - - low_frequency_scale
        - idx: 126
          version: 1
      - - high_frequency_scale
        - idx: 127
          version: 1
      - - low_frequency_translation
        - idx: 128
          version: 1
      - - high_frequency_translation
        - idx: 129
          version: 1
      - - invert
        - idx: 130
          version: 1
      outputs:
      - - out
        - idx: 9
          version: 1
      user_data:
        template: ProceduralTexture
    version: 1
  - value:
      id:
        idx: 10
        version: 1
      label: axis
      inputs:
      - - axis
        - idx: 131
          version: 1
      - - translate
        - idx: 132
          version: 1
      - - rotate
        - idx: 133
          version: 1
      - - uniform_scale
        - idx: 134
          version: 1
      outputs:
      - - out
        - idx: 10
          version: 1
      user_data:
        template: Axis
    version: 1
  - value:
      id:
        idx: 11
        version: 1
      label: material
      inputs:
      - - diffuse_colour
        - idx: 135
          version: 1
      - - diffuse_colour_texture
        - idx: 136
          version: 1
      - - specular_probability
        - idx: 137
          version: 1
      - - specular_probability_texture
        - idx: 138
          version: 1
      - - specular_roughness
        - idx: 139
          version: 1
      - - specular_roughness_texture
        - idx: 140
          version: 1
      - - specular_colour
        - idx: 141
          version: 1
      - - specular_colour_texture
        - idx: 142
          version: 1
      - - transmissive_probability
        - idx: 143
          version: 1
      - - transmissive_probability_texture
        - idx: 144
          version: 1
      - - transmissive_roughness
        - idx: 145
          version: 1
      - - transmissive_roughness_texture
        - idx: 146
          version: 1
      - - extinction_coefficient
        - idx: 147
          version: 1
      - - transmissive_colour
        - idx: 148
          version: 1
      - - transmissive_colour_texture
        - idx: 149
          version: 1
      - - emissive_intensity
        - idx: 150
          version: 1
      - - emissive_colour
        - idx: 151
          version: 1
      - - emissive_colour_texture
        - idx: 152
          version: 1
      - - refractive_index
        - idx: 153
          version: 1
      - - refractive_index_texture
        - idx: 154
          version: 1
      - - scattering_coefficient
        - idx: 155
          version: 1
      - - scattering_colour
        - idx: 156
          version: 1
      - - scattering_colour_texture
        - idx: 157
          version: 1
      outputs:
      - - out
        - idx: 11
          version: 1
      user_data:
        template: Material
    version: 1
  - value:
      id:
        idx: 12
        version: 1
      label: procedural texture
      inputs:
      - - texture_type
        - idx: 158
          version: 1
      - - scale
        - idx: 159
          version: 1
      - - black_point
        - idx: 160
          version: 1
      - - white_point
        - idx: 161
          version: 1
      - - lift
        - idx: 162
          version: 1
      - - gain
        - idx: 163
          version: 1
      - - octaves
        - idx: 164
          version: 1
      - - lacunarity
        - idx: 165
          version: 1
      - - amplitude_gain
        - idx: 166
          version: 1
      - - gamma
        - idx: 167
          version: 1
      - - low_frequency_scale
        - idx: 168
          version: 1
      - - high_frequency_scale
        - idx: 169
          version: 1
      - - low_frequency_translation
        - idx: 170
          version: 1
      - - high_frequency_translation
        - idx: 171
          version: 1
      - - invert
        - idx: 172
          version: 1
      outputs:
      - - out
        - idx: 12
          version: 1
      user_data:
        template: ProceduralTexture
    version: 1
  inputs:
  - value: null
    version: 0
  - value:
      id:
        idx: 1
        version: 3
      typ: Float
      value: !Float
        value:
          value: 1000000000.0
          ui_data:
            tooltip: |-
              The maximum brightness of a pixel. This protects
              against overflowing to infinity.
            hidden: false
          range:
            start: 1.0
            end: 1000000.0
      kind: ConstantOnly
      node:
        idx: 5
        version: 1
      shown_inline: true
      _phantom: null
    version: 3
  - value:
      id:
        idx: 2
        version: 3
      typ: Float
      value: !Float
        value:
          value: 1.0
          ui_data:
            tooltip: |-
              After intersecting an object the ray is offset from
              the surface before continuing. Multiply that offset
              distance by this factor.
            hidden: false
          range:
            start: 1.0
            end: 5.0
      kind: ConstantOnly
      node:
        idx: 5
        version: 1
      shown_inline: true
      _phantom: null
    version: 3
  - value:
      id:
        idx: 3
        version: 3
      typ: Float
      value: !Float
        value:
          value: 0.0001
          ui_data:
            tooltip: |-
              The ray will be considered to have hit an object
              when it is within this distance of its surface.
            hidden: false
          range:
            start: 0.00001
            end: 0.1
      kind: ConstantOnly
      node:
        idx: 5
        version: 1
      shown_inline: true
      _phantom: null
    version: 3
  - value:
      id:
        idx: 4
        version: 3
      typ: UnsignedInteger
      value: !UnsignedInteger
        value:
          value: 1
          ui_data:
            tooltip: |-
              Limits the number of times the rays can intersect
              an object per subpixel.
            hidden: false
          range:
            start: 1
            end: 100
      kind: ConstantOnly
      node:
        idx: 5
        version: 1
      shown_inline: true
      _phantom: null
    version: 3
  - value:
      id:
        idx: 5
        version: 3
      typ: UnsignedInteger
      value: !UnsignedInteger
        value:
          value: 1000
          ui_data:
            tooltip: |-
              Limits the number of times the rays can intersect
              an object per subpixel.
            hidden: false
          range:
            start: 100
            end: 100000
      kind: ConstantOnly
      node:
        idx: 5
        version: 1
      shown_inline: true
      _phantom: null
    version: 3
  - value:
      id:
        idx: 6
        version: 3
      typ: Float
      value: !Float
        value:
          value: 100.0
          ui_data:
            tooltip: |-
              Each ray, once spawned is only allowed to travel
              this distance before it is culled.
            hidden: false
          range:
            start: 10.0
            end: 10000.0
      kind: ConstantOnly
      node:
        idx: 5
        version: 1
      shown_inline: true
      _phantom: null
    version: 3
  - value:
      id:
        idx: 7
        version: 3
      typ: Scene
      value: !Scene
        value:
          render_camera:
            aspect_ratio: 1.0
            focal_length: 50.0
            horizontal_aperture: 24.576
            near_plane: 0.1
            far_plane: 10000.0
            focal_distance: 2.0
            f_stop: 16.0
            world_matrix:
            - 1.0
            - 0.0
            - 0.0
            - 0.0
            - 0.0
            - 1.0
            - 0.0
            - 0.0
            - 0.0
            - 0.0
            - 1.0
            - 0.0
            - 0.0
            - 0.0
            - 0.0
            - 1.0
            enable_depth_of_field: false
          primitives: []
          lights: []
          atmosphere:
            diffuse_colour:
            - 0.0
            - 0.0
            - 0.0
            diffuse_colour_texture:
              texture_type: None
              scale:
              - 1.0
              - 1.0
              - 1.0
              - 1.0
              black_point: 0.0
              white_point: 1.0
              lift: 0.0
              gain: 1.0
              octaves: 10
              lacunarity: 2.0
              amplitude_gain: 0.75
              gamma: 1.0
              low_frequency_scale:
              - 1.0
              - 1.0
              - 1.0
              - 1.0
              high_frequency_scale:
              - 1.0
              - 1.0
              - 1.0
              - 1.0
              low_frequency_translation:
              - 0.0
              - 0.0
              - 0.0
              - 0.0
              high_frequency_translation:
              - 0.0
              - 0.0
              - 0.0
              - 0.0
              invert: false
            specular_probability: 0.0
            specular_probability_texture:
              texture_type: None
              scale:
              - 1.0
              - 1.0
              - 1.0
              - 1.0
              black_point: 0.0
              white_point: 1.0
              lift: 0.0
              gain: 1.0
              octaves: 10
              lacunarity: 2.0
              amplitude_gain: 0.75
              gamma: 1.0
              low_frequency_scale:
              - 1.0
              - 1.0
              - 1.0
              - 1.0
              high_frequency_scale:
              - 1.0
              - 1.0
              - 1.0
              - 1.0
              low_frequency_translation:
              - 0.0
              - 0.0
              - 0.0
              - 0.0
              high_frequency_translation:
              - 0.0
              - 0.0
              - 0.0
              - 0.0
              invert: false
            specular_roughness: 0.0
            specular_roughness_texture:
              texture_type: None
              scale:
              - 1.0
              - 1.0
              - 1.0
              - 1.0
              black_point: 0.0
              white_point: 1.0
              lift: 0.0
              gain: 1.0
              octaves: 10
              lacunarity: 2.0
              amplitude_gain: 0.75
              gamma: 1.0
              low_frequency_scale:
              - 1.0
              - 1.0
              - 1.0
              - 1.0
              high_frequency_scale:
              - 1.0
              - 1.0
              - 1.0
              - 1.0
              low_frequency_translation:
              - 0.0
              - 0.0
              - 0.0
              - 0.0
              high_frequency_translation:
              - 0.0
              - 0.0
              - 0.0
              - 0.0
              invert: false
            specular_colour:
            - 1.0
            - 1.0
            - 1.0
            specular_colour_texture:
              texture_type: None
              scale:
              - 1.0
              - 1.0
              - 1.0
              - 1.0
              black_point: 0.0
              white_point: 1.0
              lift: 0.0
              gain: 1.0
              octaves: 10
              lacunarity: 2.0
              amplitude_gain: 0.75
              gamma: 1.0
              low_frequency_scale:
              - 1.0
              - 1.0
              - 1.0
              - 1.0
              high_frequency_scale:
              - 1.0
              - 1.0
              - 1.0
              - 1.0
              low_frequency_translation:
              - 0.0
              - 0.0
              - 0.0
              - 0.0
              high_frequency_translation:
              - 0.0
              - 0.0
              - 0.0
              - 0.0
              invert: false
            transmissive_probability: 0.0
            transmissive_probability_texture:
              texture_type: None
              scale:
              - 1.0
              - 1.0
              - 1.0
              - 1.0
              black_point: 0.0
              white_point: 1.0
              lift: 0.0
              gain: 1.0
              octaves: 10
              lacunarity: 2.0
              amplitude_gain: 0.75
              gamma: 1.0
              low_frequency_scale:
              - 1.0
              - 1.0
              - 1.0
              - 1.0
              high_frequency_scale:
              - 1.0
              - 1.0
              - 1.0
              - 1.0
              low_frequency_translation:
              - 0.0
              - 0.0
              - 0.0
              - 0.0
              high_frequency_translation:
              - 0.0
              - 0.0
              - 0.0
              - 0.0
              invert: false
            transmissive_roughness: 0.0
            transmissive_roughness_texture:
              texture_type: None
              scale:
              - 1.0
              - 1.0
              - 1.0
              - 1.0
              black_point: 0.0
              white_point: 1.0
              lift: 0.0
              gain: 1.0
              octaves: 10
              lacunarity: 2.0
              amplitude_gain: 0.75
              gamma: 1.0
              low_frequency_scale:
              - 1.0
              - 1.0
              - 1.0
              - 1.0
              high_frequency_scale:
              - 1.0
              - 1.0
              - 1.0
              - 1.0
              low_frequency_translation:
              - 0.0
              - 0.0
              - 0.0
              - 0.0
              high_frequency_translation:
              - 0.0
              - 0.0
              - 0.0
              - 0.0
              invert: false
            extinction_coefficient: 0.0
            transmissive_colour:
            - 1.0
            - 1.0
            - 1.0
            transmissive_colour_texture:
              texture_type: None
              scale:
              - 1.0
              - 1.0
              - 1.0
              - 1.0
              black_point: 0.0
              white_point: 1.0
              lift: 0.0
              gain: 1.0
              octaves: 10
              lacunarity: 2.0
              amplitude_gain: 0.75
              gamma: 1.0
              low_frequency_scale:
              - 1.0
              - 1.0
              - 1.0
              - 1.0
              high_frequency_scale:
              - 1.0
              - 1.0
              - 1.0
              - 1.0
              low_frequency_translation:
              - 0.0
              - 0.0
              - 0.0
              - 0.0
              high_frequency_translation:
              - 0.0
              - 0.0
              - 0.0
              - 0.0
              invert: false
            emissive_intensity: 0.0
            emissive_colour:
            - 1.0
            - 0.8
            - 0.5
            emissive_colour_texture:
              texture_type: None
              scale:
              - 1.0
              - 1.0
              - 1.0
              - 1.0
              black_point: 0.0
              white_point: 1.0
              lift: 0.0
              gain: 1.0
              octaves: 10
              lacunarity: 2.0
              amplitude_gain: 0.75
              gamma: 1.0
              low_frequency_scale:
              - 1.0
              - 1.0
              - 1.0
              - 1.0
              high_frequency_scale:
              - 1.0
              - 1.0
              - 1.0
              - 1.0
              low_frequency_translation:
              - 0.0
              - 0.0
              - 0.0
              - 0.0
              high_frequency_translation:
              - 0.0
              - 0.0
              - 0.0
              - 0.0
              invert: false
            refractive_index: 1.0
            refractive_index_texture:
              texture_type: None
              scale:
              - 1.0
              - 1.0
              - 1.0
              - 1.0
              black_point: 0.0
              white_point: 1.0
              lift: 0.0
              gain: 1.0
              octaves: 10
              lacunarity: 2.0
              amplitude_gain: 0.75
              gamma: 1.0
              low_frequency_scale:
              - 1.0
              - 1.0
              - 1.0
              - 1.0
              high_frequency_scale:
              - 1.0
              - 1.0
              - 1.0
              - 1.0
              low_frequency_translation:
              - 0.0
              - 0.0
              - 0.0
              - 0.0
              high_frequency_translation:
              - 0.0
              - 0.0
              - 0.0
              - 0.0
              invert: false
            scattering_coefficient: 0.0
            scattering_colour:
            - 1.0
            - 1.0
            - 1.0
            scattering_colour_texture:
              texture_type: None
              scale:
              - 1.0
              - 1.0
              - 1.0
              - 1.0
              black_point: 0.0
              white_point: 1.0
              lift: 0.0
              gain: 1.0
              octaves: 10
              lacunarity: 2.0
              amplitude_gain: 0.75
              gamma: 1.0
              low_frequency_scale:
              - 1.0
              - 1.0
              - 1.0
              - 1.0
              high_frequency_scale:
              - 1.0
              - 1.0
              - 1.0
              - 1.0
              low_frequency_translation:
              - 0.0
              - 0.0
              - 0.0
              - 0.0
              high_frequency_translation:
              - 0.0
              - 0.0
              - 0.0
              - 0.0
              invert: false
      kind: ConnectionOnly
      node:
        idx: 5
        version: 1
      shown_inline: true
      _phantom: null
    version: 3
  - value:
      id:
        idx: 8
        version: 3
      typ: Material
      value: !Material
        value:
          diffuse_colour:
          - 0.0
          - 0.0
          - 0.0
          diffuse_colour_texture:
            texture_type: None
            scale:
            - 1.0
            - 1.0
            - 1.0
            - 1.0
            black_point: 0.0
            white_point: 1.0
            lift: 0.0
            gain: 1.0
            octaves: 10
            lacunarity: 2.0
            amplitude_gain: 0.75
            gamma: 1.0
            low_frequency_scale:
            - 1.0
            - 1.0
            - 1.0
            - 1.0
            high_frequency_scale:
            - 1.0
            - 1.0
            - 1.0
            - 1.0
            low_frequency_translation:
            - 0.0
            - 0.0
            - 0.0
            - 0.0
            high_frequency_translation:
            - 0.0
            - 0.0
            - 0.0
            - 0.0
            invert: false
          specular_probability: 0.0
          specular_probability_texture:
            texture_type: None
            scale:
            - 1.0
            - 1.0
            - 1.0
            - 1.0
            black_point: 0.0
            white_point: 1.0
            lift: 0.0
            gain: 1.0
            octaves: 10
            lacunarity: 2.0
            amplitude_gain: 0.75
            gamma: 1.0
            low_frequency_scale:
            - 1.0
            - 1.0
            - 1.0
            - 1.0
            high_frequency_scale:
            - 1.0
            - 1.0
            - 1.0
            - 1.0
            low_frequency_translation:
            - 0.0
            - 0.0
            - 0.0
            - 0.0
            high_frequency_translation:
            - 0.0
            - 0.0
            - 0.0
            - 0.0
            invert: false
          specular_roughness: 0.0
          specular_roughness_texture:
            texture_type: None
            scale:
            - 1.0
            - 1.0
            - 1.0
            - 1.0
            black_point: 0.0
            white_point: 1.0
            lift: 0.0
            gain: 1.0
            octaves: 10
            lacunarity: 2.0
            amplitude_gain: 0.75
            gamma: 1.0
            low_frequency_scale:
            - 1.0
            - 1.0
            - 1.0
            - 1.0
            high_frequency_scale:
            - 1.0
            - 1.0
            - 1.0
            - 1.0
            low_frequency_translation:
            - 0.0
            - 0.0
            - 0.0
            - 0.0
            high_frequency_translation:
            - 0.0
            - 0.0
            - 0.0
            - 0.0
            invert: false
          specular_colour:
          - 1.0
          - 1.0
          - 1.0
          specular_colour_texture:
            texture_type: None
            scale:
            - 1.0
            - 1.0
            - 1.0
            - 1.0
            black_point: 0.0
            white_point: 1.0
            lift: 0.0
            gain: 1.0
            octaves: 10
            lacunarity: 2.0
            amplitude_gain: 0.75
            gamma: 1.0
            low_frequency_scale:
            - 1.0
            - 1.0
            - 1.0
            - 1.0
            high_frequency_scale:
            - 1.0
            - 1.0
            - 1.0
            - 1.0
            low_frequency_translation:
            - 0.0
            - 0.0
            - 0.0
            - 0.0
            high_frequency_translation:
            - 0.0
            - 0.0
            - 0.0
            - 0.0
            invert: false
          transmissive_probability: 0.0
          transmissive_probability_texture:
            texture_type: None
            scale:
            - 1.0
            - 1.0
            - 1.0
            - 1.0
            black_point: 0.0
            white_point: 1.0
            lift: 0.0
            gain: 1.0
            octaves: 10
            lacunarity: 2.0
            amplitude_gain: 0.75
            gamma: 1.0
            low_frequency_scale:
            - 1.0
            - 1.0
            - 1.0
            - 1.0
            high_frequency_scale:
            - 1.0
            - 1.0
            - 1.0
            - 1.0
            low_frequency_translation:
            - 0.0
            - 0.0
            - 0.0
            - 0.0
            high_frequency_translation:
            - 0.0
            - 0.0
            - 0.0
            - 0.0
            invert: false
          transmissive_roughness: 0.0
          transmissive_roughness_texture:
            texture_type: None
            scale:
            - 1.0
            - 1.0
            - 1.0
            - 1.0
            black_point: 0.0
            white_point: 1.0
            lift: 0.0
            gain: 1.0
            octaves: 10
            lacunarity: 2.0
            amplitude_gain: 0.75
            gamma: 1.0
            low_frequency_scale:
            - 1.0
            - 1.0
            - 1.0
            - 1.0
            high_frequency_scale:
            - 1.0
            - 1.0
            - 1.0
            - 1.0
            low_frequency_translation:
            - 0.0
            - 0.0
            - 0.0
            - 0.0
            high_frequency_translation:
            - 0.0
            - 0.0
            - 0.0
            - 0.0
            invert: false
          extinction_coefficient: 0.0
          transmissive_colour:
          - 1.0
          - 1.0
          - 1.0
          transmissive_colour_texture:
            texture_type: None
            scale:
            - 1.0
            - 1.0
            - 1.0
            - 1.0
            black_point: 0.0
            white_point: 1.0
            lift: 0.0
            gain: 1.0
            octaves: 10
            lacunarity: 2.0
            amplitude_gain: 0.75
            gamma: 1.0
            low_frequency_scale:
            - 1.0
            - 1.0
            - 1.0
            - 1.0
            high_frequency_scale:
            - 1.0
            - 1.0
            - 1.0
            - 1.0
            low_frequency_translation:
            - 0.0
            - 0.0
            - 0.0
            - 0.0
            high_frequency_translation:
            - 0.0
            - 0.0
            - 0.0
            - 0.0
            invert: false
          emissive_intensity: 0.0
          emissive_colour:
          - 1.0
          - 0.8
          - 0.5
          emissive_colour_texture:
            texture_type: None
            scale:
            - 1.0
            - 1.0
            - 1.0
            - 1.0
            black_point: 0.0
            white_point: 1.0
            lift: 0.0
            gain: 1.0
            octaves: 10
            lacunarity: 2.0
            amplitude_gain: 0.75
            gamma: 1.0
            low_frequency_scale:
            - 1.0
            - 1.0
            - 1.0
            - 1.0
            high_frequency_scale:
            - 1.0
            - 1.0
            - 1.0
            - 1.0
            low_frequency_translation:
            - 0.0
            - 0.0
            - 0.0
            - 0.0
            high_frequency_translation:
            - 0.0
            - 0.0
            - 0.0
            - 0.0
            invert: false
          refractive_index: 1.0
          refractive_index_texture:
            texture_type: None
            scale:
            - 1.0
            - 1.0
            - 1.0
            - 1.0
            black_point: 0.0
            white_point: 1.0
            lift: 0.0
            gain: 1.0
            octaves: 10
            lacunarity: 2.0
            amplitude_gain: 0.75
            gamma: 1.0
            low_frequency_scale:
            - 1.0
            - 1.0
            - 1.0
            - 1.0
            high_frequency_scale:
            - 1.0
            - 1.0
            - 1.0
            - 1.0
            low_frequency_translation:
            - 0.0
            - 0.0
            - 0.0
            - 0.0
            high_frequency_translation:
            - 0.0
            - 0.0
            - 0.0
            - 0.0
            invert: false
          scattering_coefficient: 0.0
          scattering_colour:
          - 1.0
          - 1.0
          - 1.0
          scattering_colour_texture:
            texture_type: None
            scale:
            - 1.0
            - 1.0
            - 1.0
            - 1.0
            black_point: 0.0
            white_point: 1.0
            lift: 0.0
            gain: 1.0
            octaves: 10
            lacunarity: 2.0
            amplitude_gain: 0.75
            gamma: 1.0
            low_frequency_scale:
            - 1.0
            - 1.0
            - 1.0
            - 1.0
            high_frequency_scale:
            - 1.0
            - 1.0
            - 1.0
            - 1.0
            low_frequency_translation:
            - 0.0
            - 0.0
            - 0.0
            - 0.0
            high_frequency_translation:
            - 0.0
            - 0.0
            - 0.0
            - 0.0
            invert: false
      kind: ConnectionOnly
      node:
        idx: 4
        version: 1
      shown_inline: true
      _phantom: null
    version: 3
  - value:
      id:
        idx: 9
        version: 3
      typ: Light
      value: !Light
        value: []
      kind: ConnectionOnly
      node:
        idx: 4
        version: 1
      shown_inline: true
      _phantom: null
    version: 3
  - value:
      id:
        idx: 10
        version: 3
      typ: Primitive
      value: !Primitive
        value: []
      kind: ConnectionOnly
      node:
        idx: 4
        version: 1
      shown_inline: true
      _phantom: null
    version: 3
  - value:
      id:
        idx: 11
        version: 3
      typ: Camera
      value: !Camera
        value:
          aspect_ratio: 1.0
          focal_length: 50.0
          horizontal_aperture: 24.576
          near_plane: 0.1
          far_plane: 10000.0
          focal_distance: 2.0
          f_stop: 16.0
          world_matrix:
          - 1.0
          - 0.0
          - 0.0
          - 0.0
          - 0.0
          - 1.0
          - 0.0
          - 0.0
          - 0.0
          - 0.0
          - 1.0
          - 0.0
          - 0.0
          - 0.0
          - 0.0
          - 1.0
          enable_depth_of_field: false
      kind: ConnectionOnly
      node:
        idx: 4
        version: 1
      shown_inline: true
      _phantom: null
    version: 3
  - value:
      id:
        idx: 12
        version: 3
      typ: Bool
      value: !Bool
        value:
          value: false
          ui_data:
            tooltip: If enabled, this camera will render with depth of field.
            hidden: false
      kind: ConstantOnly
      node:
        idx: 1
        version: 3
      shown_inline: true
      _phantom: null
    version: 3
  - value:
      id:
        idx: 13
        version: 3
      typ: Mat4
      value: !Mat4
        value:
          value:
          - 1.0
          - 0.0
          - 0.0
          - 0.0
          - 0.0
          - 1.0
          - 0.0
          - 0.0
          - 0.0
          - 0.0
          - 1.0
          - 0.0
          - 0.0
          - 0.0
          - 0.0
          - 1.0
          ui_data:
            tooltip: The world matrix/axis of the camera.
            hidden: false
      kind: ConnectionOrConstant
      node:
        idx: 1
        version: 3
      shown_inline: true
      _phantom: null
    version: 3
  - value:
      id:
        idx: 14
        version: 3
      typ: Float
      value: !Float
        value:
          value: 10000.0
          ui_data:
            tooltip: The distance to the far plane of the camera.
            hidden: false
          range:
            start: 11.0
            end: 10000.0
      kind: ConstantOnly
      node:
        idx: 1
        version: 3
      shown_inline: true
      _phantom: null
    version: 3
  - value:
      id:
        idx: 15
        version: 3
      typ: Float
      value: !Float
        value:
          value: 0.1
          ui_data:
            tooltip: The distance to the near plane of the camera.
            hidden: false
          range:
            start: 0.1
            end: 10.0
      kind: ConstantOnly
      node:
        idx: 1
        version: 3
      shown_inline: true
      _phantom: null
    version: 3
  - value:
      id:
        idx: 16
        version: 3
      typ: Float
      value: !Float
        value:
          value: 24.576
          ui_data:
            tooltip: The horizontal aperture of the camera.
            hidden: false
          range:
            start: 0.1
            end: 50.0
      kind: ConstantOnly
      node:
        idx: 1
        version: 3
      shown_inline: true
      _phantom: null
    version: 3
  - value:
      id:
        idx: 17
        version: 3
      typ: Float
      value: !Float
        value:
          value: 16.0
          ui_data:
            tooltip: The f-stop of the camera.
            hidden: false
          range:
            start: 0.1
            end: 30.0
      kind: ConstantOnly
      node:
        idx: 1
        version: 3
      shown_inline: true
      _phantom: null
    version: 3
  - value:
      id:
        idx: 18
        version: 3
      typ: Float
      value: !Float
        value:
          value: 2.0
          ui_data:
            tooltip: The focal distance of the camera.
            hidden: false
          range:
            start: 0.1
            end: 10.0
      kind: ConstantOnly
      node:
        idx: 1
        version: 3
      shown_inline: true
      _phantom: null
    version: 3
  - value:
      id:
        idx: 19
        version: 3
      typ: Float
      value: !Float
        value:
          value: 50.0
          ui_data:
            tooltip: The focal length of the camera.
            hidden: false
          range:
            start: 5.0
            end: 100.0
      kind: ConstantOnly
      node:
        idx: 1
        version: 3
      shown_inline: true
      _phantom: null
    version: 3
  - value:
      id:
        idx: 20
        version: 3
      typ: Float
      value: !Float
        value:
          value: 1.0
          ui_data:
            tooltip: "The uniform scale of this axis.\n\nWe use uniform scale because the signed distance \nfields cannot have their individual axes scaled."
            hidden: false
          range:
            start: 0.01
            end: 10.0
      kind: ConstantOnly
      node:
        idx: 2
        version: 3
      shown_inline: true
      _phantom: null
    version: 3
  - value:
      id:
        idx: 21
        version: 3
      typ: Vec3
      value: !Vec3
        value:
          value:
          - 0.0
          - 0.0
          - 0.0
          ui_data:
            tooltip: The rotation of this axis.
            hidden: false
          is_colour: false
      kind: ConstantOnly
      node:
        idx: 2
        version: 3
      shown_inline: true
      _phantom: null
    version: 3
  - value:
      id:
        idx: 22
        version: 3
      typ: Vec3
      value: !Vec3
        value:
          value:
          - 0.0
          - 0.0
          - 6.0
          ui_data:
            tooltip: The translation of this axis.
            hidden: false
          is_colour: false
      kind: ConstantOnly
      node:
        idx: 2
        version: 3
      shown_inline: true
      _phantom: null
    version: 3
  - value:
      id:
        idx: 23
        version: 3
      typ: Mat4
      value: !Mat4
        value:
          value:
          - 1.0
          - 0.0
          - 0.0
          - 0.0
          - 0.0
          - 1.0
          - 0.0
          - 0.0
          - 0.0
          - 0.0
          - 1.0
          - 0.0
          - 0.0
          - 0.0
          - 0.0
          - 1.0
          ui_data:
            tooltip: The parent axis.
            hidden: false
      kind: ConnectionOrConstant
      node:
        idx: 2
        version: 3
      shown_inline: true
      _phantom: null
    version: 3
  - value:
      id:
        idx: 24
        version: 3
      typ: Float
      value: !Float
        value:
          value: 1.0
          ui_data:
            tooltip: "The uniform scale of this axis.\n\nWe use uniform scale because the signed distance \nfields cannot have their individual axes scaled."
            hidden: false
          range:
            start: 0.01
            end: 10.0
      kind: ConstantOnly
      node:
        idx: 3
        version: 3
      shown_inline: true
      _phantom: null
    version: 3
  - value:
      id:
        idx: 25
        version: 3
      typ: Vec3
      value: !Vec3
        value:
          value:
          - 0.0
          - 0.0
          - 0.0
          ui_data:
            tooltip: The rotation of this axis.
            hidden: false
          is_colour: false
      kind: ConstantOnly
      node:
        idx: 3
        version: 3
      shown_inline: true
      _phantom: null
    version: 3
  - value:
      id:
        idx: 26
        version: 3
      typ: Vec3
      value: !Vec3
        value:
          value:
          - 0.0
          - 0.0
          - 0.0
          ui_data:
            tooltip: The translation of this axis.
            hidden: false
          is_colour: false
      kind: ConstantOnly
      node:
        idx: 3
        version: 3
      shown_inline: true
      _phantom: null
    version: 3
  - value:
      id:
        idx: 27
        version: 3
      typ: Mat4
      value: !Mat4
        value:
          value:
          - 1.0
          - 0.0
          - 0.0
          - 0.0
          - 0.0
          - 1.0
          - 0.0
          - 0.0
          - 0.0
          - 0.0
          - 1.0
          - 0.0
          - 0.0
          - 0.0
          - 0.0
          - 1.0
          ui_data:
            tooltip: The parent axis.
            hidden: false
      kind: ConnectionOrConstant
      node:
        idx: 3
        version: 3
      shown_inline: true
      _phantom: null
    version: 3
  - value:
      id:
        idx: 28
        version: 1
      typ: Vec3
      value: !Vec3
        value:
          value:
          - 1111.0
          - 2222.0
          - 3333.0
          ui_data:
            tooltip: |-
              The seeds used to generate per-pixel, random seeds.
              Be sure these are different on each render used for
              adaptive sampling.
            hidden: false
          is_colour: false
      kind: ConstantOnly
      node:
        idx: 5
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 29
        version: 1
      typ: Bool
      value: !Bool
        value:
          value: true
          ui_data:
            tooltip: |-
              Increase the hit tolerance the farther the ray
              travels without hitting a surface. This has performance
              and antialiasing benefits.
            hidden: false
      kind: ConstantOnly
      node:
        idx: 5
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 30
        version: 1
      typ: UnsignedInteger
      value: !UnsignedInteger
        value:
          value: 0
          ui_data:
            tooltip: |-
              The number of equi-angular samples to perform if
              the extinction/scattering coefficients are greater
              than 0. This enables participating media such as
              fog/smoke/clouds to be traced.
            hidden: false
          range:
            start: 0
            end: 10
      kind: ConstantOnly
      node:
        idx: 5
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 31
        version: 1
      typ: UnsignedInteger
      value: !UnsignedInteger
        value:
          value: 1
          ui_data:
            tooltip: |-
              The maximum number of bounces during light sampling.
              Light sampling will be disabled if this is 0. Light
              sampling means that each time a surface is hit, the
              direct illumination from lights in the scene will be
              computed, which helps to reduce noise very quickly.
              TODO
            hidden: false
          range:
            start: 0
            end: 50
      kind: ConstantOnly
      node:
        idx: 5
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 32
        version: 1
      typ: Bool
      value: !Bool
        value:
          value: false
          ui_data:
            tooltip: |-
              Include the skybox in the list of lights that can be
              sampled during light sampling.
            hidden: false
      kind: ConstantOnly
      node:
        idx: 5
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 33
        version: 1
      typ: Float
      value: !Float
        value:
          value: 0.0
          ui_data:
            tooltip: |-
              A fully biased (1) light sampling means that on each
              light sample the ray will be initialised pointing
              directly at the light. Reducing this bias means that
              some rays will be pointed away from the light. This,
              when combined with multiple 'max light sampling
              bounces' allows the renderer to find difficult paths,
              such as volumetric caustics.
              TODO
            hidden: false
          range:
            start: 0.0
            end: 1.0
      kind: ConstantOnly
      node:
        idx: 5
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 34
        version: 1
      typ: Bool
      value: !Bool
        value:
          value: false
          ui_data:
            tooltip: |-
              Sample the artificial lights (those in the 'lights'
              input) while casting shadow rays for light sampling.
              TODO
            hidden: false
      kind: ConstantOnly
      node:
        idx: 5
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 35
        version: 1
      typ: ComboBox
      value: !ComboBox
        value:
          selected: Beauty
          options:
          - Beauty
          - WorldPosition
          - LocalPosition
          - Normals
          - Depth
          - Cryptomatte
          - Stats
          ui_data:
            tooltip: |-
              The AOV type to output.
              The stats AOV has the
              average number of bounces in the red channel,
              average number of steps in the green channel,
              and the distance travelled in the blue channel.
              Each is displayed as a fraction of the maximums.
            hidden: false
      kind: ConstantOnly
      node:
        idx: 5
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 36
        version: 1
      typ: Bool
      value: !Bool
        value:
          value: false
          ui_data:
            tooltip: Output a LatLong, 360 degree field of view image.
            hidden: false
      kind: ConstantOnly
      node:
        idx: 5
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 37
        version: 1
      typ: Primitive
      value: !Primitive
        value: []
      kind: ConnectionOnly
      node:
        idx: 6
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 38
        version: 1
      typ: Primitive
      value: !Primitive
        value: []
      kind: ConnectionOnly
      node:
        idx: 6
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 39
        version: 1
      typ: Material
      value: !Material
        value:
          diffuse_colour:
          - 1.0
          - 1.0
          - 1.0
          diffuse_colour_texture:
            texture_type: None
            scale:
            - 1.0
            - 1.0
            - 1.0
            - 1.0
            black_point: 0.0
            white_point: 1.0
            lift: 0.0
            gain: 1.0
            octaves: 10
            lacunarity: 2.0
            amplitude_gain: 0.75
            gamma: 1.0
            low_frequency_scale:
            - 1.0
            - 1.0
            - 1.0
            - 1.0
            high_frequency_scale:
            - 1.0
            - 1.0
            - 1.0
            - 1.0
            low_frequency_translation:
            - 0.0
            - 0.0
            - 0.0
            - 0.0
            high_frequency_translation:
            - 0.0
            - 0.0
            - 0.0
            - 0.0
            invert: false
          specular_probability: 0.0
          specular_probability_texture:
            texture_type: None
            scale:
            - 1.0
            - 1.0
            - 1.0
            - 1.0
            black_point: 0.0
            white_point: 1.0
            lift: 0.0
            gain: 1.0
            octaves: 10
            lacunarity: 2.0
            amplitude_gain: 0.75
            gamma: 1.0
            low_frequency_scale:
            - 1.0
            - 1.0
            - 1.0
            - 1.0
            high_frequency_scale:
            - 1.0
            - 1.0
            - 1.0
            - 1.0
            low_frequency_translation:
            - 0.0
            - 0.0
            - 0.0
            - 0.0
            high_frequency_translation:
            - 0.0
            - 0.0
            - 0.0
            - 0.0
            invert: false
          specular_roughness: 0.0
          specular_roughness_texture:
            texture_type: None
            scale:
            - 1.0
            - 1.0
            - 1.0
            - 1.0
            black_point: 0.0
            white_point: 1.0
            lift: 0.0
            gain: 1.0
            octaves: 10
            lacunarity: 2.0
            amplitude_gain: 0.75
            gamma: 1.0
            low_frequency_scale:
            - 1.0
            - 1.0
            - 1.0
            - 1.0
            high_frequency_scale:
            - 1.0
            - 1.0
            - 1.0
            - 1.0
            low_frequency_translation:
            - 0.0
            - 0.0
            - 0.0
            - 0.0
            high_frequency_translation:
            - 0.0
            - 0.0
            - 0.0
            - 0.0
            invert: false
          specular_colour:
          - 1.0
          - 1.0
          - 1.0
          specular_colour_texture:
            texture_type: None
            scale:
            - 1.0
            - 1.0
            - 1.0
            - 1.0
            black_point: 0.0
            white_point: 1.0
            lift: 0.0
            gain: 1.0
            octaves: 10
            lacunarity: 2.0
            amplitude_gain: 0.75
            gamma: 1.0
            low_frequency_scale:
            - 1.0
            - 1.0
            - 1.0
            - 1.0
            high_frequency_scale:
            - 1.0
            - 1.0
            - 1.0
            - 1.0
            low_frequency_translation:
            - 0.0
            - 0.0
            - 0.0
            - 0.0
            high_frequency_translation:
            - 0.0
            - 0.0
            - 0.0
            - 0.0
            invert: false
          transmissive_probability: 0.0
          transmissive_probability_texture:
            texture_type: None
            scale:
            - 1.0
            - 1.0
            - 1.0
            - 1.0
            black_point: 0.0
            white_point: 1.0
            lift: 0.0
            gain: 1.0
            octaves: 10
            lacunarity: 2.0
            amplitude_gain: 0.75
            gamma: 1.0
            low_frequency_scale:
            - 1.0
            - 1.0
            - 1.0
            - 1.0
            high_frequency_scale:
            - 1.0
            - 1.0
            - 1.0
            - 1.0
            low_frequency_translation:
            - 0.0
            - 0.0
            - 0.0
            - 0.0
            high_frequency_translation:
            - 0.0
            - 0.0
            - 0.0
            - 0.0
            invert: false
          transmissive_roughness: 0.0
          transmissive_roughness_texture:
            texture_type: None
            scale:
            - 1.0
            - 1.0
            - 1.0
            - 1.0
            black_point: 0.0
            white_point: 1.0
            lift: 0.0
            gain: 1.0
            octaves: 10
            lacunarity: 2.0
            amplitude_gain: 0.75
            gamma: 1.0
            low_frequency_scale:
            - 1.0
            - 1.0
            - 1.0
            - 1.0
            high_frequency_scale:
            - 1.0
            - 1.0
            - 1.0
            - 1.0
            low_frequency_translation:
            - 0.0
            - 0.0
            - 0.0
            - 0.0
            high_frequency_translation:
            - 0.0
            - 0.0
            - 0.0
            - 0.0
            invert: false
          extinction_coefficient: 0.0
          transmissive_colour:
          - 1.0
          - 1.0
          - 1.0
          transmissive_colour_texture:
            texture_type: None
            scale:
            - 1.0
            - 1.0
            - 1.0
            - 1.0
            black_point: 0.0
            white_point: 1.0
            lift: 0.0
            gain: 1.0
            octaves: 10
            lacunarity: 2.0
            amplitude_gain: 0.75
            gamma: 1.0
            low_frequency_scale:
            - 1.0
            - 1.0
            - 1.0
            - 1.0
            high_frequency_scale:
            - 1.0
            - 1.0
            - 1.0
            - 1.0
            low_frequency_translation:
            - 0.0
            - 0.0
            - 0.0
            - 0.0
            high_frequency_translation:
            - 0.0
            - 0.0
            - 0.0
            - 0.0
            invert: false
          emissive_intensity: 0.0
          emissive_colour:
          - 1.0
          - 0.8
          - 0.5
          emissive_colour_texture:
            texture_type: None
            scale:
            - 1.0
            - 1.0
            - 1.0
            - 1.0
            black_point: 0.0
            white_point: 1.0
            lift: 0.0
            gain: 1.0
            octaves: 10
            lacunarity: 2.0
            amplitude_gain: 0.75
            gamma: 1.0
            low_frequency_scale:
            - 1.0
            - 1.0
            - 1.0
            - 1.0
            high_frequency_scale:
            - 1.0
            - 1.0
            - 1.0
            - 1.0
            low_frequency_translation:
            - 0.0
            - 0.0
            - 0.0
            - 0.0
            high_frequency_translation:
            - 0.0
            - 0.0
            - 0.0
            - 0.0
            invert: false
          refractive_index: 1.3
          refractive_index_texture:
            texture_type: None
            scale:
            - 1.0
            - 1.0
            - 1.0
            - 1.0
            black_point: 0.0
            white_point: 1.0
            lift: 0.0
            gain: 1.0
            octaves: 10
            lacunarity: 2.0
            amplitude_gain: 0.75
            gamma: 1.0
            low_frequency_scale:
            - 1.0
            - 1.0
            - 1.0
            - 1.0
            high_frequency_scale:
            - 1.0
            - 1.0
            - 1.0
            - 1.0
            low_frequency_translation:
            - 0.0
            - 0.0
            - 0.0
            - 0.0
            high_frequency_translation:
            - 0.0
            - 0.0
            - 0.0
            - 0.0
            invert: false
          scattering_coefficient: 0.0
          scattering_colour:
          - 1.0
          - 1.0
          - 1.0
          scattering_colour_texture:
            texture_type: None
            scale:
            - 1.0
            - 1.0
            - 1.0
            - 1.0
            black_point: 0.0
            white_point: 1.0
            lift: 0.0
            gain: 1.0
            octaves: 10
            lacunarity: 2.0
            amplitude_gain: 0.75
            gamma: 1.0
            low_frequency_scale:
            - 1.0
            - 1.0
            - 1.0
            - 1.0
            high_frequency_scale:
            - 1.0
            - 1.0
            - 1.0
            - 1.0
            low_frequency_translation:
            - 0.0
            - 0.0
            - 0.0
            - 0.0
            high_frequency_translation:
            - 0.0
            - 0.0
            - 0.0
            - 0.0
            invert: false
      kind: ConnectionOnly
      node:
        idx: 6
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 40
        version: 1
      typ: ComboBox
      value: !ComboBox
        value:
          selected: Torus
          options:
          - CappedCone
          - CappedTorus
          - Capsule
          - Cone
          - CutSphere
          - Cylinder
          - DeathStar
          - Ellipsoid
          - HexagonalPrism
          - HollowSphere
          - InfiniteCone
          - InfiniteCylinder
          - Link
          - Mandelbox
          - Mandelbulb
          - Octahedron
          - Plane
          - RectangularPrism
          - RectangularPrismFrame
          - Rhombus
          - RoundedCone
          - SolidAngle
          - Sphere
          - Torus
          - TriangularPrism
          ui_data:
            tooltip: The shape of the primitive.
            hidden: false
      kind: ConstantOnly
      node:
        idx: 6
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 41
        version: 1
      typ: Float
      value: !Float
        value:
          value: 0.5
          ui_data:
            tooltip: The radius.
            hidden: true
          range:
            start: 0.0
            end: 10.0
      kind: ConstantOnly
      node:
        idx: 6
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 42
        version: 1
      typ: Vec3
      value: !Vec3
        value:
          value:
          - 0.5
          - 0.5
          - 0.5
          ui_data:
            tooltip: The radii of the ellipsoid.
            hidden: true
          is_colour: false
      kind: ConstantOnly
      node:
        idx: 6
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 43
        version: 1
      typ: Float
      value: !Float
        value:
          value: 0.25
          ui_data:
            tooltip: The height (y-axis).
            hidden: true
          range:
            start: 0.0
            end: 10.0
      kind: ConstantOnly
      node:
        idx: 6
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 44
        version: 1
      typ: Float
      value: !Float
        value:
          value: 0.5
          ui_data:
            tooltip: The radius of the sphere that is cut from the solid.
            hidden: true
          range:
            start: 0.0
            end: 10.0
      kind: ConstantOnly
      node:
        idx: 6
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 45
        version: 1
      typ: Float
      value: !Float
        value:
          value: 0.75
          ui_data:
            tooltip: |-
              The height (y-axis) of the center of the sphere
              that is cut from the solid, above solidRadius +
              hollowRadius, the result will be a standard
              sphere of radius solidRadius.
            hidden: true
          range:
            start: 0.0
            end: 10.0
      kind: ConstantOnly
      node:
        idx: 6
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 46
        version: 1
      typ: Float
      value: !Float
        value:
          value: 30.0
          ui_data:
            tooltip: |-
              The angle between the edge of the solid angle and the
              y-axis on [0-180] measured between the y-axis and wall
              of the solid angle.
            hidden: true
          range:
            start: 0.0
            end: 180.0
      kind: ConstantOnly
      node:
        idx: 6
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 47
        version: 1
      typ: Float
      value: !Float
        value:
          value: 0.5
          ui_data:
            tooltip: The width (x-axis).
            hidden: true
          range:
            start: 0.0
            end: 10.0
      kind: ConstantOnly
      node:
        idx: 6
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 48
        version: 1
      typ: Float
      value: !Float
        value:
          value: 0.75
          ui_data:
            tooltip: The depth (z-axis).
            hidden: true
          range:
            start: 0.0
            end: 10.0
      kind: ConstantOnly
      node:
        idx: 6
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 49
        version: 1
      typ: Float
      value: !Float
        value:
          value: 0.05
          ui_data:
            tooltip: The thickness of the walls.
            hidden: true
          range:
            start: 0.0
            end: 1.0
      kind: ConstantOnly
      node:
        idx: 6
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 50
        version: 1
      typ: Float
      value: !Float
        value:
          value: 0.05
          ui_data:
            tooltip: The radius of the corners of the rhombus' xy-plane parallel face.
            hidden: true
          range:
            start: 0.0
            end: 1.0
      kind: ConstantOnly
      node:
        idx: 6
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 51
        version: 1
      typ: Float
      value: !Float
        value:
          value: 0.5
          ui_data:
            tooltip: The equilateral triangles edge length (xy-plane).
            hidden: true
          range:
            start: 0.0
            end: 10.0
      kind: ConstantOnly
      node:
        idx: 6
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 52
        version: 1
      typ: Vec3
      value: !Vec3
        value:
          value:
          - 0.0
          - 0.0
          - 1.0
          ui_data:
            tooltip: The normal direction of the plane.
            hidden: true
          is_colour: false
      kind: ConstantOnly
      node:
        idx: 6
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 53
        version: 1
      typ: Float
      value: !Float
        value:
          value: 0.25
          ui_data:
            tooltip: The distance along the negative y-axis before entering the dome.
            hidden: true
          range:
            start: 0.0
            end: 10.0
      kind: ConstantOnly
      node:
        idx: 6
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 54
        version: 1
      typ: Float
      value: !Float
        value:
          value: 0.25
          ui_data:
            tooltip: The distance along the positive y-axis before entering the dome.
            hidden: true
          range:
            start: 0.0
            end: 10.0
      kind: ConstantOnly
      node:
        idx: 6
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 55
        version: 1
      typ: Float
      value: !Float
        value:
          value: 30.0
          ui_data:
            tooltip: |-
              The angle between the tip and base of the cone [0-90]
              measured between the y-axis and wall of the cone.
            hidden: true
          range:
            start: 0.0
            end: 90.0
      kind: ConstantOnly
      node:
        idx: 6
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 56
        version: 1
      typ: Float
      value: !Float
        value:
          value: 0.25
          ui_data:
            tooltip: The radius of the cone at y = -height/2.
            hidden: true
          range:
            start: 0.0
            end: 10.0
      kind: ConstantOnly
      node:
        idx: 6
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 57
        version: 1
      typ: Float
      value: !Float
        value:
          value: 0.125
          ui_data:
            tooltip: The radius of the cone at y = height/2.
            hidden: true
          range:
            start: 0.0
            end: 10.0
      kind: ConstantOnly
      node:
        idx: 6
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 58
        version: 1
      typ: Float
      value: !Float
        value:
          value: 0.3
          ui_data:
            tooltip: The radius (xy-plane) of the ring of the torus.
            hidden: false
          range:
            start: 0.0
            end: 10.0
      kind: ConstantOnly
      node:
        idx: 6
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 59
        version: 1
      typ: Float
      value: !Float
        value:
          value: 0.2
          ui_data:
            tooltip: The radius of the tube of the torus.
            hidden: false
          range:
            start: 0.0
            end: 5.0
      kind: ConstantOnly
      node:
        idx: 6
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 60
        version: 1
      typ: Float
      value: !Float
        value:
          value: 30.0
          ui_data:
            tooltip: |-
              The angle (xy-plane, symmetric about y-axis) to
              cap at, in the range [0-180.].
            hidden: true
          range:
            start: 0.0
            end: 180.0
      kind: ConstantOnly
      node:
        idx: 6
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 61
        version: 1
      typ: Float
      value: !Float
        value:
          value: 0.5
          ui_data:
            tooltip: |-
              The maximum distance along the x, y, and z axes.
              ie. The vertices are at +/-radial_extent on the x, y,
              and z axes.
            hidden: true
          range:
            start: 0.0
            end: 10.0
      kind: ConstantOnly
      node:
        idx: 6
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 62
        version: 1
      typ: Float
      value: !Float
        value:
          value: 8.0
          ui_data:
            tooltip: One greater than the axes of symmetry in the xy-plane.
            hidden: true
          range:
            start: 2.0
            end: 30.0
      kind: ConstantOnly
      node:
        idx: 6
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 63
        version: 1
      typ: UnsignedInteger
      value: !UnsignedInteger
        value:
          value: 10
          ui_data:
            tooltip: |-
              The number of iterations to compute, the higher this
              is, the slower it will be to compute, but the more
              detail the fractal will have.
            hidden: true
          range:
            start: 1
            end: 30
      kind: ConstantOnly
      node:
        idx: 6
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 64
        version: 1
      typ: Float
      value: !Float
        value:
          value: 4.0
          ui_data:
            tooltip: When the square radius has reached this length, stop iterating.
            hidden: true
          range:
            start: 1.0
            end: 9.0
      kind: ConstantOnly
      node:
        idx: 6
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 65
        version: 1
      typ: Float
      value: !Float
        value:
          value: -1.75
          ui_data:
            tooltip: |-
              The amount to scale the position between folds.
              Can be negative or positive.
            hidden: true
          range:
            start: -5.0
            end: 5.0
      kind: ConstantOnly
      node:
        idx: 6
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 66
        version: 1
      typ: Float
      value: !Float
        value:
          value: 0.001
          ui_data:
            tooltip: The minimum square radius to use when spherically folding.
            hidden: true
          range:
            start: 0.001
            end: 1.0
      kind: ConstantOnly
      node:
        idx: 6
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 67
        version: 1
      typ: Float
      value: !Float
        value:
          value: 0.8
          ui_data:
            tooltip: |-
              Clamp the position between +/- this value when
              performing the box fold. Higher values will result
              in a denser fractal.
            hidden: true
          range:
            start: 0.01
            end: 2.0
      kind: ConstantOnly
      node:
        idx: 6
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 68
        version: 1
      typ: Mat4
      value: !Mat4
        value:
          value:
          - 1.0
          - 0.0
          - 0.0
          - 0.0
          - 0.0
          - 1.0
          - 0.0
          - 0.0
          - 0.0
          - 0.0
          - 1.0
          - 0.0
          - 0.0
          - 0.0
          - 0.0
          - 1.0
          ui_data:
            tooltip: The world matrix/axis of the primitive.
            hidden: false
      kind: ConnectionOrConstant
      node:
        idx: 6
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 69
        version: 1
      typ: Float
      value: !Float
        value:
          value: 0.0
          ui_data:
            tooltip: |-
              The thickness of the walls of the shape, if
              the shape is hollow.
            hidden: false
          range:
            start: 0.0
            end: 1.0
      kind: ConstantOnly
      node:
        idx: 6
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 70
        version: 1
      typ: ComboBox
      value: !ComboBox
        value:
          selected: None
          options:
          - None
          - Finite
          - Infinite
          ui_data:
            tooltip: |-
              Repeat objects in the scene with no extra memory
              consumption. Note that if the repeated objects overlap
              some strange things can occur.
            hidden: false
      kind: ConstantOnly
      node:
        idx: 6
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 71
        version: 1
      typ: UVec3
      value: !UVec3
        value:
          value:
          - 0
          - 0
          - 0
          ui_data:
            tooltip: The number of repetitions along the negative x, y, and z axes.
            hidden: true
      kind: ConstantOnly
      node:
        idx: 6
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 72
        version: 1
      typ: UVec3
      value: !UVec3
        value:
          value:
          - 1
          - 1
          - 1
          ui_data:
            tooltip: The number of repetitions along the positive x, y, and z axes.
            hidden: true
      kind: ConstantOnly
      node:
        idx: 6
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 73
        version: 1
      typ: Vec3
      value: !Vec3
        value:
          value:
          - 1.0
          - 1.0
          - 1.0
          ui_data:
            tooltip: The spacing along each positive axis to repeat the objects.
            hidden: true
          is_colour: false
      kind: ConstantOnly
      node:
        idx: 6
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 74
        version: 1
      typ: Bool
      value: !Bool
        value:
          value: false
          ui_data:
            tooltip: |-
              If enabled, this object will act as a bounding volume
              for all its children. This means that until a ray hits
              the bounding volume, none of the child object's signed
              distance fields will be computed. This can vastly
              improve performance, especially when many complex
              objects are far from the camera. This option does
              not always play well with lighting effects that depend
              on the number of iterations in the computation such
              as 'ambient occlusion' and 'softened shadows' due
              to the variation near the surface of the bounding object.
            hidden: false
      kind: ConstantOnly
      node:
        idx: 6
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 75
        version: 1
      typ: ComboBox
      value: !ComboBox
        value:
          selected: Union
          options:
          - Union
          - Subtraction
          - Intersection
          - SmoothUnion
          - SmoothSubtraction
          - SmoothIntersection
          ui_data:
            tooltip: "The type of interaction this object will have with its children.\n\n\tUnion: All objects will appear as normal.\n\n\tSubtraction: This object will be subtracted from all of its\n\n\t\tchildren, leaving holes.\n\n\tIntersection: Only the region where this object and its\n\n\t\tchildren overlap will remain.\n\n\tSmooth Union: All children will smoothly blend together\n\n\t\twith this object according to the 'blend strength'.\n\n\tSmooth Subtraction:This object will be subtracted from all\n\n\t\tof its children,  leaving holes that are smoothed\n\n\t\taccording to the 'blend strength'.\n\n\tSmooth Intersection: Only the region where this object\n\n\t\tand its children overlap will remain, and the remaining\n\n\t\tregions will be smoothed according to the 'blend\n\n\t\tstrength'."
            hidden: false
      kind: ConstantOnly
      node:
        idx: 6
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 76
        version: 1
      typ: Float
      value: !Float
        value:
          value: 0.0
          ui_data:
            tooltip: The amount to blend with this primitive's children.
            hidden: true
          range:
            start: 0.0
            end: 1.0
      kind: ConstantOnly
      node:
        idx: 6
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 77
        version: 1
      typ: BVec3
      value: !BVec3
        value:
          value:
          - false
          - false
          - false
          ui_data:
            tooltip: Mirror along the x, y, and z axes.
            hidden: false
      kind: ConstantOnly
      node:
        idx: 6
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 78
        version: 1
      typ: Bool
      value: !Bool
        value:
          value: false
          ui_data:
            tooltip: |-
              If enabled, the object will be hollow, with a
              thickness of 'wall thickness'.
            hidden: false
      kind: ConstantOnly
      node:
        idx: 6
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 79
        version: 1
      typ: Float
      value: !Float
        value:
          value: 0.01
          ui_data:
            tooltip: |-
              The thickness of the walls of the shape, if
              the shape is hollow.
            hidden: true
          range:
            start: 0.001
            end: 1.0
      kind: ConstantOnly
      node:
        idx: 6
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 80
        version: 1
      typ: Bool
      value: !Bool
        value:
          value: false
          ui_data:
            tooltip: Enable the elongation of the object.
            hidden: false
      kind: ConstantOnly
      node:
        idx: 6
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 81
        version: 1
      typ: Vec3
      value: !Vec3
        value:
          value:
          - 0.0
          - 0.0
          - 0.0
          ui_data:
            tooltip: The elongation of the object along the respective axes.
            hidden: true
          is_colour: false
      kind: ConstantOnly
      node:
        idx: 6
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 82
        version: 1
      typ: Light
      value: !Light
        value: []
      kind: ConnectionOnly
      node:
        idx: 7
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 83
        version: 1
      typ: Mat4
      value: !Mat4
        value:
          value:
          - 1.0
          - 0.0
          - 0.0
          - 0.0
          - 0.0
          - 1.0
          - 0.0
          - 0.0
          - 0.0
          - 0.0
          - 1.0
          - 0.0
          - 0.0
          - 0.0
          - 0.0
          - 1.0
          ui_data:
            tooltip: "The world matrix to apply to the light (point and\ndirectional only).\n\n\tPoint: Will affect the position of the light.\n\n\tDirectional: Will affect the direction vector of the light."
            hidden: false
      kind: ConnectionOrConstant
      node:
        idx: 7
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 84
        version: 1
      typ: ComboBox
      value: !ComboBox
        value:
          selected: AmbientOcclusion
          options:
          - Directional
          - Point
          - Ambient
          - AmbientOcclusion
          ui_data:
            tooltip: "The type of non-physical light to create.\n\n\tPoint: A point light.\n\n\tDirectional: A directional light.\n\n\tAmbient: An ambient light (will be a uniform colour).\n\n\tAmbient Occlusion: Ambient occlusion."
            hidden: false
      kind: ConstantOnly
      node:
        idx: 7
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 85
        version: 1
      typ: Vec3
      value: !Vec3
        value:
          value:
          - 0.0
          - -1.0
          - 0.0
          ui_data:
            tooltip: The direction vector of the light.
            hidden: true
          is_colour: false
      kind: ConstantOnly
      node:
        idx: 7
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 86
        version: 1
      typ: Vec3
      value: !Vec3
        value:
          value:
          - 0.0
          - 1.0
          - 0.0
          ui_data:
            tooltip: The position of the point light.
            hidden: true
          is_colour: false
      kind: ConstantOnly
      node:
        idx: 7
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 87
        version: 1
      typ: UnsignedInteger
      value: !UnsignedInteger
        value:
          value: 1
          ui_data:
            tooltip: The number of iterations used to compute the occlusion.
            hidden: false
          range:
            start: 1
            end: 10
      kind: ConstantOnly
      node:
        idx: 7
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 88
        version: 1
      typ: Float
      value: !Float
        value:
          value: 1.0
          ui_data:
            tooltip: The intensity of the light.
            hidden: false
          range:
            start: 0.0
            end: 10.0
      kind: ConstantOnly
      node:
        idx: 7
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 89
        version: 1
      typ: UnsignedInteger
      value: !UnsignedInteger
        value:
          value: 2
          ui_data:
            tooltip: The exponent of the falloff (point lights only).
            hidden: false
          range:
            start: 0
            end: 4
      kind: ConstantOnly
      node:
        idx: 7
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 90
        version: 1
      typ: Vec3
      value: !Vec3
        value:
          value:
          - 1.0
          - 0.8000001
          - 0.5
          ui_data:
            tooltip: The light colour.
            hidden: false
          is_colour: true
      kind: ConstantOnly
      node:
        idx: 7
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 91
        version: 1
      typ: Float
      value: !Float
        value:
          value: 1.0
          ui_data:
            tooltip: The hardness of softened shadows.
            hidden: false
          range:
            start: 1.0
            end: 100.0
      kind: ConstantOnly
      node:
        idx: 7
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 92
        version: 1
      typ: Bool
      value: !Bool
        value:
          value: false
          ui_data:
            tooltip: |-
              If enabled, the shadows will be softened (directional
              and point lights only).
            hidden: false
      kind: ConstantOnly
      node:
        idx: 7
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 93
        version: 1
      typ: Vec3
      value: !Vec3
        value:
          value:
          - 1.0
          - 1.0
          - 1.0
          ui_data:
            tooltip: The diffuse colour of the material.
            hidden: false
          is_colour: true
      kind: ConstantOnly
      node:
        idx: 8
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 94
        version: 1
      typ: ProceduralTexture
      value: !ProceduralTexture
        value:
          texture_type: None
          scale:
          - 1.0
          - 1.0
          - 1.0
          - 1.0
          black_point: 0.0
          white_point: 1.0
          lift: 0.0
          gain: 1.0
          octaves: 10
          lacunarity: 2.0
          amplitude_gain: 0.75
          gamma: 1.0
          low_frequency_scale:
          - 1.0
          - 1.0
          - 1.0
          - 1.0
          high_frequency_scale:
          - 1.0
          - 1.0
          - 1.0
          - 1.0
          low_frequency_translation:
          - 0.0
          - 0.0
          - 0.0
          - 0.0
          high_frequency_translation:
          - 0.0
          - 0.0
          - 0.0
          - 0.0
          invert: false
      kind: ConnectionOnly
      node:
        idx: 8
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 95
        version: 1
      typ: Float
      value: !Float
        value:
          value: 0.0
          ui_data:
            tooltip: |-
              The probability that light will be specularly reflected
              when it interacts with this material.
            hidden: false
          range:
            start: 0.0
            end: 1.0
      kind: ConstantOnly
      node:
        idx: 8
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 96
        version: 1
      typ: ProceduralTexture
      value: !ProceduralTexture
        value:
          texture_type: None
          scale:
          - 1.0
          - 1.0
          - 1.0
          - 1.0
          black_point: 0.0
          white_point: 1.0
          lift: 0.0
          gain: 1.0
          octaves: 10
          lacunarity: 2.0
          amplitude_gain: 0.75
          gamma: 1.0
          low_frequency_scale:
          - 1.0
          - 1.0
          - 1.0
          - 1.0
          high_frequency_scale:
          - 1.0
          - 1.0
          - 1.0
          - 1.0
          low_frequency_translation:
          - 0.0
          - 0.0
          - 0.0
          - 0.0
          high_frequency_translation:
          - 0.0
          - 0.0
          - 0.0
          - 0.0
          invert: false
      kind: ConnectionOnly
      node:
        idx: 8
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 97
        version: 1
      typ: Float
      value: !Float
        value:
          value: 0.0
          ui_data:
            tooltip: The roughness of the material when specularly reflected.
            hidden: false
          range:
            start: 0.0
            end: 1.0
      kind: ConstantOnly
      node:
        idx: 8
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 98
        version: 1
      typ: ProceduralTexture
      value: !ProceduralTexture
        value:
          texture_type: None
          scale:
          - 1.0
          - 1.0
          - 1.0
          - 1.0
          black_point: 0.0
          white_point: 1.0
          lift: 0.0
          gain: 1.0
          octaves: 10
          lacunarity: 2.0
          amplitude_gain: 0.75
          gamma: 1.0
          low_frequency_scale:
          - 1.0
          - 1.0
          - 1.0
          - 1.0
          high_frequency_scale:
          - 1.0
          - 1.0
          - 1.0
          - 1.0
          low_frequency_translation:
          - 0.0
          - 0.0
          - 0.0
          - 0.0
          high_frequency_translation:
          - 0.0
          - 0.0
          - 0.0
          - 0.0
          invert: false
      kind: ConnectionOnly
      node:
        idx: 8
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 99
        version: 1
      typ: Vec3
      value: !Vec3
        value:
          value:
          - 1.0
          - 1.0
          - 1.0
          ui_data:
            tooltip: The specular colour of the material.
            hidden: false
          is_colour: true
      kind: ConstantOnly
      node:
        idx: 8
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 100
        version: 1
      typ: ProceduralTexture
      value: !ProceduralTexture
        value:
          texture_type: None
          scale:
          - 1.0
          - 1.0
          - 1.0
          - 1.0
          black_point: 0.0
          white_point: 1.0
          lift: 0.0
          gain: 1.0
          octaves: 10
          lacunarity: 2.0
          amplitude_gain: 0.75
          gamma: 1.0
          low_frequency_scale:
          - 1.0
          - 1.0
          - 1.0
          - 1.0
          high_frequency_scale:
          - 1.0
          - 1.0
          - 1.0
          - 1.0
          low_frequency_translation:
          - 0.0
          - 0.0
          - 0.0
          - 0.0
          high_frequency_translation:
          - 0.0
          - 0.0
          - 0.0
          - 0.0
          invert: false
      kind: ConnectionOnly
      node:
        idx: 8
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 101
        version: 1
      typ: Float
      value: !Float
        value:
          value: 0.0
          ui_data:
            tooltip: |-
              The probability that light will be transmitted through
              the material (before accounting for Fresnel) when it
              interacts with this material.
            hidden: false
          range:
            start: 0.0
            end: 1.0
      kind: ConstantOnly
      node:
        idx: 8
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 102
        version: 1
      typ: ProceduralTexture
      value: !ProceduralTexture
        value:
          texture_type: None
          scale:
          - 1.0
          - 1.0
          - 1.0
          - 1.0
          black_point: 0.0
          white_point: 1.0
          lift: 0.0
          gain: 1.0
          octaves: 10
          lacunarity: 2.0
          amplitude_gain: 0.75
          gamma: 1.0
          low_frequency_scale:
          - 1.0
          - 1.0
          - 1.0
          - 1.0
          high_frequency_scale:
          - 1.0
          - 1.0
          - 1.0
          - 1.0
          low_frequency_translation:
          - 0.0
          - 0.0
          - 0.0
          - 0.0
          high_frequency_translation:
          - 0.0
          - 0.0
          - 0.0
          - 0.0
          invert: false
      kind: ConnectionOnly
      node:
        idx: 8
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 103
        version: 1
      typ: Float
      value: !Float
        value:
          value: 0.0
          ui_data:
            tooltip: The roughness when transmitted through the material.
            hidden: false
          range:
            start: 0.0
            end: 1.0
      kind: ConstantOnly
      node:
        idx: 8
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 104
        version: 1
      typ: ProceduralTexture
      value: !ProceduralTexture
        value:
          texture_type: None
          scale:
          - 1.0
          - 1.0
          - 1.0
          - 1.0
          black_point: 0.0
          white_point: 1.0
          lift: 0.0
          gain: 1.0
          octaves: 10
          lacunarity: 2.0
          amplitude_gain: 0.75
          gamma: 1.0
          low_frequency_scale:
          - 1.0
          - 1.0
          - 1.0
          - 1.0
          high_frequency_scale:
          - 1.0
          - 1.0
          - 1.0
          - 1.0
          low_frequency_translation:
          - 0.0
          - 0.0
          - 0.0
          - 0.0
          high_frequency_translation:
          - 0.0
          - 0.0
          - 0.0
          - 0.0
          invert: false
      kind: ConnectionOnly
      node:
        idx: 8
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 105
        version: 1
      typ: Float
      value: !Float
        value:
          value: 0.0
          ui_data:
            tooltip: The extinction coefficient of the material.
            hidden: false
          range:
            start: 0.0
            end: 1.0
      kind: ConstantOnly
      node:
        idx: 8
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 106
        version: 1
      typ: Vec3
      value: !Vec3
        value:
          value:
          - 1.0
          - 1.0
          - 1.0
          ui_data:
            tooltip: The transmitted colour of the material.
            hidden: false
          is_colour: true
      kind: ConstantOnly
      node:
        idx: 8
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 107
        version: 1
      typ: ProceduralTexture
      value: !ProceduralTexture
        value:
          texture_type: None
          scale:
          - 1.0
          - 1.0
          - 1.0
          - 1.0
          black_point: 0.0
          white_point: 1.0
          lift: 0.0
          gain: 1.0
          octaves: 10
          lacunarity: 2.0
          amplitude_gain: 0.75
          gamma: 1.0
          low_frequency_scale:
          - 1.0
          - 1.0
          - 1.0
          - 1.0
          high_frequency_scale:
          - 1.0
          - 1.0
          - 1.0
          - 1.0
          low_frequency_translation:
          - 0.0
          - 0.0
          - 0.0
          - 0.0
          high_frequency_translation:
          - 0.0
          - 0.0
          - 0.0
          - 0.0
          invert: false
      kind: ConnectionOnly
      node:
        idx: 8
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 108
        version: 1
      typ: Float
      value: !Float
        value:
          value: 0.0
          ui_data:
            tooltip: The intensity of light that will be emitted from the material.
            hidden: false
          range:
            start: 0.0
            end: 100.0
      kind: ConstantOnly
      node:
        idx: 8
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 109
        version: 1
      typ: Vec3
      value: !Vec3
        value:
          value:
          - 1.0
          - 0.8000001
          - 0.5
          ui_data:
            tooltip: The emissive colour of the material.
            hidden: false
          is_colour: true
      kind: ConstantOnly
      node:
        idx: 8
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 110
        version: 1
      typ: ProceduralTexture
      value: !ProceduralTexture
        value:
          texture_type: None
          scale:
          - 1.0
          - 1.0
          - 1.0
          - 1.0
          black_point: 0.0
          white_point: 1.0
          lift: 0.0
          gain: 1.0
          octaves: 10
          lacunarity: 2.0
          amplitude_gain: 0.75
          gamma: 1.0
          low_frequency_scale:
          - 1.0
          - 1.0
          - 1.0
          - 1.0
          high_frequency_scale:
          - 1.0
          - 1.0
          - 1.0
          - 1.0
          low_frequency_translation:
          - 0.0
          - 0.0
          - 0.0
          - 0.0
          high_frequency_translation:
          - 0.0
          - 0.0
          - 0.0
          - 0.0
          invert: false
      kind: ConnectionOnly
      node:
        idx: 8
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 111
        version: 1
      typ: Float
      value: !Float
        value:
          value: 1.3
          ui_data:
            tooltip: The index of refraction of the material.
            hidden: false
          range:
            start: 0.1
            end: 5.0
      kind: ConstantOnly
      node:
        idx: 8
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 112
        version: 1
      typ: ProceduralTexture
      value: !ProceduralTexture
        value:
          texture_type: None
          scale:
          - 1.0
          - 1.0
          - 1.0
          - 1.0
          black_point: 0.0
          white_point: 1.0
          lift: 0.0
          gain: 1.0
          octaves: 10
          lacunarity: 2.0
          amplitude_gain: 0.75
          gamma: 1.0
          low_frequency_scale:
          - 1.0
          - 1.0
          - 1.0
          - 1.0
          high_frequency_scale:
          - 1.0
          - 1.0
          - 1.0
          - 1.0
          low_frequency_translation:
          - 0.0
          - 0.0
          - 0.0
          - 0.0
          high_frequency_translation:
          - 0.0
          - 0.0
          - 0.0
          - 0.0
          invert: false
      kind: ConnectionOnly
      node:
        idx: 8
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 113
        version: 1
      typ: Float
      value: !Float
        value:
          value: 0.0
          ui_data:
            tooltip: The scattering coefficient of the material.
            hidden: false
          range:
            start: 0.0
            end: 1.0
      kind: ConstantOnly
      node:
        idx: 8
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 114
        version: 1
      typ: Vec3
      value: !Vec3
        value:
          value:
          - 1.0
          - 1.0
          - 1.0
          ui_data:
            tooltip: The scattering colour of the material.
            hidden: false
          is_colour: true
      kind: ConstantOnly
      node:
        idx: 8
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 115
        version: 1
      typ: ProceduralTexture
      value: !ProceduralTexture
        value:
          texture_type: None
          scale:
          - 1.0
          - 1.0
          - 1.0
          - 1.0
          black_point: 0.0
          white_point: 1.0
          lift: 0.0
          gain: 1.0
          octaves: 10
          lacunarity: 2.0
          amplitude_gain: 0.75
          gamma: 1.0
          low_frequency_scale:
          - 1.0
          - 1.0
          - 1.0
          - 1.0
          high_frequency_scale:
          - 1.0
          - 1.0
          - 1.0
          - 1.0
          low_frequency_translation:
          - 0.0
          - 0.0
          - 0.0
          - 0.0
          high_frequency_translation:
          - 0.0
          - 0.0
          - 0.0
          - 0.0
          invert: false
      kind: ConnectionOnly
      node:
        idx: 8
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 116
        version: 1
      typ: ComboBox
      value: !ComboBox
        value:
          selected: Checkerboard
          options:
          - None
          - Grade
          - Checkerboard
          - FBMNoise
          - TurbulenceNoise
          ui_data:
            tooltip: The type of texture to use.
            hidden: false
      kind: ConstantOnly
      node:
        idx: 9
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 117
        version: 1
      typ: Vec4
      value: !Vec4
        value:
          value:
          - 1.0
          - 1.0
          - 1.0
          - 1.0
          ui_data:
            tooltip: The scale factor of the texture.
            hidden: false
          is_colour: false
      kind: ConstantOnly
      node:
        idx: 9
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 118
        version: 1
      typ: Float
      value: !Float
        value:
          value: 0.0
          ui_data:
            tooltip: The black point of the texture.
            hidden: false
          range:
            start: 0.0
            end: 1.0
      kind: ConstantOnly
      node:
        idx: 9
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 119
        version: 1
      typ: Float
      value: !Float
        value:
          value: 1.0
          ui_data:
            tooltip: The white point of the texture.
            hidden: false
          range:
            start: 0.0
            end: 1.0
      kind: ConstantOnly
      node:
        idx: 9
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 120
        version: 1
      typ: Float
      value: !Float
        value:
          value: 0.0
          ui_data:
            tooltip: The lift to apply to the texture.
            hidden: false
          range:
            start: 0.0
            end: 1.0
      kind: ConstantOnly
      node:
        idx: 9
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 121
        version: 1
      typ: Float
      value: !Float
        value:
          value: 1.0
          ui_data:
            tooltip: The gain to apply to the texture colour.
            hidden: false
          range:
            start: 0.0001
            end: 10.0
      kind: ConstantOnly
      node:
        idx: 9
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 122
        version: 1
      typ: UnsignedInteger
      value: !UnsignedInteger
        value:
          value: 10
          ui_data:
            tooltip: The number of different frequencies to superimpose.
            hidden: true
          range:
            start: 1
            end: 12
      kind: ConstantOnly
      node:
        idx: 9
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 123
        version: 1
      typ: Float
      value: !Float
        value:
          value: 2.0
          ui_data:
            tooltip: |-
              The lacunarity is the initial frequency of the noise,
              and the amount to scale the frequency for each octave.
            hidden: true
          range:
            start: 1.0
            end: 10.0
      kind: ConstantOnly
      node:
        idx: 9
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 124
        version: 1
      typ: Float
      value: !Float
        value:
          value: 0.75
          ui_data:
            tooltip: |-
              The gain to apply to the texture. This scales the
              noise amplitude between octaves.
            hidden: true
          range:
            start: 0.0
            end: 2.0
      kind: ConstantOnly
      node:
        idx: 9
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 125
        version: 1
      typ: Float
      value: !Float
        value:
          value: 1.0
          ui_data:
            tooltip: |-
              The gamma to apply to the texture. This is computed
              by raising the colour to the power of 1/gamma.
            hidden: false
          range:
            start: 0.01
            end: 2.0
      kind: ConstantOnly
      node:
        idx: 9
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 126
        version: 1
      typ: Vec4
      value: !Vec4
        value:
          value:
          - 1.0
          - 1.0
          - 1.0
          - 1.0
          ui_data:
            tooltip: The amount to scale lower frequencies between octaves.
            hidden: true
          is_colour: false
      kind: ConstantOnly
      node:
        idx: 9
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 127
        version: 1
      typ: Vec4
      value: !Vec4
        value:
          value:
          - 1.0
          - 1.0
          - 1.0
          - 1.0
          ui_data:
            tooltip: The amount to scale higher frequencies between octaves.
            hidden: true
          is_colour: false
      kind: ConstantOnly
      node:
        idx: 9
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 128
        version: 1
      typ: Vec4
      value: !Vec4
        value:
          value:
          - 0.0
          - 0.0
          - 0.0
          - 0.0
          ui_data:
            tooltip: The amount to translate lower frequencies between octaves.
            hidden: true
          is_colour: false
      kind: ConstantOnly
      node:
        idx: 9
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 129
        version: 1
      typ: Vec4
      value: !Vec4
        value:
          value:
          - 0.0
          - 0.0
          - 0.0
          - 0.0
          ui_data:
            tooltip: The amount to translate higher frequencies between octaves.
            hidden: true
          is_colour: false
      kind: ConstantOnly
      node:
        idx: 9
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 130
        version: 1
      typ: Bool
      value: !Bool
        value:
          value: false
          ui_data:
            tooltip: Invert the colour.
            hidden: false
      kind: ConstantOnly
      node:
        idx: 9
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 131
        version: 1
      typ: Mat4
      value: !Mat4
        value:
          value:
          - 1.0
          - 0.0
          - 0.0
          - 0.0
          - 0.0
          - 1.0
          - 0.0
          - 0.0
          - 0.0
          - 0.0
          - 1.0
          - 0.0
          - 0.0
          - 0.0
          - 0.0
          - 1.0
          ui_data:
            tooltip: The parent axis.
            hidden: false
      kind: ConnectionOrConstant
      node:
        idx: 10
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 132
        version: 1
      typ: Vec3
      value: !Vec3
        value:
          value:
          - 0.0
          - 0.0
          - 0.0
          ui_data:
            tooltip: The translation of this axis.
            hidden: false
          is_colour: false
      kind: ConstantOnly
      node:
        idx: 10
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 133
        version: 1
      typ: Vec3
      value: !Vec3
        value:
          value:
          - 0.0
          - 0.0
          - 0.0
          ui_data:
            tooltip: The rotation of this axis.
            hidden: false
          is_colour: false
      kind: ConstantOnly
      node:
        idx: 10
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 134
        version: 1
      typ: Float
      value: !Float
        value:
          value: 1.0
          ui_data:
            tooltip: "The uniform scale of this axis.\n\nWe use uniform scale because the signed distance \nfields cannot have their individual axes scaled."
            hidden: false
          range:
            start: 0.01
            end: 10.0
      kind: ConstantOnly
      node:
        idx: 10
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 135
        version: 1
      typ: Vec3
      value: !Vec3
        value:
          value:
          - 1.0
          - 1.0
          - 1.0
          ui_data:
            tooltip: The diffuse colour of the material.
            hidden: false
          is_colour: true
      kind: ConstantOnly
      node:
        idx: 11
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 136
        version: 1
      typ: ProceduralTexture
      value: !ProceduralTexture
        value:
          texture_type: None
          scale:
          - 1.0
          - 1.0
          - 1.0
          - 1.0
          black_point: 0.0
          white_point: 1.0
          lift: 0.0
          gain: 1.0
          octaves: 10
          lacunarity: 2.0
          amplitude_gain: 0.75
          gamma: 1.0
          low_frequency_scale:
          - 1.0
          - 1.0
          - 1.0
          - 1.0
          high_frequency_scale:
          - 1.0
          - 1.0
          - 1.0
          - 1.0
          low_frequency_translation:
          - 0.0
          - 0.0
          - 0.0
          - 0.0
          high_frequency_translation:
          - 0.0
          - 0.0
          - 0.0
          - 0.0
          invert: false
      kind: ConnectionOnly
      node:
        idx: 11
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 137
        version: 1
      typ: Float
      value: !Float
        value:
          value: 0.0
          ui_data:
            tooltip: |-
              The probability that light will be specularly reflected
              when it interacts with this material.
            hidden: false
          range:
            start: 0.0
            end: 1.0
      kind: ConstantOnly
      node:
        idx: 11
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 138
        version: 1
      typ: ProceduralTexture
      value: !ProceduralTexture
        value:
          texture_type: None
          scale:
          - 1.0
          - 1.0
          - 1.0
          - 1.0
          black_point: 0.0
          white_point: 1.0
          lift: 0.0
          gain: 1.0
          octaves: 10
          lacunarity: 2.0
          amplitude_gain: 0.75
          gamma: 1.0
          low_frequency_scale:
          - 1.0
          - 1.0
          - 1.0
          - 1.0
          high_frequency_scale:
          - 1.0
          - 1.0
          - 1.0
          - 1.0
          low_frequency_translation:
          - 0.0
          - 0.0
          - 0.0
          - 0.0
          high_frequency_translation:
          - 0.0
          - 0.0
          - 0.0
          - 0.0
          invert: false
      kind: ConnectionOnly
      node:
        idx: 11
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 139
        version: 1
      typ: Float
      value: !Float
        value:
          value: 0.0
          ui_data:
            tooltip: The roughness of the material when specularly reflected.
            hidden: false
          range:
            start: 0.0
            end: 1.0
      kind: ConstantOnly
      node:
        idx: 11
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 140
        version: 1
      typ: ProceduralTexture
      value: !ProceduralTexture
        value:
          texture_type: None
          scale:
          - 1.0
          - 1.0
          - 1.0
          - 1.0
          black_point: 0.0
          white_point: 1.0
          lift: 0.0
          gain: 1.0
          octaves: 10
          lacunarity: 2.0
          amplitude_gain: 0.75
          gamma: 1.0
          low_frequency_scale:
          - 1.0
          - 1.0
          - 1.0
          - 1.0
          high_frequency_scale:
          - 1.0
          - 1.0
          - 1.0
          - 1.0
          low_frequency_translation:
          - 0.0
          - 0.0
          - 0.0
          - 0.0
          high_frequency_translation:
          - 0.0
          - 0.0
          - 0.0
          - 0.0
          invert: false
      kind: ConnectionOnly
      node:
        idx: 11
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 141
        version: 1
      typ: Vec3
      value: !Vec3
        value:
          value:
          - 1.0
          - 1.0
          - 1.0
          ui_data:
            tooltip: The specular colour of the material.
            hidden: false
          is_colour: true
      kind: ConstantOnly
      node:
        idx: 11
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 142
        version: 1
      typ: ProceduralTexture
      value: !ProceduralTexture
        value:
          texture_type: None
          scale:
          - 1.0
          - 1.0
          - 1.0
          - 1.0
          black_point: 0.0
          white_point: 1.0
          lift: 0.0
          gain: 1.0
          octaves: 10
          lacunarity: 2.0
          amplitude_gain: 0.75
          gamma: 1.0
          low_frequency_scale:
          - 1.0
          - 1.0
          - 1.0
          - 1.0
          high_frequency_scale:
          - 1.0
          - 1.0
          - 1.0
          - 1.0
          low_frequency_translation:
          - 0.0
          - 0.0
          - 0.0
          - 0.0
          high_frequency_translation:
          - 0.0
          - 0.0
          - 0.0
          - 0.0
          invert: false
      kind: ConnectionOnly
      node:
        idx: 11
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 143
        version: 1
      typ: Float
      value: !Float
        value:
          value: 0.0
          ui_data:
            tooltip: |-
              The probability that light will be transmitted through
              the material (before accounting for Fresnel) when it
              interacts with this material.
            hidden: false
          range:
            start: 0.0
            end: 1.0
      kind: ConstantOnly
      node:
        idx: 11
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 144
        version: 1
      typ: ProceduralTexture
      value: !ProceduralTexture
        value:
          texture_type: None
          scale:
          - 1.0
          - 1.0
          - 1.0
          - 1.0
          black_point: 0.0
          white_point: 1.0
          lift: 0.0
          gain: 1.0
          octaves: 10
          lacunarity: 2.0
          amplitude_gain: 0.75
          gamma: 1.0
          low_frequency_scale:
          - 1.0
          - 1.0
          - 1.0
          - 1.0
          high_frequency_scale:
          - 1.0
          - 1.0
          - 1.0
          - 1.0
          low_frequency_translation:
          - 0.0
          - 0.0
          - 0.0
          - 0.0
          high_frequency_translation:
          - 0.0
          - 0.0
          - 0.0
          - 0.0
          invert: false
      kind: ConnectionOnly
      node:
        idx: 11
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 145
        version: 1
      typ: Float
      value: !Float
        value:
          value: 0.0
          ui_data:
            tooltip: The roughness when transmitted through the material.
            hidden: false
          range:
            start: 0.0
            end: 1.0
      kind: ConstantOnly
      node:
        idx: 11
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 146
        version: 1
      typ: ProceduralTexture
      value: !ProceduralTexture
        value:
          texture_type: None
          scale:
          - 1.0
          - 1.0
          - 1.0
          - 1.0
          black_point: 0.0
          white_point: 1.0
          lift: 0.0
          gain: 1.0
          octaves: 10
          lacunarity: 2.0
          amplitude_gain: 0.75
          gamma: 1.0
          low_frequency_scale:
          - 1.0
          - 1.0
          - 1.0
          - 1.0
          high_frequency_scale:
          - 1.0
          - 1.0
          - 1.0
          - 1.0
          low_frequency_translation:
          - 0.0
          - 0.0
          - 0.0
          - 0.0
          high_frequency_translation:
          - 0.0
          - 0.0
          - 0.0
          - 0.0
          invert: false
      kind: ConnectionOnly
      node:
        idx: 11
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 147
        version: 1
      typ: Float
      value: !Float
        value:
          value: 0.0
          ui_data:
            tooltip: The extinction coefficient of the material.
            hidden: false
          range:
            start: 0.0
            end: 1.0
      kind: ConstantOnly
      node:
        idx: 11
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 148
        version: 1
      typ: Vec3
      value: !Vec3
        value:
          value:
          - 1.0
          - 1.0
          - 1.0
          ui_data:
            tooltip: The transmitted colour of the material.
            hidden: false
          is_colour: true
      kind: ConstantOnly
      node:
        idx: 11
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 149
        version: 1
      typ: ProceduralTexture
      value: !ProceduralTexture
        value:
          texture_type: None
          scale:
          - 1.0
          - 1.0
          - 1.0
          - 1.0
          black_point: 0.0
          white_point: 1.0
          lift: 0.0
          gain: 1.0
          octaves: 10
          lacunarity: 2.0
          amplitude_gain: 0.75
          gamma: 1.0
          low_frequency_scale:
          - 1.0
          - 1.0
          - 1.0
          - 1.0
          high_frequency_scale:
          - 1.0
          - 1.0
          - 1.0
          - 1.0
          low_frequency_translation:
          - 0.0
          - 0.0
          - 0.0
          - 0.0
          high_frequency_translation:
          - 0.0
          - 0.0
          - 0.0
          - 0.0
          invert: false
      kind: ConnectionOnly
      node:
        idx: 11
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 150
        version: 1
      typ: Float
      value: !Float
        value:
          value: 0.0
          ui_data:
            tooltip: The intensity of light that will be emitted from the material.
            hidden: false
          range:
            start: 0.0
            end: 100.0
      kind: ConstantOnly
      node:
        idx: 11
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 151
        version: 1
      typ: Vec3
      value: !Vec3
        value:
          value:
          - 1.0
          - 0.8000001
          - 0.5
          ui_data:
            tooltip: The emissive colour of the material.
            hidden: false
          is_colour: true
      kind: ConstantOnly
      node:
        idx: 11
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 152
        version: 1
      typ: ProceduralTexture
      value: !ProceduralTexture
        value:
          texture_type: None
          scale:
          - 1.0
          - 1.0
          - 1.0
          - 1.0
          black_point: 0.0
          white_point: 1.0
          lift: 0.0
          gain: 1.0
          octaves: 10
          lacunarity: 2.0
          amplitude_gain: 0.75
          gamma: 1.0
          low_frequency_scale:
          - 1.0
          - 1.0
          - 1.0
          - 1.0
          high_frequency_scale:
          - 1.0
          - 1.0
          - 1.0
          - 1.0
          low_frequency_translation:
          - 0.0
          - 0.0
          - 0.0
          - 0.0
          high_frequency_translation:
          - 0.0
          - 0.0
          - 0.0
          - 0.0
          invert: false
      kind: ConnectionOnly
      node:
        idx: 11
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 153
        version: 1
      typ: Float
      value: !Float
        value:
          value: 1.3
          ui_data:
            tooltip: The index of refraction of the material.
            hidden: false
          range:
            start: 0.1
            end: 5.0
      kind: ConstantOnly
      node:
        idx: 11
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 154
        version: 1
      typ: ProceduralTexture
      value: !ProceduralTexture
        value:
          texture_type: None
          scale:
          - 1.0
          - 1.0
          - 1.0
          - 1.0
          black_point: 0.0
          white_point: 1.0
          lift: 0.0
          gain: 1.0
          octaves: 10
          lacunarity: 2.0
          amplitude_gain: 0.75
          gamma: 1.0
          low_frequency_scale:
          - 1.0
          - 1.0
          - 1.0
          - 1.0
          high_frequency_scale:
          - 1.0
          - 1.0
          - 1.0
          - 1.0
          low_frequency_translation:
          - 0.0
          - 0.0
          - 0.0
          - 0.0
          high_frequency_translation:
          - 0.0
          - 0.0
          - 0.0
          - 0.0
          invert: false
      kind: ConnectionOnly
      node:
        idx: 11
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 155
        version: 1
      typ: Float
      value: !Float
        value:
          value: 0.0
          ui_data:
            tooltip: The scattering coefficient of the material.
            hidden: false
          range:
            start: 0.0
            end: 1.0
      kind: ConstantOnly
      node:
        idx: 11
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 156
        version: 1
      typ: Vec3
      value: !Vec3
        value:
          value:
          - 1.0
          - 1.0
          - 1.0
          ui_data:
            tooltip: The scattering colour of the material.
            hidden: false
          is_colour: true
      kind: ConstantOnly
      node:
        idx: 11
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 157
        version: 1
      typ: ProceduralTexture
      value: !ProceduralTexture
        value:
          texture_type: None
          scale:
          - 1.0
          - 1.0
          - 1.0
          - 1.0
          black_point: 0.0
          white_point: 1.0
          lift: 0.0
          gain: 1.0
          octaves: 10
          lacunarity: 2.0
          amplitude_gain: 0.75
          gamma: 1.0
          low_frequency_scale:
          - 1.0
          - 1.0
          - 1.0
          - 1.0
          high_frequency_scale:
          - 1.0
          - 1.0
          - 1.0
          - 1.0
          low_frequency_translation:
          - 0.0
          - 0.0
          - 0.0
          - 0.0
          high_frequency_translation:
          - 0.0
          - 0.0
          - 0.0
          - 0.0
          invert: false
      kind: ConnectionOnly
      node:
        idx: 11
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 158
        version: 1
      typ: ComboBox
      value: !ComboBox
        value:
          selected: TurbulenceNoise
          options:
          - None
          - Grade
          - Checkerboard
          - FBMNoise
          - TurbulenceNoise
          ui_data:
            tooltip: The type of texture to use.
            hidden: false
      kind: ConstantOnly
      node:
        idx: 12
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 159
        version: 1
      typ: Vec4
      value: !Vec4
        value:
          value:
          - 1.0
          - 1.0
          - 1.0
          - 1.0
          ui_data:
            tooltip: The scale factor of the texture.
            hidden: false
          is_colour: false
      kind: ConstantOnly
      node:
        idx: 12
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 160
        version: 1
      typ: Float
      value: !Float
        value:
          value: 0.43
          ui_data:
            tooltip: The black point of the texture.
            hidden: false
          range:
            start: 0.0
            end: 1.0
      kind: ConstantOnly
      node:
        idx: 12
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 161
        version: 1
      typ: Float
      value: !Float
        value:
          value: 1.0
          ui_data:
            tooltip: The white point of the texture.
            hidden: false
          range:
            start: 0.0
            end: 1.0
      kind: ConstantOnly
      node:
        idx: 12
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 162
        version: 1
      typ: Float
      value: !Float
        value:
          value: 0.0
          ui_data:
            tooltip: The lift to apply to the texture.
            hidden: false
          range:
            start: 0.0
            end: 1.0
      kind: ConstantOnly
      node:
        idx: 12
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 163
        version: 1
      typ: Float
      value: !Float
        value:
          value: 1.0
          ui_data:
            tooltip: The gain to apply to the texture colour.
            hidden: false
          range:
            start: 0.0001
            end: 10.0
      kind: ConstantOnly
      node:
        idx: 12
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 164
        version: 1
      typ: UnsignedInteger
      value: !UnsignedInteger
        value:
          value: 10
          ui_data:
            tooltip: The number of different frequencies to superimpose.
            hidden: false
          range:
            start: 1
            end: 12
      kind: ConstantOnly
      node:
        idx: 12
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 165
        version: 1
      typ: Float
      value: !Float
        value:
          value: 2.0
          ui_data:
            tooltip: |-
              The lacunarity is the initial frequency of the noise,
              and the amount to scale the frequency for each octave.
            hidden: false
          range:
            start: 1.0
            end: 10.0
      kind: ConstantOnly
      node:
        idx: 12
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 166
        version: 1
      typ: Float
      value: !Float
        value:
          value: 0.75
          ui_data:
            tooltip: |-
              The gain to apply to the texture. This scales the
              noise amplitude between octaves.
            hidden: false
          range:
            start: 0.0
            end: 2.0
      kind: ConstantOnly
      node:
        idx: 12
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 167
        version: 1
      typ: Float
      value: !Float
        value:
          value: 1.0
          ui_data:
            tooltip: |-
              The gamma to apply to the texture. This is computed
              by raising the colour to the power of 1/gamma.
            hidden: false
          range:
            start: 0.01
            end: 2.0
      kind: ConstantOnly
      node:
        idx: 12
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 168
        version: 1
      typ: Vec4
      value: !Vec4
        value:
          value:
          - 1.0
          - 1.0
          - 1.0
          - 1.0
          ui_data:
            tooltip: The amount to scale lower frequencies between octaves.
            hidden: false
          is_colour: false
      kind: ConstantOnly
      node:
        idx: 12
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 169
        version: 1
      typ: Vec4
      value: !Vec4
        value:
          value:
          - 1.0
          - 1.0
          - 1.0
          - 1.0
          ui_data:
            tooltip: The amount to scale higher frequencies between octaves.
            hidden: false
          is_colour: false
      kind: ConstantOnly
      node:
        idx: 12
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 170
        version: 1
      typ: Vec4
      value: !Vec4
        value:
          value:
          - 0.0
          - 0.0
          - 0.0
          - 0.0
          ui_data:
            tooltip: The amount to translate lower frequencies between octaves.
            hidden: false
          is_colour: false
      kind: ConstantOnly
      node:
        idx: 12
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 171
        version: 1
      typ: Vec4
      value: !Vec4
        value:
          value:
          - 0.0
          - 0.0
          - 0.0
          - 0.0
          ui_data:
            tooltip: The amount to translate higher frequencies between octaves.
            hidden: false
          is_colour: false
      kind: ConstantOnly
      node:
        idx: 12
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  - value:
      id:
        idx: 172
        version: 1
      typ: Bool
      value: !Bool
        value:
          value: true
          ui_data:
            tooltip: Invert the colour.
            hidden: false
      kind: ConstantOnly
      node:
        idx: 12
        version: 1
      shown_inline: true
      _phantom: null
    version: 1
  outputs:
  - value: null
    version: 0
  - value:
      id:
        idx: 1
        version: 3
      node:
        idx: 1
        version: 3
      typ: Camera
      _phantom: null
    version: 3
  - value:
      id:
        idx: 2
        version: 3
      node:
        idx: 2
        version: 3
      typ: Mat4
      _phantom: null
    version: 3
  - value:
      id:
        idx: 3
        version: 3
      node:
        idx: 3
        version: 3
      typ: Mat4
      _phantom: null
    version: 3
  - value:
      id:
        idx: 4
        version: 1
      node:
        idx: 4
        version: 1
      typ: Scene
      _phantom: null
    version: 1
  - value:
      id:
        idx: 5
        version: 1
      node:
        idx: 5
        version: 1
      typ: RayMarcher
      _phantom: null
    version: 1
  - value:
      id:
        idx: 6
        version: 1
      node:
        idx: 6
        version: 1
      typ: Primitive
      _phantom: null
    version: 1
  - value:
      id:
        idx: 7
        version: 1
      node:
        idx: 7
        version: 1
      typ: Light
      _phantom: null
    version: 1
  - value:
      id:
        idx: 8
        version: 1
      node:
        idx: 8
        version: 1
      typ: Material
      _phantom: null
    version: 1
  - value:
      id:
        idx: 9
        version: 1
      node:
        idx: 9
        version: 1
      typ: ProceduralTexture
      _phantom: null
    version: 1
  - value:
      id:
        idx: 10
        version: 1
      node:
        idx: 10
        version: 1
      typ: Mat4
      _phantom: null
    version: 1
  - value:
      id:
        idx: 11
        version: 1
      node:
        idx: 11
        version: 1
      typ: Material
      _phantom: null
    version: 1
  - value:
      id:
        idx: 12
        version: 1
      node:
        idx: 12
        version: 1
      typ: ProceduralTexture
      _phantom: null
    version: 1
  connections:
  - value: null
    version: 0
  - value: null
    version: 0
  - value: null
    version: 0
  - value: null
    version: 0
  - value: null
    version: 0
  - value: null
    version: 0
  - value: null
    version: 0
  - value:
      idx: 4
      version: 1
    version: 3
  - value:
      idx: 8
      version: 1
    version: 3
  - value:
      idx: 7
      version: 1
    version: 3
  - value:
      idx: 6
      version: 1
    version: 3
  - value:
      idx: 1
      version: 3
    version: 3
  - value: null
    version: 0
  - value:
      idx: 2
      version: 3
    version: 3
  - value: null
    version: 0
  - value: null
    version: 0
  - value: null
    version: 0
  - value: null
    version: 0
  - value: null
    version: 0
  - value: null
    version: 0
  - value: null
    version: 0
  - value: null
    version: 0
  - value: null
    version: 0
  - value:
      idx: 3
      version: 3
    version: 3
  - value: null
    version: 0
  - value: null
    version: 0
  - value: null
    version: 0
  - value: null
    version: 0
  - value: null
    version: 0
  - value: null
    version: 0
  - value: null
    version: 0
  - value: null
    version: 0
  - value: null
    version: 0
  - value: null
    version: 0
  - value: null
    version: 0
  - value: null
    version: 0
  - value: null
    version: 0
  - value: null
    version: 0
  - value: null
    version: 0
  - value:
      idx: 11
      version: 1
    version: 1
  - value: null
    version: 0
  - value: null
    version: 0
  - value: null
    version: 0
  - value: null
    version: 0
  - value: null
    version: 0
  - value: null
    version: 0
  - value: null
    version: 0
  - value: null
    version: 0
  - value: null
    version: 0
  - value: null
    version: 0
  - value: null
    version: 0
  - value: null
    version: 0
  - value: null
    version: 0
  - value: null
    version: 0
  - value: null
    version: 0
  - value: null
    version: 0
  - value: null
    version: 0
  - value: null
    version: 0
  - value: null
    version: 0
  - value: null
    version: 0
  - value: null
    version: 0
  - value: null
    version: 0
  - value: null
    version: 0
  - value: null
    version: 0
  - value: null
    version: 0
  - value: null
    version: 0
  - value: null
    version: 0
  - value: null
    version: 0
  - value:
      idx: 10
      version: 1
    version: 1
  - value: null
    version: 0
  - value: null
    version: 0
  - value: null
    version: 0
  - value: null
    version: 0
  - value: null
    version: 0
  - value: null
    version: 0
  - value: null
    version: 0
  - value: null
    version: 0
  - value: null
    version: 0
  - value: null
    version: 0
  - value: null
    version: 0
  - value: null
    version: 0
  - value: null
    version: 0
  - value: null
    version: 0
  - value: null
    version: 0
  - value: null
    version: 0
  - value: null
    version: 0
  - value: null
    version: 0
  - value: null
    version: 0
  - value: null
    version: 0
  - value: null
    version: 0
  - value: null
    version: 0
  - value: null
    version: 0
  - value: null
    version: 0
  - value: null
    version: 0
  - value:
      idx: 9
      version: 1
    version: 1
  - value: null
    version: 0
  - value: null
    version: 0
  - value: null
    version: 0
  - value: null
    version: 0
  - value: null
    version: 0
  - value: null
    version: 0
  - value: null
    version: 0
  - value: null
    version: 0
  - value: null
    version: 0
  - value: null
    version: 0
  - value: null
    version: 0
  - value: null
    version: 0
  - value: null
    version: 0
  - value: null
    version: 0
  - value: null
    version: 0
  - value: null
    version: 0
  - value: null
    version: 0
  - value: null
    version: 0
  - value: null
    version: 0
  - value: null
    version: 0
  - value: null
    version: 0
  - value: null
    version: 0
  - value: null
    version: 0
  - value: null
    version: 0
  - value: null
    version: 0
  - value: null
    version: 0
  - value: null
    version: 0
  - value: null
    version: 0
  - value: null
    version: 0
  - value: null
    version: 0
  - value: null
    version: 0
  - value: null
    version: 0
  - value: null
    version: 0
  - value: null
    version: 0
  - value: null
    version: 0
  - value: null
    version: 0
  - value: null
    version: 0
  - value: null
    version: 0
  - value: null
    version: 0
  - value: null
    version: 0
  - value: null
    version: 0
  - value:
      idx: 12
      version: 1
    version: 1
node_order:
- idx: 3
  version: 3
- idx: 2
  version: 3
- idx: 1
  version: 3
- idx: 4
  version: 1
- idx: 5
  version: 1
- idx: 6
  version: 1
- idx: 7
  version: 1
- idx: 8
  version: 1
- idx: 9
  version: 1
- idx: 11
  version: 1
- idx: 10
  version: 1
- idx: 12
  version: 1
connection_in_progress: null
selected_nodes: []
copied_nodes: []
ongoing_box_selection: null
node_positions:
- value: null
  version: 0
- value:
    x: 1333.572
    y: -61.604614
  version: 3
- value:
    x: 1048.4265
    y: 13.734192
  version: 3
- value:
    x: 771.3644
    y: -50.594543
  version: 3
- value:
    x: 1726.4617
    y: 182.26099
  version: 1
- value:
    x: 1914.8278
    y: 117.808716
  version: 1
- value:
    x: 1341.0967
    y: 193.9943
  version: 1
- value:
    x: 1345.6233
    y: 581.8655
  version: 1
- value:
    x: 1312.601
    y: 907.5979
  version: 1
- value:
    x: 1032.8799
    y: 910.195
  version: 1
- value:
    x: 985.37054
    y: 664.424
  version: 1
- value:
    x: 925.5736
    y: 179.8096
  version: 1
- value:
    x: 553.25244
    y: 179.72858
  version: 1
node_finder: null
pan_zoom:
  pan:
    x: -345.88184
    y: -19.008545
  zoom: 0.9048365
  enable_zoom_from_out_of_rect: false
_user_state: null
